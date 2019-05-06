use rowan::SyntaxKind;

#[macro_export]
macro_rules! tokenset {
    [ $( $kind:ident ),* ] => {
        $crate::TokenSet::new(&[$($kind),*])
    };
}

#[derive(Clone, Debug)]
pub struct TokenSet<'a>(&'a [SyntaxKind]);

impl<'a> TokenSet<'a> {
    pub const fn new(tokens: &'a [SyntaxKind]) -> TokenSet {
        TokenSet(tokens)
    }

    pub fn tokens(&self) -> impl Iterator<Item = &SyntaxKind> {
        self.0.iter()
    }

    pub fn contains(&self, kind: &SyntaxKind) -> bool {
        self.0.into_iter().any(|el| *el == *kind)
    }
}
