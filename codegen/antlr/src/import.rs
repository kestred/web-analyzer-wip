use crate::ast::Grammar;
use crate::grammar::grammar;
use combine::parser::Parser;
use combine::stream::state::State;
use std::{fs, io, path::PathBuf};

impl Grammar {
    pub fn from_file(path: &str) -> Result<Grammar, ImportError> {
        let mut grammar = Grammar {
            name: Default::default(),
            rules: Default::default(),
            imports: Default::default(),
        };
        grammar.name = grammar.import(path)?;
        Ok(grammar)
    }

    pub fn import(&mut self, path: &str) -> Result<String, ImportError> {
        let content = fs::read_to_string(path)?;
        let result = grammar().easy_parse(State::new(content.as_str()));
        let (mut root, _) = match result {
            Err(err) => return Err(ImportError::Parse(path.into(), err.to_string())),
            Ok(ok) => ok,
        };
        for import in &root.imports {
            if let Some(literal) = &import.path {
                let relative = PathBuf::from(path)
                    .parent()
                    .unwrap()
                    .join(&literal[1 .. literal.len() - 1]);
                let absolute = relative.canonicalize()?;
                self.import(absolute.to_str().unwrap())?;
            }
        }
        self.rules.retain(|old| root.rules.iter().all(|new| old.name != new.name));
        root.rules.extend(self.rules.drain(0..));
        self.rules = root.rules;
        Ok(root.name)
    }
}

#[derive(Debug)]
pub enum ImportError {
    Io(io::Error),
    Parse(String, String),
}

impl From<io::Error> for ImportError {
    fn from(err: io::Error) -> Self {
        ImportError::Io(err)
    }
}
