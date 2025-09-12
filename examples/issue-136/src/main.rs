use std::fs::File;
use std::io::BufReader;

use xsd_parser::quick_xml::{DeserializeSync, IoReader, XmlReader};

use crate::schema::Container;

#[allow(
    dead_code,
    unused_mut,
    unused_variables,
    clippy::never_loop,
    clippy::single_match,
    clippy::redundant_field_names
)]
#[rustfmt::skip]
mod schema;

fn main() {
    let input_file = File::open("example.xml").unwrap();
    let reader = BufReader::new(input_file);
    let mut reader = IoReader::new(reader).with_error_info();
    let container = Container::deserialize(&mut reader).unwrap();
    print!("Container={container:#?}");
}
