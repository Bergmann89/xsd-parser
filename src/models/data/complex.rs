use std::ops::Deref;

use proc_macro2::{Ident as Ident2, Literal, TokenStream};

use crate::models::{
    meta::{AttributeMeta, ElementMeta},
    schema::{MaxOccurs, MinOccurs},
    Ident,
};

use super::{Occurs, PathData};

/// Contains additional information for the rendering process of a
/// [`MetaTypeVariant::All`](crate::models::meta::MetaTypeVariant::All),
/// [`MetaTypeVariant::Choice`](crate::models::meta::MetaTypeVariant::Choice),
/// [`MetaTypeVariant::Sequence`](crate::models::meta::MetaTypeVariant::Sequence)
/// or [`MetaTypeVariant::ComplexType`](crate::models::meta::MetaTypeVariant::ComplexType)
/// type.
///
/// To simplify the rendering process this recursive type was added to the
/// generator. It basically boils down to the following:
///
///   - A complex type with a `choice` will result in a struct with a enum
///     content type.
///   - A complex type with a `all` or `sequence` will result in a struct
///     with a struct content type.
///   - A simple `choice` will result in a single enum type.
///   - A simple `all` or `sequence` will result in a single sequence.
///
/// Additional improvements may be applied to the type, to reduce the complexity
/// of the generated type (for example flattening the content if possible).
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum ComplexData<'types> {
    /// The type represents an enumeration.
    ///
    /// This is normally used for `choice` elements.
    Enum {
        /// The main type.
        type_: ComplexDataEnum<'types>,

        /// The content of the main type (if needed).
        content_type: Option<Box<ComplexData<'types>>>,
    },

    /// The type represents a struct.
    ///
    /// This is normally used for `all`
    Struct {
        /// The main type.
        type_: ComplexDataStruct<'types>,

        /// The content of the main type (if needed).
        content_type: Option<Box<ComplexData<'types>>>,
    },
}

/// Contains basic information for that is shared between [`ComplexDataEnum`]
/// and [`ComplexDataStruct`].
#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct ComplexBase {
    /// The identifier of the rendered type.
    pub type_ident: Ident2,

    /// List of traits that needs to be implemented by this type.
    pub trait_impls: Vec<TokenStream>,

    /// Name of the XML tag of the type (if the type represents an element in the XML).
    pub tag_name: Option<String>,

    /// Whether the type has at least one `xs:any` element or not.
    pub has_any: bool,

    /// `true` if the type is a complex type, `false` otherwise.
    pub is_complex: bool,

    /// `true` if the type is dynamic, `false` otherwise.
    pub is_dynamic: bool,

    /// Identifier of the serializer for this type.
    pub serializer_ident: Ident2,

    /// Identifier of the state of the serializer for this type.
    pub serializer_state_ident: Ident2,

    /// Identifier of the deserializer for this type.
    pub deserializer_ident: Ident2,

    /// Identifier of the state of the deserializer for this type.
    pub deserializer_state_ident: Ident2,
}

/// Represents a rust enum.
///
/// Is used as part of the [`ComplexData`].
#[derive(Debug)]
pub struct ComplexDataEnum<'types> {
    /// Basic type information.
    pub base: ComplexBase,

    /// List of `xs:element`s or variants contained in this enum
    pub elements: Vec<ComplexDataElement<'types>>,

    /// Whether any kind of element is allowed in the enum or not
    ///
    /// This is only true if no type for `xs:any` element is defined.
    pub allow_any: bool,

    /// Whether any kind of attribute is allowed in the enum or not
    ///
    /// This is only true if no type for `xs:anyAttribute` element is defined.
    pub allow_any_attribute: bool,
}

/// Represents a rust struct.
///
/// Is used as part of the [`ComplexData`].
#[derive(Debug)]
pub struct ComplexDataStruct<'types> {
    /// Basic type information.
    pub base: ComplexBase,

    /// Additional information about the content of the struct.
    pub mode: StructMode<'types>,

    /// List of `xs:attribute`s contained in this struct.
    pub attributes: Vec<ComplexDataAttribute<'types>>,

    /// Whether any kind of attribute is allowed in the struct or not
    ///
    /// This is only true if no type for `xs:anyAttribute` element is defined.
    pub allow_any_attribute: bool,
}

/// Content of a rust struct.
///
/// Used by [`ComplexDataStruct`] to tell how the actual content of the struct
/// should be rendered.
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum StructMode<'types> {
    /// The struct does not contain any `xs:element`s.
    Empty {
        /// Whether any kind of element is allowed in the struct or not
        ///
        /// This is only true if no type for `xs:any` element is defined.
        allow_any: bool,
    },

    /// The content of the struct is another generated type that contains
    /// the actual data.
    Content {
        /// Information about the content of the struct.
        content: ComplexDataContent<'types>,
    },

    /// The content of the struct is a `xs:all` group.
    All {
        /// List of `xs:element`s inside the group.
        elements: Vec<ComplexDataElement<'types>>,

        /// Whether any kind of element is allowed in the struct or not
        ///
        /// This is only true if no type for `xs:any` element is defined.
        allow_any: bool,
    },

    /// The content of the struct is a `xs:sequence` group.
    Sequence {
        /// List of `xs:element`s inside the group.
        elements: Vec<ComplexDataElement<'types>>,

        /// Whether any kind of element is allowed in the struct or not
        ///
        /// This is only true if no type for `xs:any` element is defined.
        allow_any: bool,
    },
}

/// Contains details about the content of a struct.
///
/// Is used by [`StructMode`] to define the content of a struct.
#[derive(Debug)]
pub struct ComplexDataContent<'types> {
    /// Occurrence of the content within this struct.
    pub occurs: Occurs,

    /// Type identifier of the content type if this `ComplexDataContent` was
    /// constructed from a complex type with simple content.
    pub simple_type: Option<&'types Ident>,

    /// Minimum occurrence.
    pub min_occurs: MinOccurs,

    /// Maximum occurrence.
    pub max_occurs: MaxOccurs,

    /// Actual target type of the content.
    pub target_type: PathData,
}

/// Contains the details of an XML element.
///
/// Is used in [`ComplexDataEnum`] or [`StructMode`].
#[derive(Debug)]
pub struct ComplexDataElement<'types> {
    /// Origin of the element
    pub origin: ComplexDataElementOrigin<'types>,

    /// Occurrence of the element within it's parent type.
    pub occurs: Occurs,

    /// Name of the element as string.
    pub s_name: String,

    /// Name of the element as byte string literal.
    pub b_name: Literal,

    /// Name of the XML tag of the element.
    pub tag_name: String,

    /// Field identifier of the element.
    pub field_ident: Ident2,

    /// Variant identifier of the element.
    pub variant_ident: Ident2,

    /// Actual target type of the element.
    pub target_type: PathData,

    /// `true` if this element needs some indirection
    /// (like a `Box` or a `Vec`), `false` otherwise.
    pub need_indirection: bool,

    /// `true` if the target type of this element is dynamic,
    /// `false` otherwise.
    pub target_is_dynamic: bool,
}

/// Origin of a [`ComplexDataElement`].
#[derive(Debug)]
pub enum ComplexDataElementOrigin<'types> {
    /// The element was created from a corresponding [`ElementMeta`]
    Meta(&'types ElementMeta),

    /// The element was generated as text element.
    Generated(Box<ElementMeta>),
}

/// Contains the details of an XML attribute.
///
/// Is used in [`ComplexDataStruct`].
#[derive(Debug)]
pub struct ComplexDataAttribute<'types> {
    /// Reference to the original type information.
    pub meta: &'types AttributeMeta,

    /// Identifier of the attribute.
    pub ident: Ident2,

    /// Name of the attribute as string.
    pub s_name: String,

    /// Name of the attribute as byte string literal.
    pub b_name: Literal,

    /// Name of the attribute inside the XML tag.
    pub tag_name: String,

    /// `true` if this attribute is optional, `false` otherwise.
    pub is_option: bool,

    /// The actual target type of the attribute.
    pub target_type: PathData,

    /// The default value of the attribute.
    pub default_value: Option<TokenStream>,
}

impl ComplexBase {
    /// Returns the name of the element tag, if type is represented by a XML element.
    #[must_use]
    pub fn element_tag(&self) -> Option<&String> {
        self.is_complex.then_some(self.tag_name.as_ref()).flatten()
    }

    /// Returns `true` if this type represents an element, `false` otherwise.
    #[must_use]
    pub fn represents_element(&self) -> bool {
        self.is_complex && self.tag_name.is_some() && !self.is_dynamic
    }
}

impl Deref for ComplexDataEnum<'_> {
    type Target = ComplexBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'types> ComplexDataStruct<'types> {
    /// Returns `true` if this struct is a unit struct, `false` otherwise.
    #[must_use]
    pub fn is_unit_struct(&self) -> bool {
        matches!(&self.mode, StructMode::Empty { .. }) && !self.has_attributes()
    }

    /// Returns `true` if this struct has attributes, `false` otherwise.
    #[must_use]
    pub fn has_attributes(&self) -> bool {
        !self.attributes.is_empty()
    }

    /// Returns `true` if this struct has a content field, `false` otherwise.
    #[must_use]
    pub fn has_content(&self) -> bool {
        match &self.mode {
            StructMode::All { elements, .. } | StructMode::Sequence { elements, .. } => {
                !elements.is_empty()
            }
            StructMode::Content { .. } => true,
            StructMode::Empty { .. } => false,
        }
    }

    /// Returns the elements (fields) of this struct.
    #[must_use]
    pub fn elements(&self) -> &[ComplexDataElement<'_>] {
        if let StructMode::All { elements, .. } | StructMode::Sequence { elements, .. } = &self.mode
        {
            elements
        } else {
            &[]
        }
    }

    /// Returns `true` if any kind of element is allowed in the struct, `false` otherwise.
    #[must_use]
    pub fn allow_any(&self) -> bool {
        if let StructMode::Empty { allow_any }
        | StructMode::All { allow_any, .. }
        | StructMode::Sequence { allow_any, .. } = &self.mode
        {
            *allow_any
        } else {
            false
        }
    }

    /// Returns the content type if this struct has one.
    #[must_use]
    pub fn content(&self) -> Option<&ComplexDataContent<'types>> {
        if let StructMode::Content { content, .. } = &self.mode {
            Some(content)
        } else {
            None
        }
    }
}

impl Deref for ComplexDataStruct<'_> {
    type Target = ComplexBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl ComplexDataElement<'_> {
    /// Returns the [`ElementMeta`] this element was created for.
    #[inline]
    #[must_use]
    pub fn meta(&self) -> &ElementMeta {
        match &self.origin {
            ComplexDataElementOrigin::Generated(meta) => meta,
            ComplexDataElementOrigin::Meta(meta) => meta,
        }
    }
}

impl ComplexDataContent<'_> {
    /// returns `true` if the content is a simple type (e.g. a enum, union,
    /// string, integer, ...), `false` otherwise.
    #[must_use]
    pub fn is_simple(&self) -> bool {
        self.simple_type.is_some()
    }
}
