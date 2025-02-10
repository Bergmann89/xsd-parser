use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::ops::Deref;

use bitflags::bitflags;
use inflector::Inflector;
use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote};

use crate::schema::{MaxOccurs, MinOccurs, NamespaceId};
use crate::types::{DynamicInfo, Ident, Name, Type, Types};

use super::Error;

bitflags! {
    /// Different flags that control what code the [`Generator`](super::Generator)
    /// is generating.
    #[derive(Debug, Clone, Copy)]
    pub struct GenerateFlags: u32 {
        /// None of the features are enabled.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/generate_flags/schema.xsd")]
        /// ```
        ///
        /// Setting none of the flags will result in the following code:
        /// ```rust
        #[doc = include_str!("../../tests/generator/generate_flags/expected/empty.rs")]
        /// ```
        const NONE = 0;

        /// The generated code uses modules for the different namespaces.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/generate_flags/schema.xsd")]
        /// ```
        ///
        /// Enable the `USE_MODULES` feature only will result in the following code:
        /// ```rust,ignore
        #[doc = include_str!("../../tests/generator/generate_flags/expected/use_modules.rs")]
        /// ```
        const USE_MODULES = 1 << 0;

        /// The generator flattens the content type of choice types if it does not
        /// define any element attributes.
        ///
        /// # Examples
        ///
        /// Consider the following XML schema:
        /// ```xml
        #[doc = include_str!("../../tests/generator/generate_flags/schema.xsd")]
        /// ```
        ///
        /// Enable the `FLATTEN_CONTENT` feature only will result in the following code:
        /// ```rust
        #[doc = include_str!("../../tests/generator/generate_flags/expected/flatten_content.rs")]
        /// ```
        const FLATTEN_CONTENT = 1 << 1;

        /// The generator will generate code to serialize the generated types using
        /// the `quick_xml` crate.
        const QUICK_XML_SERIALIZE = 1 << 2;

        /// The generator will generate code to deserialize the generated types using
        /// the `quick_xml` crate.
        const QUICK_XML_DESERIALIZE = 1 << 3;

        /// Combination of `QUICK_XML_SERIALIZE` and [`QUICK_XML_DESERIALIZE`].
        const QUICK_XML = Self::QUICK_XML_SERIALIZE.bits() | Self::QUICK_XML_DESERIALIZE.bits();

        /// Implement the [`WithNamespace`](crate::WithNamespace) trait for the generated types.
        const WITH_NAMESPACE_TRAIT = 1 << 4;
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

/// Tells the [`Generator`](super::Generator) what type should be generated for
/// the content of an XML element.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum ContentMode {
    /// The mode is selected depending on the type definition of the element.
    /// `xs:choice` types will be rendered as enum, `xs:all` and `xs:sequence`
    /// types will be rendered as struct.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/generator/content_mode/schema.xsd")]
    /// ```
    ///
    /// If the content mode is set to [`ContentMode::Auto`] the following code is rendered:
    /// ```rust
    #[doc = include_str!("../../tests/generator/content_mode/expected/auto.rs")]
    /// ```
    #[default]
    Auto,

    /// The content of a XML element is always rendered as enum.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/generator/content_mode/schema.xsd")]
    /// ```
    ///
    /// If the content mode is set to [`ContentMode::Enum`] the following code is rendered:
    /// ```rust
    #[doc = include_str!("../../tests/generator/content_mode/expected/enum.rs")]
    /// ```
    Enum,

    /// The content of a XML element is always rendered as struct.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/generator/content_mode/schema.xsd")]
    /// ```
    ///
    /// If the content mode is set to [`ContentMode::Struct`] the following code is rendered:
    /// ```rust
    #[doc = include_str!("../../tests/generator/content_mode/expected/struct.rs")]
    /// ```
    Struct,
}

/// Tells the [`Generator`](super::Generator) how to deal with type definitions.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum TypedefMode {
    /// The [`Generator`](super::Generator) will automatically detect if a
    /// new type struct or a simple type definition should be used
    /// for a [`Reference`](Type::Reference) type.
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
    /// for a [`Reference`](Type::Reference) type.
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
    /// for a [`Reference`](Type::Reference) type.
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
}

/* Modules */

#[derive(Default, Debug)]
pub(super) struct Modules(pub BTreeMap<Option<NamespaceId>, Module>);

impl Modules {
    pub(super) fn get_mut(&mut self, ns: Option<NamespaceId>) -> &mut Module {
        self.0.entry(ns).or_default()
    }

    pub(super) fn add_code(&mut self, ns: Option<NamespaceId>, code: TokenStream) {
        self.get_mut(ns).code.extend(code);
    }
}

/* Module */

#[derive(Default, Debug)]
pub(super) struct Module {
    pub code: TokenStream,
    pub quick_xml_serialize: Option<TokenStream>,
    pub quick_xml_deserialize: Option<TokenStream>,
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
    pub ns: Option<NamespaceId>,
    pub module_ident: Option<Ident2>,
    pub type_ident: Ident2,
    pub flags: StateFlags,
    pub boxed_elements: HashSet<Ident>,
}

/* TraitInfos */

#[derive(Debug)]
pub(super) struct TraitInfos(BTreeMap<Ident, TraitInfo>);

impl TraitInfos {
    #[must_use]
    pub(super) fn new(types: &Types) -> Self {
        let mut ret = Self(BTreeMap::new());

        for (base_ident, ty) in types.iter() {
            let Type::Dynamic(ai) = ty else {
                continue;
            };

            for type_ident in &ai.derived_types {
                ret.0
                    .entry(type_ident.clone())
                    .or_default()
                    .traits_all
                    .insert(base_ident.clone());

                match types.get(type_ident) {
                    Some(Type::Dynamic(DynamicInfo {
                        type_: Some(type_ident),
                        ..
                    })) => {
                        ret.0
                            .entry(type_ident.clone())
                            .or_default()
                            .traits_all
                            .insert(base_ident.clone());
                    }
                    Some(Type::Reference(ri)) if ri.is_single() => {
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

/* StateFlags */

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub(super) struct StateFlags: u32 {
        const HAS_TYPE = 1 << 0;
        const HAS_IMPL = 1 << 1;

        const HAS_QUICK_XML_SERIALIZE = 1 << 2;
        const HAS_QUICK_XML_DESERIALIZE = 1 << 3;

        const HAS_QUICK_XML = Self::HAS_QUICK_XML_SERIALIZE.bits() | Self::HAS_QUICK_XML_DESERIALIZE.bits();
    }
}

/* Occurs */

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum Occurs {
    #[default]
    None,
    Single,
    Optional,
    DynamicList,
    StaticList(usize),
}

impl Occurs {
    pub(super) fn from_occurs(min: MinOccurs, max: MaxOccurs) -> Self {
        match (min, max) {
            (0, MaxOccurs::Bounded(0)) => Self::None,
            (1, MaxOccurs::Bounded(1)) => Self::Single,
            (0, MaxOccurs::Bounded(1)) => Self::Optional,
            (a, MaxOccurs::Bounded(b)) if a == b => Self::StaticList(a),
            (_, _) => Self::DynamicList,
        }
    }

    pub(super) fn make_type(
        self,
        ident: &TokenStream,
        need_indirection: bool,
    ) -> Option<TokenStream> {
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

    pub(super) fn is_direct(&self) -> bool {
        matches!(self, Self::Single | Self::Optional | Self::StaticList(_))
    }
}

/* TypeMode */

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum TypeMode {
    All,
    Choice,
    Sequence,
    Simple,
}

/* DynTypeTraits */

#[derive(Default, Debug)]
pub(super) enum DynTypeTraits {
    #[default]
    Auto,
    Custom(Vec<TokenStream>),
}

/* Helper */

pub(super) fn format_field_name(name: &Name, display_name: Option<&str>) -> Cow<'static, str> {
    if let Some(display_name) = display_name {
        return Cow::Owned(display_name.to_snake_case());
    }

    let ident = name
        .to_type_name(false, None)
        .as_str()
        .unwrap()
        .to_snake_case();

    match KEYWORDS.binary_search_by(|(key, _)| key.cmp(&ident.as_str())) {
        Ok(idx) => Cow::Borrowed(KEYWORDS[idx].1),
        Err(_) => {
            if ident.starts_with(char::is_numeric) {
                Cow::Owned(format!("_{ident}"))
            } else {
                Cow::Owned(ident)
            }
        }
    }
}

pub(super) fn format_field_ident(name: &Name, display_name: Option<&str>) -> Ident2 {
    let ident = format_field_name(name, display_name);

    format_ident!("{ident}")
}

pub(super) fn format_module_ident(name: &Name) -> Ident2 {
    format_field_ident(name, None)
}

pub(super) fn format_type_name(name: &Name, display_name: Option<&str>) -> String {
    if let Some(display_name) = display_name {
        return display_name.to_pascal_case();
    }

    let name = name
        .to_type_name(false, None)
        .as_str()
        .unwrap()
        .to_pascal_case();

    if name.starts_with(char::is_numeric) {
        format!("_{name}")
    } else {
        name
    }
}

pub(super) fn format_type_ident(name: &Name, display_name: Option<&str>) -> Ident2 {
    let ident = format_type_name(name, display_name);

    format_ident!("{ident}")
}

pub(super) fn format_variant_ident(name: &Name, display_name: Option<&str>) -> Ident2 {
    format_type_ident(name, display_name)
}

pub(super) fn format_module(
    types: &Types,
    ns: Option<NamespaceId>,
) -> Result<Option<Ident2>, Error> {
    let Some(ns) = ns else {
        return Ok(None);
    };

    let module = types.modules.get(&ns).ok_or(Error::UnknownNamespace(ns))?;
    let Some(name) = &module.name else {
        return Ok(None);
    };

    Ok(Some(format_module_ident(name)))
}

pub(super) fn format_type_ref(current_ns: Option<NamespaceId>, type_: &TypeRef) -> TokenStream {
    format_type_ref_ex(current_ns, None, &type_.type_ident, type_)
}

pub(super) fn format_type_ref_ex(
    current_ns: Option<NamespaceId>,
    extra: Option<&TokenStream>,
    type_ident: &Ident2,
    type_: &TypeRef,
) -> TokenStream {
    let module_ident = match (current_ns, type_.ns) {
        (Some(a), Some(b)) if a != b => Some(&type_.module_ident),
        (_, _) => None,
    };

    if let Some(module_ident) = module_ident {
        quote! {
            #module_ident #extra :: #type_ident
        }
    } else {
        quote! {
            #type_ident
        }
    }
}

pub(super) fn make_type_name(postfixes: &[String], ty: &Type, ident: &Ident) -> Name {
    match (ty, &ident.name) {
        (Type::Reference(ti), Name::Unnamed { .. }) if ti.type_.name.is_named() => {
            match Occurs::from_occurs(ti.min_occurs, ti.max_occurs) {
                Occurs::DynamicList => return Name::new(format!("{}List", ti.type_.name)),
                Occurs::Optional => return Name::new(format!("{}Opt", ti.type_.name)),
                _ => (),
            }
        }
        (_, _) => (),
    };

    let postfix = postfixes
        .get(ident.type_ as usize)
        .map_or("", |s| s.as_str());

    match &ident.name {
        Name::Named(s) if s.ends_with(postfix) => Name::Named(s.clone()),
        Name::Named(s) => Name::Named(Cow::Owned(format!("{s}{postfix}"))),
        name => name.to_type_name(false, None),
    }
}

const KEYWORDS: &[(&str, &str)] = &[
    ("abstract", "abstract_"),
    ("as", "as_"),
    ("become", "become_"),
    ("box", "box_"),
    ("break", "break_"),
    ("const", "const_"),
    ("continue", "continue_"),
    ("crate", "crate_"),
    ("do", "do_"),
    ("else", "else_"),
    ("enum", "enum_"),
    ("extern", "extern_"),
    ("false", "false_"),
    ("final", "final_"),
    ("fn", "fn_"),
    ("for", "for_"),
    ("if", "if_"),
    ("impl", "impl_"),
    ("in", "in_"),
    ("let", "let_"),
    ("loop", "loop_"),
    ("macro", "macro_"),
    ("match", "match_"),
    ("mod", "mod_"),
    ("move", "move_"),
    ("mut", "mut_"),
    ("override", "override_"),
    ("priv", "priv_"),
    ("pub", "pub_"),
    ("ref", "ref_"),
    ("return", "return_"),
    ("self", "self_"),
    ("Self", "Self_"),
    ("static", "static_"),
    ("struct", "struct_"),
    ("super", "super_"),
    ("trait", "trait_"),
    ("true", "true_"),
    ("try", "try_"),
    ("type", "type_"),
    ("typeof", "typeof_"),
    ("union", "union_"),
    ("unsafe", "unsafe_"),
    ("unsized", "unsized_"),
    ("use", "use_"),
    ("virtual", "virtual_"),
    ("where", "where_"),
    ("while", "while_"),
    ("yield", "yield_"),
];
