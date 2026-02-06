use std::fs::{create_dir_all, remove_dir_all};

use anyhow::{Context, Error};

use xsd_parser::{
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags},
    config::{IdentQuadruple, ParserFlags, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::meta::{AttributeMetaVariant, MetaTypeVariant},
    Config, IdentType, MetaTypes, Schemas,
};

fn main() -> Result<(), Error> {
    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let schema_path = std::path::Path::new(&cargo_dir).join("schema.xsd");
    let schema_path = schema_path.canonicalize().with_context(|| {
        format!(
            "Error while resolving schema file `{}`",
            schema_path.display()
        )
    })?;

    println!("cargo::rerun-if-changed={}", schema_path.display());

    let mut config = Config::default()
        .with_parser_flags(ParserFlags::all())
        .with_interpreter_flags(InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT)
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(GeneratorFlags::all())
        .with_quick_xml_serialize()
        .with_quick_xml_deserialize_config(true)
        .with_schema(Schema::File(schema_path))
        .with_generate([(IdentType::Element, "ifc:ifcXML")]);

    // Use custom type postfixes to avoid conflicts with standard Rust type names
    // and to make the generated types more descriptive for the IFC schema
    config.generator.type_postfix.type_ = "XType".into();
    config.generator.type_postfix.element = "XElement".into();
    config.generator.type_postfix.element_type = "XElementType".into();

    let debug_dir = std::path::Path::new(&cargo_dir).join("../../target/examples/ifc");
    create_dir_all(&debug_dir).context("Unable to create debug output dir!")?;
    config.parser.debug_output = Some(debug_dir.join("parser.log"));
    config.interpreter.debug_output = Some(debug_dir.join("interpreter.log"));
    config.optimizer.debug_output = Some(debug_dir.join("optimizer.log"));
    generate(config)?;

    Ok(())
}

fn generate(config: Config) -> Result<(), Error> {
    let schemas = exec_parser(config.parser).context("Error while parsing the schema")?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)
        .context("Error while interpreting the schema")?;
    let meta_types = resolve_hex_binary_conflict(&schemas, meta_types);
    let meta_types = resolve_compound_plane_angle_measure_conflict(&schemas, meta_types);
    let meta_types = resolve_naming_conflicts(&schemas, meta_types);
    let meta_types = exec_optimizer(config.optimizer, meta_types)
        .context("Error while optimizing the schema")?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)
        .context("Error while generating the code")?;
    let modules =
        exec_render(config.renderer, &data_types).context("Error while rendering the code")?;

    let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let output_path = std::path::Path::new(&cargo_dir).join("src/schema");
    let _ = remove_dir_all(&output_path);
    create_dir_all(&output_path).context("Unable to create output directory")?;
    modules
        .write_to_files(output_path)
        .context("Unable to write generated code")?;

    Ok(())
}

// The IFC schema incorrectly uses the complex type `ifc:IfcBinary` as the type for the
// `Pixel` attribute in `ifc:IfcPixelTexture`. This function corrects this by replacing
// the reference to IfcBinary with xs:hexBinary, which is the correct simple type for
// binary data in XML Schema. Lists can only have simple types, not complex types.
fn resolve_hex_binary_conflict(schemas: &Schemas, mut types: MetaTypes) -> MetaTypes {
    // Resolve `IfcPixelTexture` Type
    let ident = IdentQuadruple::from((IdentType::Type, "ifc:IfcPixelTexture"))
        .resolve(schemas)
        .unwrap();
    let ty = types.items.get(&ident).unwrap();
    let MetaTypeVariant::ComplexType(ci) = &ty.variant else {
        panic!("Unexpected type variant!");
    };

    // Resolve `Pixel` Attribute
    let attribute = ci
        .attributes
        .iter()
        .find(|a| a.ident.name.as_str() == "Pixel")
        .unwrap();
    let AttributeMetaVariant::Type(ident) = &attribute.variant else {
        panic!("Unexpected attribute variant!");
    };
    let ident = ident.clone();

    // Resolve Type of `Pixel` Attribute
    let ty = types.items.get_mut(&ident).unwrap();
    let MetaTypeVariant::Reference(ri) = &mut ty.variant else {
        panic!("Unexpected type variant!");
    };
    ri.type_ = IdentQuadruple::from((IdentType::Type, "xs:hexBinary"))
        .resolve(schemas)
        .unwrap();

    types
}

// `IfcCompoundPlaneAngleMeasure` is a complex type and can't be used as type for attributes.
// We implement a fallback to it's simple base type `List-IfcCompoundPlaneAngleMeasure`.
fn resolve_compound_plane_angle_measure_conflict(
    schemas: &Schemas,
    mut types: MetaTypes,
) -> MetaTypes {
    let ident = IdentQuadruple::from((IdentType::Type, "ifc:IfcCompoundPlaneAngleMeasure"))
        .resolve(schemas)
        .unwrap();
    let list_ident =
        IdentQuadruple::from((IdentType::Type, "ifc:List-IfcCompoundPlaneAngleMeasure"))
            .resolve(schemas)
            .unwrap();

    for ty in types.items.values_mut() {
        let MetaTypeVariant::ComplexType(ci) = &mut ty.variant else {
            continue;
        };

        for attrib in &mut *ci.attributes {
            if matches!(&attrib.variant, AttributeMetaVariant::Type(x) if *x == ident) {
                attrib.variant = AttributeMetaVariant::Type(list_ident.clone());
            }
        }
    }

    types
}

fn resolve_naming_conflicts(schemas: &Schemas, mut types: MetaTypes) -> MetaTypes {
    rename_attribute(
        schemas,
        &mut types,
        (IdentType::Type, "ifc:IfcTextLiteral"),
        "Path",
        "text_path",
    );
    rename_attribute(
        schemas,
        &mut types,
        (IdentType::Type, "ifc:IfcTextLiteralWithExtent"),
        "Path",
        "text_path",
    );

    types
}

fn rename_attribute<T, S>(schemas: &Schemas, types: &mut MetaTypes, ty: T, old: &str, new: S)
where
    IdentQuadruple: From<T>,
    S: Into<String>,
{
    let ident = IdentQuadruple::from(ty).resolve(schemas).unwrap();
    let ty = types.items.get_mut(&ident).unwrap();
    let MetaTypeVariant::ComplexType(ci) = &mut ty.variant else {
        panic!("Unexpected type variant");
    };

    for attrib in &mut *ci.attributes {
        if attrib.ident.name.as_str() == old {
            attrib.display_name = Some(new.into());
            break;
        }
    }
}
