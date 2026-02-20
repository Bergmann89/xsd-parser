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
mod state;

use std::fmt::Debug;

use quote::quote;
use tracing::instrument;

use xsd_parser_types::misc::Namespace;

use crate::models::meta::{
    AnyAttributeMeta, AnyMeta, AttributeMeta, BuildInMeta, ComplexMeta, CustomMeta, ElementMeta,
    GroupMeta, MetaType, MetaTypeVariant, MetaTypes, ReferenceMeta, SimpleMeta,
};
use crate::models::schema::{
    xs::{FormChoiceType, ProcessContentsType},
    MaxOccurs, NamespaceId, Schemas,
};
use crate::models::{AttributeIdent, ElementIdent, IdentCache, Name, TypeIdent};
use crate::pipeline::generator::{
    Context as GeneratorContext, Error as GeneratorError, ValueGenerator, ValueGeneratorMode,
};
use crate::pipeline::renderer::{Context as RendererContext, ValueRendererBox};
use crate::traits::{NameBuilderExt as _, Naming};

pub use error::Error;

use self::state::State;

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
}

impl<'a> Interpreter<'a> {
    /// Create a new [`Interpreter`] instance using the passed `schemas` reference.
    pub fn new(schemas: &'a Schemas) -> Self {
        let state = State::new(schemas);

        Self { state }
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
        I: Into<TypeIdent> + Debug,
        T: Into<MetaType> + Debug,
    {
        self.state.add_type(ident, type_)?;

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
        I: Into<TypeIdent> + Debug,
        T: Into<TypeIdent> + Debug,
    {
        self.state.add_type(ident, ReferenceMeta::new(type_))?;

        Ok(self)
    }

    /// Adds the default build-in types to the resulting [`MetaTypes`] structure.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn with_buildin_types(mut self) -> Result<Self, Error> {
        let anonymous = self
            .state
            .schemas()
            .resolve_namespace(&None)
            .ok_or_else(|| Error::AnonymousNamespaceIsUndefined)?;

        macro_rules! add {
            ($ident:ident, $type:ident) => {{
                let ident = TypeIdent::$ident.with_ns(anonymous);
                let ty = BuildInMeta::$type;

                self.state.add_type(ident, ty)?;
            }};
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
        add!(STR, Str);
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
        let anonymous = self
            .state
            .schemas()
            .resolve_namespace(&None)
            .ok_or_else(|| Error::AnonymousNamespaceIsUndefined)?;
        let xs = self
            .state
            .schemas()
            .resolve_namespace(&Some(Namespace::XS))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;

        macro_rules! add {
            ($src:expr, $dst:ident) => {{
                let src = TypeIdent::type_($src).with_ns(xs);
                let dst = TypeIdent::$dst.with_ns(anonymous);

                self.state.add_type(src, ReferenceMeta::new(dst))?;
            }};
        }
        macro_rules! add_list {
            ($src:expr, $dst:ident) => {{
                let src = TypeIdent::type_($src).with_ns(xs);
                let dst = TypeIdent::$dst.with_ns(anonymous);

                self.state.add_type(
                    src,
                    ReferenceMeta::new(dst)
                        .min_occurs(0)
                        .max_occurs(MaxOccurs::Unbounded),
                )?;
            }};
        }

        /* Primitive Types */

        add!("string", STRING);
        add!("boolean", BOOL);
        add!("decimal", F64);
        add!("float", F32);
        add!("double", F64);

        /* time related types */

        add!("duration", STRING);
        add!("dateTime", STRING);
        add!("time", STRING);
        add!("date", STRING);
        add!("gYearMonth", STRING);
        add!("gYear", STRING);
        add!("gMonthDay", STRING);
        add!("gMonth", STRING);
        add!("gDay", STRING);

        /* Data related types */

        add!("hexBinary", STRING);
        add!("base64Binary", STRING);

        /* URL related types */

        add!("anyURI", STRING);
        add!("QName", STRING);
        add!("NOTATION", STRING);

        /* Numeric Types */

        add!("long", I64);
        add!("int", I32);
        add!("integer", I32);
        add!("short", I16);
        add!("byte", I8);
        add!("negativeInteger", ISIZE);
        add!("nonPositiveInteger", ISIZE);

        add!("unsignedLong", U64);
        add!("unsignedInt", U32);
        add!("unsignedShort", U16);
        add!("unsignedByte", U8);
        add!("positiveInteger", USIZE);
        add!("nonNegativeInteger", USIZE);

        /* String Types */

        add!("normalizedString", STRING);
        add!("token", STRING);
        add!("language", STRING);
        add!("NMTOKEN", STRING);
        add!("Name", STRING);
        add!("NCName", STRING);
        add!("ID", STRING);
        add!("IDREF", STRING);
        add!("ENTITY", STRING);

        add!("anySimpleType", STRING);

        add_list!("NMTOKENS", STRING);
        add_list!("IDREFS", STRING);
        add_list!("ENTITIES", STRING);

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
            .state
            .schemas()
            .resolve_namespace(&Some(Namespace::XS))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;

        /* content type */

        let any_ident = ElementIdent::new(Name::ANY);
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

        let mut content_sequence = GroupMeta {
            is_mixed: true,
            ..GroupMeta::default()
        };
        content_sequence.elements.push(any);

        let content_name = self.state.name_builder().shared_name("Content").finish();
        let content_ident = TypeIdent::new(content_name).with_ns(xs);
        let content_variant = MetaTypeVariant::Sequence(content_sequence);
        let content_type = MetaType::new(content_variant);

        self.state.add_type(content_ident.clone(), content_type)?;

        /* xs:anyType */

        let ident = TypeIdent::new(Name::ANY_TYPE).with_ns(xs);
        let any_attribute_ident = AttributeIdent::new(Name::ANY_ATTRIBUTE);
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
            is_mixed: true,
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
            ..Default::default()
        };
        complex.attributes.push(any_attribute);

        let variant = MetaTypeVariant::ComplexType(complex);
        let type_ = MetaType::new(variant);

        self.state.add_type(ident, type_)?;

        Ok(self)
    }

    /// Adds a default type definition for `xs:anySimpleType`.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn with_xs_any_simple_type(mut self) -> Result<Self, Error> {
        let xs = self
            .state
            .schemas()
            .resolve_namespace(&Some(Namespace::XS))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;
        let xsi = self
            .state
            .schemas()
            .resolve_namespace(&Some(Namespace::XSI))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XSI.clone()))?;

        /* content type */

        let content_name = self.state.name_builder().shared_name("Content").finish();
        let content_ident = TypeIdent::new(content_name).with_ns(xs);
        let content_type = MetaType::new(MetaTypeVariant::SimpleType(SimpleMeta::new(
            TypeIdent::STRING,
        )));

        self.state.add_type(content_ident.clone(), content_type)?;

        /* xs:anySimpleType */

        let type_attribute_ident = AttributeIdent::new(Name::TYPE).with_ns(xsi);
        let type_attribute = AttributeMeta::new(
            type_attribute_ident,
            TypeIdent::STRING,
            FormChoiceType::Qualified,
        );

        let mut complex = ComplexMeta {
            content: Some(content_ident),
            is_mixed: true,
            min_occurs: 1,
            max_occurs: MaxOccurs::Bounded(1),
            ..Default::default()
        };
        complex.attributes.push(type_attribute);

        let ident = TypeIdent::new(Name::ANY_SIMPLE_TYPE).with_ns(xs);
        let variant = MetaTypeVariant::ComplexType(complex);
        let type_ = MetaType::new(variant);

        self.state.add_type(ident, type_)?;

        Ok(self)
    }

    /// Add a type definition for `xs:QName` that uses the
    /// `xsd_parser_types::xml::QName` type.
    pub fn with_qname_type(self) -> Result<Self, Error> {
        self.with_qname_type_from("::xsd_parser_types::xml::QName")
    }

    /// Add a type definition for `xs:QName` that uses the type defined at the passed `path`.
    pub fn with_qname_type_from(self, path: &str) -> Result<Self, Error> {
        let xs = self
            .state
            .schemas()
            .resolve_namespace(&Some(Namespace::XS))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;

        let name = path.rsplit_once("::").map_or(path, |(_, name)| name);

        self.with_type(
            TypeIdent::type_("QName").with_ns(xs),
            CustomMeta::new(name)
                .include_from(path)
                .with_namespace(xs)
                .with_default(crate::misc::qname_default),
        )
    }

    /// Add type definitions for numeric XML types (like `xs:int`) that
    /// uses `num::BigInt` and `num::BigUint` instead of build-in integer types.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    pub fn with_num_big_int(mut self) -> Result<Self, Error> {
        let xs = self
            .state
            .schemas()
            .resolve_namespace(&Some(Namespace::XS))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;

        macro_rules! add {
            ($src:expr, $dst:expr) => {{
                self.state
                    .add_type(TypeIdent::type_($src).with_ns(xs), ReferenceMeta::new($dst))?;
            }};
        }

        let big_int = CustomMeta::new("BigInt")
            .include_from("::num::BigInt")
            .with_default(make_from_str_value_generator("::num::BigInt"));

        let big_uint = CustomMeta::new("BigUint")
            .include_from("::num::BigUint")
            .with_default(make_from_str_value_generator("::num::BigUint"));

        let ident_big_int = TypeIdent::type_("BigInt").with_ns(NamespaceId::ANONYMOUS);
        let ident_big_uint = TypeIdent::type_("BigUint").with_ns(NamespaceId::ANONYMOUS);

        self.state.add_type(ident_big_int.clone(), big_int)?;
        self.state.add_type(ident_big_uint.clone(), big_uint)?;

        add!("integer", ident_big_int.clone());
        add!("negativeInteger", ident_big_int.clone());
        add!("nonPositiveInteger", ident_big_int);

        add!("positiveInteger", ident_big_uint.clone());
        add!("nonNegativeInteger", ident_big_uint);

        Ok(self)
    }

    /// Add type definitions for numeric XML types (like `xs:positiveInteger`) that
    /// uses `::core::num::NonZeroIsize` and `::core::num::NonZeroUsize` instead
    /// of the simple integer types.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    pub fn with_nonzero_typedefs(mut self) -> Result<Self, Error> {
        let xs = self
            .state
            .schemas()
            .resolve_namespace(&Some(Namespace::XS))
            .ok_or_else(|| Error::UnknownNamespace(Namespace::XS.clone()))?;

        macro_rules! add {
            ($src:expr, $dst:expr) => {{
                self.state
                    .add_type(TypeIdent::type_($src).with_ns(xs), ReferenceMeta::new($dst))?;
            }};
        }

        let non_zero_usize = CustomMeta::new("NonZeroUsize")
            .include_from("::core::num::NonZeroUsize")
            .with_default(make_from_str_value_generator("::core::num::NonZeroUsize"));
        let non_zero_isize = CustomMeta::new("NonZeroIsize")
            .include_from("::core::num::NonZeroIsize")
            .with_default(make_from_str_value_generator("::core::num::NonZeroIsize"));

        let ident_non_zero_usize = TypeIdent::type_("NonZeroUsize").with_ns(NamespaceId::ANONYMOUS);
        let ident_non_zero_isize = TypeIdent::type_("NonZeroIsize").with_ns(NamespaceId::ANONYMOUS);

        self.state
            .add_type(ident_non_zero_usize.clone(), non_zero_usize)?;
        self.state
            .add_type(ident_non_zero_isize.clone(), non_zero_isize)?;

        add!("positiveInteger", ident_non_zero_usize);
        add!("negativeInteger", ident_non_zero_isize);

        Ok(self)
    }

    /// Set the [`Naming`](Naming) trait that is used to generate and format names.
    ///
    /// This accepts any type that implements the [`Naming`](Naming) trait.
    /// If you want to use an already boxed version have a look at
    /// [`with_naming_boxed`](Self::with_naming_boxed).
    #[instrument(level = "trace", skip(self))]
    pub fn with_naming<X>(self, naming: X) -> Self
    where
        X: Naming + 'static,
    {
        self.with_naming_boxed(Box::new(naming))
    }

    /// Set the [`Naming`] trait that is used to generate and format names.
    ///
    /// This accepts only boxed [`Naming`](Naming) trait. For easier use you can
    /// use [`with_naming`](Self::with_naming) instead.
    #[instrument(level = "trace", skip(self))]
    pub fn with_naming_boxed(mut self, naming: Box<dyn Naming>) -> Self {
        self.state.set_naming(naming);

        self
    }

    /// Finishes the interpretation of the [`Schemas`] structure and returns
    /// the [`MetaTypes`] structure with the generated type information.
    ///
    /// # Errors
    ///
    /// Returns a suitable [`Error`] if the operation was not successful.
    #[instrument(err, level = "trace", skip(self))]
    pub fn finish(self) -> Result<(MetaTypes, IdentCache), Error> {
        self.state.finish()
    }
}

fn make_from_str_value_generator(type_path: &'static str) -> impl ValueGenerator + 'static {
    move |ctx: &GeneratorContext<'_, '_>,
          value: &str,
          mode: ValueGeneratorMode|
          -> Result<ValueRendererBox, GeneratorError> {
        if mode != ValueGeneratorMode::Value {
            return Err(GeneratorError::InvalidDefaultValue {
                ident: ctx.ident.clone(),
                value: value.into(),
                mode,
            });
        }

        let s = value.to_string();

        Ok(Box::new(move |ctx: &RendererContext<'_, '_>| {
            let type_ = ctx.resolve_ident_path(type_path);
            let from_str = ctx.resolve_ident_path("::core::str::FromStr");

            quote! {
                <#type_ as #from_str>::from_str(#s).unwrap()
            }
        }))
    }
}
