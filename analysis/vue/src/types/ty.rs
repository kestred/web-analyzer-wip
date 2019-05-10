use crate::definitions::{ClassId, FunctionId};
use grammar_utils::SmolStr;

pub type Ty<'ty> = &'ty TyKind<'ty>;

pub enum TyKind<'ty> {
    Null,
    Number,
    String,
    Boolean,
    Undefined,

    /// This type will "duck type" with any other type.
    /// Effectively indicates that type checking should be disabled.
    Any,

    /// This type represents an unspecified type (aka. "Mixed" in Flow);
    /// it _could_ be any type, but does not cast to any other type.
    Unknown,

    // TODO: Maybe add an "anonymous" `Object` type?
    // Object,

    Array(Ty<'ty>),
    Union(Vec<Ty<'ty>>),

    /// A duck-typed value
    Partial(&'ty InterfaceTy<'ty>),

    /// A nominally-typed instance of a class
    Instance(ClassId),

    /// The concrete type of a function
    FunctionDef(FunctionId),

    /// A duck-typed function reference
    FunctionPtr(&'ty FunctionSig<'ty>),
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
pub struct InterfaceTy<'ty> {
    pub properties: Vec<PropertyTy<'ty>>,
}

/// A property name / type pair.
pub struct PropertyTy<'ty> {
    pub ident: SmolStr,
    pub type_: Ty<'ty>,
}


/// Signature of a function type.
pub struct FunctionSig<'ty> {
    /// The arguments to the function.
    pub inputs: Vec<Ty<'ty>>,

    /// The return type of the function.
    pub output: Ty<'ty>,
}
