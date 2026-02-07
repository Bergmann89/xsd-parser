//! Example how to generate code for the VSME schema and use it to deserialize an XML file.

#[allow(clippy::all, dead_code, missing_docs, unreachable_pub, unused)]
#[rustfmt::skip]
mod schema;

use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Error};
use xsd_parser_types::quick_xml::{DeserializeSync, IoReader, XmlReader};

use schema::xbrli::Xbrl;

fn main() -> Result<(), Error> {
    let input_file =
        File::open("xml/example.xml").context("Unable to open example XML for reading")?;
    let reader = BufReader::new(input_file);
    let mut reader = IoReader::new(reader).with_error_info();
    let obj = Xbrl::deserialize(&mut reader).context("Error while deserializing XML content")?;

    println!("{obj:#?}");

    Ok(())
}
