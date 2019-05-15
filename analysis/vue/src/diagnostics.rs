use crate::VueDatabase;
use code_analysis::{FileId, SourceId};
use code_grammar::{AstNode, SyntaxElement, SyntaxError, TextUnit, TextRange, WalkEvent};
use javascript_grammar::ast as js;
use typescript_analysis::ty::{infer_property_name, infer_expression_type, InterfaceTy, PropertyDef, Ty, TypeOf};
use typescript_grammar::ast as ts;
use typescript_grammar::syntax_kind::*;
use vue_grammar::ast as vue;
use vue_grammar::syntax_kind::*;
use rustc_hash::FxHashSet;

pub(crate) fn check(db: &impl VueDatabase, file_id: FileId) -> Vec<String> {
    let mut results = Vec::new();
    let path = db.file_relative_path(file_id);
    let src_id = db.file_source(file_id);
    match path.extension() {
        Some("js") | Some("ts") => {
            let module = db.typescript_ast(src_id);
            syntax_errors(&mut results, db, src_id, path.as_str(), TextUnit::default(), module.errors());
            return results;
        }
        Some("vue") => (),
        _ => {
            results.push("error(usage): expected file extension to be 'vue'".into());
            return results;
        }
    }

    // Parse the vue component
    let component = db.vue_ast(src_id);
    syntax_errors(&mut results, db, src_id, path.as_str(), TextUnit::default(), component.errors());

    // Check all expressions in the template have valid syntax
    let (expr_ranges, decl_ranges) = component.template()
        .map(collect_template_scope)
        .unwrap_or_default();
    let mut template_expressions = Vec::new();
    for range in expr_ranges {
        let raw_expr = &db.source_text(src_id)[range];
        let trim_offset = raw_expr.chars().take_while(|&c| c.is_whitespace()).count();
        let trim_range = range + TextUnit::from_usize(trim_offset);
        let trim_expr = raw_expr.trim();
        let (expr, _) = js::Expression::parse(trim_expr);
        let errors = expr.errors();
        if errors.is_empty() {
            template_expressions.push((expr, trim_range));
        } else {
            syntax_errors(&mut results, db, src_id, path.as_str(), trim_range.start(), errors);
        }
    }
    let mut template_declarations = Vec::new();
    for (scope_range, ident_range) in decl_ranges {
        let raw_expr = &db.source_text(src_id)[ident_range];
        template_declarations.push((raw_expr.to_string(), scope_range, ident_range));
    }

    // Find the component script
    let (source_id, _) = match db.component_script(src_id) {
        Some(id) => id,
        None => return results,
    };
    let root = db.typescript_ast(source_id);
    let script_block = component.script().unwrap().script().unwrap();
    let script_pos = script_block.syntax.range().start();
    {
        let errors = root.errors();
        if !errors.is_empty() {
            syntax_errors(&mut results, db, src_id, path.as_str(), script_pos, errors);
            return results
        }
    }

    let maybe_default_export = root.syntax.children().find_map(ts::ExportDefaultDeclaration::cast);
    let maybe_default_expr = maybe_default_export.and_then(|n| n.syntax.children().find_map(ts::Expression::cast));
    let maybe_options = maybe_default_expr.and_then(|expr| match expr.kind() {
        ts::ExpressionKind::CallExpression(call) => {
            let maybe_vue_extend = call.syntax.first_child().and_then(ts::MemberExpression::cast)?;
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
                .and_then(ts::ObjectExpression::cast)
        }
        ts::ExpressionKind::ObjectExpression(object) => Some(object),
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
    let vue_mixins = get_object_property(vue_options, "mixins")
        .map(AstNode::syntax)
        .and_then(ts::Expression::cast)
        .map(infer_expression_type);
    if vue_mixins.is_some() {
        // TODO: Lookup mixin.... and mix it in!
        return results;
    }
    let vue_data_property = get_object_property(vue_options, "data");
    let vue_data = vue_data_property
        .and_then(|expr| {
            match expr.kind() {
                ts::ExpressionKind::ObjectExpression(object) => Some(object.into()),
                ts::ExpressionKind::FunctionExpression(func) => Some(func)
                    .and_then(|f| f.body())
                    .and_then(|f| f.body().last())
                    .and_then(|f| ts::ReturnStatement::cast(&f.syntax).or_else(|| {
                        let pos = error_at(db, src_id, path.as_str(), script_pos + f.syntax.range().start());
                        results.push(format!("warn(internal): [{}] could not find `return ...` in component's `data` method", pos));
                        None
                    }))
                    .and_then(|f| f.argument()),
                ts::ExpressionKind::ArrowFunctionExpression(func) => Some(func)
                    .and_then(|f| f.body())
                    .and_then(|b| match b {
                        ts::ArrowFunctionBody::FunctionBody(block) => block.body().last()
                            .and_then(|f| ts::ReturnStatement::cast(&f.syntax))
                            .and_then(|f| f.argument()),
                        ts::ArrowFunctionBody::Expression(expr) => Some(expr),
                    }),
                _ => None,
            }
        })
        .map(infer_expression_type);

    if let Some(partial) = vue_data.as_ref().and_then(Ty::as_interface) {
        vm.merge(partial);
    }  else if let Some(data) = vue_data_property {
        let pos = error_at(db, src_id, path.as_str(), script_pos + data.syntax.range().start());
        results.push(format!("warn(internal): [{}] could not infer type of component's `data`", pos));
        return results;
    }
    let vue_computed = get_object_property(vue_options, "computed")
        .map(AstNode::syntax)
        .and_then(ts::Expression::cast)
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
        .and_then(ts::Expression::cast)
        .map(infer_expression_type);
    if let Some(partial) = vue_methods.as_ref().and_then(Ty::as_interface) {
        vm.merge(partial);
    }
    let vue_filters = get_object_property(vue_options, "filters")
        .map(AstNode::syntax)
        .and_then(ts::Expression::cast)
        .map(infer_expression_type)
        .and_then(|ty| match ty {
            Ty::Interface(ty) => Some(ty),
            _ => None,
        })
        .unwrap_or_else(|| InterfaceTy::default().into());

    // TODO: Move `vue_store` into some sort of `extensions` or `contrib` module
    let vue_apollo = get_object_property(vue_options, "store")
        .map(AstNode::syntax)
        .and_then(ts::Expression::cast)
        .map(infer_expression_type);
    if let Some(partial) = vue_apollo.as_ref().and_then(Ty::as_interface) {
        let mut tmp = partial.clone();
        tmp.properties = tmp.properties.into_iter().map(|prop| {
            // N.B. Since we don't infer function return types yet,
            //      convert computed properties to the _any_ type.
            PropertyDef { ident: prop.ident, type_: Ty::Any.into() }
        }).collect();
        vm.merge(&tmp);
    }

    // TODO: Move `vue_apollo` into some sort of `extensions` or `contrib` module
    let vue_apollo = get_object_property(vue_options, "apollo")
        .map(AstNode::syntax)
        .and_then(ts::Expression::cast)
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
    let root_id = db.file_source_root(file_id);
    let config = db.vue_config(root_id);
    let global = db.global_registry(root_id);
    let is_decl_in_template = |name: &str, range: TextRange| -> bool {
        template_declarations.iter().any(|(decl, scope, _item)| decl == name && range.is_subrange(scope))
    };
    for (expr, range) in template_expressions {
        for (ident, node) in find_captured_environment(&expr) {
            if !vm.properties.iter().any(|p| p.ident == ident) &&
                !ident.starts_with('$') &&
                !is_global(ident) &&
                !is_decl_in_template(ident, node.syntax.range() + range.start()) &&
                // TODO: Only perform these check if the expression is in a filter
                !vue_filters.properties.iter().any(|p| p.ident == ident) &&
                !config.global.filters.iter().any(|f| f == ident) &&
                !global.filters.contains(ident)
            {
                let pos = error_at(db, src_id, path.as_str(), range.start() + node.syntax.range().start());
                results.push(format!("error(vue): [{}] property `{}` is not defined on the component", pos, ident));
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

fn syntax_errors(
    results: &mut Vec<String>,
    db: &impl VueDatabase,
    src_id: SourceId,
    filename: &str,
    base: TextUnit,
    errors: Vec<SyntaxError>,
) {
    // TODO: include filename in error message
    // let filename = db.input_filename(src_id);
    let mut offset_set = FxHashSet::default();
    results.extend(errors.into_iter().filter_map(|err| {
        // Only display the first _syntax_ error for each line.
        // TODO: Maybe the parser should just detect this case and handle it during `finalize`?
        let offset = err.offset();
        if !offset_set.contains(&offset) {
            offset_set.insert(offset);
            let pos = error_at(db, src_id, filename, base + offset);
            Some(format!("error(syntax): [{}] {}", pos, err.message))
        } else {
            None
        }
    }));
}

fn error_at(db: &impl VueDatabase, src_id: SourceId, filename: &str, pos: TextUnit) -> String {
    let line_index = db.source_line_index(src_id);
    let line_col = line_index.line_col(pos);
    format!("{}:{}:{}", filename, line_col.line + 1, line_col.col_utf16 + 1)
}

/// Find all of the variables captured by a closure (or other expression),
/// returning a reference to each site that a captured variable is referenced.
///
/// This works by finding all "global" or undeclared variables in an expression,
/// including references to `this`; which has uses in other contexts outside
/// of closure expressions.
fn find_captured_environment(expr: &ts::Expression) -> Vec<(&str, &ts::Expression)> {
    let mut captures = Vec::new();
    collect_captures(expr, &[], &mut captures);
    captures
}


#[inline(always)]
fn maybe_collect_captures<'a>(
    expr: Option<&'a ts::Expression>,
    decls: &[&'a str],
    captures: &mut Vec<(&'a str, &'a ts::Expression)>,
) {
    if let Some(expr) = expr {
        collect_captures(expr, decls, captures)
    }
}
fn collect_captures<'a>(
    expr: &'a ts::Expression,
    decls: &[&'a str],
    captures: &mut Vec<(&'a str, &'a ts::Expression)>,
) {
    match expr.kind() {
        ts::ExpressionKind::Identifier(node) => {
            let name = node.name();
            if decls.iter().all(|decl| *decl != name) {
                captures.push((name, expr));
            }
        }
        ts::ExpressionKind::Literal(node) => {
            match node.kind() {
                // TODO: Detect expressions used inside templates
                ts::LiteralKind::Template(_token) => (),
                _ => (),
            }
        }
        ts::ExpressionKind::ThisExpression(_) => {
            captures.push(("this", expr));
        }
        ts::ExpressionKind::ArrayExpression(node) => {
            for el in node.elements() {
                collect_captures(el, decls, captures);
            }
        }
        ts::ExpressionKind::ObjectExpression(node) => {
            for prop in node.properties() {
                if prop.computed() {
                    maybe_collect_captures(prop.key(), decls, captures);
                }
                maybe_collect_captures(prop.value(), decls, captures);
            }
        },
        ts::ExpressionKind::FunctionExpression(node) => {
            let mut fn_decls = decls.to_vec();
            for param in node.params() {
                collect_pattern_decls_and_captures(param, &mut fn_decls, captures, true);
            }
            if let Some(block) = node.body() {
                collect_block_captures(block, &fn_decls, captures);
            }
        }
        ts::ExpressionKind::UnaryExpression(node) => {
            maybe_collect_captures(node.argument(), decls, captures);
        }
        ts::ExpressionKind::UpdateExpression(node) => {
            maybe_collect_captures(node.argument(), decls, captures);
        }
        ts::ExpressionKind::BinaryExpression(node) => {
            maybe_collect_captures(node.left(), decls, captures);
            maybe_collect_captures(node.right(), decls, captures);
        }
        ts::ExpressionKind::AssignmentExpression(node) => {
            let mut new_decls = decls.to_vec();
            if let Some(pattern) = node.left() {
                collect_pattern_decls_and_captures(pattern, &mut new_decls, captures, false);
            }
            maybe_collect_captures(node.right(), decls /* N.B. assignment can't capture its own decls! */, captures);
        }
        ts::ExpressionKind::LogicalExpression(node) => {
            maybe_collect_captures(node.left(), decls, captures);
            maybe_collect_captures(node.right(), decls, captures);
        }
        ts::ExpressionKind::MemberExpression(node) => {
            maybe_collect_captures(node.object(), decls, captures);
            if node.computed() {
                maybe_collect_captures(node.property(), decls, captures);
            }
        }
        ts::ExpressionKind::ConditionalExpression(node) => {
            maybe_collect_captures(node.test(), decls, captures);
            maybe_collect_captures(node.alternate(), decls, captures);
            maybe_collect_captures(node.consequent(), decls, captures);
        }
        ts::ExpressionKind::CallExpression(node) => {
            maybe_collect_captures(node.callee(), decls, captures);
            for arg in node.arguments() {
                collect_captures(arg, decls, captures);
            }
        }
        ts::ExpressionKind::NewExpression(node) => {
            maybe_collect_captures(node.callee(), decls, captures);
            for arg in node.arguments() {
                collect_captures(arg, decls, captures);
            }
        }
        ts::ExpressionKind::SequenceExpression(node) => {
            for expr in node.expressions() {
                collect_captures(expr, decls, captures);
            }
        }
        ts::ExpressionKind::ArrowFunctionExpression(node) => {
            let mut fn_decls = decls.to_vec();
            for param in node.params() {
                collect_pattern_decls_and_captures(param, &mut fn_decls, captures, true);
            }
            match node.body() {
                Some(ts::ArrowFunctionBody::FunctionBody(block)) => {
                    collect_block_captures(block, &fn_decls, captures);
                }
                Some(ts::ArrowFunctionBody::Expression(expr)) => {
                    collect_captures(expr, &fn_decls, captures);
                }
                None => (),
            }
        }
        ts::ExpressionKind::YieldExpression(node) => {
            maybe_collect_captures(node.argument(), decls, captures);
        }
        ts::ExpressionKind::TemplateLiteral(_node) => (), // TODO: Detect expressions used inside template literals
        ts::ExpressionKind::TaggedTemplateExpression(node) => {
            maybe_collect_captures(node.tag(), decls, captures);
            // TODO: Detect expressions used inside template literals
        }
        ts::ExpressionKind::ClassExpression(_node) => (), // TODO: Implement
        ts::ExpressionKind::MetaProperty(_node) => (), // TODO: Implement
        ts::ExpressionKind::AwaitExpression(node) => {
            maybe_collect_captures(node.argument(), decls, captures);
        }

        ts::ExpressionKind::TSAsExpression(_) |
        ts::ExpressionKind::TSNonNullExpression(_) => {
            unreachable!() // N.B. typescript isn't valid in the template
        }
    }
}
fn collect_block_captures<'a> (
    block: &'a ts::BlockStatement,
    decls: &[&'a str],
    captures: &mut Vec<(&'a str, &'a ts::Expression)>,
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
    stmt: &'a ts::Statement,
    decls: &mut Vec<&'a str>,
    captures: &mut Vec<(&'a str, &'a ts::Expression)>,
) {
    match stmt.kind() {
        ts::StatementKind::ExpressionStatement(node) => {
            maybe_collect_captures(node.expression(), decls, captures);
        }
        ts::StatementKind::BlockStatement(node) => {
            collect_block_captures(node, decls, captures);
        }
        ts::StatementKind::EmptyStatement(_) => (),
        ts::StatementKind::DebuggerStatement(_) => (),

        // TODO: See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/with
        ts::StatementKind::WithStatement(_) => unimplemented!(),

        ts::StatementKind::ReturnStatement(node) => {
            maybe_collect_captures(node.argument(), decls, captures);
        }
        ts::StatementKind::LabeledStatement(node) => {
            if let Some(stmt) = node.body() {
                collect_statement_decls_and_captures(stmt, decls, captures);
            }
        }
        ts::StatementKind::BreakStatement(_) => (),
        ts::StatementKind::ContinueStatement(_) => (),
        // IfStatement = IF_STATEMENT,
        // SwitchStatement = SWITCH_STATEMENT,
        ts::StatementKind::ThrowStatement(node) => {
            maybe_collect_captures(node.argument(), decls, captures);
        }
        // TryStatement = TRY_STATEMENT,
        // WhileStatement = WHILE_STATEMENT,
        // DoWhileStatement = DO_WHILE_STATEMENT,
        // ForStatement = FOR_STATEMENT,
        // ForInStatement = FOR_IN_STATEMENT,
        // ForOfStatement = FOR_OF_STATEMENT,
        ts::StatementKind::Declaration(decl) => {
            match decl.kind() {
                ts::DeclarationKind::FunctionDeclaration(node) => {
                    decls.push(node.id().name());
                    let mut fn_decls = decls.clone();
                    for param in node.params() {
                        collect_pattern_decls_and_captures(param, &mut fn_decls, captures, true);
                    }
                    if let Some(block) = node.body() {
                        collect_block_captures(block, &mut fn_decls, captures);
                    }
                }
                ts::DeclarationKind::VariableDeclaration(node) => {
                    for decl in node.declarations() {
                        if let Some(pattern) = decl.id() {
                            collect_pattern_decls_and_captures(pattern, decls, captures, true);
                        }
                        if let Some(expr) = decl.init() {
                            collect_captures(expr, decls, captures);
                        }
                    }
                }
                ts::DeclarationKind::ClassDeclaration(node) => {
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
    pat: &'a ts::Pattern,
    decls: &mut Vec<&'a str>,
    captures: &mut Vec<(&'a str, &'a ts::Expression)>,
    declaration: bool,
) {
    match pat.kind() {
        ts::PatternKind::Identifier(ident) => {
            if declaration {
                decls.push(ident.name());
            } else {
                captures.push((ident.name(), ident.into()));
            }
        }
        ts::PatternKind::MemberExpression(expr) => {
            maybe_collect_captures(expr.object(), decls, captures);
            if expr.computed() {
                maybe_collect_captures(expr.property(), decls, captures);
            }
        }
        ts::PatternKind::ObjectPattern(obj) => {
            for prop in obj.properties() {
                if let Some(pattern) = prop.value() {
                    collect_pattern_decls_and_captures(pattern, decls, captures, true);
                }
            }
        }
        ts::PatternKind::ArrayPattern(arr) => {
            for el in arr.elements() {
                if let Some(pattern) = el {
                    collect_pattern_decls_and_captures(pattern, decls, captures, true);
                }
            }
        }

        // FIXME: Implement other patterns...
        kind => {
            eprintln!("UNIMPLEMENTED: {:?} of {}", kind, pat.syntax.parent().and_then(ts::Node::cast).unwrap().type_());
            unimplemented!()
        }
    }
}

fn collect_template_scope(template: &vue::Template) -> (Vec<(TextRange)>, Vec<(TextRange, TextRange)>) {
    let mut expressions = Vec::new();
    let mut declarations = Vec::new();
    for visit in template.syntax.preorder_with_tokens() {
        let syn_elem = match visit {
            WalkEvent::Enter(syntax) => syntax,
            _ => continue,
        };
        match (syn_elem.kind(), syn_elem) {
            (ATTRIBUTE, SyntaxElement::Node(node)) => {
                let name = node.first_token().map(|tok| tok.text().as_str()).unwrap_or("");
                match name {
                    "slot-scope" => {
                        let value = node.children_with_tokens()
                            .skip_while(|syn| syn.kind() != EQ)
                            .skip(1) // eat `EQ`
                            .skip_while(|syn| syn.kind() == WS)
                            .next();
                        if let Some(SyntaxElement::Token(token)) = value {
                            if token.kind() != QUOTED {
                                continue; // N.B. not a valid `sloc-scope` value
                            }

                            // Parse the scope as a pattern
                            let text = token.text().as_str();
                            let range = token.range();
                            let trimmed = text[1 .. text.len() - 1].trim();
                            let trim_offset = text[1..].chars().take_while(|&c| c.is_whitespace()).count();
                            let (pattern, _) = js::Pattern::parse(trimmed);
                            if !pattern.errors().is_empty() {
                                continue;
                            }


                            let mut pat_decls = Vec::new();
                            let mut pat_captures = Vec::new();
                            collect_pattern_decls_and_captures(&pattern, &mut pat_decls, &mut pat_captures, true);

                            // TODO: Maybe also save _captures_ to expressions
                            // captures: &mut Vec<(&'a str, &'a ts::Expression)>
                            for decl in pat_decls {
                                if let Some((decl_offset, _)) = text.match_indices(decl).next() {
                                    let start = TextUnit::from_usize(range.start().to_usize() + trim_offset + decl_offset);
                                    let end = TextUnit::from_usize(start.to_usize() + decl.len());
                                    declarations.push((
                                        node.parent().unwrap().range(),
                                        TextRange::from_to(start, end)
                                    ));
                                }
                            }
                        }
                    }
                    "v-if" | "v-else-if" | "v-model" => {
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
                    "v-for" => {
                        let value = node.children_with_tokens()
                            .skip_while(|syn| syn.kind() != EQ)
                            .skip(1) // eat `EQ`
                            .skip_while(|syn| syn.kind() == WS)
                            .next();
                        if let Some(SyntaxElement::Token(token)) = value {
                            if token.kind() != QUOTED {
                                continue; // N.B. not a valid `v-for` value
                            }

                            // Define simple scanner
                            let text = token.text().as_str();
                            let range = token.range();
                            let mut pos = 1; // skip '"' character
                            let eat_whitespace = |pos: &mut usize| {
                                while text.len() > *pos && text.chars().nth(*pos).map(char::is_whitespace).unwrap_or(false) {
                                    *pos += 1;
                                }
                            };

                            // Collect declarations
                            eat_whitespace(&mut pos);
                            if text.chars().nth(pos) == Some('(') {
                                while text.len() > pos {
                                    eat_whitespace(&mut pos);
                                    if text.chars().nth(pos) == Some(')') {
                                        pos += 1; // eat R_PAREN
                                        break;
                                    }
                                    pos += 1; // eat L_PAREN or COMMA
                                    eat_whitespace(&mut pos);

                                    let len = text[pos..]
                                        .chars()
                                        .take_while(|&c| c != ',' && c != ')' && !c.is_whitespace())
                                        .count();
                                    let start = TextUnit::from_usize(range.start().to_usize() + pos);
                                    let end = TextUnit::from_usize(start.to_usize() + len);
                                    declarations.push((
                                        node.parent().unwrap().range(),
                                        TextRange::from_to(start, end),
                                    ));
                                    pos += len;
                                }
                            } else {
                                eat_whitespace(&mut pos);
                                let len = text[pos..]
                                    .chars()
                                    .take_while(|&c| c != ',' && c != ')' && !c.is_whitespace())
                                    .count();
                                let start = TextUnit::from_usize(range.start().to_usize() + pos);
                                let end = TextUnit::from_usize(start.to_usize() + len);
                                declarations.push((
                                    node.parent().unwrap().range(),
                                    TextRange::from_to(start, end),
                                ));
                                pos += len;
                            }

                            // Eat "in" keyword
                            eat_whitespace(&mut pos);
                            pos += text[pos..]
                                .chars()
                                .take_while(|&c| !c.is_whitespace())
                                .count();
                            eat_whitespace(&mut pos);

                            // The rest is an expression
                            let start = TextUnit::from_usize(range.start().to_usize() + pos);
                            let end = TextUnit::from_usize(range.end().to_usize() - 1);
                            expressions.push(TextRange::from_to(start, end));
                        }
                    }
                    _ => (),
                }
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
                let range = token.range();
                let start = TextUnit::from_usize(range.start().to_usize() + 2);
                let end = TextUnit::from_usize(range.end().to_usize() - 2);
                expressions.push(TextRange::from_to(start, end));
            }
            _ => (),
        }
    }
    (expressions, declarations)
}

fn get_object_property<'a>(obj: &'a ts::ObjectExpression, key: &str) -> Option<&'a ts::Expression> {
    obj.properties()
        .find(|prop| infer_property_name(prop).as_ref().map(|x| x.as_str()) == Some(key))
        .and_then(|prop| prop.value())
}

fn infer_props_types(props: &ts::Expression) -> Result<(InterfaceTy, Vec<String>), Vec<String>> {
    let mut object = InterfaceTy::default();
    let mut messages = Vec::new();
    match props.kind() {
        ts::ExpressionKind::ArrayExpression(arr) => {
            for el in arr.elements() {
                match el.kind() {
                    ts::ExpressionKind::Literal(lit) => {
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
        ts::ExpressionKind::ObjectExpression(obj) => {
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
                    ts::ExpressionKind::Identifier(ident) => {
                        match ident.syntax.first_token().map(|t| t.text().as_str()) {
                            Some("Array") => Ty::Union(vec![Ty::Array(Ty::Any.into()), Ty::Null, Ty::Undefined].into()),
                            Some("String") => Ty::Union(vec![Ty::String, Ty::Null, Ty::Undefined].into()),
                            Some("Object") => Ty::Union(vec![Ty::Object, Ty::Null, Ty::Undefined].into()),
                            Some("Boolean") => Ty::Union(vec![Ty::Boolean, Ty::Null, Ty::Undefined].into()),
                            _ => Ty::Hint(TypeOf::Null),
                        }
                    }
                    ts::ExpressionKind::ObjectExpression(prop_options) => {
                        let mut is_required = false;
                        if let Some(required) = get_object_property(prop_options, "required") {
                            let required_raw = ts::Literal::cast(&required.syntax)
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
                            .and_then(ts::Identifier::cast)
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

const GLOBALS: &[&str] = &[
    // Values
    "Infinity",
    "NaN",
    "undefined",
    "null",
    "globalThis",

    // Objects / constructors / modules
    "Object",
    "Function",
    "Boolean",
    "Symbol",
    "Error",
    "EvalError",
    "InternalError",
    "RangeError",
    "ReferenceError",
    "SyntaxError",
    "TypeError",
    "URIError",
    "Number",
    "BigInt",
    "Math",
    "Date",
    "String",
    "RegExp",
    "Array",
    "Map",
    "Set",
    "JSON",
    "Promise",
];
fn is_global(name: &str) -> bool {
    GLOBALS.into_iter().any(|&g| g == name)
}
