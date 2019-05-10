use crate::db::RootDatabase;
use crate::parse::{FileLikeId, ParseDatabase, ScriptSource, SourceLanguage};
use rustc_hash::FxHashSet;
use grammar_utils::{AstNode, SyntaxElement, SyntaxError, TextUnit, TextRange, WalkEvent};
use html_grammar::ast as html;
use javascript_grammar::ast as js;
use javascript_grammar::syntax_kind::*;
use vue_grammar::ast as vue;
use vue_grammar::syntax_kind::*;

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
    let mut vm_properties = Vec::new();
    // TODO: Build proper type inference to replace this spaghetti
    // TODO: Also probably just build proper AST accessors like `Property::key`
    let vue_options = vue_options.properties().collect::<Vec<_>>();
    let vue_props = vue_options.iter()
        .find(|prop| property_name(prop).as_ref().map(|x| x.as_str()) == Some("props"))
        .and_then(|prop| prop.value());
    let vue_data = vue_options.iter()
        .find(|prop| property_name(prop).as_ref().map(|x| x.as_str()) == Some("data"))
        .and_then(|prop| prop.value())
        .and_then(|expr| match expr.kind() {
            js::ExpressionKind::ObjectExpression(object) => Some(object),
            js::ExpressionKind::FunctionExpression(func) => Some(func)
                .and_then(|f| f.syntax.last_child())
                .and_then(js::BlockStatement::cast)
                .and_then(|f| f.syntax.last_child())
                .and_then(js::ReturnStatement::cast)
                .and_then(|f| f.syntax.last_child())
                .and_then(js::ObjectExpression::cast),
            _ => None,
        });
    let vue_computed = vue_options.iter()
        .find(|prop| property_name(prop).as_ref().map(|x| x.as_str()) == Some("computed"))
        .and_then(|prop| prop.value()).map(AstNode::syntax)
        .and_then(js::ObjectExpression::cast);
    let vue_methods = vue_options.iter()
        .find(|prop| property_name(prop).as_ref().map(|x| x.as_str()) == Some("methods"))
        .and_then(|prop| prop.value()).map(AstNode::syntax)
        .and_then(js::ObjectExpression::cast);

    // TODO: Move `vue_apollo` into some sort of `extensions` or `contrib` module
    let vue_apollo = vue_options.iter()
        .find(|prop| property_name(prop).as_ref().map(|x| x.as_str()) == Some("apollo"))
        .and_then(|prop| prop.value()).map(AstNode::syntax)
        .and_then(js::ObjectExpression::cast);

    // Compute "partial" type for rpops
    if let Some(vue_props) = vue_props.map(AstNode::syntax) {
        if let Some(arr) = js::ArrayExpression::cast(vue_props) {
            for el in arr.elements() {
                match el.kind() {
                    js::ExpressionKind::Literal(lit) => {
                        if let Some(str_lit) = lit.syntax.first_token() {
                            if str_lit.kind() == STRING_LITERAL {
                                let raw = str_lit.text();
                                let text = &raw[1 .. raw.len() - 1];
                                if text.chars().all(|c| c.is_alphanumeric() || c == '_') {
                                    vm_properties.push((text.to_string(), ("any", true)));
                                } else {
                                    results.push(format!("warn(style): vue `props` names should be valid identifiers, but found \"{}\"", text));
                                }
                                continue;
                            }
                        }
                    }
                    _ => (),
                }
                results.push("error(correctness): vue `props` array must be an array of strings".into());
                return results; // quit immediately; we probably can't figure out the vm type correctly
            }
        } else if let Some(obj) = js::ObjectExpression::cast(vue_props) {
            for prop in obj.properties() {
                if prop.computed() {
                    results.push("error(pedantic): vue `props` keys should not be computed, but got `[...]: ...`".into());
                    continue;
                }

                let type_ = match prop.value().unwrap().kind() {
                    js::ExpressionKind::Identifier(ident) => {
                        match ident.syntax.first_token().map(|t| t.text().as_str()) {
                            Some("Array") => ("array", true /* or_undefined: true */), // TODO: Build a proper type structure
                            Some("String") => ("string", true),
                            Some("Object") => ("object", true),
                            Some("Boolean") => ("bool", true),
                            _ => ("any", false),
                        }
                    }
                    js::ExpressionKind::ObjectExpression(prop_obj) => {
                        let maybe_required = prop_obj.properties()
                            .find(|prop| property_name(prop).as_ref().map(|x| x.as_str()) == Some("required"))
                            .and_then(|prop| prop.value());
                        let mut is_required = false;
                        if let Some(required) = maybe_required {
                            let required_raw = js::Literal::cast(&required.syntax)
                                .and_then(|l| l.syntax.first_token())
                                .map(|t| t.text().as_str())
                                .unwrap();
                            match required_raw {
                                "true" => is_required = true,
                                "false" => is_required = false,
                                text =>  {
                                    results.push(format!("error(pedantic): vue `prop.required` should be `true` or `false`, but got `{}`", text));
                                }
                            }
                        }
                        let maybe_type = prop_obj.properties()
                            .find(|prop| property_name(prop).as_ref().map(|x| x.as_str()) == Some("type"))
                            .and_then(js::Property::value).map(AstNode::syntax)
                            .and_then(js::Identifier::cast).map(AstNode::syntax)
                            .and_then(|x| x.first_token())
                            .map(|x| x.text().as_str());
                        let type_ = match maybe_type {
                            Some("Array") => "array",
                            Some("String") => "string",
                            Some("Object") => "object",
                            Some("Boolean") => "bool",
                            _ => "any"
                        };
                        (type_, !is_required)
                    }
                    _ => ("any", false),
                };
                vm_properties.push((property_name(prop).unwrap(), type_));
            }
        } else {
            results.push("error(pedantic): vue `props` must be an object or an array".into());
            return results;
        }
    }

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

/// The string value of the key, if it not computed and is an identifier or literal
///
/// These example would return a value:
///
///     `0`, `hello`, `"goodbye"`, `false`
///
/// But these examples would not:
///
///     `0x55`, `["evaluated_literal"]`, `[1 + 2]`, `['Hello ${name}']`,
///
fn property_name(prop: &js::Property) -> Option<String> {
    if prop.computed() {
        return None;
    }

    let key = prop.key()?;
    match key.kind() {
        js::ExpressionKind::Identifier(ident) => {
            let token = ident.syntax.first_token()?;
            Some(token.text().to_string())
        }
        js::ExpressionKind::Literal(lit) => {
            let lit = lit.syntax.first_token()?;
            if lit.kind() == STRING_LITERAL {
                let raw = lit.text();
                unescape::unescape(&raw[1 .. raw.len() - 1])
            } else if lit.kind() == NUMBER_LITERAL {
                let num: f64 = lit.text().parse().ok()?;
                Some(num.to_string())
            } else {
                Some(lit.text().to_string())
            }
        }
        js::ExpressionKind::FunctionExpression(func) => {
            let ident = func.syntax.children().find_map(js::Identifier::cast)?;
            let token = ident.syntax.first_token()?;
            Some(token.text().to_string())
        }
        _ => None,
    }
}
