use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::ops::Deref;

use bitflags::bitflags;
use proc_macro2::{Ident as Ident2, TokenStream};
use quote::quote;

use crate::code::IdentPath;
use crate::schema::{MaxOccurs, MinOccurs};
use crate::types::{DynamicInfo, Ident, Type, TypeVariant, Types};

bitflags! {
    /// Different flags that control what code the [`Generator`](super::Generator)
    /// is generating.
    #[derive(Debug, Clone, Copy)]
    pub struct GeneratorFlags: u32 {
        /// None of the features are enabled.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/generator_flags/schema.xsd")]
        /// ```
        ///
        /// Setting none of the flags will result in the following code:
        /// ```rust
        #[doc = include_str!("../../tests/generator/generator_flags/expected/empty.rs")]
        /// ```
        const NONE = 0;

        /// The generated code uses modules for the different namespaces.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/generator_flags/schema.xsd")]
        /// ```
        ///
        /// Enable the `USE_MODULES` feature only will result in the following code:
        /// ```rust,ignore
        #[doc = include_str!("../../tests/generator/generator_flags/expected/use_modules.rs")]
        /// ```
        const USE_MODULES = 1 << 0;

        /// The generator flattens the content type of choice types if it does not
        /// define any element attributes.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/generator_flags/schema.xsd")]
        /// ```
        ///
        /// Enable the `FLATTEN_CONTENT` feature only will result in the following code:
        /// ```rust
        #[doc = include_str!("../../tests/generator/generator_flags/expected/flatten_content.rs")]
        /// ```
        const FLATTEN_CONTENT = Self::FLATTEN_ENUM_CONTENT.bits()
            | Self::FLATTEN_STRUCT_CONTENT.bits();

        /// The generator flattens the content of enum types if possible.
        ///
        /// See [`FLATTEN_CONTENT`](Self::FLATTEN_CONTENT) for details.
        const FLATTEN_ENUM_CONTENT = 1 << 1;

        /// The generator flattens the content of struct types if possible.
        ///
        /// See [`FLATTEN_CONTENT`](Self::FLATTEN_CONTENT) for details.
        const FLATTEN_STRUCT_CONTENT = 1 << 2;

        /// The generator adds documentation to the rendered code if a
        /// `xs:documentation` element was placed in the schema.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/generator_flags/schema_with_docs.xsd")]
        /// ```
        ///
        /// Enable the `RENDER_DOCS` feature only will result in the following code:
        /// ```rust
        #[doc = include_str!("../../tests/generator/generator_flags/expected/render_docs.rs")]
        /// ```
        const RENDER_DOCS = Self::RENDER_TYPE_DOCS.bits()
            | Self::RENDER_ELEMENT_DOCS.bits()
            | Self::RENDER_ATTRIBUTE_DOCS.bits()
            | Self::RENDER_VARIANT_DOCS.bits();

        /// The generator adds documentation to the rendered types if a
        /// `xs:documentation` element was placed in the schema.
        ///
        /// See [`RENDER_DOCS`](Self::RENDER_DOCS) for details.
        const RENDER_TYPE_DOCS = 1 << 3;

        /// The generator adds documentation to the rendered elements if a
        /// `xs:documentation` element was placed in the schema.
        ///
        /// See [`RENDER_DOCS`](Self::RENDER_DOCS) for details.
        const RENDER_ELEMENT_DOCS = 1 << 4;

        /// The generator adds documentation to the rendered attributes if a
        /// `xs:documentation` element was placed in the schema.
        ///
        /// See [`RENDER_DOCS`](Self::RENDER_DOCS) for details.
        const RENDER_ATTRIBUTE_DOCS = 1 << 5;

        /// The generator adds documentation to the rendered enum variants for
        /// `xs:enumeration` types if a `xs:documentation` element was placed
        /// in the schema.
        ///
        /// See [`RENDER_DOCS`](Self::RENDER_DOCS) for details.
        const RENDER_VARIANT_DOCS = 1 << 6;
    }
}

bitflags! {
    /// Flags to tell the [`Generator`](super::Generator) how to deal with boxed
    /// types.
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
    pub struct BoxFlags: u32 {
        /// Boxed types will only be used if necessary.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/box_flags/schema.xsd")]
        /// ```
        ///
        /// Enable the `AUTO` feature only will result in the following code:
        /// ```rust
        #[doc = include_str!("../../tests/generator/box_flags/expected/auto.rs")]
        /// ```
        const AUTO = 0;

        /// Elements in a `xs:choice` type will always be boxed.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/box_flags/schema.xsd")]
        /// ```
        ///
        /// Enable the `ENUM_ELEMENTS` feature only will result in the following code:
        /// ```rust
        #[doc = include_str!("../../tests/generator/box_flags/expected/enum_elements.rs")]
        /// ```
        const ENUM_ELEMENTS = 1 << 0;

        /// Elements in a `xs:all` or `xs:sequence` type will always be boxed.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/box_flags/schema.xsd")]
        /// ```
        ///
        /// Enable the `STRUCT_ELEMENTS` feature only will result in the following code:
        /// ```rust
        #[doc = include_str!("../../tests/generator/box_flags/expected/struct_elements.rs")]
        /// ```
        const STRUCT_ELEMENTS = 1 << 1;
    }
}

/// Tells the [`Generator`](super::Generator) how to deal with type definitions.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum TypedefMode {
    /// The [`Generator`](super::Generator) will automatically detect if a
    /// new type struct or a simple type definition should be used
    /// for a [`Reference`](TypeVariant::Reference) type.
    ///
    /// Detecting the correct type automatically depends basically on the
    /// occurrence of the references type. If the target type is only referenced
    /// exactly once, a type definition is rendered. If a different
    /// occurrence is used, it is wrapped in a new type struct because usually
    /// additional code needs to be implemented for such types.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/generator/typedef_mode/schema.xsd")]
    /// ```
    ///
    /// If the typedef mode is set to [`TypedefMode::Auto`] the following code is rendered:
    /// ```rust
    #[doc = include_str!("../../tests/generator/typedef_mode/expected/auto.rs")]
    /// ```
    #[default]
    Auto,

    /// The [`Generator`](super::Generator) will always use a simple type definition
    /// for a [`Reference`](TypeVariant::Reference) type.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/generator/typedef_mode/schema.xsd")]
    /// ```
    ///
    /// If the typedef mode is set to [`TypedefMode::Typedef`] the following code is rendered:
    /// ```rust
    #[doc = include_str!("../../tests/generator/typedef_mode/expected/typedef.rs")]
    /// ```
    Typedef,

    /// The [`Generator`](super::Generator) will always use a new type struct
    /// for a [`Reference`](TypeVariant::Reference) type.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/generator/typedef_mode/schema.xsd")]
    /// ```
    ///
    /// If the typedef mode is set to [`TypedefMode::NewType`] the following code is rendered:
    /// ```rust
    #[doc = include_str!("../../tests/generator/typedef_mode/expected/new_type.rs")]
    /// ```
    NewType,
}

/// Tells the [`Generator`](super::Generator) how to generate code for the
/// [`serde`] crate.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum SerdeSupport {
    /// No code for the [`serde`] crate is generated.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/generator/serde_support/schema.xsd")]
    /// ```
    ///
    /// If the serde support mode is set to [`SerdeSupport::None`] the following code is rendered:
    /// ```rust
    #[doc = include_str!("../../tests/generator/serde_support/expected/none.rs")]
    /// ```
    #[default]
    None,

    /// Generates code that can be serialized and deserialized using the
    /// [`serde`] create in combination with the with [`quick_xml`] crate.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/generator/serde_support/schema.xsd")]
    /// ```
    ///
    /// If the serde support mode is set to [`SerdeSupport::QuickXml`] the following code is rendered:
    /// ```rust
    #[doc = include_str!("../../tests/generator/serde_support/expected/quick_xml.rs")]
    /// ```
    QuickXml,

    /// Generates code that can be serialized and deserialized using the
    /// [`serde`] create in combination with the with [`serde-xml-rs`](https://docs.rs/serde-xml-rs) crate.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/generator/serde_support/schema.xsd")]
    /// ```
    ///
    /// If the serde support mode is set to [`SerdeSupport::SerdeXmlRs`] the following code is rendered:
    /// ```rust
    #[doc = include_str!("../../tests/generator/serde_support/expected/serde_xml_rs.rs")]
    /// ```
    SerdeXmlRs,
}

impl SerdeSupport {
    /// Returns `true` if this is equal to [`SerdeSupport::None`], `false` otherwise.
    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `false` if this is equal to [`SerdeSupport::None`], `true` otherwise.
    #[must_use]
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Defines the occurrence (how often the field is used) of a field in a specific type.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Occurs {
    /// The field is not used at all.
    #[default]
    None,

    /// The field is used exactly one time.
    Single,

    /// The field is used optional (zero or exactly one time).
    Optional,

    /// The field is used as a dynamic list.
    DynamicList,

    /// The field is used as a list with a fixed size.
    StaticList(usize),
}

impl Occurs {
    /// Create the [`Occurs`] value from the [`MinOccurs`] and [`MaxOccurs`] from
    /// the XSD schema.
    #[must_use]
    pub fn from_occurs(min: MinOccurs, max: MaxOccurs) -> Self {
        match (min, max) {
            (0, MaxOccurs::Bounded(0)) => Self::None,
            (1, MaxOccurs::Bounded(1)) => Self::Single,
            (0, MaxOccurs::Bounded(1)) => Self::Optional,
            (a, MaxOccurs::Bounded(b)) if a == b => Self::StaticList(a),
            (_, _) => Self::DynamicList,
        }
    }

    /// Wrapped the passed type `ident` into a suitable rust type depending on
    /// the occurrence and the need of indirection (boxing).
    ///
    /// # Examples
    /// - `Occurs::Single` will return the type as is, or as `Box<T>`
    /// - `Occurs::Optional` will return the type as `Option<T>`
    /// - `Occurs::DynamicList` will return the type as `Vec<T>`
    /// - `Occurs::StaticList` will return the type as array `[T; SIZE]`
    #[must_use]
    pub fn make_type(self, ident: &TokenStream, need_indirection: bool) -> Option<TokenStream> {
        match self {
            Self::None => None,
            Self::Single if need_indirection => Some(quote! { Box<#ident> }),
            Self::Single => Some(quote! { #ident }),
            Self::Optional if need_indirection => Some(quote! { Option<Box<#ident>> }),
            Self::Optional => Some(quote! { Option<#ident> }),
            Self::DynamicList => Some(quote! { Vec<#ident> }),
            Self::StaticList(sz) if need_indirection => Some(quote! { [Box<#ident>; #sz] }),
            Self::StaticList(sz) => Some(quote! { [#ident; #sz] }),
        }
    }

    /// Returns `true` if not `Occurs::None`
    #[must_use]
    pub fn is_some(&self) -> bool {
        *self != Self::None
    }

    /// Returns `true` if [`make_type`](Self::make_type) would generate a static type
    /// (a type without memory indirection), like `T`, `Option<T>` or `[T; SIZE]`.
    #[must_use]
    pub fn is_direct(&self) -> bool {
        matches!(self, Self::Single | Self::Optional | Self::StaticList(_))
    }
}

/// Defines which additional traits should be implemented by the generated traits
/// of dynamic types.
#[derive(Default, Debug)]
pub enum DynTypeTraits {
    /// The traits will be detected automatically.
    #[default]
    Auto,

    /// List of trait identifiers that will be implemented.
    Custom(Vec<IdentPath>),
}

/* PendingType */

#[derive(Debug)]
pub(super) struct PendingType<'types> {
    pub ty: &'types Type,
    pub ident: Ident,
}

/* TypeRef */

#[derive(Debug)]
pub(super) struct TypeRef {
    pub ident: Ident,
    pub type_ident: Ident2,
    pub module_ident: Option<Ident2>,
    pub boxed_elements: HashSet<Ident>,
}

impl TypeRef {
    pub(super) fn to_ident_path(&self) -> IdentPath {
        if self.ident.is_build_in() {
            IdentPath::from_ident(self.type_ident.clone())
        } else {
            IdentPath::from_ident(self.type_ident.clone()).with_path(self.module_ident.clone())
        }
    }
}

/* TraitInfos */

#[derive(Debug)]
pub(super) struct TraitInfos(BTreeMap<Ident, TraitInfo>);

impl TraitInfos {
    #[must_use]
    pub(super) fn new(types: &Types) -> Self {
        let mut ret = Self(BTreeMap::new());

        for (base_ident, ty) in types.iter() {
            let TypeVariant::Dynamic(ai) = &ty.variant else {
                continue;
            };

            for type_ident in &ai.derived_types {
                ret.0
                    .entry(type_ident.clone())
                    .or_default()
                    .traits_all
                    .insert(base_ident.clone());

                match types.get_variant(type_ident) {
                    Some(TypeVariant::Dynamic(DynamicInfo {
                        type_: Some(type_ident),
                        ..
                    })) => {
                        ret.0
                            .entry(type_ident.clone())
                            .or_default()
                            .traits_all
                            .insert(base_ident.clone());
                    }
                    Some(TypeVariant::Reference(ri)) if ri.is_single() => {
                        ret.0
                            .entry(ri.type_.clone())
                            .or_default()
                            .traits_all
                            .insert(base_ident.clone());
                    }
                    _ => (),
                }
            }
        }

        for ident in ret.0.keys().cloned().collect::<Vec<_>>() {
            let mut traits_second_level = BTreeSet::new();

            ret.collect_traits(&ident, 0, &mut traits_second_level);

            let info = ret.0.get_mut(&ident).unwrap();
            info.traits_direct = info
                .traits_all
                .difference(&traits_second_level)
                .cloned()
                .collect();
        }

        ret
    }

    fn collect_traits(
        &self,
        ident: &Ident,
        depth: usize,
        traits_second_level: &mut BTreeSet<Ident>,
    ) {
        if depth > 1 {
            traits_second_level.insert(ident.clone());
        }

        if let Some(info) = self.0.get(ident) {
            for trait_ in &info.traits_all {
                self.collect_traits(trait_, depth + 1, traits_second_level);
            }
        }
    }
}

impl Deref for TraitInfos {
    type Target = BTreeMap<Ident, TraitInfo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/* TraitInfo */

#[derive(Default, Debug)]
pub(super) struct TraitInfo {
    pub traits_all: BTreeSet<Ident>,
    pub traits_direct: BTreeSet<Ident>,
}
