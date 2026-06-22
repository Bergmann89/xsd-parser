# VSME example

This example generates and uses the code for the XBRL based [VSME](https://xbrl.efrag.org/) taxonomy (the *Voluntary standard for non-listed Small- and Medium-sized Enterprises*).

XBRL instance documents store their facts as members of the `xbrli:item` substitution group. The VSME taxonomy defines **thousands** of such facts, but most of them share the same underlying type and only differ by their XML tag name (e.g. `vsme:Assets`, `vsme:AverageNumberOfAnnualTrainingHoursPerMaleEmployee`, ...). Generating a dedicated Rust type (and `enum` variant) for every single element would produce a huge, unwieldy amount of code.

## What this example shows

Instead of generating one type per element, the [build script](build.rs) installs a custom [`RenderStep`] (`FixItemType`) that:

1. Looks up the content of the `xbrli:item` element (a big `xs:choice`).
2. Groups all element variants of the choice by their type and removes them from the choice.
3. Adds a single group element per type that references a synthetic `XxxWrapped` custom type.
4. Emits a type definition and an [`ItemTags`](src/item.rs) implementation for each of those wrapper types:

   ```rust,ignore
   pub type AmountOfEmissionToAirWrapped =
       crate::item::ItemWrapper<vsme::AmountOfEmissionToAirDyn, AmountOfEmissionToAirWrappedTags>;

   pub struct AmountOfEmissionToAirWrappedTags;

   impl crate::item::ItemTags for AmountOfEmissionToAirWrappedTags {
       fn tags() -> &'static [crate::item::ItemTag] {
           static TAGS: [crate::item::ItemTag; 78] = [
               crate::item::ItemTag {
                   tag: "vsme:Assets",
                   name: "Assets",
                   namespace: NS_VSME,
               },
               /* ... */
           ];

           &TAGS
       }
   }
   ```

The [`ItemWrapper`](src/item.rs) type defined in this example provides the runtime support for this. It wraps the shared inner type and implements a `Serializer` and `Deserializer` that:

- on **deserialization**, only accept an element whose namespace-resolved tag is part of the associated `ItemTags` (so a document may use any prefix for the namespace), and remember which tag was used; and
- on **serialization**, write the value back using that remembered tag.

This way a few hundred elements collapse into just a handful of generated types while the round-trip still preserves the exact XML tag of every fact.

## Running

```sh
cargo run -p vsme
```

This deserializes [`xml/example.xml`](xml/example.xml), prints the parsed object and serializes it back to XML.
