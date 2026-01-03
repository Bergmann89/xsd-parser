mod bpmn;

use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Error};
use xsd_parser_types::quick_xml::{DeserializeSync, IoReader, XmlReader};

use bpmn::bpmn_20::Definitions;

fn main() -> Result<(), Error> {
    let input_file = File::open("xmls/triso-order-process-for-pizza-v4.bpmn.xml")
        .context("Unable to open example XML for reading")?;
    let reader = BufReader::new(input_file);
    let mut reader = IoReader::new(reader).with_error_info();
    let definitions =
        Definitions::deserialize(&mut reader).context("Error while deserializing XML content")?;

    println!("{definitions:?}");

    Ok(())
}
