use bitflags::bitflags;

use super::IdentTriple;

/// Configuration for the code generator.
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Types to add to the generator before the actual code is generated.
    ///
    /// See [`with_type`](crate::Generator::with_type) for more details.
    pub types: Vec<IdentTriple>,

    /// Postfixes that should be applied to the name of the different generated
    /// types.
    ///
    /// See [`with_type_postfix`](crate::Generator::with_type_postfix) for more details.
    pub type_postfix: TypePostfix,

    /// Tell the generator how to deal with boxing.
    pub box_flags: BoxFlags,

    /// Tells the generator how to deal with type definitions.
    pub typedef_mode: TypedefMode,

    /// Tells the generator how to generate code for the [`serde`] crate.
    pub serde_support: SerdeSupport,

    /// Specify which types the generator should generate code for.
    pub generate: Generate,

    /// Additional flags to control the generator.
    pub flags: GeneratorFlags,

    /// Type to use to store unstructured `xs:any` elements.
    ///
    /// See [`Generator::any_type`](crate::Generator::any_type) for details.
    pub any_type: Option<String>,

    /// Type to use to store unstructured `xs:anyAttribute` attributes.
    ///
    /// See [`Generator::any_attribute_type`](crate::Generator::any_attribute_type)
    /// for details.
    pub any_attribute_type: Option<String>,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            types: vec![],
            type_postfix: TypePostfix::default(),
            box_flags: BoxFlags::AUTO,
            typedef_mode: TypedefMode::Auto,
            serde_support: SerdeSupport::None,
            generate: Generate::Named,
            flags: GeneratorFlags::empty(),
            any_type: None,
            any_attribute_type: None,
        }
    }
}

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

/// Postfixed that will be added to the different types generated by the code generator.
#[derive(Debug, Clone)]
pub struct TypePostfix {
    /// Postfix added to normal types (like `xs:simpleType` or `xs:complexType`).
    pub type_: String,

    /// Postfixes added to elements (like `xs:element`).
    pub element: String,

    /// Postfixes added to inline types if elements (like `xs:element`).
    pub element_type: String,
}

impl Default for TypePostfix {
    fn default() -> Self {
        Self {
            type_: String::from("Type"),
            element: String::new(),
            element_type: String::from("ElementType"),
        }
    }
}

/// Tells the [`Generator`](super::Generator) how to deal with type definitions.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum TypedefMode {
    /// The [`Generator`](super::Generator) will automatically detect if a
    /// new type struct or a simple type definition should be used
    /// for a [`Reference`](MetaTypeVariant::Reference) type.
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
    /// for a [`Reference`](MetaTypeVariant::Reference) type.
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
    /// for a [`Reference`](MetaTypeVariant::Reference) type.
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

/// Configuration which types the [`Generator`](crate::Generator) should generate
/// code for used in [`GeneratorConfig`].
#[derive(Debug, Clone)]
pub enum Generate {
    /// The generator will generate code for all types of the schemas.
    All,

    /// The generator will generate code for all types that have a name.
    Named,

    /// List of identifiers the generator will generate code for.
    Types(Vec<IdentTriple>),
}
