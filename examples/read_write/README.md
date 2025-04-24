A more complex example.

It integrates XSD-PARSER in the build phase of the application,
to generate Rust code at build time.

For this, [build-rs](build.rs) is involded to parse [my_schema.xsd](my-schema.xsd) and generate [my_schema.rs](src/my_schema.rs).

This Rust source file contains all the types, structures and constants defined from
the XSD file. It contains also:
- a DESERIALIZER, capable of mapping an XML document to generated structures.  
- a SERIALIZER to write the above structures to an XML file.

