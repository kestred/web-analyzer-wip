use web_grammar_utils::Scanner;

pub fn is_html_tag_prefix(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

pub fn is_html_tag_suffix(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '-' || c == '_' || c == '.'
}

/// Assumes preceding `<`
pub fn scan_html_comment(s: &mut Scanner, nested: bool) -> bool {
    if s.at_str("!--") {
        s.bump();
        s.bump();
        s.bump();
        loop {
            if s.at_str("-->") {
                s.bump();
                s.bump();
                s.bump();
                break;
            } else if nested && s.at_str("<!--") || s.at_str("<![") {
                s.bump();
                scan_html_comment(s, nested);
            } else if s.bump().is_none() {
                break;
            }
        }
        true
    } else if s.at_str("![") {
        s.bump();
        s.bump();
        loop {
            if s.at_str("]>") {
                s.bump();
                s.bump();
                break;
            } else if nested && s.at_str("<!--") || s.at_str("<![") {
                s.bump();
                scan_html_comment(s, nested);
            } else if s.bump().is_none() {
                break;
            }
        }
        true
    } else {
        false
    }
}
