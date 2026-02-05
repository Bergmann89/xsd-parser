#![recursion_limit = "256"]

#[allow(
    dead_code,
    unused_mut,
    unused_variables,
    clippy::len_zero,
    clippy::single_match,
    clippy::enum_variant_names,
    clippy::large_enum_variant,
    clippy::never_loop,
    clippy::redundant_field_names,
    clippy::suspicious_else_formatting,
    clippy::ptr_arg
)]
mod schema;

use std::{env::args, fs::File, io::BufReader, thread::Builder};

use xsd_parser_types::quick_xml::{DeserializeSync, IoReader, XmlReader};

fn main() {
    let stack_size = args().skip(1).next().unwrap().parse().unwrap();
    Builder::new()
        .stack_size(stack_size)
        .spawn(parse)
        .unwrap()
        .join()
        .unwrap();
}

fn parse() {
    let file = File::open("example.xml").unwrap();
    let reader = BufReader::new(file);
    let mut reader = IoReader::new(reader).with_error_info();
    let _doc = schema::ifc::IfcXmlXElement::deserialize(&mut reader).unwrap();
}
