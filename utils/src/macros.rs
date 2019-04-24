#[macro_export]
macro_rules! ast_node {
    ($node:ident, $kind:expr) => {
        #[derive(Debug, Eq, PartialEq, Hash)]
        #[repr(transparent)]
        pub struct $node { pub syntax: $crate::SyntaxNode }
        unsafe impl $crate::TransparentNewType for $node {
            type Repr = $crate::SyntaxNode;
        }
        impl $crate::AstNode for $node {
            fn cast(syntax: &$crate::SyntaxNode) -> Option<&Self> {
                use $crate::TransparentNewType;

                if syntax.kind() == $kind {
                    Some(Self::from_repr(syntax))
                } else {
                    None
                }
            }

            fn syntax(&self) -> &$crate::SyntaxNode { &self.syntax }
        }
        impl ToOwned for $node {
            type Owned = $crate::TreeArc<Self>;
            fn to_owned(&self) -> $crate::TreeArc<Self> {
                $crate::TreeArc::cast(self.syntax.to_owned())
            }
        }
    };
    ($node:ident, enum $enum:ident {
        $($variant:ident $(= $kind:expr)? ),* $(,)?
    }) => {
        #[derive(Debug, Eq, PartialEq, Hash)]
        #[repr(transparent)]
        pub struct $node { pub syntax: $crate::SyntaxNode }
        unsafe impl $crate::TransparentNewType for $node {
            type Repr = $crate::SyntaxNode;
        }
        impl $node {
            pub fn kind(&self) -> $enum {
                use $crate::AstNode;

                match self.syntax.kind() {
                    // Match quickly on the syntax kind if possible
                    $($(k if k == $kind => $enum::$variant($variant::cast(&self.syntax).unwrap()),)*)*

                    // Otherwise test all nested enums
                    _ => {
                        $(ast_node!(@try_kind self $enum $variant $($kind)*);)*
                        unreachable!()
                    }
                }
            }
        }
        impl $crate::AstNode for $node {
            fn cast(syntax: &$crate::SyntaxNode) -> Option<&Self> {
                use $crate::{AstNode, TransparentNewType};

                match syntax.kind() {
                    $(_k if ast_node!(@try_cast _k $($kind)*) => ast_node!(@cast syntax $node $variant $($kind)*),)*
                    _ => None,
                }
            }

            fn syntax(&self) -> &$crate::SyntaxNode { &self.syntax }
        }
        impl ToOwned for $node {
            type Owned = $crate::TreeArc<Self>;
            fn to_owned(&self) -> $crate::TreeArc<Self> {
                $crate::TreeArc::cast(self.syntax.to_owned())
            }
        }

        #[derive(Debug, Eq, PartialEq)]
        pub enum $enum<'a> {$(
            $variant(&'a $variant),
        )*}
        $(impl<'a> From<&'a $variant> for &'a $node {
            fn from(node: &'a $variant) -> &'a $node {
                <$node as $crate::AstNode>::cast(&node.syntax).unwrap()
            }
        })*
    };

    (@try_kind $self:ident $enum:ident $variant:ident $kind:expr) => {};
    (@try_kind $self:ident $enum:ident $variant:ident) => {
        if let Some(value) = $variant::cast(&$self.syntax) {
            return $enum::$variant(value);
        }
    };
    (@try_cast $binding:ident $kind:expr) => {
        $binding == $kind
    };
    (@try_cast $binding:ident) => {
        true
    };
    (@cast $syntax:ident $node:ident $variant:ident $kind:expr) => {
        Some($node::from_repr($syntax))
    };
    (@cast $syntax:ident $node:ident $variant:ident) => {
        $variant::cast($syntax).map(|_| $node::from_repr($syntax))
    };
}

#[macro_export]
macro_rules! syntax_kinds {
    {
        language $lang:ident;

        $(
            $(#[doc($hidden:tt)])?
            $label:ident {
                $($kind:ident $num:tt $(($raw:tt))?)*
            }
        )*
    } => {
        $(
            $(#[doc($hidden)])*
            pub mod $label {
                use super::$lang;

                $(#[doc(hidden)] pub const $kind: $crate::SyntaxKind = $lang.syntax_kind($num);)*

                /// Get the canonical string representation of the token, if one exists
                pub fn as_str(k: $crate::SyntaxKind) -> Option<&'static str> {
                    match k {
                        $(_ if k == $kind => None $(.or(Some($raw)))*,)*
                        _ => None,
                    }
                }

                /// Convert the syntax kind into a value with extra debug information
                /// that can be used with `std::fmt::Debug` format strings.
                pub fn as_debug_repr(k: $crate::SyntaxKind) -> Option<$crate::syntax_kind::SyntaxKindMeta> {
                    match k {
                        $(
                            _ if k == $kind => {
                                Some($crate::syntax_kind::SyntaxKindMeta {
                                    name: stringify!($kind),
                                    kind: $kind,
                                    canonical: None $(.or(Some($raw)))*
                                })
                            }
                        )*,
                        _ => None,
                    }
                }
            }
            pub use self::$label::{
                $($kind,)*
            };
        )*
    }
}