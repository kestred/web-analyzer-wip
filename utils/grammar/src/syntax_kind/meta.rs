use rowan::SyntaxKind;

/// Metadata about a syntax kind.
#[derive(Debug)]
pub struct SyntaxKindMeta {
  /// The name of the syntax kind (e.g. `IDENT`)
  pub name: &'static str,

  /// The kind of the symbol
  pub kind: SyntaxKind,

  /// The canonical source text string representation (if one exists)
  pub canonical: Option<&'static str>,
}
