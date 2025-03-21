use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use bitflags::bitflags;
use inflector::Inflector;
use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote, ToTokens};
use smallvec::SmallVec;

use crate::schema::{MaxOccurs, MinOccurs, NamespaceId};
use crate::types::{DynamicInfo, Ident, Name, Type, Types};

use super::helper::render_usings;
use super::Error;

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

        /// The generator will generate code to serialize the generated types using
        /// the `quick_xml` crate.
        const QUICK_XML_SERIALIZE = 1 << 3;

        /// The generator will generate code to deserialize the generated types using
        /// the `quick_xml` crate.
        const QUICK_XML_DESERIALIZE = 1 << 4;

        /// Combination of [`WITH_NAMESPACE_CONSTANTS`](Self::WITH_NAMESPACE_CONSTANTS),
        /// [`QUICK_XML_SERIALIZE`](Self::QUICK_XML_SERIALIZE)
        /// and [`QUICK_XML_DESERIALIZE`](Self::QUICK_XML_DESERIALIZE).
        const QUICK_XML = Self::WITH_NAMESPACE_CONSTANTS.bits()
            | Self::QUICK_XML_SERIALIZE.bits()
            | Self::QUICK_XML_DESERIALIZE.bits();

        /// Implement the [`WithNamespace`](crate::WithNamespace) trait for the generated types.
        const WITH_NAMESPACE_TRAIT = 1 << 5;

        /// Will generate constants for the different namespace used by the schema.
        const WITH_NAMESPACE_CONSTANTS = 1 << 6;
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

    /// Returns `false` if this is equal to [`SerdeSupport::None`], `true` otherwise.
    #[must_use]
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }
}

/* Modules */

#[derive(Default, Debug)]
pub(super) struct Modules(pub BTreeMap<Option<NamespaceId>, Module>);

impl Modules {
    pub(super) fn get_mut(&mut self, ns: Option<NamespaceId>) -> &mut Module {
        self.0.entry(ns).or_default()
    }
}

impl Deref for Modules {
    type Target = BTreeMap<Option<NamespaceId>, Module>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/* Module */

#[derive(Default, Debug)]
pub(super) struct Module {
    pub main: ModuleCode,
    pub quick_xml_serialize: Option<ModuleCode>,
    pub quick_xml_deserialize: Option<ModuleCode>,
}

/* ModuleCode */

#[derive(Default, Debug)]
pub(super) struct ModuleCode {
    pub code: TokenStream,
    pub usings: BTreeSet<String>,
}

impl ModuleCode {
    pub(super) fn code(&mut self, code: TokenStream) -> &mut Self {
        self.code.extend(code);

        self
    }

    pub(super) fn usings<I>(&mut self, usings: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        for using in usings {
            self.usings.insert(using.to_string());
        }

        self
    }
}

impl ToTokens for ModuleCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { code, usings } = self;
        let usings = render_usings(usings.iter());

        tokens.extend(quote! {
            #usings
            #code
        });
    }
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

    pub(super) fn is_some(&self) -> bool {
        *self != Self::None
    }

    pub(super) fn is_direct(&self) -> bool {
        matches!(self, Self::Single | Self::Optional | Self::StaticList(_))
    }
}

/* DynTypeTraits */

#[derive(Default, Debug)]
pub(super) enum DynTypeTraits {
    #[default]
    Auto,
    Custom(Vec<TokenStream>),
}

/* TypePath */

#[derive(Debug, Clone)]
pub(super) struct IdentPath {
    path: Option<ModulePath>,
    ident: Ident2,
}

#[derive(Default, Debug, Clone)]
pub(super) struct ModulePath(pub SmallVec<[Ident2; 2]>);

impl IdentPath {
    pub(super) fn from_type_ref(type_ref: &TypeRef) -> Self {
        if type_ref.ident.is_build_in() {
            Self::from_ident(type_ref.type_ident.clone())
        } else {
            Self::from_ident(type_ref.type_ident.clone()).with_path(type_ref.module_ident.clone())
        }
    }

    pub(super) fn from_parts<I>(path: I, ident: Ident2) -> Self
    where
        I: IntoIterator<Item = Ident2>,
    {
        Self::from_ident(ident).with_path(path)
    }

    pub(super) fn from_ident(ident: Ident2) -> Self {
        Self { ident, path: None }
    }

    pub(super) fn with_ident(mut self, ident: Ident2) -> Self {
        self.ident = ident;

        self
    }

    pub(super) fn with_path<I>(mut self, path: I) -> Self
    where
        I: IntoIterator<Item = Ident2>,
    {
        self.path = Some(ModulePath(path.into_iter().collect()));

        self
    }

    pub(super) fn into_parts(self) -> (Ident2, Option<ModulePath>) {
        let Self { ident, path } = self;

        (ident, path)
    }

    pub(super) fn ident(&self) -> &Ident2 {
        &self.ident
    }

    pub(super) fn relative_to(&self, dst: &ModulePath) -> TokenStream {
        let ident = &self.ident;

        let Some(src) = &self.path else {
            return quote!(#ident);
        };

        let mut ret = TokenStream::new();
        let mut src = src.0.iter().fuse();
        let mut dst = dst.0.iter().fuse();

        macro_rules! push {
            ($x:expr) => {{
                let x = $x;
                if ret.is_empty() {
                    ret.extend(x)
                } else {
                    ret.extend(quote!(::#x))
                }
            }};
        }

        loop {
            match (src.next(), dst.next()) {
                (Some(a), Some(b)) if a == b => {}
                (Some(a), Some(_)) => {
                    push!(quote!(super));
                    while dst.next().is_some() {
                        push!(quote!(super));
                    }

                    push!(quote!(#a));
                    for a in src {
                        push!(quote!(#a));
                    }

                    push!(quote!(#ident));

                    return ret;
                }
                (Some(a), None) => push!(quote!(#a)),
                (None, Some(_)) => push!(quote!(super)),
                (None, None) => {
                    push!(quote!(#ident));
                    return ret;
                }
            }
        }
    }
}

impl FromStr for IdentPath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ident = None;
        let mut path = ModulePath::default();

        for part in s.split("::") {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            if let Some(ident) = ident.take() {
                path.0.push(ident);
            }

            ident = Some(format_ident!("{part}"));
        }

        Ok(Self {
            ident: ident.ok_or(())?,
            path: Some(path),
        })
    }
}

impl ModulePath {
    pub(super) fn from_namespace(ns: Option<NamespaceId>, types: &Types) -> Self {
        let ident = ns
            .and_then(|id| types.modules.get(&id))
            .and_then(|module| module.name.as_ref())
            .map(format_module_ident);

        Self(ident.into_iter().collect())
    }

    pub(super) fn join(mut self, other: Ident2) -> Self {
        self.0.push(other);

        self
    }
}

impl Deref for ModulePath {
    type Target = SmallVec<[Ident2; 2]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ModulePath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
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

#[cfg(test)]
mod tests {
    use quote::{format_ident, quote};

    use crate::generator::misc::ModulePath;

    use super::IdentPath;

    #[test]
    #[rustfmt::skip]
    fn type_path() {
        let string = IdentPath::from_ident(format_ident!("String"));
        let my_type = IdentPath::from_parts(
            [format_ident!("my_module")],
            format_ident!("MyType"),
        );
        let serializer = IdentPath::from_parts(
            [
                format_ident!("my_module"),
                format_ident!("quick_xml_serialize"),
            ],
            format_ident!("MyTypeSerializer"),
        );
        let deserializer = IdentPath::from_parts(
            [
                format_ident!("my_module"),
                format_ident!("quick_xml_deserialize"),
            ],
            format_ident!("MyTypeDeserializer"),
        );

        let empty_path = ModulePath::default();
        let module_path = ModulePath::default().join(format_ident!("my_module"));
        let other_module_path = ModulePath::default().join(format_ident!("other_module"));
        let serializer_path = module_path.clone().join(format_ident!("quick_xml_serialize"));
        let deserializer_path = module_path.clone().join(format_ident!("quick_xml_deserialize"));

        macro_rules! test {
            ($actual:expr, $( $expected:tt )*) => {{
                let a = $actual.to_string();
                let b = quote!($( $expected )*).to_string();

                assert_eq!(a, b);
            }};
        }

        /* With modules */

        test!(string.relative_to(&empty_path), String);
        test!(string.relative_to(&module_path), String);
        test!(string.relative_to(&other_module_path), String);
        test!(string.relative_to(&serializer_path), String);
        test!(string.relative_to(&deserializer_path), String);

        test!(my_type.relative_to(&empty_path), my_module::MyType);
        test!(my_type.relative_to(&module_path), MyType);
        test!(my_type.relative_to(&other_module_path), super::my_module::MyType);
        test!(my_type.relative_to(&serializer_path), super::MyType);
        test!(my_type.relative_to(&deserializer_path), super::MyType);

        test!(serializer.relative_to(&empty_path), my_module::quick_xml_serialize::MyTypeSerializer);
        test!(serializer.relative_to(&module_path), quick_xml_serialize::MyTypeSerializer);
        test!(serializer.relative_to(&other_module_path), super::my_module::quick_xml_serialize::MyTypeSerializer);
        test!(serializer.relative_to(&serializer_path), MyTypeSerializer);
        test!(serializer.relative_to(&deserializer_path), super::quick_xml_serialize::MyTypeSerializer);

        test!(deserializer.relative_to(&empty_path), my_module::quick_xml_deserialize::MyTypeDeserializer);
        test!(deserializer.relative_to(&module_path), quick_xml_deserialize::MyTypeDeserializer);
        test!(deserializer.relative_to(&other_module_path), super::my_module::quick_xml_deserialize::MyTypeDeserializer);
        test!(deserializer.relative_to(&serializer_path), super::quick_xml_deserialize::MyTypeDeserializer);
        test!(deserializer.relative_to(&deserializer_path), MyTypeDeserializer);
    }
}
