use crate::ast::ClassId;
use code_grammar::SmolStr;
use std::sync::Arc;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TypeOf {
    Null,
    Number,
    String,
    Boolean,
    Undefined,
    Function,
    Object,
}

#[derive(Clone, Debug)]
pub enum Ty {
    Null,
    Number,
    String,
    Boolean,
    Undefined,
    Object,
    Array(Arc<Ty>),

    /// This type will "duck type" with any other type.
    /// Effectively indicates that type checking should be disabled.
    Any,

    /// This type is treated like `Any` but with some extra hint
    /// information to help in "fuzzy" type-checking.
    Hint(TypeOf),

    /// This type represents an unspecified type (aka. "Mixed" in Flow);
    /// it _could_ be any type, but does not cast to any other type.
    Unknown,

    /// It is impossible for a value of this type to exist (by definition);
    /// typically used to represent types in code that is "unreachable".
    Never,

    // TODO: Maybe add a concrete `function` type?
    // Function(FunctionId),

    /// A nominally-typed instance of a class.
    Instance(ClassId),

    /// A duck-typed value.
    Interface(Arc<InterfaceTy>),

    /// A value which has all of the properties of the component types
    Intersection(Arc<[Ty]>),

    /// A value which satisifies (at least) one of the given types
    Union(Arc<[Ty]>),

    // TODO: Add a `Constant` variant to support values which are one exact value?
    // Constant(js::Literal),
}


impl Ty {
    pub fn as_interface(&self) -> Option<&InterfaceTy> {
        match self {
            Ty::Interface(ty) => Some(ty),
            _ => None,
        }
    }
}

/// InterfaceTy is used to represent an object that has (at least)
/// all of the methods and properties describe by the interface.
///
/// Plain objects are often represented as a partial with all of
/// their known properties.
///
/// > If this type looks like a duck and quacks like a duck,
/// > then let us consider it to, in-fact, be a duck.
///
#[derive(Clone, Debug, Default)]
pub struct InterfaceTy {
    pub properties: Vec<PropertyDef>,
    /// The signature the `value[index]` operator on this type.
    pub indexable: Option<(Arc<Ty>, Arc<Ty>)>,
    /// The signature of the value when called as a function
    pub callable: Option<FunctionSig>,
    /// The valid `typeof` values.
    pub typeof_: Option<Arc<[TypeOf]>>,
}

impl InterfaceTy {
    pub fn delete_property(&mut self, name: &str) {
        self.properties.retain(|l| l.ident != name);
    }

    pub fn merge(&mut self, other: &InterfaceTy) {
        self.properties.retain(|l| other.properties.iter().all(|r| l.ident != r.ident));
        self.properties.extend(other.properties.iter().cloned());
        if let Some(sig) = other.indexable.clone() {
            self.indexable = Some(sig);
        }
        if let Some(sig) = other.callable.clone() {
            self.callable = Some(sig);
        }
        match (&self.typeof_, &other.typeof_) {
            (Some(left), Some(right)) if left == right => (),
            (Some(left), Some(right)) => {
                let mut typeof_ = left.into_iter()
                    .filter(|l| right.into_iter().all(|r| *l != r))
                    .chain(right.into_iter())
                    .cloned()
                    .collect::<Vec<_>>();
                typeof_.sort();
                self.typeof_ = Some(typeof_.into());
            }
            _ => {
                self.typeof_ = None;
            }
        }
    }
}

impl From<InterfaceTy> for Ty {
    fn from(data: InterfaceTy) -> Ty {
        Ty::Interface(Arc::new(data))
    }
}

/// A property name + type pair.
#[derive(Clone, Debug)]
pub struct PropertyDef {
    pub ident: SmolStr,
    pub type_: Ty,
}

/// Signature of a function type.
#[derive(Clone, Debug)]
pub struct FunctionSig {
    // TODO: Handle spread parameters in function signatures

    /// The arguments to the function.
    pub inputs: Vec<Ty>,

    /// The return type of the function.
    pub output: Ty,
}
