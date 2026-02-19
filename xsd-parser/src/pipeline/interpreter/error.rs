use thiserror::Error;

use xsd_parser_types::misc::{Namespace, NamespacePrefix, RawByteStr};

use crate::models::{
    schema::xs::{AttributeType, Facet},
    TypeIdent,
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

    /// Expected simple type.
    ///
    /// Expected the specified type to be simple because it is referenced
    /// in a context that requires a simple type.
    #[error("Expected simple type: {0}!")]
    ExpectedSimpleType(TypeIdent),

    /// Expected complex type.
    ///
    /// Expected the specified type to be complex because it is referenced
    /// in a context that requires a complex type.
    #[error("Expected complex type: {0}!")]
    ExpectedComplexType(TypeIdent),

    /// Expected dynamic element.
    ///
    /// Expected the specified element to be dynamic because it is referenced
    /// as substitution group.
    #[error("Expected dynamic element: {0}!")]
    ExpectedDynamicElement(TypeIdent),

    /// Unknown type.
    ///
    /// Is raised if a type identifier could not been resolved to the actual
    /// type information.
    #[error("Unknown type: {0}!")]
    UnknownType(TypeIdent),

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
    ///
    /// Before resolving any type that is defined in the anonymous namespace
    /// you have to add it to the [`Schemas`](crate::models::schema::Schemas)
    /// by either adding a schema file that uses it (see
    /// [`add_schema_from_str`](crate::pipeline::parser::Parser::add_schema_from_str)
    /// or related add_schema_xxx methods) or by defining is manually (see
    /// [`with_anonymous_namespace`](crate::pipeline::parser::Parser::with_anonymous_namespace)).
    #[error("Anonymous namespace is undefined!")]
    AnonymousNamespaceIsUndefined,

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

    /// Circular dependency.
    ///
    /// Is raised if the interpreter detects a circular strong dependency between
    /// types during type generation.
    #[error("Circular dependency detected for type: {0}!")]
    CircularDependency(TypeIdent),
}
