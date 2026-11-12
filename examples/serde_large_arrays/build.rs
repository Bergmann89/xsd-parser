use std::{env, error::Error, path::PathBuf};

use xsd_parser::{
    config::{GeneratorFlags, Schema},
    generate_modules, Config, IdentType,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=schema.xsd");
    let config = Config::default()
        .with_schema(Schema::file("schema.xsd"))
        .with_generate([(IdentType::Element, "ex:Measurements")])
        .with_generator_flags(GeneratorFlags::FLATTEN_STRUCT_CONTENT)
        .with_derive(["Debug", "PartialEq"])
        .with_serde_quick_xml_config(true, true, true);
    let output = PathBuf::from(env::var_os("OUT_DIR").ok_or("Cargo did not set OUT_DIR")?);
    generate_modules(config)?.write_to_files(output.join("bindings"))?;
    Ok(())
}
