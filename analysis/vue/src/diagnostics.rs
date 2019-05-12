use crate::VueDatabase;
use code_analysis::{FileId, SourceDatabase, SourceId};
use code_grammar::{AstNode, SyntaxElement, SyntaxError, TextUnit, TextRange, WalkEvent};
use javascript_analysis::AstDatabase;
use javascript_analysis::ty::{infer_property_name, infer_expression_type, InterfaceTy, PropertyDef, Ty, TypeOf};
use javascript_grammar::ast as js;
use javascript_grammar::syntax_kind::*;
use vue_grammar::ast as vue;
use vue_grammar::syntax_kind::*;
use rustc_hash::FxHashSet;

pub(crate) fn check(db: &impl VueDatabase, file_id: FileId) -> Vec<String> {
    let mut results = Vec::new();
    let ext = db.file_extension(file_id);
    match ext.as_ref().map(|ext| ext.as_str()) {
        Some("vue") => (),
        _ => {
            results.push("error(usage): expected file extension to be 'vue'".into());
            return results;
        }
    }

    // Parse the vue component
    let file_id = db.file_source(file_id);
    let component = db.vue_ast(file_id);
    syntax_errors(&mut results, db, file_id, TextUnit::default(), component.errors());

    // Check all expressions in the template have valid syntax
    let mut expressions = Vec::new();
    let expression_ranges = component.template().map(collect_expressions).unwrap_or_default();
    for range in expression_ranges {
        let raw_expr = &db.source_text(file_id)[range];
        let (expr, _) = js::Expression::parse(raw_expr);
        let errors = expr.errors();
        if errors.is_empty() {
            expressions.push((expr, range));
        } else {
            syntax_errors(&mut results, db, file_id, expr.syntax.range().start(), errors);
        }
    }

    // Find the component script
    let (source_id, _) = match db.component_script(file_id) {
        Some(id) => id,
        None => return results,
    };
    let root = db.javascript_ast(source_id);
    {
        let errors = root.errors();
        if !errors.is_empty() {
            let script_block = component.script().unwrap().script().unwrap();
            syntax_errors(&mut results, db, file_id, script_block.syntax.range().start(), errors);
            return results
        }
    }

    let maybe_default_export = root.syntax.children().find_map(js::ExportDefaultDeclaration::cast);
    let maybe_default_expr = maybe_default_export.and_then(|n| n.syntax.children().find_map(js::Expression::cast));
    let maybe_options = maybe_default_expr.and_then(|expr| match expr.kind() {
        js::ExpressionKind::CallExpression(call) => {
            let maybe_vue_extend = call.syntax.first_child().and_then(js::MemberExpression::cast)?;
            let maybe_vue = maybe_vue_extend.syntax.first_token()?;
            let maybe_extend = maybe_vue_extend.syntax.last_token()?;
            if maybe_vue.text() != "Vue" || maybe_extend.text() != "extend" {
                return None;
            }
            let start_args = call.syntax.children_with_tokens().find(|c| c.kind() == L_PAREN)?;
            start_args.next_sibling_or_token()
                .and_then(|el| match el {
                    SyntaxElement::Node(node) => Some(node),
                    SyntaxElement::Token(_) => None,
                })
                .and_then(js::ObjectExpression::cast)
        }
        js::ExpressionKind::ObjectExpression(object) => Some(object),
        _ => None,
    });
    let vue_options = match maybe_options {
        Some(object) => object,
        None => return results,
    };

    // Compute the `vm` (ViewModel) properties/accessors.
    let mut vm = InterfaceTy::default();
    vm.typeof_ = Some(vec![TypeOf::Object].into());
    match get_object_property(vue_options, "props") {
        Some(options) => match infer_props_types(options) {
            Ok((partial, warnings)) => {
                results.extend(warnings);
                vm.merge(&partial);
            }
            Err(errors) => {
                results.extend(errors);
                return results;
            }
        },
        None => (),
    };
    let vue_data_property = get_object_property(vue_options, "data");
    let vue_data = vue_data_property
        .and_then(|expr| {
            match expr.kind() {
                js::ExpressionKind::ObjectExpression(object) => Some(object.into()),
                js::ExpressionKind::FunctionExpression(func) => Some(func)
                    .and_then(|f| f.body())
                    .and_then(|f| f.body().last())
                    .and_then(|f| js::ReturnStatement::cast(&f.syntax).or_else(|| {
                        results.push("warn(internal): could not find `return ...` in component's `data` method".into());
                        None
                    }))
                    .and_then(|f| f.argument()),
                js::ExpressionKind::ArrowFunctionExpression(func) => Some(func)
                    .and_then(|f| f.body())
                    .and_then(|b| match b {
                        js::ArrowFunctionBody::FunctionBody(block) => block.body().last()
                            .and_then(|f| js::ReturnStatement::cast(&f.syntax))
                            .and_then(|f| f.argument()),
                        js::ArrowFunctionBody::Expression(expr) => Some(expr),
                    }),
                _ => None,
            }
        })
        .map(infer_expression_type);

    if let Some(partial) = vue_data.as_ref().and_then(Ty::as_interface) {
        vm.merge(partial);
    }  else if vue_data_property.is_some() {
        results.push("warn(internal): could not infer type of component's `data`".into());
    }
    let vue_computed = get_object_property(vue_options, "computed")
        .map(AstNode::syntax)
        .and_then(js::Expression::cast)
        .map(infer_expression_type);
    if let Some(partial) = vue_computed.as_ref().and_then(Ty::as_interface) {
        let mut tmp = partial.clone();
        tmp.properties = tmp.properties.into_iter().map(|prop| {
            // N.B. Since we don't infer function return types yet,
            //      convert computed properties to the _any_ type.
            PropertyDef { ident: prop.ident, type_: Ty::Any.into() }
        }).collect();
        vm.merge(&partial);
    }
    let vue_methods = get_object_property(vue_options, "methods")
        .map(AstNode::syntax)
        .and_then(js::Expression::cast)
        .map(infer_expression_type);
    if let Some(partial) = vue_methods.as_ref().and_then(Ty::as_interface) {
        vm.merge(partial);
    }

    // TODO: Move `vue_apollo` into some sort of `extensions` or `contrib` module
    let vue_apollo = get_object_property(vue_options, "apollo")
        .map(AstNode::syntax)
        .and_then(js::Expression::cast)
        .map(infer_expression_type);
    if let Some(partial) = vue_apollo.as_ref().and_then(Ty::as_interface) {
        let mut tmp = partial.clone();
        tmp.properties = tmp.properties.into_iter().map(|prop| {
            // N.B. Since we don't infer function return types yet,
            //      convert computed properties to the _any_ type.
            PropertyDef { ident: prop.ident, type_: Ty::Any.into() }
        }).collect();

        // Other properties (notably `data`) take precedence over apollo props
        tmp.merge(&vm);
        vm = tmp;
    }

    // Check that all expressions in the template reference known vm properties
    for (expr, range) in expressions {
        for (ident, node) in find_captured_environment(&expr) {
            let exists_on_vm = vm.properties.iter().any(|p| p.ident == ident);
            if !exists_on_vm {
                let pos = error_line_col(db, file_id, range.start() + node.syntax.range().start());
                results.push(format!("error(vue): [{}] property `{}` is not defined on the component", pos, ident))
            }
        }
    }

    // ==== TODOs =====
    // 1. Check that all `bullet-case` and `CamelCase` DOM tags are define
    //    globally in the project (via `Vue.component`) or included as a `Component`.
    //
    // 2. Check the `this.{property_name}` references exist in Vue apollo functions
    //
    // 3. Check whether the methods and properties accessed in the DOM exist in the corresponding VM property's type
    //

    results
}

fn syntax_errors(results: &mut Vec<String>, db: &impl VueDatabase, file_id: SourceId, base: TextUnit, errors: Vec<SyntaxError>) {
    // TODO: include filename in error message
    // let filename = db.input_filename(file_id);
    let mut offset_set = FxHashSet::default();
    results.extend(errors.into_iter().filter_map(|err| {
        // Only display the first _syntax_ error for each line.
        // TODO: Maybe the parser should just detect this case and handle it during `finalize`?
        let offset = err.offset();
        if !offset_set.contains(&offset) {
            offset_set.insert(offset);
            let pos = error_line_col(db, file_id, base + offset);
            Some(format!("error(syntax): [{}] {}", pos, err.message))
        } else {
            None
        }
    }));
}

fn error_line_col(db: &impl VueDatabase, file_id: SourceId, pos: TextUnit) -> String {
    let line_index = db.source_line_index(file_id);
    let line_col = line_index.line_col(pos);
    format!("line {}, col {}", line_col.line + 1, line_col.col_utf16 + 1)
}

/// Find all of the variables captured by a closure (or other expression),
/// returning a reference to each site that a captured variable is referenced.
///
/// This works by finding all "global" or undeclared variables in an expression,
/// including references to `this`; which has uses in other contexts outside
/// of closure expressions.
fn find_captured_environment(expr: &js::Expression) -> Vec<(&str, &js::Expression)> {
    let mut captures = Vec::new();
    collect_captures(expr, &[], &mut captures);
    captures
}


fn collect_captures<'a>(
    expr: &'a js::Expression,
    decls: &[&'a str],
    captures: &mut Vec<(&'a str, &'a js::Expression)>,
) {
    match expr.kind() {
        js::ExpressionKind::Identifier(node) => {
            let name = node.name();
            if decls.iter().all(|decl| *decl != name) {
                captures.push((name, expr));
            }
        }
        js::ExpressionKind::Literal(node) => {
            match node.kind() {
                // TODO: Detect expressions used inside templates
                js::LiteralKind::Template(token) => (),
                _ => (),
            }
        }
        js::ExpressionKind::ThisExpression(node) => {
            captures.push(("this", expr));
        }
        js::ExpressionKind::ArrayExpression(node) => {
            for el in node.elements() {
                collect_captures(el, decls, captures);
            }
        }
        js::ExpressionKind::ObjectExpression(node) => {
            for prop in node.properties() {
                if prop.computed() {
                    collect_captures(prop.key().unwrap(), decls, captures);
                }
                collect_captures(prop.value().unwrap(), decls, captures);
            }
        },
        js::ExpressionKind::FunctionExpression(node) => {
            let mut fn_decls = decls.to_vec();
            for param in node.params() {
                collect_pattern_decls_and_captures(param, &mut fn_decls, captures, true);
            }
            collect_block_captures(node.body().unwrap(), &fn_decls, captures);
        }
        js::ExpressionKind::UnaryExpression(node) => {
            collect_captures(node.argument().unwrap(), decls, captures);
        }
        js::ExpressionKind::UpdateExpression(node) => {
            collect_captures(node.argument().unwrap(), decls, captures);
        }
        js::ExpressionKind::BinaryExpression(node) => {
            collect_captures(node.left().unwrap(), decls, captures);
            collect_captures(node.right().unwrap(), decls, captures);
        }
        js::ExpressionKind::AssignmentExpression(node) => {
            let mut new_decls = decls.to_vec();
            collect_pattern_decls_and_captures(node.left().unwrap(), &mut new_decls, captures, false);
            collect_captures(node.right().unwrap(), decls /* N.B. assignment can't capture its own decls! */, captures);
        }
        js::ExpressionKind::LogicalExpression(node) => {
            collect_captures(node.left().unwrap(), decls, captures);
            collect_captures(node.right().unwrap(), decls, captures);
        }
        js::ExpressionKind::MemberExpression(node) => {
            collect_captures(node.object().unwrap(), decls, captures);
            if node.computed() {
                collect_captures(node.property().unwrap(), decls, captures);
            }
        }
        js::ExpressionKind::ConditionalExpression(node) => {
            collect_captures(node.test().unwrap(), decls, captures);
            collect_captures(node.alternate().unwrap(), decls, captures);
            collect_captures(node.consequent().unwrap(), decls, captures);
        }
        js::ExpressionKind::CallExpression(node) => {
            collect_captures(node.callee().unwrap(), decls, captures);
            for arg in node.arguments() {
                collect_captures(arg, decls, captures);
            }
        }
        js::ExpressionKind::NewExpression(node) => {
            collect_captures(node.callee().unwrap(), decls, captures);
            for arg in node.arguments() {
                collect_captures(arg, decls, captures);
            }
        }
        js::ExpressionKind::SequenceExpression(node) => {
            for expr in node.expressions() {
                collect_captures(expr, decls, captures);
            }
        }
        js::ExpressionKind::ArrowFunctionExpression(node) => {
            let mut fn_decls = decls.to_vec();
            for param in node.params() {
                collect_pattern_decls_and_captures(param, &mut fn_decls, captures, true);
            }
            match node.body().unwrap() {
                js::ArrowFunctionBody::FunctionBody(block) => {
                    collect_block_captures(block, &fn_decls, captures);
                }
                js::ArrowFunctionBody::Expression(expr) => {
                    collect_captures(expr, &fn_decls, captures);
                }
            }
        }
        js::ExpressionKind::YieldExpression(node) => {
            collect_captures(node.argument().unwrap(), decls, captures);
        }
        js::ExpressionKind::TemplateLiteral(node) => (), // TODO: Detect expressions used inside template literals
        js::ExpressionKind::TaggedTemplateExpression(node) => {
            collect_captures(node.tag().unwrap(), decls, captures);
            // TODO: Detect expressions used inside template literals
        }
        js::ExpressionKind::ClassExpression(node) => (), // TODO: Implement
        js::ExpressionKind::MetaProperty(node) => (), // TODO: Implement
        js::ExpressionKind::AwaitExpression(node) => {
            collect_captures(node.argument().unwrap(), decls, captures);
        }
    }
}
fn collect_block_captures<'a> (
    block: &'a js::BlockStatement,
    decls: &[&'a str],
    captures: &mut Vec<(&'a str, &'a js::Expression)>,
) {
    let mut decls = decls.to_vec();
    for stmt in block.body() {
        // TODO: Handle "hoisting" declarations.
        //
        // For `class`, `function` and `const` declarations, their names are hosted
        // as if they were at the top of the lexical scope.
        //
        // With a `let` declaration it shadows outside variables that would otherwise
        // be in scope but it is an error to reference it in before it's declared
        // lexical location.
        //
        // With a `var` declaration it shadows outside variables that would otherwise
        // be in scope but it is `undefined` if it is referenced before it's declared
        // lexical location.
        //
        collect_statement_decls_and_captures(stmt, &mut decls, captures);
    }
}
fn collect_statement_decls_and_captures<'a>(
    stmt: &'a js::Statement,
    decls: &mut Vec<&'a str>,
    captures: &mut Vec<(&'a str, &'a js::Expression)>,
) {
    match stmt.kind() {
        js::StatementKind::ExpressionStatement(node) => {
            collect_captures(node.expression().unwrap(), decls, captures);
        }
        js::StatementKind::BlockStatement(node) => {
            collect_block_captures(node, decls, captures);
        }
        js::StatementKind::EmptyStatement(_) => (),
        js::StatementKind::DebuggerStatement(_) => (),

        // TODO: See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/with
        js::StatementKind::WithStatement(_) => unimplemented!(),

        js::StatementKind::ReturnStatement(node) => {
            collect_captures(node.argument().unwrap(), decls, captures);
        }
        js::StatementKind::LabeledStatement(node) => {
            collect_statement_decls_and_captures(node.body().unwrap(), decls, captures);
        }
        js::StatementKind::BreakStatement(_) => (),
        js::StatementKind::ContinueStatement(_) => (),
        // IfStatement = IF_STATEMENT,
        // SwitchStatement = SWITCH_STATEMENT,
        js::StatementKind::ThrowStatement(node) => {
            collect_captures(node.argument().unwrap(), decls, captures);
        }
        // TryStatement = TRY_STATEMENT,
        // WhileStatement = WHILE_STATEMENT,
        // DoWhileStatement = DO_WHILE_STATEMENT,
        // ForStatement = FOR_STATEMENT,
        // ForInStatement = FOR_IN_STATEMENT,
        // ForOfStatement = FOR_OF_STATEMENT,
        js::StatementKind::Declaration(decl) => {
            match decl.kind() {
                js::DeclarationKind::FunctionDeclaration(node) => {
                    decls.push(node.id().name());
                    let mut fn_decls = decls.clone();
                    for param in node.params() {
                        collect_pattern_decls_and_captures(param, &mut fn_decls, captures, true);
                    }
                    collect_block_captures(node.body().unwrap(), &mut fn_decls, captures);
                }
                js::DeclarationKind::VariableDeclaration(node) => {
                    for decl in node.declarations() {
                        collect_pattern_decls_and_captures(decl.id().unwrap(), decls, captures, true);
                        if let Some(expr) = decl.init() {
                            collect_captures(expr, decls, captures);
                        }
                    }
                }
                js::DeclarationKind::ClassDeclaration(node) => {
                    decls.push(node.id().name());
                    // TODO: Probably we need to recurse into the definition of the class here...
                }
            }
        }
        _ => {
            // TODO: Implement all the block variations (If, Switch, Try, While, DoWhile, For, ForIn, ForOf)
        }
    }
}
fn collect_pattern_decls_and_captures<'a>(
    pat: &'a js::Pattern,
    decls: &mut Vec<&'a str>,
    captures: &mut Vec<(&'a str, &'a js::Expression)>,
    declaration: bool,
) {
    match pat.kind() {
        js::PatternKind::Identifier(ident) => {
            if declaration {
                decls.push(ident.name());
            } else {
                captures.push((ident.name(), ident.into()));
            }
        }

        // TODO: Implement other patterns; with the current parser they should be `unreachable!()` but...
        _ => unimplemented!(),
    }
}

fn collect_expressions(template: &vue::Template) -> Vec<TextRange> {
    let mut expressions = Vec::new();
    for visit in template.syntax.preorder_with_tokens() {
        let syn_elem = match visit {
            WalkEvent::Enter(syntax) => syntax,
            _ => continue,
        };
        match (syn_elem.kind(), syn_elem) {
            (ATTRIBUTE, SyntaxElement::Node(node)) => {
                // TODO: Capture `v-model`, `v-for`, `v-if` and `v-else-if` attributes
            }
            (ATTRIBUTE_KEY, SyntaxElement::Node(node)) => {
                let computed_key = node.children_with_tokens()
                    .skip_while(|syn| syn.kind() != L_SQUARE)
                    .skip(1) // eat `L_SQUARE`
                    .skip_while(|syn| syn.kind() == WS)
                    .next();
                if let Some(ident) = computed_key {
                    if ident.kind() != ERROR {
                        assert_eq!(ident.kind(), IDENTIFIER);
                        expressions.push(ident.range());
                    }
                }
            }
            (ATTRIBUTE_BINDING, SyntaxElement::Node(node)) |
            (ATTRIBUTE_LISTENER, SyntaxElement::Node(node)) => {
                let value = node.children_with_tokens()
                    .skip_while(|syn| syn.kind() != EQ)
                    .skip(1) // eat `EQ`
                    .skip_while(|syn| syn.kind() == WS)
                    .next();
                if let Some(value) = value {
                    if value.kind() == QUOTED {
                        let range = value.range();
                        let start = TextUnit::from_usize(range.start().to_usize() + 1);
                        let end = TextUnit::from_usize(range.end().to_usize() - 1);
                        expressions.push(TextRange::from_to(start, end));
                    } else if value.kind() == IDENT {
                        expressions.push(value.range());
                    }
                }
            }
            (MUSTACHE, SyntaxElement::Token(token)) => {
                let mustache = token.text();
                let range = token.range();
                let start = TextUnit::from_usize(range.start().to_usize() + 2);
                let end = TextUnit::from_usize(range.end().to_usize() - 2);
                expressions.push(TextRange::from_to(start, end));
            }
            _ => (),
        }
    }
    expressions
}

fn get_object_property<'a>(obj: &'a js::ObjectExpression, key: &str) -> Option<&'a js::Expression> {
    obj.properties()
        .find(|prop| infer_property_name(prop).as_ref().map(|x| x.as_str()) == Some(key))
        .and_then(|prop| prop.value())
}

fn infer_props_types(props: &js::Expression) -> Result<(InterfaceTy, Vec<String>), Vec<String>> {
    let mut object = InterfaceTy::default();
    let mut messages = Vec::new();
    match props.kind() {
        js::ExpressionKind::ArrayExpression(arr) => {
            for el in arr.elements() {
                match el.kind() {
                    js::ExpressionKind::Literal(lit) => {
                        if let Some(str_lit) = lit.syntax.first_token() {
                            if str_lit.kind() == STRING_LITERAL {
                                let text = str_lit.text();
                                let ident = &text[1 .. text.len() - 1];
                                if ident.chars().all(|c| c.is_alphanumeric() || c == '_') {
                                    object.properties.push(PropertyDef { ident: ident.into(), type_: Ty::Any.into() });
                                } else {
                                    messages.push(format!("warn(style): vue `props` names should be valid identifiers, but found \"{}\"", text));
                                }
                                continue;
                            }
                        }
                    }
                    _ => (),
                }
                messages.push("error(correctness): vue `props` array must be an array of strings".into());
                return Err(messages);
            }
        }
        js::ExpressionKind::ObjectExpression(obj) => {
            for prop in obj.properties() {
                if prop.computed() {
                    messages.push("error(pedantic): vue `props` keys should not be computed, but got `[...]: ...`".into());
                    continue;
                }
                let ident = match infer_property_name(prop) {
                    Some(name) => name,
                    None => continue,
                };
                let type_ = match prop.value().unwrap().kind() {
                    js::ExpressionKind::Identifier(ident) => {
                        match ident.syntax.first_token().map(|t| t.text().as_str()) {
                            Some("Array") => Ty::Union(vec![Ty::Array(Ty::Any.into()), Ty::Null, Ty::Undefined].into()),
                            Some("String") => Ty::Union(vec![Ty::String, Ty::Null, Ty::Undefined].into()),
                            Some("Object") => Ty::Union(vec![Ty::Object, Ty::Null, Ty::Undefined].into()),
                            Some("Boolean") => Ty::Union(vec![Ty::Boolean, Ty::Null, Ty::Undefined].into()),
                            _ => Ty::Hint(TypeOf::Null),
                        }
                    }
                    js::ExpressionKind::ObjectExpression(prop_options) => {
                        let mut is_required = false;
                        if let Some(required) = get_object_property(prop_options, "required") {
                            let required_raw = js::Literal::cast(&required.syntax)
                                .and_then(|l| l.syntax.first_token())
                                .map(|t| t.text().as_str())
                                .unwrap();
                            match required_raw {
                                "true" => is_required = true,
                                "false" => is_required = false,
                                text =>  {
                                    messages.push(format!("error(pedantic): vue `prop.required` should be `true` or `false`, but got `{}`", text));
                                }
                            }
                        }
                        // NOTE: incorrect but convenient to assume a default implies non-null
                        let has_default = get_object_property(prop_options, "default").is_some();
                        let maybe_type = get_object_property(prop_options, "type")
                            .map(AstNode::syntax)
                            .and_then(js::Identifier::cast)
                            .map(AstNode::syntax)
                            .and_then(|x| x.first_token())
                            .map(|x| x.text().as_str());
                        let type_ = match maybe_type {
                            Some("Array") => Ty::Array(Ty::Any.into()),
                            Some("String") => Ty::String,
                            Some("Object") => Ty::Object,
                            Some("Boolean") => Ty::Boolean,
                            _ => Ty::Any,
                        };
                        if is_required || has_default {
                            type_
                        } else {
                            Ty::Union(vec![type_, Ty::Null, Ty::Undefined].into())
                        }
                    }
                    _ => Ty::Any,
                };
                object.properties.push(PropertyDef { ident: ident.into(), type_: type_.into() });
            }
        }
        _ => {
            messages.push("error(pedantic): vue `props` must be an object or an array".into());
            return Err(messages);
        }
    }
    object.typeof_ = Some(vec![TypeOf::Object].into());
    Ok((object, messages))
}
