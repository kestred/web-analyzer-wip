use combine::parser::Parser;
use combine::stream::state::State;
use estree_grammar::{ast, scan, grammar};
use heck::ShoutySnakeCase;
use std::collections::{HashMap, HashSet};
use std::fs;

const BASE_SYNTAX_KIND: u16 = 205;

fn main() -> Result<(), std::io::Error> {
    let filepaths = &[
        "lang/estree/spec/es5.md",
        "lang/estree/spec/es2015.md",
        "lang/estree/spec/es2016.md",
        "lang/estree/spec/es2017.md",
        "lang/estree/spec/es2018.md",
        "lang/estree/spec/es2019.md",
    ];
    let mut spec = Tree::default();
    for filepath in filepaths {
        let fulltext = fs::read_to_string(filepath)?;
        let blocks = scan::extract_code_blocks(&fulltext);
        for (idx, block) in blocks.into_iter().enumerate() {
            let result = grammar::code_block().easy_parse(State::new(block));
            let (nodes, _) = match result {
                Ok(ok) => ok,
                Err(err) => {
                    eprintln!("In file `{}` at block {}:\n{}", filepath, idx, err);
                    eprintln!("{}", block);
                    std::process::exit(1);
                }
            };
            for node in nodes {
                spec.extend(node);
            }
        }
    }

    let mut out = String::new();

    // Generate ast nodes
    out.push_str("pub mod ast {\n");
    out.push_str("    use crate::syntax_kind::*;\n");
    out.push_str("    use web_grammars_utils::ast_node;\n\n");
    let mut leaf_nodes = Vec::new();
    let mut emitted_nodes = HashSet::new();
    {
        let root = spec.find_node("Node").expect("expected estree to contain `Node` interface");
        try_emit_enum(&mut out, &spec, root);
        emitted_nodes.insert(&root.name);
    }
    for node in spec.children.get("Node")
        .expect("expected estree to contain `Node` interface")
        .iter()
        .flat_map(|n| spec.find_node(&n))
    {
        if !is_leaf_node(&spec, node) && try_emit_enum(&mut out, &spec, node) {
            emitted_nodes.insert(&node.name);
        }
    }
    for node in &spec.nodes {
        if emitted_nodes.contains(&node.name) {
            continue;
        }

        if is_ast_node(&spec, node) {
            if is_leaf_node(&spec, node) {
                leaf_nodes.push(node);
            } else if try_emit_enum(&mut out, &spec, node) {
                emitted_nodes.insert(&node.name);
            }
        }
    }
    leaf_nodes.sort_by(|a, b| a.name.cmp(&b.name));
    for node in &leaf_nodes {
        out.push_str("    ast_node!(");
        out.push_str(&node.name);
        out.push_str(", ");
        out.push_str(&node.name.to_shouty_snake_case());
        out.push_str(");\n");
        let type_ = node.fields.iter().find(|f| f.name == "type").map(|f| &f.type_);
        if let Some(ast::Type::StringLiteral(literal)) = type_ {
            out.push_str("    impl ");
            out.push_str(&node.name);
            out.push_str(" {\n");
            out.push_str("        fn type_() -> &'static str {\n");
            out.push_str("            ");
            out.push_str(&literal);
            out.push('\n');
            out.push_str("        }\n");
            out.push_str("    }\n");
        }
    }
    out.push_str("}\n\n");

    // Generate syntax kinds
    let mut next_syntax_kind = BASE_SYNTAX_KIND;
    out.push_str("pub mod syntax_kind {\n");
    out.push_str("    use crate::syntax_kind::JAVASCRIPT;\n");
    out.push_str("    use web_grammars_utils::SyntaxKind;\n\n");
    for node in &leaf_nodes {
        out.push_str("    pub const ");
        out.push_str(&node.name.to_shouty_snake_case());
        out.push_str(": SyntaxKind = JAVASCRIPT.syntax_kind(");
        out.push_str(&next_syntax_kind.to_string());
        out.push_str(");\n");
        next_syntax_kind += 1;
    }
    out.push_str("}\n");

    fs::write("lang/javascript/src/generated.rs", out.as_bytes())?;

    Ok(())
}

fn is_ast_node(spec: &Tree, node: &ast::Interface) -> bool {
    spec.find_ancestors(&node.name).any(|n| n.name == "Node")
}

fn is_leaf_node(spec: &Tree, node: &ast::Interface) -> bool {
    let _ = spec;
    node.fields.iter().any(|f| {
        match &f.type_ {
            ast::Type::StringLiteral(lit) if f.name == "type" => &lit[1..lit.len()-1] == &node.name,
            _ => false
        }
    })
}

fn has_leaf_nodes(spec: &Tree, node: &ast::Interface) -> bool {
    if is_leaf_node(&spec, node) {
        return true;
    }

    match spec.children.get(&node.name) {
        Some(children) => children.iter().any(|c| {
            spec.find_node(c)
                .map(|n| has_leaf_nodes(spec, n))
                .unwrap_or(false)
        }),
        None => false
    }
}

fn try_emit_enum(out: &mut String, spec: &Tree, node: &ast::Interface) -> bool {
    if let Some(children) = spec.children.get(&node.name) {
        out.push_str("    ast_node!(");
        out.push_str(&node.name);
        out.push_str(", enum ");
        out.push_str(&node.name);
        out.push_str("Kind {\n");
        for child in children {
            let child = spec.find_node(child).unwrap();
            if has_leaf_nodes(spec, child) {
                out.push_str("        ");
                out.push_str(&child.name);

                if is_leaf_node(spec, child) {
                    out.push_str(" = ");
                    out.push_str(&child.name.to_shouty_snake_case());
                }
                out.push_str(",\n");
            }
        }
        out.push_str("    });\n");
        true
    } else {
        false
    }
}

#[derive(Default)]
pub struct Tree {
    pub nodes: Vec<ast::Interface>,
    pub children: HashMap<String, Vec<String>>,
    pub dictionary: HashMap<String, usize>,
}

impl Tree {
    pub fn find_node(&self, name: &str) -> Option<&ast::Interface> {
        self.dictionary.get(name).and_then(|&i| self.nodes.get(i))
    }

    pub fn find_ancestors(&self, name: &str) -> impl Iterator<Item=&ast::Interface> {
        let mut results = HashSet::new();
        let mut queries = vec![name];
        while let Some(node) = queries.pop().and_then(|q| self.find_node(q)) {
            if &node.name != name {
                results.insert(&node.name);
            }
            for parent in &node.parents {
                queries.push(&parent);
            }
        }
        results.into_iter().flat_map(move |r| self.find_node(r))
    }

    pub fn find_descendents(&self, name: &str) -> impl Iterator<Item=&ast::Interface> {
        let mut results = HashSet::new();
        let mut queries = vec![name];
        while let Some(node) = queries.pop().and_then(|q| self.find_node(q)) {
            if &node.name != name {
                results.insert(&node.name);
            }
            if let Some(children) = self.children.get(&node.name) {
                for child in children {
                    queries.push(&child);
                }
            }
        }
        results.into_iter().flat_map(move |r| self.find_node(r))
    }

    pub fn extend(&mut self, node: ast::Definition) {
        match node {
            ast::Definition::Enum(_) => {
                // TODO: Do something with enums
            }
            ast::Definition::Interface(node) => {
                if let Some(&curr) = self.dictionary.get(&node.name) {
                    assert!(node.is_extension);
                    assert!(node.parents.is_empty());

                    let curr = self.nodes.get_mut(curr).unwrap();
                    for field in node.fields {
                        let name = field.name.clone();
                        curr.fields = curr.fields
                            .drain(0..)
                            .filter(|x| x.name != name)
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
    }
}
