#[macro_export]
macro_rules! ast_node {
    ($ast:ident, $kind:expr) => {
        #[derive(Eq, PartialEq, Hash)]
        #[repr(transparent)]
        struct $ast { pub syntax: $crate::SyntaxNode }
        unsafe impl $crate::TransparentNewType for $ast {
            type Repr = $crate::SyntaxNode;
        }
        impl $ast {
            #[allow(unused)]
            fn cast(syntax: &$crate::SyntaxNode) -> Option<&Self> {
                if syntax.kind() == $kind {
                    Some(Self::from_repr(syntax.into_repr()))
                } else {
                    None
                }
            }
        }
        impl ToOwned for $ast {
            type Owned = $crate::TreeArc<Self>;
            fn to_owned(&self) -> $crate::TreeArc<Self> {
                $crate::TreeArc::cast(self.syntax.to_owned())
            }
        }
    };
    ($ast:ident as enum $enum:ident $(<$lifetime:tt>)? {
        $($varname:ident($vartype:ty) = $kind:expr),*
    }) => {
        #[derive(Debug, Eq, PartialEq, Hash)]
        #[repr(transparent)]
        struct $ast { pub syntax: $crate::SyntaxNode }
        unsafe impl $crate::TransparentNewType for $ast {
            type Repr = $crate::SyntaxNode;
        }

        impl Expr {
            pub fn kind(&self) -> $enum {
                match self.syntax.kind() {
                    $(k if k == $kind => $enum::$varname($vartype::cast(&self.syntax).unwrap()),)*
                    _ => unreachable!(),
                }
            }

            fn cast(syntax: &$crate::SyntaxNode) -> Option<&Self> {
                let is_match = match syntax.kind() {
                    $(k if k == $kind => true,)*
                    _ => false,
                };
                if is_match {
                    Some($ast::from_repr(syntax.into_repr()))
                } else {
                    None
                }
            }
        }

        impl ToOwned for $ast {
            type Owned = $crate::TreeArc<Self>;
            fn to_owned(&self) -> $crate::TreeArc<Self> {
                $crate::TreeArc::cast(self.syntax.to_owned())
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $enum $(<$lifetime>)* {$(
            $varname($(&$lifetime)* $vartype),
        )*}
        $(impl$(<$lifetime>)* From<$(&$lifetime)* $vartype> for $(&$lifetime)* $ast {
            fn from(node: $(&$lifetime)* $vartype) -> $(&$lifetime)* $ast {
                Expr::cast(&node.syntax).unwrap()
            }
        })*
    };
}