use std::mem::take;
use std::ops::Deref;

use parking_lot::Mutex;
use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::{format_ident, quote};

use crate::{
    schema::{xs::Use, MaxOccurs, MinOccurs, NamespaceId},
    types::{
        AnyAttributeInfo, AnyInfo, AttributeInfo, BuildInInfo, ComplexInfo, DynamicInfo,
        ElementInfo, EnumerationInfo, GroupInfo, Ident, ReferenceInfo, Type, Types, UnionInfo,
        UnionTypeInfo, VariantInfo,
    },
};

use super::{
    misc::{format_field_ident, format_variant_ident, IdentPath, ModuleCode, ModulePath, Occurs},
    BoxFlags, Config, Error, GeneratorFlags, Modules, State, TraitInfos, TypeRef, TypedefMode,
};

/* Request */

/// Helper type that is used to request the code generation for a specific type.
pub(super) struct Request<'a, 'types> {
    pub ident: &'a Ident,
    pub config: &'a Config<'types>,

    state: &'a mut State<'types>,
}

impl<'a, 'types> Request<'a, 'types> {
    pub(super) fn new(
        ident: &'a Ident,
        config: &'a Config<'types>,
        state: &'a mut State<'types>,
    ) -> Self {
        Self {
            ident,
            config,
            state,
        }
    }

    pub(super) fn into_union_type(
        self,
        info: &'types UnionInfo,
    ) -> Result<UnionType<'types>, Error> {
        UnionType::new(info, self)
    }

    pub(super) fn into_dynamic_type(
        self,
        info: &'types DynamicInfo,
    ) -> Result<DynamicType<'types>, Error> {
        DynamicType::new(info, self)
    }

    pub(super) fn into_reference_type(
        self,
        info: &'types ReferenceInfo,
    ) -> Result<ReferenceType<'types>, Error> {
        ReferenceType::new(info, self)
    }

    pub(super) fn into_enumeration_type(
        self,
        info: &'types EnumerationInfo,
    ) -> Result<EnumerationType<'types>, Error> {
        EnumerationType::new(info, self)
    }

    pub(super) fn into_all_type(
        self,
        info: &'types GroupInfo,
    ) -> Result<ComplexType<'types>, Error> {
        ComplexType::new_all(info, self)
    }

    pub(super) fn into_choice_type(
        self,
        info: &'types GroupInfo,
    ) -> Result<ComplexType<'types>, Error> {
        ComplexType::new_choice(info, self)
    }

    pub(super) fn into_sequence_type(
        self,
        info: &'types GroupInfo,
    ) -> Result<ComplexType<'types>, Error> {
        ComplexType::new_sequence(info, self)
    }

    pub(super) fn into_complex_type(
        self,
        info: &'types ComplexInfo,
    ) -> Result<ComplexType<'types>, Error> {
        ComplexType::new_complex(info, self)
    }

    fn current_module(&self) -> Option<NamespaceId> {
        self.check_flags(GeneratorFlags::USE_MODULES)
            .then_some(self.ident.ns)
            .flatten()
    }

    fn current_type_ref(&self) -> &TypeRef {
        self.state.cache.get(self.ident).unwrap()
    }

    fn get_trait_infos(&mut self) -> &TraitInfos {
        self.state
            .trait_infos
            .get_or_insert_with(|| TraitInfos::new(self.config.types))
    }

    fn get_or_create_type_ref(&mut self, ident: Ident) -> Result<&TypeRef, Error> {
        self.state.get_or_create_type_ref(self.config, ident)
    }

    fn make_trait_impls(&mut self) -> Result<Vec<IdentPath>, Error> {
        let ident = self.ident.clone();

        self.get_trait_infos()
            .get(&ident)
            .into_iter()
            .flat_map(|info| &info.traits_all)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|ident| {
                let type_ref = self.get_or_create_type_ref(ident.clone())?;
                let ident = format_ident!("{}Trait", type_ref.type_ident);
                let trait_type = IdentPath::from_type_ref(type_ref).with_ident(ident);

                Ok(trait_type)
            })
            .collect::<Result<Vec<_>, _>>()
    }

    fn get_default(
        &mut self,
        current_ns: Option<NamespaceId>,
        default: &str,
        ident: &Ident,
    ) -> Result<TokenStream, Error> {
        let types = self.types;
        let ty = types
            .get(ident)
            .ok_or_else(|| Error::UnknownType(ident.clone()))?;
        let type_ref = self.get_or_create_type_ref(ident.clone())?;

        macro_rules! build_in {
            ($ty:ty) => {
                if let Ok(val) = default.parse::<$ty>() {
                    return Ok(quote!(#val));
                }
            };
        }

        match ty {
            Type::BuildIn(BuildInInfo::U8) => build_in!(u8),
            Type::BuildIn(BuildInInfo::U16) => build_in!(u16),
            Type::BuildIn(BuildInInfo::U32) => build_in!(u32),
            Type::BuildIn(BuildInInfo::U64) => build_in!(u64),
            Type::BuildIn(BuildInInfo::U128) => build_in!(u128),
            Type::BuildIn(BuildInInfo::Usize) => build_in!(usize),

            Type::BuildIn(BuildInInfo::I8) => build_in!(i8),
            Type::BuildIn(BuildInInfo::I16) => build_in!(i16),
            Type::BuildIn(BuildInInfo::I32) => build_in!(i32),
            Type::BuildIn(BuildInInfo::I64) => build_in!(i64),
            Type::BuildIn(BuildInInfo::I128) => build_in!(i128),
            Type::BuildIn(BuildInInfo::Isize) => build_in!(isize),

            Type::BuildIn(BuildInInfo::F32) => build_in!(f32),
            Type::BuildIn(BuildInInfo::F64) => build_in!(f64),

            Type::BuildIn(BuildInInfo::Bool) => match default.to_ascii_lowercase().as_str() {
                "true" | "yes" | "1" => return Ok(quote!(true)),
                "false" | "no" | "0" => return Ok(quote!(false)),
                _ => (),
            },
            Type::BuildIn(BuildInInfo::String) => {
                return Ok(quote!(String::from(#default)));
            }
            Type::BuildIn(BuildInInfo::Custom(x)) => {
                if let Some(x) = x.default(default) {
                    return Ok(x);
                }
            }

            Type::Enumeration(ei) => {
                let module_path = ModulePath::from_namespace(current_ns, types);
                let target_type = IdentPath::from_type_ref(type_ref).relative_to(&module_path);

                for var in &*ei.variants {
                    if var.type_.is_none()
                        && matches!(var.ident.name.as_str(), Some(x) if x == default)
                    {
                        let variant_ident =
                            format_variant_ident(&var.ident.name, var.display_name.as_deref());

                        return Ok(quote!(#target_type :: #variant_ident));
                    }

                    if let Some(target_ident) = &var.type_ {
                        if let Ok(default) = self.get_default(current_ns, default, target_ident) {
                            let variant_ident = match self.state.cache.get(target_ident) {
                                Some(type_ref) if var.ident.name.is_unnamed() => {
                                    type_ref.type_ident.clone()
                                }
                                _ => format_variant_ident(
                                    &var.ident.name,
                                    var.display_name.as_deref(),
                                ),
                            };

                            return Ok(quote!(#target_type :: #variant_ident(#default)));
                        }
                    }
                }
            }

            Type::Union(ui) => {
                let module_path = ModulePath::from_namespace(current_ns, types);
                let target_type = IdentPath::from_type_ref(type_ref).relative_to(&module_path);

                for ty in &*ui.types {
                    if let Ok(code) = self.get_default(current_ns, default, &ty.type_) {
                        let variant_ident =
                            format_variant_ident(&ty.type_.name, ty.display_name.as_deref());

                        return Ok(quote! {
                            #target_type :: #variant_ident ( #code )
                        });
                    }
                }
            }

            Type::Reference(ti) => match Occurs::from_occurs(ti.min_occurs, ti.max_occurs) {
                Occurs::Single => return self.get_default(current_ns, default, &ti.type_),
                Occurs::DynamicList if default.is_empty() => {
                    let module_path = ModulePath::from_namespace(current_ns, types);
                    let target_type = IdentPath::from_type_ref(type_ref).relative_to(&module_path);

                    return Ok(quote! { #target_type(Vec::new()) });
                }
                _ => (),
            },

            _ => (),
        }

        Err(Error::InvalidDefaultValue(
            ident.clone(),
            default.to_owned(),
        ))
    }
}

impl<'types> Deref for Request<'_, 'types> {
    type Target = Config<'types>;

    fn deref(&self) -> &Self::Target {
        self.config
    }
}

/* Code */

/// Helper type that is used to collect the generated code.
pub(super) struct Context<'a, 'types> {
    ident: &'a Ident,
    config: &'a Config<'types>,
    modules: Mutex<&'a mut Modules>,

    module_path: ModulePath,
    serialize_module_path: ModulePath,
    deserialize_module_path: ModulePath,
}

#[allow(dead_code)] // TODO
impl<'a, 'types> Context<'a, 'types> {
    pub(super) fn new(
        ident: &'a Ident,
        config: &'a Config<'types>,
        modules: &'a mut Modules,
    ) -> Self {
        let ns = config
            .check_flags(GeneratorFlags::USE_MODULES)
            .then_some(ident.ns)
            .flatten();
        let module_path = ModulePath::from_namespace(ns, config.types);
        let serialize_module_path = module_path
            .clone()
            .join(format_ident!("quick_xml_serialize"));
        let deserialize_module_path = module_path
            .clone()
            .join(format_ident!("quick_xml_deserialize"));

        Self {
            ident,
            config,
            modules: Mutex::new(modules),

            module_path,
            serialize_module_path,
            deserialize_module_path,
        }
    }

    pub(super) fn current_ns(&self) -> Option<NamespaceId> {
        self.check_flags(GeneratorFlags::USE_MODULES)
            .then_some(self.ident.ns)
            .flatten()
    }

    pub(super) fn resolve_type_for_module(&self, ident: &IdentPath) -> TokenStream {
        ident.relative_to(&self.module_path)
    }

    pub(super) fn resolve_type_for_serialize_module(&self, ident: &IdentPath) -> TokenStream {
        ident.relative_to(&self.serialize_module_path)
    }

    pub(super) fn resolve_type_for_deserialize_module(&self, ident: &IdentPath) -> TokenStream {
        ident.relative_to(&self.deserialize_module_path)
    }

    pub(super) fn main(&mut self) -> &mut ModuleCode {
        let ns = self.current_ns();

        &mut self.modules.get_mut().get_mut(ns).main
    }

    pub(super) fn quick_xml_serialize(&mut self) -> &mut ModuleCode {
        let ns = self.current_ns();

        self.modules
            .get_mut()
            .get_mut(ns)
            .quick_xml_serialize
            .get_or_insert_default()
    }

    pub(super) fn quick_xml_deserialize(&mut self) -> &mut ModuleCode {
        let ns = self.current_ns();

        self.modules
            .get_mut()
            .get_mut(ns)
            .quick_xml_deserialize
            .get_or_insert_default()
    }

    pub(super) fn add_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let ns = self.current_ns();

        self.modules.lock().get_mut(ns).main.usings(usings);
    }

    pub(super) fn add_quick_xml_serialize_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let ns = self.current_ns();

        self.modules
            .lock()
            .get_mut(ns)
            .quick_xml_serialize
            .get_or_insert_default()
            .usings(usings);
    }

    pub(super) fn add_quick_xml_deserialize_usings<I>(&self, usings: I)
    where
        I: IntoIterator,
        I::Item: ToString,
    {
        let ns = self.current_ns();

        self.modules
            .lock()
            .get_mut(ns)
            .quick_xml_deserialize
            .get_or_insert_default()
            .usings(usings);
    }
}

impl<'types> Deref for Context<'_, 'types> {
    type Target = Config<'types>;

    fn deref(&self) -> &Self::Target {
        self.config
    }
}

/* UnionType */

#[derive(Debug)]
#[allow(dead_code)] // TODO
pub(super) struct UnionType<'types> {
    pub info: &'types UnionInfo,
    pub type_ident: Ident2,
    pub variants: Vec<UnionTypeVariant<'types>>,
    pub trait_impls: Vec<IdentPath>,
}

#[derive(Debug)]
#[allow(dead_code)] // TODO
pub(super) struct UnionTypeVariant<'types> {
    pub info: &'types UnionTypeInfo,
    pub target_type: IdentPath,
    pub variant_ident: Ident2,
}

impl<'types> UnionType<'types> {
    fn new(info: &'types UnionInfo, mut req: Request<'_, 'types>) -> Result<Self, Error> {
        let type_ident = req.current_type_ref().type_ident.clone();
        let trait_impls = req.make_trait_impls()?;
        let variants = info
            .types
            .iter()
            .map(|info| info.make_variant(&mut req))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            info,
            type_ident,
            variants,
            trait_impls,
        })
    }
}

impl UnionTypeInfo {
    fn make_variant<'types>(
        &'types self,
        req: &mut Request<'_, 'types>,
    ) -> Result<UnionTypeVariant<'types>, Error> {
        let type_ref = req.get_or_create_type_ref(self.type_.clone())?;
        let target_type = IdentPath::from_type_ref(type_ref);
        let variant_ident = format_variant_ident(&self.type_.name, self.display_name.as_deref());

        Ok(UnionTypeVariant {
            info: self,
            target_type,
            variant_ident,
        })
    }
}

/* DynamicType */

#[derive(Debug)]
#[allow(dead_code)] // TODO
pub(super) struct DynamicType<'types> {
    pub info: &'types DynamicInfo,
    pub type_ident: Ident2,
    pub trait_ident: Ident2,
    pub deserializer_ident: Ident2,
    pub sub_traits: Option<Vec<IdentPath>>,
    pub derived_types: Vec<DerivedType>,
}

#[derive(Debug)]
#[allow(dead_code)] // TODO
pub(super) struct DerivedType {
    pub ident: Ident,
    pub s_name: String,
    pub b_name: Literal,
    pub target_type: IdentPath,
    pub variant_ident: Ident2,
}

impl<'types> DynamicType<'types> {
    fn new(info: &'types DynamicInfo, mut req: Request<'_, 'types>) -> Result<Self, Error> {
        let type_ident = req.current_type_ref().type_ident.clone();
        let trait_ident = format_ident!("{type_ident}Trait");
        let ident = req.ident.clone();
        let sub_traits = req
            .get_trait_infos()
            .get(&ident)
            .map(|info| info.traits_direct.clone())
            .map(|traits_direct| {
                traits_direct
                    .iter()
                    .map(|ident| {
                        req.get_or_create_type_ref(ident.clone()).map(|x| {
                            let ident = format_ident!("{}Trait", x.type_ident);

                            IdentPath::from_type_ref(x).with_ident(ident)
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        let derived_types = info
            .derived_types
            .iter()
            .map(|ident| make_derived_type_data(&mut req, ident))
            .collect::<Result<Vec<_>, _>>()?;

        let deserializer_ident = format_ident!("{type_ident}Deserializer");

        Ok(Self {
            info,
            type_ident,
            trait_ident,
            deserializer_ident,
            sub_traits,
            derived_types,
        })
    }
}

/* ReferenceType */

#[derive(Debug)]
#[allow(dead_code)] // TODO
pub(super) struct ReferenceType<'types> {
    pub info: &'types ReferenceInfo,
    pub mode: TypedefMode,
    pub occurs: Occurs,
    pub type_ident: Ident2,
    pub target_type: IdentPath,
    pub trait_impls: Vec<IdentPath>,
}

impl<'types> ReferenceType<'types> {
    fn new(info: &'types ReferenceInfo, mut req: Request<'_, 'types>) -> Result<Self, Error> {
        let occurs = Occurs::from_occurs(info.min_occurs, info.max_occurs);
        let type_ident = req.current_type_ref().type_ident.clone();
        let target_ref = req.get_or_create_type_ref(info.type_.clone())?;
        let target_type = IdentPath::from_type_ref(target_ref);
        let trait_impls = req.make_trait_impls()?;

        let mode = match (req.typedef_mode, occurs) {
            (TypedefMode::Auto, Occurs::None | Occurs::Single) => TypedefMode::Typedef,
            (TypedefMode::Auto, _) => TypedefMode::NewType,
            (mode, _) => mode,
        };

        Ok(Self {
            info,
            mode,
            occurs,
            type_ident,
            target_type,
            trait_impls,
        })
    }
}

/* EnumerationType */

#[derive(Debug)]
#[allow(dead_code)] // TODO
pub(super) struct EnumerationType<'types> {
    pub info: &'types EnumerationInfo,
    pub type_ident: Ident2,
    pub variants: Vec<EnumerationTypeVariant<'types>>,
    pub trait_impls: Vec<IdentPath>,
}

#[derive(Debug)]
pub(super) struct EnumerationTypeVariant<'types> {
    pub info: &'types VariantInfo,
    pub variant_ident: Ident2,
    pub target_type: Option<IdentPath>,
}

impl<'types> EnumerationType<'types> {
    fn new(info: &'types EnumerationInfo, mut req: Request<'_, 'types>) -> Result<Self, Error> {
        let mut unknown = 0usize;
        let type_ident = req.current_type_ref().type_ident.clone();
        let trait_impls = req.make_trait_impls()?;

        let variants = info
            .variants
            .iter()
            .filter_map(|var| var.make_variant(&mut unknown, &mut req))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(EnumerationType {
            info,
            type_ident,
            variants,
            trait_impls,
        })
    }
}

impl VariantInfo {
    fn make_variant<'types>(
        &'types self,
        unknown: &mut usize,
        req: &mut Request<'_, 'types>,
    ) -> Option<Result<EnumerationTypeVariant<'types>, Error>> {
        match self.use_ {
            Use::Prohibited => None,
            Use::Required | Use::Optional => {
                let type_ref = if let Some(t) = &self.type_ {
                    match req.get_or_create_type_ref(t.clone()) {
                        Ok(target_ref) => Some(target_ref),
                        Err(error) => return Some(Err(error)),
                    }
                } else {
                    None
                };

                let variant_ident = if let Some(display_name) = self.display_name.as_deref() {
                    format_ident!("{display_name}")
                } else if let (Some(type_ref), true) = (type_ref, self.ident.name.is_unnamed()) {
                    type_ref.type_ident.clone()
                } else if matches!(self.ident.name.as_str(), Some("")) {
                    *unknown += 1;

                    format_ident!("Unknown{unknown}")
                } else {
                    format_variant_ident(&self.ident.name, self.display_name.as_deref())
                };

                let target_type = type_ref.map(IdentPath::from_type_ref);

                Some(Ok(EnumerationTypeVariant {
                    info: self,
                    variant_ident,
                    target_type,
                }))
            }
        }
    }
}

/* ComplexType */

#[derive(Debug)]
#[allow(dead_code, clippy::large_enum_variant)] // TODO
pub(super) enum ComplexType<'types> {
    Enum {
        type_: ComplexTypeEnum<'types>,
        content_type: Option<Box<ComplexType<'types>>>,
    },
    Struct {
        type_: ComplexTypeStruct<'types>,
        content_type: Option<Box<ComplexType<'types>>>,
    },
}

#[derive(Debug)]
pub(super) struct ComplexTypeBase {
    pub type_ident: Ident2,
    pub trait_impls: Vec<IdentPath>,

    pub tag_name: Option<String>,
    pub is_complex: bool,
    pub is_dynamic: bool,

    pub serializer_ident: Ident2,
    pub serializer_state_ident: Ident2,

    pub deserializer_ident: Ident2,
    pub deserializer_state_ident: Ident2,
}

#[derive(Debug)]
#[allow(dead_code)] // TODO
pub(super) struct ComplexTypeEnum<'types> {
    pub base: ComplexTypeBase,

    pub elements: Vec<ComplexTypeElement<'types>>,
    pub any_element: Option<&'types AnyInfo>,
    pub any_attribute: Option<&'types AnyAttributeInfo>,
}

#[derive(Debug)]
#[allow(dead_code)] // TODO
pub(super) struct ComplexTypeStruct<'types> {
    pub base: ComplexTypeBase,
    pub mode: StructMode<'types>,

    pub attributes: Vec<ComplexTypeAttribute<'types>>,
    pub any_attribute: Option<&'types AnyAttributeInfo>,
}

#[derive(Debug)]
#[allow(dead_code)] // TODO
pub(super) enum StructMode<'types> {
    Empty {
        any_element: Option<&'types AnyInfo>,
    },
    Content {
        content: ComplexTypeContent,
    },
    All {
        elements: Vec<ComplexTypeElement<'types>>,
        any_element: Option<&'types AnyInfo>,
    },
    Sequence {
        elements: Vec<ComplexTypeElement<'types>>,
        any_element: Option<&'types AnyInfo>,
    },
}

#[derive(Debug)]
#[allow(dead_code)] // TODO
pub(super) struct ComplexTypeContent {
    pub occurs: Occurs,
    pub is_simple: bool,
    pub min_occurs: MinOccurs,
    pub max_occurs: MaxOccurs,
    pub target_type: IdentPath,
}

#[derive(Debug)]
pub(super) struct ComplexTypeElement<'types> {
    pub info: &'types ElementInfo,
    pub occurs: Occurs,
    pub s_name: String,
    pub b_name: Literal,
    pub tag_name: String,
    pub field_ident: Ident2,
    pub variant_ident: Ident2,
    pub target_type: IdentPath,
    pub need_indirection: bool,
    pub target_is_dynamic: bool,
}

#[derive(Debug)]
pub(super) struct ComplexTypeAttribute<'types> {
    pub info: &'types AttributeInfo,
    pub ident: Ident2,
    pub s_name: String,
    pub b_name: Literal,
    pub tag_name: String,
    pub is_option: bool,
    pub target_type: IdentPath,
    pub default_value: Option<TokenStream>,
}

#[derive(Debug, Clone)]
enum TypeMode {
    All,
    Choice,
    Sequence,
    Simple { target_type: IdentPath },
}

impl<'types> ComplexType<'types> {
    fn new_all(info: &'types GroupInfo, req: Request<'_, 'types>) -> Result<Self, Error> {
        Self::new(
            req,
            TypeMode::All,
            1,
            MaxOccurs::Bounded(1),
            &[],
            None,
            &info.elements,
            info.any.as_ref(),
        )
    }

    fn new_choice(info: &'types GroupInfo, req: Request<'_, 'types>) -> Result<Self, Error> {
        Self::new(
            req,
            TypeMode::Choice,
            1,
            MaxOccurs::Bounded(1),
            &[],
            None,
            &info.elements,
            info.any.as_ref(),
        )
    }

    fn new_sequence(info: &'types GroupInfo, req: Request<'_, 'types>) -> Result<Self, Error> {
        Self::new(
            req,
            TypeMode::Sequence,
            1,
            MaxOccurs::Bounded(1),
            &[],
            None,
            &info.elements,
            info.any.as_ref(),
        )
    }

    fn new_complex(info: &'types ComplexInfo, mut req: Request<'_, 'types>) -> Result<Self, Error> {
        let (type_mode, elements, any_element) = match info
            .content
            .as_ref()
            .and_then(|ident| req.types.get_resolved(ident).map(|ty| (ty, ident)))
        {
            None => (TypeMode::Sequence, &[][..], None),
            Some((Type::All(si), _)) => (TypeMode::All, &si.elements[..], si.any.as_ref()),
            Some((Type::Choice(si), _)) => (TypeMode::Choice, &si.elements[..], si.any.as_ref()),
            Some((Type::Sequence(si), _)) => {
                (TypeMode::Sequence, &si.elements[..], si.any.as_ref())
            }
            Some((
                Type::BuildIn(_) | Type::Union(_) | Type::Enumeration(_) | Type::Reference(_),
                ident,
            )) => {
                let content_ref = req.get_or_create_type_ref(ident.clone())?;
                let target_type = IdentPath::from_type_ref(content_ref);

                (TypeMode::Simple { target_type }, &[][..], None)
            }
            Some((x, _)) => {
                let ident = &req.current_type_ref().type_ident;

                tracing::warn!("Complex type has unexpected content: ident={ident}, info={info:#?}, content={x:#?}!");

                (TypeMode::Sequence, &[][..], None)
            }
        };

        Self::new(
            req,
            type_mode,
            info.min_occurs,
            info.max_occurs,
            &info.attributes,
            info.any_attribute.as_ref(),
            elements,
            any_element,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        req: Request<'_, 'types>,
        type_mode: TypeMode,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeInfo],
        any_attribute: Option<&'types AnyAttributeInfo>,
        elements: &'types [ElementInfo],
        any_element: Option<&'types AnyInfo>,
    ) -> Result<Self, Error> {
        match type_mode {
            TypeMode::Simple { target_type } => Self::new_simple(
                req,
                target_type,
                min_occurs,
                max_occurs,
                attributes,
                any_attribute,
            ),
            TypeMode::Choice => Self::new_enum(
                req,
                min_occurs,
                max_occurs,
                attributes,
                any_attribute,
                elements,
                any_element,
            ),
            TypeMode::All | TypeMode::Sequence => Self::new_struct(
                req,
                &type_mode,
                min_occurs,
                max_occurs,
                attributes,
                any_attribute,
                elements,
                any_element,
            ),
        }
    }

    fn new_simple(
        mut req: Request<'_, 'types>,
        target_type: IdentPath,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeInfo],
        any_attribute: Option<&'types AnyAttributeInfo>,
    ) -> Result<Self, Error> {
        let base = ComplexTypeBase::new(&mut req)?;
        let occurs = Occurs::from_occurs(min_occurs, max_occurs);
        let attributes = attributes
            .iter()
            .filter_map(|info| ComplexTypeAttribute::new_field(info, &mut req).transpose())
            .collect::<Result<Vec<_>, _>>()?;

        let content = ComplexTypeContent {
            occurs,
            is_simple: true,
            min_occurs,
            max_occurs,
            target_type,
        };
        let type_ = ComplexTypeStruct {
            base,

            mode: StructMode::Content { content },

            attributes,
            any_attribute,
        };

        Ok(Self::Struct {
            type_,
            content_type: None,
        })
    }

    fn new_enum(
        mut req: Request<'_, 'types>,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeInfo],
        any_attribute: Option<&'types AnyAttributeInfo>,
        elements: &'types [ElementInfo],
        any_element: Option<&'types AnyInfo>,
    ) -> Result<Self, Error> {
        let base = ComplexTypeBase::new(&mut req)?;
        let occurs = Occurs::from_occurs(min_occurs, max_occurs);
        let flatten = occurs == Occurs::Single
            && attributes.is_empty()
            && req.check_flags(GeneratorFlags::FLATTEN_ENUM_CONTENT);

        let attributes = attributes
            .iter()
            .filter_map(|info| ComplexTypeAttribute::new_field(info, &mut req).transpose())
            .collect::<Result<Vec<_>, _>>()?;

        let mut any_element = any_element;
        let mut elements = elements
            .iter()
            .filter_map(|info| {
                ComplexTypeElement::new_variant(info, &mut req, occurs.is_direct()).transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if flatten {
            let type_ = ComplexTypeEnum {
                base,
                elements,
                any_element,
                any_attribute,
            };

            return Ok(ComplexType::Enum {
                type_,
                content_type: None,
            });
        }

        let type_ident = &base.type_ident;
        let content_ident = format_ident!("{type_ident}Content");
        let has_content = occurs.is_some() && !elements.is_empty();

        let content_type = has_content.then(|| {
            let type_ = ComplexTypeEnum {
                base: ComplexTypeBase::new_empty(content_ident.clone()),
                elements: take(&mut elements),
                any_element: any_element.take(),
                any_attribute: None,
            };

            Box::new(ComplexType::Enum {
                type_,
                content_type: None,
            })
        });

        let mode = if has_content {
            let type_ref = req.current_type_ref();
            let target_type = IdentPath::from_type_ref(type_ref).with_ident(content_ident.clone());
            let content = ComplexTypeContent {
                occurs,
                is_simple: false,
                min_occurs,
                max_occurs,
                target_type,
            };

            StructMode::Content { content }
        } else {
            StructMode::Empty { any_element }
        };

        let type_ = ComplexTypeStruct {
            base,
            mode,

            attributes,
            any_attribute,
        };

        Ok(ComplexType::Struct {
            type_,
            content_type,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn new_struct(
        mut req: Request<'_, 'types>,
        type_mode: &TypeMode,
        min_occurs: MinOccurs,
        max_occurs: MaxOccurs,
        attributes: &'types [AttributeInfo],
        any_attribute: Option<&'types AnyAttributeInfo>,
        elements: &'types [ElementInfo],
        any_element: Option<&'types AnyInfo>,
    ) -> Result<Self, Error> {
        let base = ComplexTypeBase::new(&mut req)?;
        let occurs = Occurs::from_occurs(min_occurs, max_occurs);
        let flatten =
            occurs == Occurs::Single && req.check_flags(GeneratorFlags::FLATTEN_STRUCT_CONTENT);

        let attributes = attributes
            .iter()
            .filter_map(|info| ComplexTypeAttribute::new_field(info, &mut req).transpose())
            .collect::<Result<Vec<_>, _>>()?;

        let elements = elements
            .iter()
            .filter_map(|info| {
                ComplexTypeElement::new_field(info, &mut req, occurs.is_direct()).transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if flatten {
            let mode = match type_mode {
                _ if elements.is_empty() => StructMode::Empty { any_element },
                TypeMode::All => StructMode::All {
                    elements,
                    any_element,
                },
                TypeMode::Sequence => StructMode::Sequence {
                    elements,
                    any_element,
                },
                _ => crate::unreachable!(),
            };

            let type_ = ComplexTypeStruct {
                base,
                mode,

                attributes,
                any_attribute,
            };

            return Ok(ComplexType::Struct {
                type_,
                content_type: None,
            });
        }

        let type_ident = &base.type_ident;
        let content_ident = format_ident!("{type_ident}Content");
        let has_content = occurs.is_some() && !elements.is_empty();

        let content_type = has_content.then(|| {
            let mode = match type_mode {
                TypeMode::All => StructMode::All {
                    elements,
                    any_element,
                },
                TypeMode::Sequence => StructMode::Sequence {
                    elements,
                    any_element,
                },
                _ => crate::unreachable!(),
            };

            let type_ = ComplexTypeStruct {
                base: ComplexTypeBase::new_empty(content_ident.clone()),
                mode,

                attributes: Vec::new(),
                any_attribute: None,
            };

            Box::new(ComplexType::Struct {
                type_,
                content_type: None,
            })
        });

        let mode = if has_content {
            let type_ref = req.current_type_ref();
            let target_type = IdentPath::from_type_ref(type_ref).with_ident(content_ident.clone());
            let content = ComplexTypeContent {
                occurs,
                is_simple: false,
                min_occurs,
                max_occurs,
                target_type,
            };

            StructMode::Content { content }
        } else {
            StructMode::Empty { any_element }
        };

        let type_ = ComplexTypeStruct {
            base,
            mode,

            attributes,
            any_attribute,
        };

        Ok(ComplexType::Struct {
            type_,
            content_type,
        })
    }
}

impl ComplexTypeBase {
    pub(super) fn element_tag(&self) -> Option<&String> {
        self.is_complex.then_some(self.tag_name.as_ref()).flatten()
    }

    pub(crate) fn represents_element(&self) -> bool {
        self.is_complex && self.tag_name.is_some() && !self.is_dynamic
    }

    fn new(req: &mut Request<'_, '_>) -> Result<Self, Error> {
        let type_ref = req.current_type_ref();
        let type_ident = type_ref.type_ident.clone();

        let mut ret = Self::new_empty(type_ident);
        ret.tag_name = Some(make_tag_name(req.types, req.ident));
        ret.trait_impls = req.make_trait_impls()?;

        if let Some(Type::ComplexType(ci)) = req.types.get(req.ident) {
            ret.is_complex = true;
            ret.is_dynamic = ci.is_dynamic;
        }

        Ok(ret)
    }

    fn new_empty(type_ident: Ident2) -> Self {
        let serializer_ident = format_ident!("{type_ident}Serializer");
        let serializer_state_ident = format_ident!("{type_ident}SerializerState");

        let deserializer_ident = format_ident!("{type_ident}Deserializer");
        let deserializer_state_ident = format_ident!("{type_ident}DeserializerState");

        Self {
            type_ident,
            trait_impls: Vec::new(),

            tag_name: None,
            is_complex: false,
            is_dynamic: false,

            serializer_ident,
            serializer_state_ident,

            deserializer_ident,
            deserializer_state_ident,
        }
    }
}

impl Deref for ComplexTypeEnum<'_> {
    type Target = ComplexTypeBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl ComplexTypeStruct<'_> {
    pub(super) fn has_attributes(&self) -> bool {
        !self.attributes.is_empty()
    }

    pub(super) fn has_content(&self) -> bool {
        match &self.mode {
            StructMode::All { elements, .. } | StructMode::Sequence { elements, .. } => {
                !elements.is_empty()
            }
            StructMode::Content { .. } => true,
            StructMode::Empty { .. } => false,
        }
    }

    pub(super) fn elements(&self) -> &[ComplexTypeElement<'_>] {
        if let StructMode::All { elements, .. } | StructMode::Sequence { elements, .. } = &self.mode
        {
            elements
        } else {
            &[]
        }
    }

    pub(super) fn any_element(&self) -> Option<&AnyInfo> {
        if let StructMode::All { any_element, .. } | StructMode::Sequence { any_element, .. } =
            &self.mode
        {
            *any_element
        } else {
            None
        }
    }

    pub(super) fn content(&self) -> Option<&ComplexTypeContent> {
        if let StructMode::Content { content, .. } = &self.mode {
            Some(content)
        } else {
            None
        }
    }
}

impl Deref for ComplexTypeStruct<'_> {
    type Target = ComplexTypeBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'types> ComplexTypeElement<'types> {
    fn new_variant(
        info: &'types ElementInfo,
        req: &mut Request<'_, 'types>,
        direct_usage: bool,
    ) -> Result<Option<Self>, Error> {
        let force_box = req.box_flags.intersects(BoxFlags::ENUM_ELEMENTS);

        Self::new(info, req, direct_usage, force_box)
    }

    fn new_field(
        info: &'types ElementInfo,
        req: &mut Request<'_, 'types>,
        direct_usage: bool,
    ) -> Result<Option<Self>, Error> {
        let force_box = req.box_flags.intersects(BoxFlags::STRUCT_ELEMENTS);

        Self::new(info, req, direct_usage, force_box)
    }

    fn new(
        info: &'types ElementInfo,
        req: &mut Request<'_, 'types>,
        direct_usage: bool,
        force_box: bool,
    ) -> Result<Option<Self>, Error> {
        let occurs = Occurs::from_occurs(info.min_occurs, info.max_occurs);
        if occurs == Occurs::None {
            return Ok(None);
        }

        let tag_name = make_tag_name(req.types, &info.ident);
        let s_name = info.ident.name.to_string();
        let b_name = Literal::byte_string(s_name.as_bytes());
        let field_ident = format_field_ident(&info.ident.name, info.display_name.as_deref());
        let variant_ident = format_variant_ident(&info.ident.name, info.display_name.as_deref());

        let target_ref = req.get_or_create_type_ref(info.type_.clone())?;
        let target_type = IdentPath::from_type_ref(target_ref);

        let need_box = req.current_type_ref().boxed_elements.contains(&info.ident);
        let need_indirection = (direct_usage && need_box) || force_box;
        let target_is_dynamic = is_dynamic(&info.type_, req.types);

        Ok(Some(Self {
            info,
            occurs,
            s_name,
            b_name,
            tag_name,
            field_ident,
            variant_ident,
            target_type,
            need_indirection,
            target_is_dynamic,
        }))
    }
}

impl<'types> ComplexTypeAttribute<'types> {
    fn new_field(
        info: &'types AttributeInfo,
        req: &mut Request<'_, 'types>,
    ) -> Result<Option<Self>, Error> {
        if info.use_ == Use::Prohibited {
            return Ok(None);
        }

        let current_module = req.current_module();
        let target_ref = req.get_or_create_type_ref(info.type_.clone())?;
        let ident = format_field_ident(&info.ident.name, info.display_name.as_deref());
        let target_type = IdentPath::from_type_ref(target_ref);
        let s_name = info.ident.name.to_string();
        let b_name = Literal::byte_string(s_name.as_bytes());
        let tag_name = make_tag_name(req.types, &info.ident);

        let default_value = info
            .default
            .as_ref()
            .map(|default| req.get_default(current_module, default, &info.type_))
            .transpose()?;
        let is_option = matches!((&info.use_, &default_value), (Use::Optional, None));

        Ok(Some(Self {
            info,
            ident,
            s_name,
            b_name,
            tag_name,
            is_option,
            target_type,
            default_value,
        }))
    }
}

/* Helper */

macro_rules! impl_render {
    ($ty:ident) => {
        impl<'types> $ty<'types> {
            pub(super) fn render(&self, config: &Config<'types>, ctx: &mut Context<'_, 'types>) {
                self.render_types(ctx);
                self.render_impl(ctx);

                if config.flags.intersects(GeneratorFlags::QUICK_XML_SERIALIZE) {
                    self.render_serializer(ctx);
                }

                if config
                    .flags
                    .intersects(GeneratorFlags::QUICK_XML_DESERIALIZE)
                {
                    self.render_deserializer(ctx);
                }
            }
        }
    };
}

impl_render!(UnionType);
impl_render!(DynamicType);
impl_render!(ReferenceType);
impl_render!(EnumerationType);
impl_render!(ComplexType);

fn is_dynamic(ident: &Ident, types: &Types) -> bool {
    let Some(ty) = types.get(ident) else {
        return false;
    };

    match ty {
        Type::Dynamic(_) => true,
        Type::ComplexType(ci) => ci.is_dynamic,
        Type::Reference(x) if x.is_single() => is_dynamic(&x.type_, types),
        _ => false,
    }
}

fn make_tag_name(types: &Types, ident: &Ident) -> String {
    let name = ident.name.to_string();

    if let Some(module) = ident
        .ns
        .as_ref()
        .and_then(|ns| types.modules.get(ns))
        .and_then(|module| module.name.as_ref())
    {
        format!("{module}:{name}")
    } else {
        name
    }
}

fn make_derived_type_data<'types>(
    req: &mut Request<'_, 'types>,
    ident: &'types Ident,
) -> Result<DerivedType, Error> {
    let s_name = ident.name.to_string();
    let b_name = Literal::byte_string(s_name.as_bytes());

    let ty = req
        .types
        .get(ident)
        .ok_or_else(|| Error::UnknownType(ident.clone()))?;
    let ident = (if let Type::Dynamic(di) = ty {
        di.type_.clone()
    } else {
        None
    })
    .unwrap_or(ident.clone());

    let target_ref = req.get_or_create_type_ref(ident.clone())?;
    let target_type = IdentPath::from_type_ref(target_ref);
    let variant_ident = format_variant_ident(&ident.name, None);

    Ok(DerivedType {
        ident,
        s_name,
        b_name,
        target_type,
        variant_ident,
    })
}
