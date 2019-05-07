use crate::parse::{AstId, SourceLanguage};
use analysis_utils::{impl_intern_key, FileId};
use html_grammar::ast as html;
use salsa;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum InputId {
    File(FileId),
    Script(ScriptId),
}

impl From<FileId> for InputId {
    fn from(id: FileId) -> InputId {
        InputId::File(id)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ScriptId(salsa::InternId);
impl_intern_key!(ScriptId);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ScriptSource {
    pub(crate) ast_id: AstId<html::Script>,
    pub(crate) language: SourceLanguage,
}
