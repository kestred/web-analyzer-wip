use crate::database::RootDatabase;
use crate::parse::{InputId, ParseDatabase, SourceLanguage};
use analysis_utils::LineIndex;
use grammar_utils::SyntaxError;

pub(crate) fn check(db: &RootDatabase, input_id: InputId) -> Vec<String> {
    let mut result = Vec::new();
    match db.input_language(input_id) {
        Some(SourceLanguage::Vue) => (),

        // Emit syntax errors only for `.html` and `.js` files
        Some(SourceLanguage::Html) => {
            let line_index = db.input_line_index(input_id);
            let document = db.parse_html(input_id);
            syntax_errors(&mut result, &line_index, document.errors());
            return result;
        }
        Some(SourceLanguage::Javascript) => {
            let line_index = db.input_line_index(input_id);
            let program = db.parse_javascript(input_id);
            syntax_errors(&mut result, &line_index, program.errors());
            return result;
        }

        // TODO: Handle typescript
        Some(SourceLanguage::Typescript) => return result,

        None => {
            result.push("(failed) could not detect source language".into())
        }
    }

    // TODO: Implement
    result
}

fn syntax_errors(results: &mut Vec<String>, index: &LineIndex, errors: Vec<SyntaxError>) {
    results.extend(errors.into_iter().map(|err| {
        let line_col = index.line_col(err.offset());
        format!("(syntax error) {}:{}: {}", line_col.line, line_col.col_utf16, err.message)
    }));
}
