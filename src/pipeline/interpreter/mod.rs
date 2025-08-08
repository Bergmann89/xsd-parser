//! Schema interpretation logic for transforming parsed XML schemas into semantic
//! type definitions.
//!
//! This module defines the [`Interpreter`] type, which processes raw [`Schemas`] loaded
//! by the [`Parser`](crate::Parser) and converts them into semantic [`MetaTypes`].
//! These types represent meaningful, structured representations such as complex types,
//! enums, references, and attributes.
//!
//! The interpreter is capable of:
//! - registering custom or user-defined types
//! - resolving XSD primitive types and typedefs
//! - adding default build-in or XML-specific types (e.g., `xs:string`, `xs:anyType`)
//! - integrating numeric backends (e.g., `num::BigInt`) for large integers
//!
//! The resulting [`MetaTypes`] structure can then be passed to the generator to
//! generate Rust specific type structures.
//!
//! # Example
//! ```rust,ignore
//! let meta_types = Interpreter::new(&schemas)
//!     .with_buildin_types()?
//!     .with_default_typedefs()?
//!     .finish()?;
//! ```

mod error;
mod name_builder;
mod schema;
mod state;
mod variant_builder;

use std::fmt::Debug;

use quote::quote;
use tracing::instrument;

use crate::config::Namespace;
use crate::models::{
    meta::{
        AnyAttributeMeta, AnyMeta, AttributeMeta, BuildInMeta, ComplexMeta, CustomMeta,
        ElementMeta, GroupMeta, MetaType, MetaTypeVariant, MetaTypes, ModuleMeta, ReferenceMeta,
    },
    schema::{xs::ProcessContentsType, MaxOccurs, Schemas},
    Ident, IdentType, Name,
};

pub use error::Error;

use self::schema::SchemaInterpreter;
use self::state::{Node, StackEntry, State};
use self::variant_builder::VariantBuilder;

/// The `Interpreter` transforms raw parsed XML schema data into semantically
/// meaningful Rust-compatible type metadata.
///
/// It operates on a [`Schemas`] structure produced by the [`Parser`](crate::Parser)
/// and produces a [`MetaTypes`] structure, which is the central format used for
/// code generation.
///
/// This abstraction allows the intermediate schema format to be reshaped into a form
/// suitable for deterministic and idiomatic Rust code generation.
#[must_use]
#[derive(Debug)]
pub struct Interpreter<'a> {
    state: State<'a>,
    schemas: &'a Schemas,
}

impl<'a> Interpreter<'a> {
    /// Create a new [`Interpreter`] instance using the passed `schemas` reference.
    pub fn new(schemas: &'a Schemas) -> Self {
        let state = State::default();

        Self { state, schemas }
    }

    /// Add a custom [`MetaType`] information for the passed `ident`ifier to the
    /// resulting [`MetaTypes`] structure.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn with_type<I, T>(mut self, ident: I, type_: T) -> Result<Self, Error>
    where
        I: Into<Ident> + Debug,
        T: Into<MetaType> + Debug,
    {
        self.state.add_type(ident, type_, true)?;

        Ok(self)
    }

    /// Add a simple type definition to the resulting [`MetaTypes`] structure using
    /// `ident` as identifier for the new type and `type_` as target type for the
    /// type definition.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn with_typedef<I, T>(mut self, ident: I, type_: T) -> Result<Self, Error>
    where
        I: Into<Ident> + Debug,
        T: Into<Ident> + Debug,
    {
        self.state
            .add_type(ident, ReferenceMeta::new(type_), true)?;

        Ok(self)
    }

    /// Adds the default build-in types to the resulting [`MetaTypes`] structure.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn with_buildin_types(mut self) -> Result<Self, Error> {
        macro_rules! add {
            ($ident:ident, $type:ident) => {
                self.state
                    .add_type(Ident::$ident, BuildInMeta::$type, true)?;
            };
        }

        add!(U8, U8);
        add!(U16, U16);
        add!(U32, U32);
        add!(U64, U64);
        add!(U128, U128);
        add!(USIZE, Usize);

        add!(I8, I8);
        add!(I16, I16);
        add!(I32, I32);
        add!(I64, I64);
        add!(I128, I128);
        add!(ISIZE, Isize);

        add!(F32, F32);
        add!(F64, F64);

        add!(BOOL, Bool);
        add!(STRING, String);

        Ok(self)
    }

    /// Adds the type definitions for common XML types (like `xs:string` or `xs:int`)
    /// to the resulting [`MetaTypes`] structure.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn with_default_typedefs(mut self) -> Result<Self, Error> {
        let xs = self
            .schemas
            .resolve_namespace(&Some(Namespace::XS))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;

        macro_rules! add {
            ($ns:ident, $src:expr, $dst:ident) => {
                self.state.add_type(
                    Ident::type_($src).with_ns(Some($ns)),
                    ReferenceMeta::new(Ident::$dst),
                    true,
                )?;
            };
        }
        macro_rules! add_list {
            ($ns:ident, $src:expr, $dst:ident) => {
                self.state.add_type(
                    Ident::type_($src).with_ns(Some($ns)),
                    ReferenceMeta::new(Ident::$dst)
                        .min_occurs(0)
                        .max_occurs(MaxOccurs::Unbounded),
                    true,
                )?;
            };
        }

        /* Primitive Types */

        add!(xs, "string", STRING);
        add!(xs, "boolean", BOOL);
        add!(xs, "decimal", F64);
        add!(xs, "float", F32);
        add!(xs, "double", F64);

        /* time related types */

        add!(xs, "duration", STRING);
        add!(xs, "dateTime", STRING);
        add!(xs, "time", STRING);
        add!(xs, "date", STRING);
        add!(xs, "gYearMonth", STRING);
        add!(xs, "gYear", STRING);
        add!(xs, "gMonthDay", STRING);
        add!(xs, "gMonth", STRING);
        add!(xs, "gDay", STRING);

        /* Data related types */

        add!(xs, "hexBinary", STRING);
        add!(xs, "base64Binary", STRING);

        /* URL related types */

        add!(xs, "anyURI", STRING);
        add!(xs, "QName", STRING);
        add!(xs, "NOTATION", STRING);

        /* Numeric Types */

        add!(xs, "long", I64);
        add!(xs, "int", I32);
        add!(xs, "integer", I32);
        add!(xs, "short", I16);
        add!(xs, "byte", I8);
        add!(xs, "negativeInteger", ISIZE);
        add!(xs, "nonPositiveInteger", ISIZE);

        add!(xs, "unsignedLong", U64);
        add!(xs, "unsignedInt", U32);
        add!(xs, "unsignedShort", U16);
        add!(xs, "unsignedByte", U8);
        add!(xs, "positiveInteger", USIZE);
        add!(xs, "nonNegativeInteger", USIZE);

        /* String Types */

        add!(xs, "normalizedString", STRING);
        add!(xs, "token", STRING);
        add!(xs, "language", STRING);
        add!(xs, "NMTOKEN", STRING);
        add!(xs, "Name", STRING);
        add!(xs, "NCName", STRING);
        add!(xs, "ID", STRING);
        add!(xs, "IDREF", STRING);

        add!(xs, "anySimpleType", STRING);

        add_list!(xs, "NMTOKENS", STRING);
        add_list!(xs, "IDREFS", STRING);
        add_list!(xs, "ENTITY", STRING);
        add_list!(xs, "ENTITIES", STRING);

        Ok(self)
    }

    /// Adds a default type definition for `xs:anyType`.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn with_xs_any_type(mut self) -> Result<Self, Error> {
        let xs = self
            .schemas
            .resolve_namespace(&Some(Namespace::XS))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;

        /* content type */

        let any_name = Name::named("any");
        let any_ident = Ident::new(any_name).with_type(IdentType::Element);
        let mut any = ElementMeta::any(
            any_ident,
            AnyMeta {
                id: None,
                namespace: None,
                not_q_name: None,
                not_namespace: None,
                process_contents: ProcessContentsType::Lax,
            },
        );
        any.min_occurs = 0;
        any.max_occurs = MaxOccurs::Unbounded;

        let mut content_sequence = GroupMeta::default();
        content_sequence.elements.push_any(any);

        let content_name = self.state.name_builder().shared_name("Content").finish();
        let content_ident = Ident::new(content_name).with_ns(Some(xs));
        let content_variant = MetaTypeVariant::Sequence(content_sequence);
        let content_type = MetaType::new(content_variant);

        self.state
            .add_type(content_ident.clone(), content_type, true)?;

        /* xs:anyType */

        let ident = Ident::type_("anyType").with_ns(Some(xs));

        let any_attribute_name = Name::named("any_attribute");
        let any_attribute_ident = Ident::new(any_attribute_name).with_type(IdentType::Attribute);
        let any_attribute = AttributeMeta::any(
            any_attribute_ident,
            AnyAttributeMeta {
                id: None,
                namespace: None,
                not_q_name: None,
                not_namespace: None,
                process_contents: ProcessContentsType::Lax,
            },
        );

        let mut complex = ComplexMeta {
            content: Some(content_ident),
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
            ..Default::default()
        };
        complex.attributes.push(any_attribute);

        let variant = MetaTypeVariant::ComplexType(complex);
        let type_ = MetaType::new(variant);

        self.state.add_type(ident, type_, true)?;

        Ok(self)
    }

    /// Add type definitions for numeric XML types (like `xs:int`) that
    /// uses `num::BigInt` and `num::BigUint` instead of build-in integer types.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    pub fn with_num_big_int(mut self) -> Result<Self, Error> {
        let xs = self
            .schemas
            .resolve_namespace(&Some(Namespace::XS))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;

        macro_rules! add {
            ($ns:ident, $src:expr, $dst:literal) => {{
                self.state.add_type(
                    Ident::type_($src).with_ns(Some($ns)),
                    ReferenceMeta::new(Ident::type_($dst)),
                    true,
                )?;
            }};
        }

        let big_int = CustomMeta::new("BigInt")
            .include_from("num::BigInt")
            .with_default(|s: &str| {
                let code = quote! {
                    <num::BigInt as core::str::FromStr>::from_str(#s).unwrap()
                };

                Some(code)
            });

        let big_uint = CustomMeta::new("BigUint")
            .include_from("num::BigUint")
            .with_default(|s: &str| {
                let code = quote! {
                    <num::BigUint as core::str::FromStr>::from_str(#s).unwrap()
                };

                Some(code)
            });

        self.state.add_type(Ident::type_("BigInt"), big_int, true)?;
        self.state
            .add_type(Ident::type_("BigUint"), big_uint, true)?;

        add!(xs, "integer", "BigInt");
        add!(xs, "positiveInteger", "BigUint");
        add!(xs, "nonNegativeInteger", "BigUint");
        add!(xs, "negativeInteger", "BigInt");
        add!(xs, "nonPositiveInteger", "BigInt");

        Ok(self)
    }

    /// Finishes the interpretation of the [`Schemas`] structure and returns
    /// the [`MetaTypes`] structure with the generated type information.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn finish(mut self) -> Result<MetaTypes, Error> {
        for (id, info) in self.schemas.namespaces() {
            let prefix = info
                .prefix
                .as_ref()
                .map(ToString::to_string)
                .map(Name::new_named);
            let name = info
                .module_name
                .clone()
                .map(Name::new_named)
                .or_else(|| prefix.clone());
            let namespace = info.namespace.clone();

            let module = ModuleMeta {
                name,
                prefix,
                namespace,
            };

            self.state.types.modules.insert(*id, module);
        }

        for (_id, schema) in self.schemas.schemas() {
            SchemaInterpreter::process(&mut self.state, schema, self.schemas)?;
        }

        Ok(self.state.types)
    }
}
