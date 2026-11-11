use std::{env, error::Error, path::PathBuf};
use xsd_parser::{
    config::{GeneratorFlags, RendererFlags, Schema},
    generate_modules,
    models::meta::CustomMeta,
    Config, IdentType,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=schema.xsd");
    let config = Config::default()
        .with_schema(Schema::file("schema.xsd"))
        .with_type(
            (IdentType::Type, "ex:Label"),
            CustomMeta::new("Text")
                .include_from("crate::native::Text")
                .with_alias(true),
        )
        .with_type(
            (IdentType::Type, "ex:Description"),
            CustomMeta::new("Text")
                .include_from("self::native::Text")
                .with_alias(true),
        )
        .with_generate([(IdentType::Element, "ex:Record")])
        .with_generator_flags(
            GeneratorFlags::USE_NAMESPACE_MODULES | GeneratorFlags::FLATTEN_CONTENT,
        )
        .with_renderer_flags(RendererFlags::RENDER_DOCS)
        .with_quick_xml();
    let output = PathBuf::from(env::var_os("OUT_DIR").ok_or("Cargo did not set OUT_DIR")?);
    generate_modules(config)?.write_to_files(output.join("bindings"))?;
    Ok(())
}
