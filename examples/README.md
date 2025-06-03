# simple

Probably the most simple example, at least, a starting point to understand `xsd-parser` or start more complex applications.


# update_schema

It demonstrates a more advanced use of the generator to create types from the schema with deserialization support enabled.


# custom_names

Short example that shows how to assign custom defined names to the generated types.


# custom_renderer

Example that explains how to implement user defined renderers. This is useful if you want to generate additional structures from the already parsed and interpreted schema.


# read_write

A more complex example.

It integrates `xsd-parser` in the build phase (`build.rs`) of the application,
to generate Rust code and use it at run time.
