use bitflags::bitflags;

use super::IdentTriple;

/// Configuration for the code generator.
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Additional flags to control the generator.
    pub flags: GeneratorFlags,

    /// Types to add to the generator before the actual data types are generated.
    ///
    /// See [`with_type`](crate::Generator::with_type) for more details.
    pub types: Vec<IdentTriple>,

    /// Specify which meta types the generator should generate data types for.
    pub generate: Generate,

    /// Postfixes that should be applied to the name of the different generated
    /// types.
    ///
    /// See [`with_type_postfix`](crate::Generator::with_type_postfix) for more details.
    pub type_postfix: TypePostfix,

    /// Tell the generator how to deal with boxing.
    pub box_flags: BoxFlags,

    /// Tells the generator how to deal with type definitions.
    pub typedef_mode: TypedefMode,

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
            generate: Generate::Named,
            flags: GeneratorFlags::empty(),
            any_type: None,
            any_attribute_type: None,
        }
    }
}

bitflags! {
    /// Different flags that control what code the [`Generator`](crate::Generator)
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
        const USE_MODULES = Self::USE_NAMESPACE_MODULES.bits();

        /// The generated code uses modules for the different namespaces.
        ///
        /// See [`USE_MODULES`](Self::USE_MODULES) for details.
        const USE_NAMESPACE_MODULES = 1 << 0;

        /// The generated code uses modules for the different schemas.
        ///
        /// See [`USE_MODULES`](Self::USE_MODULES) for details.
        const USE_SCHEMA_MODULES = 1 << 1;

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
        const FLATTEN_ENUM_CONTENT = 1 << 2;

        /// The generator flattens the content of struct types if possible.
        ///
        /// See [`FLATTEN_CONTENT`](Self::FLATTEN_CONTENT) for details.
        const FLATTEN_STRUCT_CONTENT = 1 << 3;

        /// Enable support for mixed types.
        ///
        /// This will enable code generation for mixed types. This feature needs
        /// to be used with caution, because support for mixed types when using
        /// `serde` is quite limited.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/generator_flags/schema.xsd")]
        /// ```
        ///
        /// Enable the `MIXED_TYPE_SUPPORT` feature only will result in the following code:
        /// ```rust,ignore
        #[doc = include_str!("../../tests/generator/generator_flags/expected/mixed_type_support.rs")]
        /// ```
        const MIXED_TYPE_SUPPORT = 1 << 4;

        /// Enable support for nillable types.
        ///
        /// This will enable code generation for nillable types. This feature needs
        /// to be used with caution, because support for nillable types when using
        /// `serde` is quite limited.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/generator_flags/schema.xsd")]
        /// ```
        ///
        /// Enable the `NILLABLE_TYPE_SUPPORT` feature only will result in the following code:
        /// ```rust,ignore
        #[doc = include_str!("../../tests/generator/generator_flags/expected/nillable_type_support.rs")]
        /// ```
        const NILLABLE_TYPE_SUPPORT = 1 << 5;

        /// Use absolute paths for build in types.
        ///
        /// Using this flag will instruct the generator to use absolute paths
        /// for build in types (e.g. `usize` or `String`) to avoid naming
        /// conflicts with generated types.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/generator_flags/schema.xsd")]
        /// ```
        ///
        /// Enable the `BUILD_IN_ABSOLUTE_PATHS` feature only will result in the following code:
        /// ```rust,ignore
        #[doc = include_str!("../../tests/generator/generator_flags/expected/build_in_absolute_paths.rs")]
        /// ```
        const BUILD_IN_ABSOLUTE_PATHS = 1 << 6;
    }
}

bitflags! {
    /// Flags to tell the [`Generator`](crate::Generator) how to deal with boxed
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

/// Tells the [`Generator`](crate::Generator) how to deal with type definitions.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum TypedefMode {
    /// The [`Generator`](crate::Generator) will automatically detect if a
    /// new type struct or a simple type definition should be used
    /// for a [`Reference`](crate::models::meta::MetaTypeVariant::Reference) type.
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

    /// The [`Generator`](crate::Generator) will always use a simple type definition
    /// for a [`Reference`](crate::models::meta::MetaTypeVariant::Reference) type.
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

    /// The [`Generator`](crate::Generator) will always use a new type struct
    /// for a [`Reference`](crate::models::meta::MetaTypeVariant::Reference) type.
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
