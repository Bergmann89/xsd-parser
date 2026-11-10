use std::{env, path::PathBuf};

use xsd_parser::{
    config::{GeneratorFlags, Schema},
    generate_modules,
    models::code::CodeFormat,
    Config, Error, IdentType,
};

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=schema.xsd");
    let config = Config::default()
        .with_schema(Schema::file("schema.xsd"))
        .with_generate([(IdentType::Element, "shop:order")])
        .with_generator_flags(
            GeneratorFlags::USE_TYPE_MODULES
                | GeneratorFlags::USE_NAMESPACE_MODULES
                | GeneratorFlags::FLATTEN_CONTENT,
        )
        .with_quick_xml();
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    generate_modules(config)?
        .write_to_files_with_format(output.join("model"), CodeFormat::Pretty)?;
    Ok(())
}
