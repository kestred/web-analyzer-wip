use grammar_utils::SyntaxLanguage;
use html_grammar::syntax_kind::HTML;
use javascript_grammar::syntax_kind::JAVASCRIPT;
use typescript_grammar::syntax_kind::TYPESCRIPT;
use vue_grammar::syntax_kind::VUE;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SourceLanguage {
    Html,
    Javascript,
    Typescript,
    Vue,
}

impl SourceLanguage {
    fn from_syntax(syntax: SyntaxLanguage) -> Option<SourceLanguage> {
        match syntax {
            l if l == HTML => Some(SourceLanguage::Html),
            l if l == JAVASCRIPT => Some(SourceLanguage::Javascript),
            l if l == TYPESCRIPT => Some(SourceLanguage::Typescript),
            l if l == VUE => Some(SourceLanguage::Vue),
            _ => None
        }
    }
}

impl From<SourceLanguage> for SyntaxLanguage {
    fn from(lang: SourceLanguage) -> SyntaxLanguage {
        match lang {
            SourceLanguage::Html => HTML,
            SourceLanguage::Javascript => JAVASCRIPT,
            SourceLanguage::Typescript => TYPESCRIPT,
            SourceLanguage::Vue => VUE,
        }
    }
}
