# Native type aliases

Run from the workspace root:

```sh
cargo run -p native_aliases
```

The build script maps `Label` and `Description` to the application's `native::Text`, using `CustomMeta::with_alias(true)`. The generated bindings expose `LabelType` and `DescriptionType` aliases, preserve schema documentation, and use native values directly. The included Rust type supplies XML serialization and deserialization; an alias adds no wrapper or trait implementations.

`crate::native::Text` refers to the application's crate root. `self::native::Text` refers to the generated bindings root, where `native` is re-exported. The example reads a record from XML and writes it back using the generated aliases.
