//! Example how to generate code for the VSME schema and use it to deserialize an XML file.

#[allow(clippy::all, clippy::pedantic, clippy::nursery, dead_code, missing_docs, unreachable_pub, unused)]
#[rustfmt::skip]
mod schema;
mod item;

use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Error};
use xsd_parser_types::quick_xml::{DeserializeSync, IoReader, SerializeSync, Writer, XmlReader};

use self::schema::xbrli::Xbrl;

fn main() -> Result<(), Error> {
    // Deserialize the example XBRL instance document. The facts (members of the
    // `xbrli:item` substitution group) are deserialized into `ItemWrapper`
    // values that remember their original XML tag name (see `item.rs`).
    let input_file =
        File::open("xml/example.xml").context("Unable to open example XML for reading")?;
    let reader = BufReader::new(input_file);
    let mut reader = IoReader::new(reader).with_error_info();
    let obj = Xbrl::deserialize(&mut reader).context("Error while deserializing XML content")?;

    println!("{obj:#?}");

    // Serialize the document back to XML. This round-trip shows that the
    // `ItemWrapper` based facts are written back using their original tag name.
    let mut buffer = Vec::new();
    let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);
    obj.serialize("xbrli:xbrl", &mut writer)
        .context("Error while serializing XML content")?;

    println!("\n{}", String::from_utf8(buffer)?);

    Ok(())
}
