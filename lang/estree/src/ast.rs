#[derive(Debug)]
pub enum Type {
    Named(String),
    Array(Box<Type>),
    Union(Vec<Type>),
    Object(Vec<Field>),
    StringLiteral(String),
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub parents: Vec<String>,
    pub fields: Vec<Field>,
    pub is_extension: bool,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub literals: Vec<String>,
    pub is_extension: bool,
}

#[derive(Debug)]
pub enum Definition {
  Enum(Enum),
  Interface(Interface),
}