//! The `generator` module contains the code [`Generator`] and all related types.

pub mod renderer;

mod context;
mod data;
mod error;
mod helper;
mod misc;

use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fmt::Display;
use std::str::FromStr;

use proc_macro2::{Ident as Ident2, TokenStream};
use quote::{format_ident, ToTokens};
use renderer::{Renderer, TypesRenderer};
use smallvec::{smallvec, SmallVec};
use tracing::instrument;

use crate::code::{format_module_ident, format_type_ident, IdentPath, Module};
use crate::schema::NamespaceId;
use crate::types::{ComplexTypeVariant, Ident, IdentType, Name, SimpleTypeVariant, Type, Types};

pub use self::context::Context;
pub use self::data::{
    BuildInType, ComplexType, ComplexTypeAttribute, ComplexTypeBase, ComplexTypeContent,
    ComplexTypeElement, ComplexTypeEnum, ComplexTypeStruct, DerivedType, DynamicType,
    EnumerationType, EnumerationTypeVariant, ReferenceType, StructMode, TypeData, UnionType,
    UnionTypeVariant,
};
pub use self::error::Error;
pub use self::misc::{BoxFlags, GeneratorFlags, SerdeSupport, TypedefMode};

use self::helper::Walk;
use self::misc::{DynTypeTraits, PendingType, TraitInfos, TypeRef};

/// Type that is used to generate rust code from a [`Types`] object.
#[must_use]
#[derive(Debug)]
pub struct Generator<'types> {
    config: Config<'types>,
    state: State<'types>,
    renderers: Vec<Box<dyn Renderer>>,
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
    module: Module,
    renderers: Vec<Box<dyn Renderer>>,
}

/// Contains the configuration of the generator.
#[derive(Debug)]
pub struct Config<'types> {
    /// Reference to the types the code should be generated for.
    pub types: &'types Types,

    /// Flags that controls the behavior of the generator.
    pub flags: GeneratorFlags,

    /// Traits the generator should derive the generated types from.
    pub derive: Vec<Ident2>,

    /// List of postfixed to add to the name of the generated types.
    ///
    /// This corresponds to the variants of [`IdentType`].
    pub postfixes: [String; 8],

    /// Tells the generator how to deal with boxed elements.
    pub box_flags: BoxFlags,

    /// Tells the generator how to deal with type definitions.
    pub typedef_mode: TypedefMode,

    /// Tells the generator if and how to generate code to support [`serde`].
    pub serde_support: SerdeSupport,

    /// List of traits that should be implemented by dynamic types.
    pub dyn_type_traits: DynTypeTraits,

    /// Name of the `xsd_parser` crate.
    pub xsd_parser_crate: Ident2,
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
            derive: vec![format_ident!("Debug")],
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
        let renderers = Vec::new();

        Self {
            config,
            state,
            renderers,
        }
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
    /// # Errors
    ///
    /// Will raise a [`InvalidIdentifier`](Error::InvalidIdentifier) error
    /// if the passed strings does not represent a valid identifier.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let generator = Generator::new(types)
    ///     .dyn_type_traits(["core::fmt::Debug", "core::any::Any"]);
    /// ```
    pub fn dyn_type_traits<I>(mut self, value: I) -> Result<Self, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let traits = value
            .into_iter()
            .map(|x| {
                let s = x.as_ref();
                IdentPath::from_str(s).map_err(|()| Error::InvalidIdentifier(s.into()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        self.config.dyn_type_traits = DynTypeTraits::Custom(traits);

        Ok(self)
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
            ident: ident.clone(),
            module_ident,
            type_ident,
            boxed_elements: HashSet::new(),
        };
        self.state.cache.insert(ident, type_ref);

        Ok(self)
    }

    /// Add a [`Renderer`] to the generator.
    pub fn with_renderer<X>(mut self, renderer: X) -> Self
    where
        X: Renderer + 'static,
    {
        self.renderers.push(Box::new(renderer));

        self
    }

    /// Add the default renderers to the generator.
    pub fn with_default_renderers(self) -> Self {
        self.with_renderer(TypesRenderer)
    }

    /// Remove all [`Renderer`]s from the generator.
    pub fn clear_renderers(mut self) -> Self {
        self.renderers.clear();

        self
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
        let Self {
            mut config,
            state,
            mut renderers,
        } = self;
        let module = Module::default();

        config.dyn_type_traits = match config.dyn_type_traits {
            DynTypeTraits::Auto => {
                let traits = config.derive.iter().map(|x| match x.to_string().as_ref() {
                    "Debug" => IdentPath::from_str("core::fmt::Debug").unwrap(),
                    "Hash" => IdentPath::from_str("core::hash::Hash").unwrap(),
                    _ => IdentPath::from_ident(x.clone()),
                });

                let serde: SmallVec<[IdentPath; 2]> = if config.serde_support == SerdeSupport::None
                {
                    smallvec![]
                } else {
                    smallvec![
                        IdentPath::from_str("serde::Serialize").unwrap(),
                        IdentPath::from_str("serde::DeserializeOwned").unwrap()
                    ]
                };

                let as_any = IdentPath::from_parts(
                    Some(config.xsd_parser_crate.clone()),
                    format_ident!("AsAny"),
                );

                DynTypeTraits::Custom(traits.chain(serde).chain(Some(as_any)).collect())
            }
            x => x,
        };

        for renderer in &mut renderers {
            renderer.initialize(&mut config);
        }

        GeneratorFixed {
            config,
            state,
            module,
            renderers,
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

    /// Generate the code for all types.
    ///
    /// This will generate the code for all types that are specified in
    /// the [`Types`] object passed to the generator.
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
            self.state
                .get_or_create_type_ref(&self.config, ident.clone())?;
        }
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
    ///     .generate_named_types();
    /// ```
    #[instrument(err, level = "trace", skip(self))]
    pub fn generate_named_types(mut self) -> Result<Self, Error> {
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
        let Self {
            config,
            mut module,
            state: _,
            mut renderers,
        } = self;

        for renderer in &mut renderers {
            renderer.finish(&config, &mut module);
        }

        module.to_token_stream()
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
            module,
            renderers,
        } = self;
        let ty = TypeData::new(ty, &ident, config, state)?;
        let mut ctx = Context::new(&ident, config, module);

        tracing::debug!("Render type: {ident}");

        for renderer in renderers {
            renderer.render_type(&mut ctx, &ty);
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
    #[instrument(level = "trace", skip(self, config))]
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
            let (module_ident, type_ident) = if let Some(SimpleTypeVariant::BuildIn(x)) =
                ty.simple_type_ref().map(|a| &a.variant)
            {
                (None, format_ident!("{x}"))
            } else {
                let use_modules = config.flags.intersects(GeneratorFlags::USE_MODULES);
                let module_ident =
                    format_module(config.types, use_modules.then_some(ident.ns).flatten())?;
                let type_ident = format_type_ident(&name, ty.display_name());

                (module_ident, type_ident)
            };

            tracing::debug!("Queue new type generation: {ident}");

            let boxed_elements = get_boxed_elements(&ident, ty, config.types, &self.cache);
            self.pending.push_back(PendingType {
                ty,
                ident: ident.clone(),
            });

            let type_ref = TypeRef {
                ident: ident.clone(),
                type_ident,
                module_ident,
                boxed_elements,
            };

            assert!(self.cache.insert(ident.clone(), type_ref).is_none());
        }

        Ok(self.cache.get_mut(&ident).unwrap())
    }
}

/* Helper */

fn get_boxed_elements<'a>(
    ident: &Ident,
    mut ty: &'a Type,
    types: &'a Types,
    cache: &BTreeMap<Ident, TypeRef>,
) -> HashSet<Ident> {
    if let Some(ComplexTypeVariant::ComplexType(ci)) = ty.complex_type_ref().map(|a| &a.variant) {
        if let Some(type_) = ci.content.as_ref().and_then(|ident| types.get(ident)) {
            ty = type_;
        }
    }

    match ty.complex_type_ref().map(|a| &a.variant) {
        Some(
            ComplexTypeVariant::All(si)
            | ComplexTypeVariant::Choice(si)
            | ComplexTypeVariant::Sequence(si),
        ) => si
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

fn make_type_name(postfixes: &[String], ty: &Type, ident: &Ident) -> Name {
    if let Some(ti) = ty.reference_type() {
        if ident.name.is_generated() && ti.type_.name.is_named() {
            if ti.min_occurs > 1 {
                return Name::new_generated(format!("{}List", ti.type_.name.to_type_name()));
            } else if ti.min_occurs == 0 {
                return Name::new_generated(format!("{}Opt", ti.type_.name.to_type_name()));
            }
        }
    }

    let postfix = postfixes
        .get(ident.type_ as usize)
        .map_or("", |s| s.as_str());

    let s = ident.name.to_type_name();

    if s.ends_with(postfix) {
        ident.name.clone()
    } else {
        Name::new_generated(format!("{s}{postfix}"))
    }
}

fn format_module(types: &Types, ns: Option<NamespaceId>) -> Result<Option<Ident2>, Error> {
    let Some(ns) = ns else {
        return Ok(None);
    };

    let module = types.modules.get(&ns).ok_or(Error::UnknownNamespace(ns))?;
    let Some(name) = &module.name else {
        return Ok(None);
    };

    Ok(Some(format_module_ident(name)))
}
