#[macro_export]
macro_rules! ast_node {
    ($ast:ident, $kind:expr) => {
        #[derive(Debug, Eq, PartialEq, Hash)]
        #[repr(transparent)]
        pub struct $ast { pub syntax: $crate::SyntaxNode }
        unsafe impl $crate::TransparentNewType for $ast {
            type Repr = $crate::SyntaxNode;
        }
        impl $ast {
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
        impl ToOwned for $ast {
            type Owned = $crate::TreeArc<Self>;
            fn to_owned(&self) -> $crate::TreeArc<Self> {
                $crate::TreeArc::cast(self.syntax.to_owned())
            }
        }
    };
    ($ast:ident, enum $enum:ident {
        $($varname:ident $(= $kind:expr)? ),* $(,)?
    }) => {
        #[derive(Debug, Eq, PartialEq, Hash)]
        #[repr(transparent)]
        pub struct $ast { pub syntax: $crate::SyntaxNode }
        unsafe impl $crate::TransparentNewType for $ast {
            type Repr = $crate::SyntaxNode;
        }

        impl $ast {
            pub fn kind(&self) -> $enum {
                match self.syntax.kind() {
                    $($(k if k == $kind => $enum::$varname($varname::cast(&self.syntax).unwrap()),)*)*
                    _ => unreachable!(),
                }
            }

            fn cast(syntax: &$crate::SyntaxNode) -> Option<&Self> {
                use $crate::TransparentNewType;

                let is_match = match syntax.kind() {
                    $($(k if k == $kind => true,)*)*
                    _ => false,
                };
                if is_match {
                    Some($ast::from_repr(syntax))
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

        #[derive(Debug, Eq, PartialEq)]
        pub enum $enum<'a> {$(
            $varname(&'a $varname),
        )*}
        $(impl<'a> From<&'a $varname> for &'a $ast {
            fn from(node: &'a $varname) -> &'a $ast {
                $ast::cast(&node.syntax).unwrap()
            }
        })*

        // #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        // pub enum $enum $(<$lifetime>)* {$(
        //     $varname($(&$lifetime)* $varname),
        // )*}
        // $(impl$(<$lifetime>)* From<$(&$lifetime)* $varname> for $(&$lifetime)* $ast {
        //     fn from(node: $(&$lifetime)* $varname) -> $(&$lifetime)* $ast {
        //         $ast::cast(&node.syntax).unwrap()
        //     }
        // })*
    };
}