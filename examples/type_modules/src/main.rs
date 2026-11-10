use std::error::Error;

use xsd_parser_types::quick_xml::{DeserializeSync, SliceReader};

#[allow(dead_code, unused_imports, unused_variables, unused_mut)]
mod model {
    include!(concat!(env!("OUT_DIR"), "/model/mod.rs"));
}

fn main() -> Result<(), Box<dyn Error>> {
    let xml = r#"<shop:order xmlns:shop="urn:example:shop" id="example"><shop:line sku="A1"/></shop:order>"#;
    let mut reader = SliceReader::new(xml);
    let order = model::shop::Order::deserialize(&mut reader)?;
    println!("{order:#?}");
    Ok(())
}
