use combine::parser::Parser;
use combine::stream::state::State;
use estree_grammar::{ast, scan, grammar};
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let filepaths = &[
        "lang/estree/spec/es5.md",
        "lang/estree/spec/es2015.md",
        "lang/estree/spec/es2016.md",
        "lang/estree/spec/es2017.md",
        "lang/estree/spec/es2018.md",
        "lang/estree/spec/es2019.md",
    ];
    let mut spec = EstreeBuilder::default();
    for filepath in filepaths {
        let fulltext = fs::read_to_string(filepath)?;
        let blocks = scan::extract_code_blocks(&fulltext);
        for (idx, block) in blocks.into_iter().enumerate() {
            let result = grammar::code_block().easy_parse(State::new(block));
            let (nodes, _) = match result {
                Ok(ok) => ok,
                Err(err) => {
                    eprintln!("In file `{}` at block {}:\n{}", filepath, idx, err);
                    std::process::exit(1);
                }
            };
            for node in nodes {
                spec.extend(node);
            }
        }
    }

    // TODO: Generate output

    Ok(())
}

#[derive(Default)]
pub struct EstreeBuilder {
    nodes: Vec<ast::Interface>,
    children: HashMap<String, Vec<String>>,
    dictionary: HashMap<String, usize>,
}

impl EstreeBuilder {
    pub fn extend(&mut self, node: ast::Interface) {
        if let Some(&curr) = self.dictionary.get(&node.name) {
            assert!(node.is_extension);
            assert!(node.parents.is_empty());

            let curr = self.nodes.get_mut(curr).unwrap();
            for field in node.fields {
                let name = field.name.clone();
                curr.fields = curr.fields
                    .drain(0..)
                    .filter(|x| x.name == name)
                    .chain(std::iter::once(field))
                    .collect::<Vec<_>>();
            }
        } else {
            assert!(!node.is_extension);
            for parent in &node.parents {
                self.children.entry(parent.clone()).or_default().push(node.name.clone());
            }
            self.dictionary.insert(node.name.clone(), self.nodes.len());
            self.nodes.push(node);
        }
    }
}
