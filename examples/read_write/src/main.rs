use std::io::{BufReader, BufWriter};
use xsd_parser::quick_xml::{DeserializeSync, IoReader, SerializeSync, Writer, XmlReader};

mod my_schema;

const INPUT_FILE_NAME: &str = "input.xml";
const OUTPUT_FILE_NAME: &str = "output.xml";

fn main() {
    // Parse input.xml (according to my-schema.xsd definitions) and deserialize it to object `doc`.
    let input_file = std::fs::File::open(INPUT_FILE_NAME).unwrap();
    let reader= BufReader::new(input_file);
    let mut reader = IoReader::new(reader).with_error_info();
    let mut doc = my_schema::DocumentType::deserialize(& mut reader).unwrap();
    print!("Created structure = {:#?}\n\n", doc);
    
    // Update our document counter.
    doc.count = match doc.count {
        Some(x) => Some(x+1),
        None => Some(1),
    };
    
    // Add some more stuff.
    doc.content.push(my_schema::DocumentTypeContent {
        info: "blah blah".to_string(),
    });
    
    // Write our modified version to output.xml.    
    let mut writer = std::fs::File::create(OUTPUT_FILE_NAME).unwrap();
    let writer = BufWriter::new(&mut writer);
    let mut writer   = Writer::new_with_indent(writer, '\t' as u8, 1);
    doc.serialize("document", &mut writer).unwrap();
    print!("Wrote new values to '{}'.\n", OUTPUT_FILE_NAME);
}
