//! Example demonstrating parsing of IFC (Industry Foundation Classes) XML files.
//!
//! This example shows how to use the xsd-parser-generated code to deserialize
//! large XML schemas like IFC. The parsing is performed in a separate thread
//! with a configurable stack size to handle the deep recursion required.
//!
//! **Note:** Compilation of this example can take a considerable amount of time
//! due to the large size of the generated code from the IFC schema.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example ifc <stack_size_in_bytes>
//! ```
//!
//! For example, to run with a 16MB stack:
//! ```bash
//! cargo run -p ifc 16777216
//! ```

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

use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::thread::Builder;

use xsd_parser_types::quick_xml::{DeserializeSync, IoReader, XmlReader};

fn main() {
    let stack_size = args()
        .skip(1)
        .next()
        .expect("Missing stack_size argument")
        .parse()
        .expect("Invalid stack_size: expected integer");
    Builder::new()
        .stack_size(stack_size)
        .spawn(parse)
        .expect("Failed to spawn thread")
        .join()
        .expect("Thread panicked");
}

fn parse() {
    let file = File::open("example.xml").expect("Failed to open example.xml");
    let reader = BufReader::new(file);
    let mut reader = IoReader::new(reader).with_error_info();
    let doc = schema::ifc::IfcXmlXElement::deserialize(&mut reader)
        .expect("Failed to deserialize IFC XML");
    println!("{doc:?}");
}
