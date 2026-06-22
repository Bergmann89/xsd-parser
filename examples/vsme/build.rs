//! This is a build script to generate the code for the `vsme` schema.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::env::var;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::{Context, Error};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use xsd_parser::models::data::TagName;
use xsd_parser::{
    config::{GeneratorFlags, IdentQuadruple, InterpreterFlags, OptimizerFlags, Schema},
    exec_generator_with_ident_cache, exec_interpreter_with_ident_cache, exec_optimizer,
    exec_parser, exec_render,
    models::{
        code::{Module, ModulePath},
        data::PathData,
        meta::{
            CustomMeta, ElementMeta, ElementMetaVariant, ElementMode, MetaType, MetaTypeVariant,
            MetaTypes,
        },
        schema::{xs::FormChoiceType, Schemas},
        ElementIdent, IdentType, Naming,
    },
    pipeline::{
        generator::{Context as GeneratorContext, Error as GeneratorError},
        renderer::{MetaData, RenderStep, RenderStepType},
    },
    traits::{NameBuilder as NameBuilderTrait, Naming as NamingTrait},
    Config, Name, TypeIdent,
};

fn main() -> Result<(), Error> {
    let cargo_dir =
        var("CARGO_MANIFEST_DIR").context("Missing `CARGO_MANIFEST_DIR` environment variable!")?;
    let cargo_dir = PathBuf::from(cargo_dir)
        .canonicalize()
        .context("Missing environment variable `CARGO_MANIFEST_DIR`")?;
    let schema_file = cargo_dir
        .join("schema/xbrl.efrag.org/taxonomy/vsme/2025-07-30/vsme-all.xsd")
        .canonicalize()
        .context("Missing or invalid schema file!")?;

    // This is almost the starting point defined in the main `[README.md]`.
    let fix_item_type = FixItemType::default();
    let config = Config::default()
        .with_schema(Schema::File(schema_file))
        .with_generate([(IdentType::Element, "xbrli:xbrl")])
        .with_interpreter_flags(InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT)
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(GeneratorFlags::all() - GeneratorFlags::ADVANCED_ENUMS)
        .with_naming(CustomNaming::default())
        .with_quick_xml()
        .with_render_step(fix_item_type.clone());

    // Generate the code based on the configuration above. We run the pipeline
    // manually instead of using `generate_modules`, because we need to inject
    // the `FixItemType` transformation in between the optimizer and the
    // generator.
    let schemas = exec_parser(config.parser)?;
    let (meta_types, ident_cache) =
        exec_interpreter_with_ident_cache(config.interpreter, &schemas)?;
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let meta_types = fix_item_type.prepare_types(meta_types, &schemas)?;
    let data_types = exec_generator_with_ident_cache(
        config.generator,
        &schemas,
        Some(&ident_cache),
        &meta_types,
    )?;
    let modules = exec_render(config.renderer, &data_types)?;

    // Write the generated code to the module directory specified by Cargo.
    let target_dir = cargo_dir.join("src/schema");
    let _ = remove_dir_all(&target_dir);
    create_dir_all(&target_dir).context("Unable to create `src/schema` directory!")?;
    modules
        .write_to_files(&target_dir)
        .context("Error while writing generated code")?;

    Ok(())
}

#[derive(Debug, Default)]
struct CustomNaming(Naming);

impl NamingTrait for CustomNaming {
    fn clone_boxed(&self) -> Box<dyn NamingTrait> {
        Box::new(Self(self.0.clone()))
    }

    fn builder(&self) -> Box<dyn NameBuilderTrait> {
        self.0.builder()
    }

    fn unify(&self, s: &str) -> String {
        self.0.unify(s)
    }

    fn make_type_name(&self, postfixes: &[String], ty: &MetaType, ident: &TypeIdent) -> Name {
        self.0.make_type_name(postfixes, ty, ident)
    }

    fn make_unknown_variant(&self, id: usize) -> xsd_parser::Ident2 {
        self.0.make_unknown_variant(id)
    }

    fn format_module_name(&self, s: &str) -> String {
        self.0.format_module_name(s)
    }

    fn format_type_name(&self, s: &str) -> String {
        self.0.format_type_name(s)
    }

    fn format_field_name(&self, s: &str) -> String {
        self.0.format_field_name(s)
    }

    fn format_variant_name(&self, s: &str) -> String {
        self.0.format_variant_name(s)
    }

    fn format_constant_name(&self, s: &str) -> String {
        self.0.format_constant_name(s)
    }

    fn format_attribute_field_name(&self, s: &str) -> String {
        let s = self.0.format_attribute_field_name(s);
        let s = s.trim_end_matches('_');

        format!("{s}_attr")
    }
}

/// Render step (and meta type transformation) that collapses the many concrete
/// element variants of the `xbrli:item` choice into a few `ItemWrapper` based
/// types.
///
/// XBRL defines hundreds of facts as members of the `xbrli:item` substitution
/// group. Most of them share the same (Rust) type and only differ by their XML
/// tag name. Generating a dedicated enum variant for each of them would be
/// wasteful, so we instead group the elements by their type and represent each
/// group with a single [`ItemWrapper`](crate::item::ItemWrapper) that keeps the
/// list of supported tags around at runtime.
#[derive(Default, Debug, Clone)]
struct FixItemType(Rc<RefCell<Vec<SharedWrapped>>>);

/// A [`WrappedType`] that is shared between the custom generator step (which
/// resolves and stores the path to the concrete type) and the render step
/// (which uses that information to render the actual code).
type SharedWrapped = Rc<RefCell<WrappedType>>;

/// Information about a single synthetic `XxxWrapped` type.
#[derive(Debug)]
struct WrappedType {
    /// Identifier of the synthetic `XxxWrapped` custom type.
    ident: TypeIdent,

    /// Identifier of the concrete type that is shared by all elements
    /// represented by this wrapper.
    type_: TypeIdent,

    /// XML tags of all elements that are represented by this wrapper.
    tags: Vec<TagInfo>,

    /// Path to the concrete type relative to the root module (e.g.
    /// `vsme :: AmountOfEmissionToAirDyn`). This is resolved and stored by the
    /// custom generator step (see [`WrappedType::resolve`]).
    target_type: Option<PathData>,
}

/// Information about a single XML tag represented by an [`ItemWrapper`].
#[derive(Debug)]
struct TagInfo {
    /// Identifier (namespace and local name) of the element.
    ident: ElementIdent,

    /// Form of the element, used to decide whether the tag needs a namespace
    /// prefix.
    form: FormChoiceType,
}

impl FixItemType {
    fn prepare_types(&self, mut types: MetaTypes, schemas: &Schemas) -> Result<MetaTypes, Error> {
        let item_ident = IdentQuadruple::from((IdentType::Element, "xbrli:item"));
        let item_ident = item_ident
            .resolve(schemas)
            .context("Unable to resolve `xbrli:item` element")?;

        let item_ty = types
            .items
            .get(&item_ident)
            .context("Unknown element: `xbrli:item`")?;
        let MetaTypeVariant::ComplexType(meta) = &item_ty.variant else {
            anyhow::bail!("`xbrli:item` is not a complex type")
        };
        let content_ident = meta
            .content
            .clone()
            .context("`xbrli:item` is missing a content type")?;

        let content_ty = types
            .items
            .get_mut(&content_ident)
            .context("Unknown content type for `xbrli:item`")?;
        let MetaTypeVariant::Choice(meta) = &mut content_ty.variant else {
            anyhow::bail!("Content type of `xbrli:item` is not a choice")
        };

        // Group all concrete element variants of the choice by their type and
        // remove them from the choice. We use a `BTreeMap` to get a stable
        // order of the generated types. For each removed element we remember the
        // information needed to reconstruct its XML tag name later on.
        let mut map = BTreeMap::<TypeIdent, Vec<(ElementIdent, FormChoiceType)>>::new();
        meta.elements.0.retain(|el| {
            let ElementMetaVariant::Type {
                type_,
                mode: ElementMode::Element,
            } = &el.variant
            else {
                return true;
            };

            map.entry(type_.clone())
                .or_default()
                .push((el.ident.clone(), el.form));

            false
        });

        // Add one group element per type that references the synthetic wrapper
        // type instead of the removed elements.
        let mut pending = Vec::new();
        for (concrete, tags) in map {
            let mut wrapped = concrete.clone();
            wrapped.name = Name::new_named(format!("{}Wrapped", wrapped.name));

            meta.elements.0.push(ElementMeta::new(
                concrete.to_property_ident(),
                wrapped.clone(),
                ElementMode::Group,
                FormChoiceType::Unqualified,
            ));

            pending.push((wrapped, concrete, tags));
        }

        // `meta` (and therefore the mutable borrow of `types`) is not used
        // beyond this point, so we can now resolve the tag names and register
        // the synthetic wrapper types as custom types.
        for (ident, type_, tags) in pending {
            let tags = tags
                .into_iter()
                .map(|(ident, form)| TagInfo::new(ident, form))
                .collect::<Vec<_>>();

            let wrapped = Rc::new(RefCell::new(WrappedType {
                ident: ident.clone(),
                type_,
                tags,
                target_type: None,
            }));
            self.0.borrow_mut().push(wrapped.clone());

            // The custom generator step resolves the path to the concrete type
            // during code generation and stores it in the (shared) wrapper.
            let custom = CustomMeta::new(ident.name.clone()).with_generator(
                move |ctx: &mut GeneratorContext<'_, '_>, _: &CustomMeta| {
                    wrapped.borrow_mut().resolve(ctx)
                },
            );
            types
                .items
                .insert(ident, MetaType::new(MetaTypeVariant::Custom(custom)));
        }

        Ok(types)
    }
}

impl RenderStep for FixItemType {
    fn render_step_type(&self) -> RenderStepType {
        RenderStepType::ExtraTypes
    }

    fn finish(&mut self, meta: &MetaData<'_>, module: &mut Module) {
        for wrapped in self.0.borrow().iter() {
            wrapped.borrow().render(meta, module);
        }
    }
}

impl WrappedType {
    /// Resolve the path to the concrete type (relative to the root module) and
    /// store it. This is called by the custom generator step during code
    /// generation, where the path information is available. Requesting the type
    /// reference also makes sure the concrete type is actually generated.
    fn resolve(&mut self, ctx: &mut GeneratorContext<'_, '_>) -> Result<(), GeneratorError> {
        let target_type = ctx.get_or_create_type_ref(&self.type_)?.path.clone();

        self.target_type = Some(target_type);

        Ok(())
    }

    /// Render the type definition and the `ItemTags` implementation for this
    /// wrapper into the given `module`.
    fn render(&self, meta: &MetaData<'_>, module: &mut Module) {
        let wrapped_ident = format_ident!("{}", self.ident.name.as_str());
        let tags_ident = format_ident!("{}Tags", self.ident.name.as_str());

        let target_type = self
            .target_type
            .as_ref()
            .expect("the concrete type path is resolved by the custom generator step")
            .resolve_relative_to(&ModulePath::root());

        let tags = self.tags.iter().map(|tag| tag.render(meta));
        let count = self.tags.len();

        module.append(quote! {
            pub type #wrapped_ident = crate::item::ItemWrapper<#target_type, #tags_ident>;

            #[derive(Debug)]
            pub struct #tags_ident;

            impl crate::item::ItemTags for #tags_ident {
                fn tags() -> &'static [crate::item::ItemTag] {
                    static TAGS: [crate::item::ItemTag; #count] = [ #( #tags ),* ];

                    &TAGS
                }
            }
        });
    }
}

impl TagInfo {
    /// Resolve the information for a single tag from the passed element data.
    fn new(ident: ElementIdent, form: FormChoiceType) -> Self {
        Self { ident, form }
    }

    /// Render this tag as a `crate::item::ItemTag` value.
    fn render(&self, meta: &MetaData<'_>) -> TokenStream {
        let Self { ident, form } = self;

        let types = meta.types.meta.types;
        let module = types
            .modules
            .get(&ident.ns)
            .expect("the module for the tag's namespace exists");

        // Reuse the namespace constant generated next to the schema types
        // (e.g. `NS_VSME`) instead of repeating the namespace URI.
        let namespace = module
            .make_ns_const()
            .expect("the tag has a namespace")
            .resolve_relative_to(&ModulePath::root());
        let tag = TagName::new(types, ident.ns, &ident.name, *form).get(true);
        let name = ident.name.as_str();

        quote! {
            crate::item::ItemTag {
                tag: #tag,
                name: #name,
                namespace: #namespace,
            }
        }
    }
}
