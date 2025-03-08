//! The `generator` module contains the code [`Generator`] and all related types.

mod data;
mod error;
mod helper;
mod misc;
mod renderer;

use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fmt::Display;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, quote};
use tracing::instrument;

use crate::types::{Ident, IdentType, Type, Types};

pub use self::error::Error;
pub use self::misc::{BoxFlags, GeneratorFlags, SerdeSupport, TypedefMode};

use self::data::{Code, Request};
use self::helper::Walk;
use self::misc::{
    format_module, format_type_ident, make_type_name, DynTypeTraits, Module, Modules, PendingType,
    TraitInfos, TypeRef,
};

/// Type that is used to generate rust code from a [`Types`] object.
#[must_use]
#[derive(Debug)]
pub struct Generator<'types> {
    config: Config<'types>,
    state: State<'types>,
}

/// Type that is used to generate rust code from a [`Types`] object.
///
/// In contrast to [`Generator`] this type does not allow changes to the
/// configuration of the generator, because at least one type was already
/// generated.
#[must_use]
#[derive(Debug)]
pub struct GeneratorFixed<'types> {
    config: Config<'types>,
    state: State<'types>,
    modules: Modules,
}

#[derive(Debug)]
struct Config<'types> {
    types: &'types Types,

    flags: GeneratorFlags,
    derive: Vec<Ident2>,
    postfixes: [String; 8],
    box_flags: BoxFlags,
    typedef_mode: TypedefMode,
    serde_support: SerdeSupport,
    dyn_type_traits: DynTypeTraits,
    xsd_parser_crate: Ident2,
}

#[derive(Debug)]
struct State<'types> {
    cache: BTreeMap<Ident, TypeRef>,
    trait_infos: Option<TraitInfos>,
    pending: VecDeque<PendingType<'types>>,
}

/* Generator */

impl<'types> Generator<'types> {
    /// Create a new code generator from the passed `types`.
    pub fn new(types: &'types Types) -> Self {
        let config = Config {
            types,
            flags: GeneratorFlags::empty(),
            derive: vec![format_ident!("Debug"), format_ident!("Clone")],
            postfixes: [
                String::from("Type"),        // Type = 0
                String::new(),               // Group = 1
                String::from("ElementType"), // Element = 2
                String::new(),               // ElementType = 3
                String::new(),               // Attribute = 4
                String::new(),               // AttributeGroup = 5
                String::new(),               // BuildIn = 6
                String::new(),               // Enumeration = 7
            ],
            box_flags: BoxFlags::AUTO,
            typedef_mode: TypedefMode::Auto,
            serde_support: SerdeSupport::None,
            dyn_type_traits: DynTypeTraits::Auto,
            xsd_parser_crate: format_ident!("xsd_parser"),
        };
        let state = State {
            cache: BTreeMap::new(),
            trait_infos: None,
            pending: VecDeque::new(),
        };

        Self { config, state }
    }

    /// Set the name of the `xsd-parser` create that the generator should use for
    /// generating the code.
    pub fn xsd_parser_crate<S: Display>(mut self, value: S) -> Self {
        self.config.xsd_parser_crate = format_ident!("{value}");

        self
    }

    /// Set the traits the generated types should derive from.
    ///
    /// Default is `Debug` and `Clone`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .derive(["Debug", "Clone", "Eq", "PartialEq", "Ord", "PartialOrd"]);
    /// ```
    pub fn derive<I>(mut self, value: I) -> Self
    where
        I: IntoIterator,
        I::Item: Display,
    {
        self.config.derive = value.into_iter().map(|x| format_ident!("{x}")).collect();

        self
    }

    /// Set the traits that should be implemented by dynamic types.
    ///
    /// The passed values must be valid type paths.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .dyn_type_traits(["core::fmt::Debug", "core::any::Any"]);
    /// ```
    pub fn dyn_type_traits<I>(mut self, value: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        fn parse_path(s: &str) -> TokenStream {
            let parts = s.split("::").map(|x| format_ident!("{x}"));

            quote! {
                #( #parts )*
            }
        }

        self.config.dyn_type_traits =
            DynTypeTraits::Custom(value.into_iter().map(|x| parse_path(&x.into())).collect());

        self
    }

    /// Set the [`BoxFlags`] flags the generator should use for generating the code.
    pub fn box_flags(mut self, value: BoxFlags) -> Self {
        self.config.box_flags = value;

        self
    }

    /// Set the [`TypedefMode`] value the generator should use for generating the code.
    pub fn typedef_mode(mut self, value: TypedefMode) -> Self {
        self.config.typedef_mode = value;

        self
    }

    /// Set the [`SerdeSupport`] value the generator should use for generating the code.
    pub fn serde_support(mut self, value: SerdeSupport) -> Self {
        self.config.serde_support = value;

        self
    }

    /// Set the [`GeneratorFlags`] flags the generator should use for generating the code.
    pub fn flags(mut self, value: GeneratorFlags) -> Self {
        self.config.flags = value;

        self
    }

    /// Add the passed [`GeneratorFlags`] flags the generator should use for generating the code.
    pub fn with_flags(mut self, value: GeneratorFlags) -> Self {
        self.config.flags |= value;

        self
    }

    /// Set the postfixes the generator should use for the different types.
    ///
    /// Default is `"Type"` for the [`IdentType::Type`] type and `""` for the other types.
    pub fn with_type_postfix<S: Into<String>>(mut self, type_: IdentType, postfix: S) -> Self {
        self.config.postfixes[type_ as usize] = postfix.into();

        self
    }

    /// Add a custom implemented type to the generator.
    ///
    /// This will add a custom implemented type to the generator. These types are
    /// usually implemented and provided by the user of the generated code. The
    /// generator will just reference to the type definition and will not generate
    /// any code related to this type.
    ///
    /// # Errors
    ///
    /// Returns an error if the namespace of the passed identifier is unknown.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .with_type(Ident::type_("UserDefinedType"));
    /// ```
    pub fn with_type(mut self, ident: Ident) -> Result<Self, Error> {
        let module_ident = format_module(self.config.types, ident.ns)?;
        let type_ident = format_ident!("{}", ident.name.to_string());

        let type_ref = TypeRef {
            ns: ident.ns,
            module_ident,
            type_ident,
            boxed_elements: HashSet::new(),
        };
        self.state.cache.insert(ident, type_ref);

        Ok(self)
    }

    /// Will fix the generator by call [`into_fixed`](Self::into_fixed) and then
    /// [`generate_type`](GeneratorFixed::generate_type).
    #[instrument(err, level = "trace", skip(self))]
    pub fn generate_type(self, ident: Ident) -> Result<GeneratorFixed<'types>, Error> {
        self.into_fixed().generate_type(ident)
    }

    /// Will fix the generator by call [`into_fixed`](Self::into_fixed) and then
    /// [`generate_all_types`](GeneratorFixed::generate_all_types).
    ///
    /// # Errors
    ///
    /// Will just forward the errors from [`generate_all_types`](GeneratorFixed::generate_all_types).
    pub fn generate_all_types(self) -> Result<GeneratorFixed<'types>, Error> {
        self.into_fixed().generate_all_types()
    }

    /// Will convert the generator into a [`GeneratorFixed`].
    pub fn into_fixed(self) -> GeneratorFixed<'types> {
        let Self { config, state } = self;
        let modules = Modules::default();

        GeneratorFixed {
            config,
            state,
            modules,
        }
    }
}

impl<'types> GeneratorFixed<'types> {
    /// Generate the code for the given type.
    ///
    /// This will generate the code for the passed type identifier and all
    /// dependencies of this type.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .generate_type(Ident::type_("Root"));
    /// ```
    #[instrument(err, level = "trace", skip(self))]
    pub fn generate_type(mut self, ident: Ident) -> Result<GeneratorFixed<'types>, Error> {
        self.state.get_or_create_type_ref(&self.config, ident)?;
        self.generate_pending()?;

        Ok(self)
    }

    /// Generate the code for all named types.
    ///
    /// This will generate the code for all types with an explicit name and all
    /// dependencies of these types that are specified in the [`Types`] object
    /// passed to the generator.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .generate_all_types();
    /// ```
    #[instrument(err, level = "trace", skip(self))]
    pub fn generate_all_types(mut self) -> Result<Self, Error> {
        for ident in self.config.types.keys() {
            if ident.name.is_named() {
                self.state
                    .get_or_create_type_ref(&self.config, ident.clone())?;
            }
        }
        self.generate_pending()?;

        Ok(self)
    }

    /// Finish the code generation.
    ///
    /// This will return the generated code as [`TokenStream`].
    #[instrument(level = "trace", skip(self))]
    pub fn finish(self) -> TokenStream {
        let serde = serde_includes(self.config.serde_support);

        let modules = self.modules.0.into_iter().map(|(ns, module)| {
            let Module {
                code,
                quick_xml_serialize,
                quick_xml_deserialize,
            } = module;

            let quick_xml_serialize = quick_xml_serialize.map(|code| {
                quote! {
                    pub mod quick_xml_serialize {
                        use super::*;

                        #code
                    }
                }
            });

            let quick_xml_deserialize = quick_xml_deserialize.map(|code| {
                quote! {
                    pub mod quick_xml_deserialize {
                        use super::*;

                        #code
                    }
                }
            });

            let code = quote! {
                #code
                #quick_xml_serialize
                #quick_xml_deserialize
            };

            if let Some(name) =
                ns.and_then(|ns| format_module(self.config.types, Some(ns)).unwrap())
            {
                quote! {
                    pub mod #name {
                        use super::*;

                        #code
                    }
                }
            } else {
                code
            }
        });

        quote! {
            #serde

            #( #modules )*
        }
    }

    #[instrument(err, level = "trace", skip(self))]
    fn generate_pending(&mut self) -> Result<(), Error> {
        while let Some(args) = self.state.pending.pop_front() {
            self.generate_type_intern(args)?;
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn generate_type_intern(&mut self, data: PendingType<'types>) -> Result<(), Error> {
        let PendingType { ty, ident } = data;
        let Self {
            config,
            state,
            modules,
        } = self;
        let req = Request::new(&ident, config, state);
        let mut code = Code::new(&ident, config, modules);

        tracing::debug!("Render type: {ident}");

        match ty {
            Type::BuildIn(_) => (),
            Type::Union(x) => req.into_union_type(x)?.render(config, &mut code),
            Type::Dynamic(x) => req.into_dynamic_type(x)?.render(config, &mut code),
            Type::Reference(x) => req.into_reference_type(x)?.render(config, &mut code),
            Type::Enumeration(x) => req.into_enumeration_type(x)?.render(config, &mut code),
            Type::All(x) => req.into_all_type(x)?.render(config, &mut code),
            Type::Choice(x) => req.into_choice_type(x)?.render(config, &mut code),
            Type::Sequence(x) => req.into_sequence_type(x)?.render(config, &mut code),
            Type::ComplexType(x) => req.into_complex_type(x)?.render(config, &mut code),
        }

        Ok(())
    }
}

impl Config<'_> {
    fn check_flags(&self, flags: GeneratorFlags) -> bool {
        self.flags.intersects(flags)
    }
}

impl<'types> State<'types> {
    #[instrument(level = "trace", skip(self))]
    fn get_or_create_type_ref(
        &mut self,
        config: &Config<'types>,
        ident: Ident,
    ) -> Result<&TypeRef, Error> {
        if !self.cache.contains_key(&ident) {
            let ty = config
                .types
                .get(&ident)
                .ok_or_else(|| Error::UnknownType(ident.clone()))?;
            let name = make_type_name(&config.postfixes, ty, &ident);
            let (module_ident, type_ident) = if let Type::BuildIn(x) = ty {
                (None, format_ident!("{x}"))
            } else {
                let use_modules = config.flags.intersects(GeneratorFlags::USE_MODULES);
                let module_ident =
                    format_module(config.types, use_modules.then_some(ident.ns).flatten())?;
                let type_ident = format_type_ident(&name, None);

                (module_ident, type_ident)
            };

            tracing::debug!("Queue new type generation: {ident}");

            let boxed_elements = get_boxed_elements(&ident, ty, config.types, &self.cache);
            self.pending.push_back(PendingType {
                ty,
                ident: ident.clone(),
            });

            let type_ref = TypeRef {
                module_ident,
                ns: ident.ns,
                type_ident,
                boxed_elements,
            };

            assert!(self.cache.insert(ident.clone(), type_ref).is_none());
        }

        Ok(self.cache.get_mut(&ident).unwrap())
    }
}

/* Helper */

fn serde_includes(serde_support: SerdeSupport) -> Option<TokenStream> {
    let serde = matches!(
        serde_support,
        SerdeSupport::QuickXml | SerdeSupport::SerdeXmlRs
    );

    serde.then(|| {
        quote!(
            use serde::{Serialize, Deserialize};
        )
    })
}

fn get_boxed_elements<'a>(
    ident: &Ident,
    mut ty: &'a Type,
    types: &'a Types,
    cache: &BTreeMap<Ident, TypeRef>,
) -> HashSet<Ident> {
    if let Type::ComplexType(ci) = ty {
        if let Some(type_) = ci.content.as_ref().and_then(|ident| types.get(ident)) {
            ty = type_;
        }
    }

    match ty {
        Type::All(si) | Type::Choice(si) | Type::Sequence(si) => si
            .elements
            .iter()
            .filter_map(|f| {
                if Walk::new(types, cache).is_loop(ident, &f.type_) {
                    Some(f.ident.clone())
                } else {
                    None
                }
            })
            .collect(),
        _ => HashSet::new(),
    }
}
