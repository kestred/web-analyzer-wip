use crate::database::RootDatabase;
use crate::parse::{InputId, ParseDatabase, SourceLanguage};
use rustc_hash::FxHashSet;
use analysis_utils::LineIndex;
use grammar_utils::{SyntaxElement, SyntaxError, TextUnit, WalkEvent};
use javascript_grammar::ast as js;
use vue_grammar::ast as vue;
use vue_grammar::syntax_kind::*;

pub(crate) fn check(db: &RootDatabase, input_id: InputId) -> Vec<String> {
    let mut results = Vec::new();
    match db.input_language(input_id) {
        Some(SourceLanguage::Vue) => (),

        // Emit syntax errors only for `.html` and `.js` files
        Some(SourceLanguage::Html) => {
            let line_index = db.input_line_index(input_id);
            let document = db.parse_html(input_id);
            syntax_errors(&mut results, &line_index, TextUnit::default(), document.errors());
            return results;
        }
        Some(SourceLanguage::Javascript) => {
            let line_index = db.input_line_index(input_id);
            let program = db.parse_javascript(input_id);
            syntax_errors(&mut results, &line_index, TextUnit::default(), program.errors());
            return results;
        }

        // TODO: Handle typescript
        Some(SourceLanguage::Typescript) => return results,

        None => {
            results.push("warn: `vue-analyzer` does not recognize the file extension".into())
        }
    }

    // Parse the vue component
    let component = db.parse_vue(input_id);
    syntax_errors(&mut results, &db, &line_index, TextUnit::default(), component.errors());

    // Check root vue component structure
    let templates = component.children().filter_map(vue::ComponentTemplate::cast).collect();
    if templates.len() > 1 {
        results.push("error: vue component should contain exactly one root template");
    }
    let scripts = component.children().filter_map(vue::ComponentScript::cast).collect();
    if scripts.len() > 1 {
        results.push("error: vue component should contain exactly one script");
    }

    // Check all expressions in the template have valid syntax
    let mut expressions = Vec::new();
    let raw_expressions = templates.iter().next().map(collect_expressions).unwrap_or_default();
    for raw_text in raw_expressions {
        let (expr, _) = js::Expression::parse(&*raw_text);
        let errors = expr.errors();
        if errors.is_empty() {
            expressions.push(expr);
        } else {
            syntax_errors(&mut results, &db, expr.syntax.range().start(), errors);
        }
    }

    // Find the component options
    let maybe_script = scripts.iter().next().and_then(|node| {
        if Some(block) = node.children().find_map(html::ScriptBlock::cast) {
            let (script, _) = js::Program::parse(&*node.text());
            let errors = expr.errors();
            if errors.is_empty() {
                return Some(script);
            }
            syntax_errors(&mut results, &db, block.syntax.range().start(), errors);
        }
        None
    };
    let maybe_default_export = maybe_script
        .and_then(|n| n.children().find_map(js::ExportDefaultDeclaration::cast))
        .and_then(|n| n.children().find_map(js::Expression::cast));
    let maybe_options = maybe_default_export.and_then(|expr| {
        if let Some(call_expr) = js::CallExpression::cast(expr) {
            let maybe_vue_extend = call_expr.first_child_or_token().and_then(js::MemberExpression::cast)?;
            let maybe_vue = maybe_vue_extend.first_child_or_token()?;
            let maybe_extend = maybe_vue_extend.last_child_or_token()?;
            if maybe_vue.text() != "Vue" || maybe_extend.text() != "extend" {
                return None;
            }
            let start_args = call_expr.children_with_tokens().find(|c| c.kind() == L_PAREN)?;
            start_args.next_sibling_or_token().and_then(js::ObjectExpression::Cast)
        } else {
            js::ObjectExpression::cast(expr)
        }
    });
    let vue_options = match maybe_options {
        Some(object) => object,
        None => return results,
    };

    // Compute the `vm` (ViewModel) properties/accessors.
    let mut vm_properties = Vec::new();
    // TODO: Build proper type inference to replace this spaghetti
    // TODO: Also probably just build proper AST accessors like `Property::key`
    let vue_options = vue_options.children().filter_map(js::Property::cast).collect::<Vec<_>>();
    let vue_props = vue_options.iter().find_map(|node| {
        let left = node.first_child_or_token()?;
        if left.kind() == IDENTIFIER {
            if left.text() != "props" {
                return None;
            }
            node.last_child().and_then(js::Expression::cast)
        } else {
            None
        }
    });
    let vue_data = vue_options.iter().find_map(|node| {
        let left = node.first_child_or_token()?;
        let function = if left.kind() == IDENTIFIER {
            if left.text() != "data" {
                return None;
            }
            if let Some(object) = node.last_child().and_then(js::ObjectExpression::cast) {
                return Some(object);
            }
            node.last_child().and_then(js::FunctionExpression::cast)?
        } else if let Some(shorthand) = js::FunctionExpression::cast(left) {
            let shorthand_name = shorthand.children_with_tokens().find(|t| t.kind() == IDENT)?;
            if shorthand_name.text() != "data" {
                return None;
            }
            shorthand
        };
        function
            .and_then(|f| f.last_child())
            .and_then(js::BlockStatement::cast)
            .and_then(|f| f.last_child())
            .and_then(js::ReturnStatement::cast)
            .and_then(|f| f.last_child())
            .and_then(js::ObjectExpression::cast)
    });
    let vue_computed = vue_options.iter().find_map(|node| {
        let left = node.first_child_or_token()?;
        if left.kind() == IDENTIFIER {
            if left.text() != "computed" {
                return None;
            }
            node.last_child().and_then(js::ObjectExpression::cast)
        } else {
            None
        }
    });
    let vue_methods = vue_options.iter().find_map(|node| {
        let left = node.first_child_or_token()?;
        if left.kind() == IDENTIFIER {
            if left.text() != "methods" {
                return None;
            }
            node.last_child().and_then(js::ObjectExpression::cast)
        } else {
            None
        }
    });

    // TODO: Move this into some sort of `extensions` or `contrib` module
    let vue_apollo = vue_options.iter().find_map(|node| {
        let left = node.first_child_or_token()?;
        if left.kind() == IDENTIFIER {
            if left.text() != "apollo" {
                return None;
            }
            node.last_child().and_then(js::ObjectExpression::cast)
        } else {
            None
        }
    });

    // Check that all expressions in the template reference known vm properties
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

fn syntax_errors(results: &mut Vec<String>, db: &RootDatabase, input_id: InputId, base: TextUnit, errors: Vec<SyntaxError>) {
    // TODO: include filename in error message
    // let filename = db.input_filename(input_id);

    let line_index = db.input_line_index(input_id);
    let mut offset_set = FxHashSet::default();
    results.extend(errors.into_iter().filter_map(|err| {
        // Only display the first _syntax_ error for each line.
        // TODO: Maybe the parser should just detect this case and handle it during `finalize`?
        let offset = err.offset();
        if !offset_set.contains(&offset) {
            offset_set.insert(offset);
            let line_col = index.line_col(base + offset);
            Some(format!("error: (bad syntax at line {}, col {}) {}", line_col.line, line_col.col_utf16, err.message))
        } else {
            None
        }
    }));
}

fn collect_expressions(template: &vue::ComponentTemplate) -> Vec<&str> {
    let mut expressions = Vec::new();
    for visit in template.preorder_with_tokens() {
        let syn_elem = match visit {
            WalkEvent::Enter(syntax) => syntax,
            _ => continue;
        };

        match (syn_elem.kind(), syn_elem) {
            (ATTRIBUTE_KEY, SyntaxElement::Node(node)) => {
                if node.first_token() == Some(L_SQUARE) {
                    if let Some(ident) = node.first_child() {
                        if ident.kind() != ERROR {
                            assert_eq!(ident.kind(), IDENTIFIER);
                            expressions.push(ident.text());
                        }
                    }
                }
            }
            (ATTRIBUTE_BINDING, SyntaxElement::Node(node)) => {
                if let Some(value) = node.last_child() {
                    if let Some(prev) = value.prev_sibling_or_token() {
                        if prev.kind() == EQ && value != ERROR {
                            expressions.push(value.text());
                        }
                    }
                }
            }
            (ATTRIBUTE_LISTENER, SyntaxElement::Node(node)) => {
                if let Some(value) = node.last_child() {
                    if let Some(prev) = value.prev_sibling_or_token() {
                        if prev.kind() == EQ && value != ERROR {
                            expressions.push(value.text());
                        }
                    }
                }
            }
            (MUSTACHE, SyntaxElement::Token(token)) => {
                let mustache = token.text();
                expressions.push(&mustache[2 .. mustache.len() - 2]);
            }
        }
    }
    expressions
}