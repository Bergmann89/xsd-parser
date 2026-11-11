use std::error::Error;
use xsd_parser_types::quick_xml::{DeserializeSync, SerializeSync, SliceReader, Writer};

pub mod native {
    pub type Text = String;
}

#[allow(
    dead_code,
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::possible_missing_else
)]
mod bindings {
    // Root-relative includes resolve from the root of these generated bindings.
    pub use crate::native;
    include!(concat!(env!("OUT_DIR"), "/bindings/mod.rs"));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = SliceReader::new(include_str!("../example.xml"));
    let record = bindings::ex::Record::deserialize(&mut reader)?;
    let _: &native::Text = &record.label;

    let mut output = Vec::new();
    let mut writer = Writer::new_with_indent(&mut output, b' ', 4);
    record.serialize("ex:Record", &mut writer)?;
    println!("{}", String::from_utf8(output)?);
    Ok(())
}
