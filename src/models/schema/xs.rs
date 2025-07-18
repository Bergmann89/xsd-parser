#![allow(
    unused_mut,
    missing_docs,
    unused_variables,
    clippy::len_zero,
    clippy::single_match,
    clippy::needless_pass_by_value,
    clippy::unused_self,
    clippy::unnecessary_wraps,
    clippy::redundant_else,
    clippy::redundant_field_names,
    clippy::too_many_lines,
    clippy::large_enum_variant,
    clippy::semicolon_if_nothing_returned
)]

use std::str::from_utf8;

use quick_xml::{events::Event, Writer};
use unindent::unindent;

use crate::quick_xml::WithSerializer;

pub type Use = AttributeUseType;

include!("./xs_generated.rs");

impl Annotation {
    /// Extract the `xs:documentation` nodes from this `xs:annotation` node.
    ///
    /// This will extract each `xs:documentation` node from the `xs:annotation`,
    /// serialize the content of the `xs:documentation` node into a string and
    /// return it as vector.
    ///
    /// # Errors
    ///
    /// Will raise an error if the serialization of the `xs:documentation` node
    /// has failed.
    pub fn extract_documentation(&self) -> Result<Vec<String>, Error> {
        let mut docs = Vec::new();

        self.extract_documentation_into(&mut docs)?;

        Ok(docs)
    }

    /// Extract the `xs:documentation` nodes from this `xs:annotation` node and
    /// store it in the passed `docs` vector.
    ///
    /// # Errors
    ///
    /// Will raise an error if the serialization of the `xs:documentation` node
    /// has failed.
    pub fn extract_documentation_into(&self, docs: &mut Vec<String>) -> Result<(), Error> {
        for content in &self.content {
            let AnnotationContent::Documentation(doc) = content else {
                continue;
            };

            let mut level = 0usize;
            let mut buffer = Vec::new();
            let mut writer = Writer::new(&mut buffer);

            for event in doc.serializer(None, true).into_iter().flatten() {
                let event = event?;
                let write = match &event {
                    Event::Start(_) | Event::Empty(_) => {
                        let write = level > 0;

                        level += 1;

                        write
                    }
                    Event::End(_) => {
                        level = level.saturating_sub(1);

                        level > 0
                    }
                    _ => level > 0,
                };

                if write {
                    writer.write_event(event)?;
                }
            }

            if !buffer.is_empty() {
                let docu = from_utf8(&buffer)?;
                let docu = unindent(docu).trim_end().to_owned();

                docs.push(docu);
            }
        }

        Ok(())
    }
}
