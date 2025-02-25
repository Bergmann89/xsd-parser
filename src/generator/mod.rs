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
pub use self::misc::{BoxFlags, GenerateFlags, SerdeSupport, TypedefMode};

use self::data::TypeData;
use self::helper::Walk;
use self::misc::{
    format_module, format_type_ident, make_type_name, DynTypeTraits, Module, Modules, PendingType,
    StateFlags, TraitInfos, TypeRef,
};

/// Type that is used to generate rust code from a [`Types`] object.
#[must_use]
#[derive(Debug)]
pub struct Generator<'types> {
    types: &'types Types,

    /* state */
    cache: BTreeMap<Ident, TypeRef>,
    traits: Option<TraitInfos>,
    pending: VecDeque<PendingType<'types>>,

    /* code */
    modules: Modules,

    /* config */
    derive: Vec<Ident2>,
    postfixes: [String; 8],
    box_flags: BoxFlags,
    typedef_mode: TypedefMode,
    serde_support: SerdeSupport,
    generate_flags: GenerateFlags,
    dyn_type_traits: DynTypeTraits,
    xsd_parser_crate: Ident2,
}

/* Generator */

impl<'types> Generator<'types> {
    /// Create a new code generator from the passed `types`.
    pub fn new(types: &'types Types) -> Self {
        Self {
            types,

            cache: BTreeMap::new(),
            traits: None,
            pending: VecDeque::new(),
            modules: Modules::default(),

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
            generate_flags: GenerateFlags::empty(),
            dyn_type_traits: DynTypeTraits::Auto,
            xsd_parser_crate: format_ident!("xsd_parser"),
        }
    }

    /// Set the name of the `xsd-parser` create that the generator should use for
    /// generating the code.
    pub fn xsd_parser_crate<S: Display>(mut self, value: S) -> Self {
        self.xsd_parser_crate = format_ident!("{value}");

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
        self.derive = value.into_iter().map(|x| format_ident!("{x}")).collect();

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

        self.dyn_type_traits =
            DynTypeTraits::Custom(value.into_iter().map(|x| parse_path(&x.into())).collect());

        self
    }

    /// Set the [`BoxFlags`] flags the generator should use for generating the code.
    pub fn box_flags(mut self, value: BoxFlags) -> Self {
        self.box_flags = value;

        self
    }

    /// Set the [`TypedefMode`] value the generator should use for generating the code.
    pub fn typedef_mode(mut self, value: TypedefMode) -> Self {
        self.typedef_mode = value;

        self
    }

    /// Set the [`SerdeSupport`] value the generator should use for generating the code.
    pub fn serde_support(mut self, value: SerdeSupport) -> Self {
        self.serde_support = value;

        self
    }

    /// Set the [`GenerateFlags`] flags the generator should use for generating the code.
    pub fn generate_flags(mut self, value: GenerateFlags) -> Self {
        self.generate_flags = value;

        self
    }

    /// Add the passed [`GenerateFlags`] flags the generator should use for generating the code.
    pub fn with_generate_flags(mut self, value: GenerateFlags) -> Self {
        self.generate_flags |= value;

        self
    }

    /// Set the postfixes the generator should use for the different types.
    ///
    /// Default is `"Type"` for the [`IdentType::Type`] type and `""` for the other types.
    pub fn with_type_postfix<S: Into<String>>(mut self, type_: IdentType, postfix: S) -> Self {
        self.postfixes[type_ as usize] = postfix.into();

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
        let module_ident = format_module(self.types, ident.ns)?;
        let type_ident = format_ident!("{}", ident.name.to_string());

        let type_ref = TypeRef {
            ns: ident.ns,
            module_ident,
            type_ident,
            flags: StateFlags::empty(),
            boxed_elements: HashSet::new(),
        };
        self.cache.insert(ident, type_ref);

        Ok(self)
    }

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
    pub fn generate_type(mut self, ident: Ident) -> Result<Self, Error> {
        self.get_or_create_type_ref(ident)?;
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
        for ident in self.types.keys() {
            if ident.name.is_named() {
                self.get_or_create_type_ref(ident.clone())?;
            }
        }
        self.generate_pending()?;

        Ok(self)
    }

    /// Finish the code generation.
    ///
    /// THis will return the generated code as [`TokenStream`].
    #[instrument(level = "trace", skip(self))]
    pub fn finish(self) -> TokenStream {
        let Self {
            modules,
            types,
            serde_support,
            ..
        } = self;

        let serde = serde_includes(serde_support);

        let modules = modules.0.into_iter().map(|(ns, module)| {
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

            if let Some(name) = ns.and_then(|ns| format_module(types, Some(ns)).unwrap()) {
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

    fn get_traits(&mut self) -> &TraitInfos {
        self.traits
            .get_or_insert_with(|| TraitInfos::new(self.types))
    }

    #[instrument(level = "trace", skip(self))]
    fn get_or_create_type_ref(&mut self, ident: Ident) -> Result<&TypeRef, Error> {
        let Self {
            types,
            cache,
            pending,
            postfixes,
            generate_flags,
            ..
        } = self;

        if !cache.contains_key(&ident) {
            let ty = types
                .get(&ident)
                .ok_or_else(|| Error::UnknownType(ident.clone()))?;
            let name = make_type_name(postfixes, ty, &ident);
            let (module_ident, type_ident) = if let Type::BuildIn(x) = ty {
                (None, format_ident!("{x}"))
            } else {
                let use_modules = generate_flags.intersects(GenerateFlags::USE_MODULES);
                let module_ident = format_module(types, use_modules.then_some(ident.ns).flatten())?;
                let type_ident = format_type_ident(&name, None);

                (module_ident, type_ident)
            };

            tracing::debug!("Queue new type generation: {ident}");

            let boxed_elements = get_boxed_elements(&ident, ty, types, cache);
            pending.push_back(PendingType {
                ty,
                ident: ident.clone(),
            });

            let type_ref = TypeRef {
                module_ident,
                ns: ident.ns,
                type_ident,
                flags: StateFlags::empty(),
                boxed_elements,
            };

            assert!(cache.insert(ident.clone(), type_ref).is_none());
        }

        Ok(cache.get_mut(&ident).unwrap())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn generate_pending(&mut self) -> Result<(), Error> {
        while let Some(args) = self.pending.pop_front() {
            self.generate_type_intern(args)?;
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn generate_type_intern(&mut self, data: PendingType<'types>) -> Result<(), Error> {
        let PendingType { ty, ident } = data;

        tracing::debug!("Render type: {ident}");

        let data = TypeData {
            ty,
            ident,
            generator: self,
        };

        match data.ty {
            Type::BuildIn(_) => Ok(()),
            Type::Union(x) => data.generate_union(x),
            Type::Dynamic(x) => data.generate_dynamic(x),
            Type::Reference(x) => data.generate_reference(x),
            Type::Enumeration(x) => data.generate_enumeration(x),
            Type::ComplexType(x) => data.generate_complex_type(x),
            Type::All(x) => data.generate_all(x),
            Type::Choice(x) => data.generate_choice(x),
            Type::Sequence(x) => data.generate_sequence(x),
        }
    }
}

/* TypeRef */

impl TypeRef {
    fn add_flag_checked(&mut self, flag: StateFlags) -> bool {
        let ret = self.flags.intersects(flag);

        self.flags |= flag;

        ret
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
