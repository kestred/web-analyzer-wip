#[derive(Debug)]
pub enum Type {
  Named(String),
  Array(Box<Type>),
  Union(Vec<Type>),
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
  pub extend: bool,
}