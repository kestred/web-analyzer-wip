use crate::AstDatabase;
use code_analysis::{SourceId, SourceRootId};
use code_grammar::{AstNode, SmolStr, SyntaxNode, WalkEvent};
use typescript_grammar::ast as ts;
use rustc_hash::FxHashSet;
use std::{fmt, sync::Arc};

#[salsa::query_group(AppDatabaseStorage)]
pub trait AppDatabase: AstDatabase + typescript_analysis::AstDatabase {
    /// All components & etc registered with `Vue.{component,filter,mixin}` within the source root.
    fn global_registry(&self, id: SourceRootId) -> Arc<VueRegistry>;

    /// Any components & etc registered to the global `Vue` instance by a particular script.
    fn script_registry(&self, id: SourceId) -> Arc<VueRegistry>;
}

pub fn global_registry(db: &impl AppDatabase, id: SourceRootId) -> Arc<VueRegistry> {
    let mut registry = VueRegistry::default();

    // For each supported file in the project, search it for registered components
    let project = db.source_root(id);
    for (path, file_id) in &project.files {
        let src_id = match path.extension() {
            Some("js") | Some("ts") => db.file_source(*file_id),
            Some("vue") => match db.component_script(db.file_source(*file_id)) {
                Some((id, _)) => id,
                None => continue,
            },
            _ => continue,
        };
        registry.extend(&db.script_registry(src_id));
    }
    Arc::new(registry)
}

pub fn script_registry(db: &impl AppDatabase, src_id: SourceId) -> Arc<VueRegistry> {
    let mut registry = VueRegistry::default();
    let node = db.typescript_ast(src_id);
    if !node.errors().is_empty() {
        return Arc::new(registry);
    }
    for visit in node.syntax.preorder() {
        match visit {
            WalkEvent::Enter(node) => {
                try_register(&mut registry, node);
            }
            _ => (),
        }
    }
    Arc::new(registry)
}

#[inline]
fn try_register(registry: &mut VueRegistry, node: &SyntaxNode) -> Option<()> {
    let call = ts::CallExpression::cast(node)?;
    let callee = call.callee().and_then(ts::MemberExpression::downcast)?;
    if callee.object().and_then(ts::Identifier::downcast)?.name() != "Vue" {
        return None;
    }

    let literal = call.arguments().next().and_then(ts::Literal::downcast)?;
    let key = match literal.kind() {
        ts::LiteralKind::String(tok) => {
            let raw = tok.text().as_str();
            unescape::unescape(&raw[1 .. raw.len() - 1])?
        }
        _ => return None,
    };
    match callee.property().and_then(ts::Identifier::downcast)?.name() {
        "component" => registry.components.insert(key.into()),
        "filter" => registry.filters.insert(key.into()),
        "mixin" => registry.mixins.insert(key.into()),
        _ => return None,
    };
    Some(())
}

/// A registry of components registered with `Vue.component`, `Vue.filter`, etc...
#[derive(Default, Eq, PartialEq)]
pub struct VueRegistry {
    pub components: FxHashSet<SmolStr>, // TODO: Switch to `FxHashMap<SmolStr, AstId>`
    pub filters: FxHashSet<SmolStr>,
    pub mixins: FxHashSet<SmolStr>, // TODO: Switch to `FxHashMap<SmolStr, AstId>`
}

impl fmt::Debug for VueRegistry {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("VueRegistry")
            .field("components", &self.components.len())
            .field("filters", &self.filters.len())
            .field("mixins", &self.mixins.len())
            .finish()
    }
}

impl VueRegistry {
    pub fn extend(&mut self, other: &VueRegistry) {
        self.components.extend(other.components.iter().cloned());
        self.filters.extend(other.filters.iter().cloned());
        self.mixins.extend(other.mixins.iter().cloned());
    }
}
