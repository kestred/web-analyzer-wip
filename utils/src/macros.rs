#[macro_export]
macro_rules! ast_node {
    ($node:ident, $kind:expr) => {
        #[derive(Debug, Eq, PartialEq, Hash)]
        #[repr(transparent)]
        pub struct $node { pub syntax: $crate::SyntaxNode }
        unsafe impl $crate::TransparentNewType for $node {
            type Repr = $crate::SyntaxNode;
        }
        impl $node {
            #[allow(unused)]
            fn cast(syntax: &$crate::SyntaxNode) -> Option<&Self> {
                use $crate::TransparentNewType;

                if syntax.kind() == $kind {
                    Some(Self::from_repr(syntax))
                } else {
                    None
                }
            }
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

            fn cast(syntax: &$crate::SyntaxNode) -> Option<&Self> {
                use $crate::TransparentNewType;

                match syntax.kind() {
                    $(_k if ast_node!(@try_cast _k $($kind)*) => ast_node!(@cast syntax $node $variant $($kind)*),)*
                    _ => None,
                }
            }
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
                $node::cast(&node.syntax).unwrap()
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