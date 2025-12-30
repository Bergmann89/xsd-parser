use thiserror::Error;

use xsd_parser_types::misc::{Namespace, NamespacePrefix, RawByteStr};

use crate::models::{
    schema::xs::{AttributeType, Facet},
    NodeIdent, TypeIdent,
};

/// error raised by the [`Interpreter`](super::Interpreter).
#[derive(Debug, Error)]
pub enum Error {
    /// Type has already been defined.
    ///
    /// Is raised if a new type with an already existing identifier is added
    /// to the [`MetaTypes`](crate::models::meta::MetaTypes) structure.
    #[error("Type has already been defined: {0}!")]
    TypeAlreadyDefined(TypeIdent),

    /// Ambiguous type definition
    ///
    /// Is raised by the interpreter if it tries to resolve a certain type
    /// identifier during interpretation of the schema, but multiple matching
    /// types were found.
    #[error("Ambiguous type: {0}!")]
    AmbiguousType(TypeIdent),

    /// Ambiguous node definition
    ///
    /// Is raised by the interpreter if it tries to resolve a certain type
    /// definition inside the schemas, but multiple matching types were found.
    #[error("Ambiguous node: {0}!")]
    AmbiguousNode(NodeIdent),

    /// Expected dynamic element.
    ///
    /// Expected the specified element to be dynamic because it is referenced
    /// as substitution group.
    #[error("Expected dynamic element: {0}!")]
    ExpectedDynamicElement(NodeIdent),

    /// Unknown node.
    ///
    /// Is raised if a specific node could not be found inside the schema definitions.
    #[error("Unknown node: {0}!")]
    UnknownNode(NodeIdent),

    /// Unknown type.
    ///
    /// Is raised if a type identifier could not been resolved to the actual
    /// type information.
    #[error("Unknown type: {0}!")]
    UnknownType(TypeIdent),

    /// Unknown element.
    ///
    /// Is raised if an element referenced inside the XML schema could not be resolved.
    #[error("Unknown element: {0}!")]
    UnknownElement(TypeIdent),

    /// Unknown attribute.
    ///
    /// Is raised if an attribute referenced inside the XML schema could not be resolved.
    #[error("Unknown attribute: {0}!")]
    UnknownAttribute(String),

    /// Unknown namespace.
    ///
    /// Is raised if the namespace URI could not be resolved.
    #[error("Unknown namespace: {0}!")]
    UnknownNamespace(Namespace),

    /// Unknown namespace prefix.
    ///
    /// Is raised if the namespace prefix could not be resolved.
    #[error("Unknown namespace prefix: {0}!")]
    UnknownNamespacePrefix(NamespacePrefix),

    /// Anonymous namespace is undefined.
    #[error("Anonymous namespace is undefined!")]
    AnonymousNamespaceIsUndefined,

    /// Invalid value.
    ///
    /// Is raised if a value from the XML schema is malformed or invalid.
    #[error("Invalid value for `{0}`!")]
    InvalidValue(&'static str),

    /// Invalid local name.
    ///
    /// Is raised if conversion from a raw local name to a string has failed.
    #[error("Invalid local name `{0}`!")]
    InvalidLocalName(RawByteStr),

    /// Group is missing the `ref` attribute
    ///
    /// Is raised if a group reference in the XML schema is missing the `ref` attribute.
    #[error("Group is missing the `ref` attribute!")]
    GroupMissingRef,

    /// Attribute group is missing the `ref` attribute
    ///
    /// Is raised if a attribute group reference in the XML schema is missing the `ref` attribute.
    #[error("Attribute group is missing the `ref` attribute!")]
    AttributeGroupMissingRef,

    /// Invalid attribute reference.
    ///
    /// The attribute specified in the schema is missing some information.
    #[error("Invalid attribute reference: {0:#?}!")]
    InvalidAttributeReference(Box<AttributeType>),

    /// Invalid facet.
    ///
    /// Is raised if the content of a facet could not be interpreted correctly.
    #[error("Invalid facet: {0:?}")]
    InvalidFacet(Facet),

    /// Unable to create type information.
    ///
    /// Is raised if the interpreter was not able to generate a `Type` from the
    /// provided schema information.
    #[error("Unable to create type information!")]
    NoType,

    /// The interpreter expected a group type (like `xs:all`, `xs:choice` or `xs:sequence`).
    #[error("Expected group type!")]
    ExpectedGroupType,
}
