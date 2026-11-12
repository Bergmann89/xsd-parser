# Serde with large fixed-size arrays

Run `cargo run -p serde_large_arrays` from the workspace root.

The build script selects `RenderStep::TypesSerdeQuickXml { large_arrays: true }`.
The generated model keeps `[SampleType; 360]` and receives a
`serde_with` adapter automatically. The example round-trips the model as JSON.

The option belongs to the Serde render steps and is disabled by default:

```rust
use xsd_parser::{Config, config::RenderStep};

let config = Config::default()
    .with_serde_quick_xml()
    .with_render_step(RenderStep::TypesSerdeQuickXml { large_arrays: true });
```

For `serde-xml-rs`, select `RenderStep::TypesSerdeXmlRs { version, large_arrays: true }`.
Plain `Types` and direct quick-xml serialization do not emit Serde adapters.

Generated-code consumers need a `serde_with` dependency with its `std` or `alloc`
feature. The option also supports newtype and enum payloads, nested array
aliases, and arrays inside `Option`, `Vec`, and `Box`. Arrays up to length 32
use Serde's native implementation unless they contain a larger array alias.
Optional array fields keep their missing-field behavior. Custom serialization
functions and defaults are preserved; automatic adapters fill only the missing
serialization or deserialization direction.
Array aliases are adapted at their uses in generated fields. Choose
`TypedefMode::NewType` when the array itself should be a standalone Serde value.
The Serde XML renderers can use the same option; their existing field types
remain unchanged, including the vectors emitted by serde-xml-rs v0.8.

serde-xml-rs v0.7 reads fixed arrays from repeated XML elements, but writes them
as space-separated tuples in one element. This existing backend behavior also
affects native small arrays and prevents XML round-trips for those fields.
Use quick-xml for XML round-trips with fixed arrays; serde-xml-rs v0.8 emits
vectors for repeated elements instead.

Serde quick-xml and serde-xml-rs v0.7 also retain their existing limitation for
merged enumeration fallbacks with payloads: they emit `#[serde(other)]`, which
Serde only accepts on unit variants. Leave `MERGE_ENUM_UNIONS` disabled for
these unions; their ordinary enum payloads still receive array adapters.

The dependency must be available as `serde_with`, since generated adapter paths
use that name. Adapters do not look through generic XML wrappers such as
`Nillable` or `Mixed`; these runtime types currently have no Serde implementation.
Conditional Serde attributes inside `cfg_attr` are not evaluated when choosing
automatic adapters or defaults. Configure custom attributes
at generation time instead, using the desired configuration to select an
unconditional codec or default.

Only field-level defaults are inspected; when using struct-level
`#[serde(default)]` with a custom `Default` implementation, configure field
defaults explicitly to prevent missing fields from receiving different values.
