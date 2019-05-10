use crate::parse::{AstId, SourceLanguage};
use analysis_utils::{impl_intern_key, FileId};
use html_grammar::ast as html;
use salsa;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum FileLikeId {
    File(FileId),
    Script(ScriptId),
}

impl From<FileId> for FileLikeId {
    fn from(id: FileId) -> FileLikeId {
        FileLikeId::File(id)
    }
}

impl From<ScriptId> for FileLikeId {
    fn from(id: ScriptId) -> FileLikeId {
        FileLikeId::Script(id)
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
