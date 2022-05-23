use super::{identifier::Identifier, type_identifier::TypeIdentifier};

#[derive(Debug, PartialEq)]
pub struct TypedVariable {
    pub identifier: Identifier,
    pub type_identifier: TypeIdentifier,
}
