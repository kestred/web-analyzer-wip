use rowan::SyntaxKind;
use smallset::SmallSet;

/// A bit-set of `SyntaxKind`s
pub type TokenSet = SmallSet<[SyntaxKind; 11]>;
fn _static_assert_sizeof_tokenset() { unsafe { std::mem::transmute::<SmallSet<[SyntaxKind; 1]>, TokenSet>(SmallSet::new()); } }
