use crate::parse::stable::AstId;
use analysis_utils::impl_intern_key;
use html_grammar::ast as html;
use salsa;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum InputId {
    File(FileId),
    Script(ScriptId),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ScriptId(salsa::InternId);
impl_intern_key!(ScriptId);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ScriptDefinition {
    pub(crate) ast_id: AstId<html::Script>,
    pub(crate) language: LanguageKind,
}
