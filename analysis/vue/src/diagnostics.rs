use crate::db::RootDatabase;
use crate::parse::{FileLikeId, ParseDatabase, ScriptSource, SourceLanguage};
use crate::types::{infer_property_name, infer_object_expression_type, InterfaceTy, PropertyTy, Ty, TypeOf};
use grammar_utils::{AstNode, SyntaxElement, SyntaxError, TextUnit, TextRange, WalkEvent};
use html_grammar::ast as html;
use javascript_grammar::ast as js;
use javascript_grammar::syntax_kind::*;
use vue_grammar::ast as vue;
use vue_grammar::syntax_kind::*;
use rustc_hash::FxHashSet;

pub(crate) fn check(db: &RootDatabase, file_id: FileLikeId) -> Vec<String> {
    let mut results = Vec::new();
    match db.input_language(file_id) {
        Some(SourceLanguage::Vue) => (),

        // Emit syntax errors only for `.html` and `.js` files
        Some(SourceLanguage::Html) => {
            let line_index = db.input_line_index(file_id);
            let document = db.parse_html(file_id);
            syntax_errors(&mut results, &db, file_id, TextUnit::default(), document.errors());
            return results;
        }
        Some(SourceLanguage::Javascript) => {
            let line_index = db.input_line_index(file_id);
            let program = db.parse_javascript(file_id);
            syntax_errors(&mut results, &db, file_id, TextUnit::default(), program.errors());
            return results;
        }

        // TODO: Handle typescript
        Some(SourceLanguage::Typescript) => return results,

        None => {
            results.push("warn(internal): `vue-analyzer` does not recognize the file extension".into())
        }
    }

    // Parse the vue component
    let component = db.parse_vue(file_id);
    syntax_errors(&mut results, &db, file_id, TextUnit::default(), component.errors());

    // Check root vue component structure
    let templates = component.root_templates().collect::<Vec<_>>();
    if templates.len() > 1 {
        results.push("error(correctness): vue component should contain exactly one root template".into());
    }
    let scripts = component.root_scripts().collect::<Vec<_>>();
    if scripts.len() > 1 {
        results.push("error(pedantic): vue component should contain exactly one script".into());
    }

    // Check all expressions in the template have valid syntax
    let mut expressions = Vec::new();
    let expression_ranges = templates.into_iter().next().map(collect_expressions).unwrap_or_default();
    for range in expression_ranges {
        let raw_expr = &db.input_text(file_id)[range];
        let (expr, _) = js::Expression::parse(raw_expr);
        let errors = expr.errors();
        if errors.is_empty() {
            expressions.push(expr);
        } else {
            syntax_errors(&mut results, &db, file_id, expr.syntax.range().start(), errors);
        }
    }

    // Find the component script
    let source_map = db.source_map_vue(file_id);
    let script_node = match scripts.into_iter().next() {
        Some(node) => node,
        None => return results,
    };
    let script_block = script_node.syntax
        .children()
        .find_map(html::Script::cast)
        .unwrap();

    // TODO: Detect source language (e.g. handle `lang="ts"` attribute)
    let script_id = db.script_id(ScriptSource {
        ast_id: source_map.ast_id(script_block).with_file_id(file_id),
        language: SourceLanguage::Javascript,
    });
    let script = db.parse_javascript(script_id.into());
    {
        let errors = script.errors();
        if !errors.is_empty() {
            syntax_errors(&mut results, &db, file_id, script_block.syntax.range().start(), errors);
            return results
        }
    }

    let maybe_default_export = script.syntax.children().find_map(js::ExportDefaultDeclaration::cast);
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
    let vue_data = get_object_property(vue_options, "data")
        .and_then(|expr| {
            match expr.kind() {
                js::ExpressionKind::ObjectExpression(object) => Some(object),
                js::ExpressionKind::FunctionExpression(func) => Some(func)
                    .and_then(|f| f.syntax.last_child())
                    .and_then(js::BlockStatement::cast)
                    .and_then(|f| f.syntax.last_child())
                    .and_then(js::ReturnStatement::cast)
                    .and_then(|f| f.syntax.last_child())
                    .and_then(js::SequenceExpression::cast)
                    .and_then(|f| f.syntax.last_child())
                    .and_then(js::ObjectExpression::cast),
                _ => None,
            }
        })
        .map(infer_object_expression_type);
    if let Some(partial) = vue_data.as_ref().and_then(Ty::as_interface) {
        vm.merge(partial);
    }
    let vue_computed = get_object_property(vue_options, "computed")
        .map(AstNode::syntax)
        .and_then(js::ObjectExpression::cast)
        .map(infer_object_expression_type);
    if let Some(partial) = vue_computed.as_ref().and_then(Ty::as_interface) {
        let mut tmp = partial.clone();
        tmp.properties = tmp.properties.into_iter().map(|prop| {
            // N.B. Since we don't infer function return types yet,
            //      convert computed properties to the _any_ type.
            PropertyTy { ident: prop.ident, type_: Ty::Any.into() }
        }).collect();
        vm.merge(&partial);
    }
    let vue_methods = get_object_property(vue_options, "methods")
        .map(AstNode::syntax)
        .and_then(js::ObjectExpression::cast)
        .map(infer_object_expression_type);
    if let Some(partial) = vue_methods.as_ref().and_then(Ty::as_interface) {
        vm.merge(partial);
    }

    // TODO: Move `vue_apollo` into some sort of `extensions` or `contrib` module
    let vue_apollo = get_object_property(vue_options, "apollo")
        .map(AstNode::syntax)
        .and_then(js::ObjectExpression::cast)
        .map(infer_object_expression_type);
    if let Some(partial) = vue_apollo.as_ref().and_then(Ty::as_interface) {
        let mut tmp = partial.clone();
        tmp.properties = tmp.properties.into_iter().map(|prop| {
            // N.B. Since we don't infer function return types yet,
            //      convert computed properties to the _any_ type.
            PropertyTy { ident: prop.ident, type_: Ty::Any.into() }
        }).collect();

        // Other properties (notably `data`) take precedence over apollo props
        tmp.merge(&vm);
        vm = tmp;
    }

    eprintln!("Found: {:#?}", vm);

    // Check that all expressions in the template reference known vm properties

    // TODO: Implement this
    //results.push("info: checking template vm properties".into());
    for expression in expressions {

    }

    // ==== TODOs =====
    // 1. Check that all `bullet-case` and `CamelCase` DOM tags are define
    //    globally in the project (via `Vue.component`) or included as a `Component`.
    //
    // 2. Check the `this.{property_name}` references exist in Vue apollo functions
    //

    results
}

fn syntax_errors(results: &mut Vec<String>, db: &RootDatabase, file_id: FileLikeId, base: TextUnit, errors: Vec<SyntaxError>) {
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

fn error_line_col(db: &RootDatabase, file_id: FileLikeId, pos: TextUnit) -> String {
    let line_index = db.input_line_index(file_id);
    let line_col = line_index.line_col(pos);
    format!("line {}, col {}", line_col.line + 1, line_col.col_utf16 + 1)
}

fn collect_expressions(template: &vue::ComponentTemplate) -> Vec<TextRange> {
    let mut expressions = Vec::new();
    for visit in template.syntax.preorder_with_tokens() {
        let syn_elem = match visit {
            WalkEvent::Enter(syntax) => syntax,
            _ => continue,
        };

        match (syn_elem.kind(), syn_elem) {
            (ATTRIBUTE_KEY, SyntaxElement::Node(node)) => {
                if node.first_token().map(|tok| tok.kind()) == Some(L_SQUARE) {
                    if let Some(ident) = node.first_child() {
                        if ident.kind() != ERROR {
                            assert_eq!(ident.kind(), IDENTIFIER);
                            expressions.push(ident.range());
                        }
                    }
                }
            }
            (ATTRIBUTE_BINDING, SyntaxElement::Node(node)) => {
                if let Some(value) = node.last_child() {
                    if let Some(prev) = value.prev_sibling_or_token() {
                        if prev.kind() == EQ && value.kind() != ERROR {
                            expressions.push(value.range());
                        }
                    }
                }
            }
            (ATTRIBUTE_LISTENER, SyntaxElement::Node(node)) => {
                if let Some(value) = node.last_child() {
                    if let Some(prev) = value.prev_sibling_or_token() {
                        if prev.kind() == EQ && value.kind() != ERROR {
                            expressions.push(value.range());
                        }
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
                                    object.properties.push(PropertyTy { ident: ident.into(), type_: Ty::Any.into() });
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
                object.properties.push(PropertyTy { ident: ident.into(), type_: type_.into() });
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
