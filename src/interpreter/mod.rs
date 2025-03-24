//! The `interpreter` module contains the schema [`Interpreter`] and all related types.

mod error;
mod helper;
mod schema;
mod state;
mod variant_builder;

use std::fmt::Debug;

use crate::config::Namespace;
use crate::schema::xs::ProcessContentsType;
use crate::schema::{MaxOccurs, Schemas};
use crate::types::{
    AnyAttributeInfo, AnyInfo, BuildInInfo, ComplexInfo, GroupInfo, Ident, Module, Name,
    ReferenceInfo, Type, TypeVariant, Types,
};

pub use error::Error;
use tracing::instrument;

use self::helper::{NameExtend, NameFallback, NameUnwrap};
use self::schema::SchemaInterpreter;
use self::state::{Node, State};
use self::variant_builder::VariantBuilder;

/// The [`Interpreter`] is used to interpret the XML schema information.
///
/// This structure can be used to interpret the [`Schemas`] structure that was
/// loaded by the [`Parser`](crate::parser::Parser) to generate the more common
/// [`Types`] definition out of it.
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

    /// Add a custom [`Type`] information for the passed `ident`ifier to the
    /// resulting [`Types`] structure.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn with_type<I, T>(mut self, ident: I, type_: T) -> Result<Self, Error>
    where
        I: Into<Ident> + Debug,
        T: Into<Type> + Debug,
    {
        self.state.add_type(ident, type_, true)?;

        Ok(self)
    }

    /// Add a simple type definition to the resulting [`Types`] structure using
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
            .add_type(ident, ReferenceInfo::new(type_), true)?;

        Ok(self)
    }

    /// Adds the default build-in types to the resulting [`Types`] structure.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn with_buildin_types(mut self) -> Result<Self, Error> {
        macro_rules! add {
            ($ident:ident, $type:ident) => {
                self.state
                    .add_type(Ident::$ident, BuildInInfo::$type, true)?;
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
    /// to the resulting [`Types`] structure.
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
                    ReferenceInfo::new(Ident::$dst),
                    true,
                )?;
            };
        }
        macro_rules! add_list {
            ($ns:ident, $src:expr, $dst:ident) => {
                self.state.add_type(
                    Ident::type_($src).with_ns(Some($ns)),
                    ReferenceInfo::new(Ident::$dst)
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

        /* Date related types */

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

        // content type
        let content_name = self.state.make_unnamed();
        let content_ident = Ident::new(content_name).with_ns(Some(xs));
        let content_variant = TypeVariant::Sequence(GroupInfo {
            any: Some(AnyInfo {
                min_occurs: Some(0),
                max_occurs: Some(MaxOccurs::Unbounded),
                process_contents: Some(ProcessContentsType::Lax),
                ..Default::default()
            }),
            ..Default::default()
        });
        let content_type = Type::new(content_variant);
        self.state
            .add_type(content_ident.clone(), content_type, true)?;

        // xs:anyType
        let ident = Ident::type_("anyType").with_ns(Some(xs));
        let variant = TypeVariant::ComplexType(ComplexInfo {
            content: Some(content_ident),
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
            any_attribute: Some(AnyAttributeInfo {
                process_contents: Some(ProcessContentsType::Lax),
                ..Default::default()
            }),
            ..Default::default()
        });
        let type_ = Type::new(variant);
        self.state.add_type(ident, type_, true)?;

        Ok(self)
    }

    /// Finishes the interpretation of the [`Schemas`] structure and returns
    /// the [`Types`] structure with the generated type information.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn finish(mut self) -> Result<Types, Error> {
        for (id, info) in self.schemas.namespaces() {
            let module = Module {
                name: info
                    .prefix
                    .as_ref()
                    .map(|prefix| Name::new(prefix.to_string())),
                namespace: info.namespace.clone(),
            };

            self.state.types.modules.insert(*id, module);
        }

        for (_id, schema) in self.schemas.schemas() {
            SchemaInterpreter::process(&mut self.state, schema, self.schemas)?;
        }

        Ok(self.state.types)
    }
}
