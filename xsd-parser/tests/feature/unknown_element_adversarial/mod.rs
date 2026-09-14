use xsd_parser::{config::GeneratorFlags, Config, IdentType};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT)
        .with_generate([
            (IdentType::Element, "tns:Root"),
            (IdentType::Element, "tns:Max"),
            (IdentType::Element, "tns:Outer"),
            (IdentType::Element, "tns:Node"),
            (IdentType::Element, "tns:GRoot"),
        ])
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/unknown_element_adversarial/schema.xsd",
        "tests/feature/unknown_element_adversarial/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/unknown_element_adversarial/schema.xsd",
        "tests/feature/unknown_element_adversarial/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

/* read */

#[cfg(not(feature = "update-expectations"))]
mod read {
    #![allow(unused_imports)]

    use std::fmt::Debug;

    use xsd_parser_types::quick_xml::{DeserializeSync, Error, ErrorKind, SliceReader};

    use super::quick_xml::{GRoot, Max, Node, Outer, Root, RootType};

    fn read<T>(xml: &'static str) -> Result<T, Error>
    where
        T: DeserializeSync<'static, SliceReader<'static>, Error = Error>,
        T::Error: Debug,
    {
        T::deserialize(&mut SliceReader::new(xml))
    }

    fn read_ok<T>(xml: &'static str) -> T
    where
        T: DeserializeSync<'static, SliceReader<'static>, Error = Error>,
        T::Error: Debug,
    {
        read(xml).unwrap_or_else(|e| panic!("expected successful deserialization: {e:?}"))
    }

    fn read_err<T, F>(xml: &'static str, check: F)
    where
        T: DeserializeSync<'static, SliceReader<'static>, Error = Error> + Debug,
        T::Error: Debug,
        F: Fn(&Error) -> bool,
    {
        match read::<T>(xml) {
            Ok(x) => panic!("expected error, got: {x:?}"),
            Err(e) => assert!(check(&e), "error did not pass the expectation check: {e:?}"),
        }
    }

    fn is_unexpected_event(e: &Error) -> bool {
        matches!(e.kind, ErrorKind::UnexpectedEvent(_))
    }

    fn is_missing_element(e: &Error) -> bool {
        matches!(e.kind, ErrorKind::MissingElement(_))
    }

    fn is_reader_error(e: &Error) -> bool {
        matches!(
            e.kind,
            ErrorKind::UnexpectedEvent(_) | ErrorKind::XmlError(_) | ErrorKind::IoError(_)
        )
    }

    /* Vector 1: cardinality masking via skip */

    // The unknown element must not "consume" the required `A` slot: A is
    // missing and the document has to be rejected with `MissingElement`.
    #[test]
    fn v1_unknown_does_not_satisfy_min_occurs() {
        read_err::<Root, _>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown>junk</tns:Unknown> \
             </tns:Root>",
            is_missing_element,
        );
    }

    // A known element that appears where the required `A` element was
    // expected is misplaced and must not be silently skipped.
    #[test]
    fn v1_misplaced_known_element_after_unknown() {
        read_err::<Root, _>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
            is_unexpected_event,
        );
    }

    // A known element that exceeds `maxOccurs` must still be rejected, even
    // if unknown elements where skipped in between.
    #[test]
    fn v1_max_occurs_exceeded_after_unknown() {
        read_err::<Max, _>(
            "<tns:Max xmlns:tns=\"http://example.com\"> \
                 <tns:A>1</tns:A> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:A>2</tns:A> \
                 <tns:A>3</tns:A> \
             </tns:Max>",
            is_unexpected_event,
        );
    }

    #[test]
    fn v1_max_occurs_boundary_ok_with_unknown() {
        let obj = read_ok::<Max>(
            "<tns:Max xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:A>1</tns:A> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:A>2</tns:A> \
             </tns:Max>",
        );

        assert_eq!(obj.a, ["1", "2"]);
    }

    // A duplicated known element after an unknown element must still be
    // rejected.
    #[test]
    fn v1_duplicate_known_after_unknown() {
        read_err::<Root, _>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:A>first</tns:A> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:A>second</tns:A> \
             </tns:Root>",
            is_unexpected_event,
        );
    }

    /* Vector 2: namespaces */

    // A foreign namespace tag with the local name of a known child is not
    // known to the content model (`resolve_local_name` returns `None`) and
    // must be skipped; the real element must still be parsed.
    #[test]
    fn v2_foreign_namespace_same_local_name_skipped() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\" xmlns:evil=\"urn:evil\"> \
                 <evil:A>junk</evil:A> \
                 <tns:A>value</tns:A> \
                 <evil:Pos>junk</evil:Pos> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    // Default namespace edge: the document uses a default namespace. Inside
    // the skipped subtree the default namespace is rebound to a foreign
    // namespace; after the skip the default namespace must be restored.
    #[test]
    fn v2_default_namespace_rebind_inside_skipped_subtree() {
        let obj = read_ok::<Root>(
            "<Root xmlns=\"http://example.com\"> \
                 <Unknown xmlns=\"urn:evil\"> \
                     <A>junk</A> \
                 </Unknown> \
                 <A>value</A> \
                 <Pos>after</Pos> \
             </Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    // Prefix rebinding inside a skipped subtree: `tns` is bound to a foreign
    // namespace inside the unknown element. The namespace scope stack must
    // stay balanced, so that `tns` resolves to the original namespace again
    // after the unknown element was skipped.
    #[test]
    fn v2_prefix_rebind_inside_skipped_subtree() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown xmlns:tns=\"urn:evil\"> \
                     <tns:A>junk</tns:A> \
                 </tns:Unknown> \
                 <tns:A>value</tns:A> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    // Prefix rebound twice (nested) inside a skipped subtree. The local name
    // of the inner elements equals known children, but resolves to a foreign
    // namespace and must not confuse the scope stack.
    #[test]
    fn v2_prefix_rebind_nested_inside_skipped_subtree() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown xmlns:tns=\"urn:evil1\"> \
                     <tns:Deep xmlns:tns=\"urn:evil2\"> \
                         <tns:A>junk</tns:A> \
                         <tns:Pos>junk</tns:Pos> \
                     </tns:Deep> \
                 </tns:Unknown> \
                 <tns:A>value</tns:A> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    // The default namespace is undeclared inside the skipped subtree. After
    // the skip the resolution of unprefixed known elements must work again.
    #[test]
    fn v2_default_namespace_undeclared_inside_skipped_subtree() {
        let obj = read_ok::<Root>(
            "<Root xmlns=\"http://example.com\"> \
                 <Unknown xmlns=\"\"> \
                     <A>junk</A> \
                 </Unknown> \
                 <A>value</A> \
                 <Pos>after</Pos> \
             </Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    /* Vector 3: state coherence */

    // Three or more consecutive unknown elements.
    #[test]
    fn v3_consecutive_unknowns() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:U1>junk</tns:U1> \
                 <tns:U2>junk</tns:U2> \
                 <tns:U3>junk</tns:U3> \
                 <tns:A>value</tns:A> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    // Unknown element immediately before the end of the root element.
    #[test]
    fn v3_unknown_directly_before_root_end() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:A>value</tns:A> \
                 <tns:Pos>after</tns:Pos> \
                 <tns:Unknown>junk</tns:Unknown> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    // Unknown element immediately before the end of the root element while
    // the optional element is missing.
    #[test]
    fn v3_unknown_directly_before_root_end_optional_missing() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:A>value</tns:A> \
                 <tns:Unknown>junk</tns:Unknown> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos, None);
    }

    // Unknown element while the deserializer of the complex `Child` element
    // is still in-flight (between `<Child>` and `</Child>`).
    #[test]
    fn v3_unknown_while_child_in_flight() {
        let obj = read_ok::<Outer>(
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>inner</tns:Inner> \
                     <tns:Unknown>junk</tns:Unknown> \
                 </tns:Child> \
                 <tns:B>after</tns:B> \
             </tns:Outer>",
        );

        assert_eq!(obj.child.inner, "inner");
        assert_eq!(obj.child.extra.as_deref(), None);
        assert_eq!(obj.b.as_deref(), Some("after"));
    }

    // Unknown element after the optional content of the child was consumed,
    // but before the end tag of the child element was reached.
    #[test]
    fn v3_unknown_after_child_content_before_child_end() {
        let obj = read_ok::<Outer>(
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>inner</tns:Inner> \
                     <tns:Extra>extra</tns:Extra> \
                     <tns:Unknown>junk</tns:Unknown> \
                 </tns:Child> \
                 <tns:B>after</tns:B> \
             </tns:Outer>",
        );

        assert_eq!(obj.child.inner, "inner");
        assert_eq!(obj.child.extra.as_deref(), Some("extra"));
        assert_eq!(obj.b.as_deref(), Some("after"));
    }

    // Unknown element between the child and the following optional element.
    #[test]
    fn v3_unknown_between_child_and_sibling() {
        let obj = read_ok::<Outer>(
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>inner</tns:Inner> \
                 </tns:Child> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:B>after</tns:B> \
             </tns:Outer>",
        );

        assert_eq!(obj.child.inner, "inner");
        assert_eq!(obj.b.as_deref(), Some("after"));
    }

    // A valid element that follows an unknown element must be parsed with a
    // fresh state, not into a stale state of the skipped subtree.
    #[test]
    fn v3_valid_element_after_unknown_uses_fresh_state() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown> \
                     <tns:A>junk-a</tns:A> \
                     <tns:Pos>junk-pos</tns:Pos> \
                 </tns:Unknown> \
                 <tns:A>value</tns:A> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    /* Vector 4: recursion */

    // Recursive type with unknown elements inside the recursion: must not
    // loop or overflow, the recursion must still deserialize correctly.
    #[test]
    fn v4_unknown_inside_recursion() {
        let obj = read_ok::<Node>(
            "<tns:Node xmlns:tns=\"http://example.com\"> \
                 <tns:Name>n</tns:Name> \
                 <tns:Unknown> \
                     <tns:Node><tns:Name>evil</tns:Name></tns:Node> \
                 </tns:Unknown> \
                 <tns:Kid> \
                     <tns:Name>k</tns:Name> \
                     <tns:Unknown>junk</tns:Unknown> \
                 </tns:Kid> \
             </tns:Node>",
        );

        assert_eq!(obj.name, "n");
        let kid = obj.kid.expect("kid expected");
        assert_eq!(kid.name, "k");
        assert!(kid.kid.is_none());
    }

    // Indirectly recursive model group (group -> element -> type -> group)
    // with unknown elements inside: the `is_known_start_tag` delegation
    // chain must terminate (no runtime recursion) and unknown elements must
    // be skipped without breaking the group state machines.
    #[test]
    fn v4_unknown_inside_recursive_group() {
        let obj = read_ok::<GRoot>(
            "<tns:GRoot xmlns:tns=\"http://example.com\"> \
                 <tns:V>v</tns:V> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:Via> \
                     <tns:V>inner</tns:V> \
                     <tns:Unknown>junk</tns:Unknown> \
                 </tns:Via> \
                 <tns:Unknown>junk</tns:Unknown> \
             </tns:GRoot>",
        );

        assert_eq!(obj.rec_group.v, "v");
        let via = obj.rec_group.via.expect("via expected");
        assert_eq!(via.rec_group.v, "inner");
        assert!(via.rec_group.via.is_none());
    }

    // A duplicated known element inside a group must still be rejected.
    #[test]
    fn v4_duplicate_known_inside_recursive_group() {
        read_err::<GRoot, _>(
            "<tns:GRoot xmlns:tns=\"http://example.com\"> \
                 <tns:V>1</tns:V> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:V>2</tns:V> \
             </tns:GRoot>",
            is_unexpected_event,
        );
    }

    /* Vector 5: robustness */

    // Deeply nested (1000 levels) unknown element: the skip must be iterative
    // and neither overflow the stack nor lose the state.
    #[test]
    fn v5_deep_unknown_nesting() {
        const DEPTH: usize = 1000;

        let mut xml = String::from(
            "<tns:Root xmlns:tns=\"http://example.com\"><tns:A>value</tns:A><tns:Unknown>",
        );
        xml += &"<D>".repeat(DEPTH);
        xml += &"</D>".repeat(DEPTH);
        xml += "</tns:Unknown><tns:Pos>after</tns:Pos></tns:Root>";

        let obj = read_ok::<Root>(Box::leak(xml.into_boxed_str()));

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    // Unknown element as last element before EOF: must error (the end tag of
    // the root element is missing), but neither hang nor panic.
    #[test]
    fn v5_unknown_last_before_eof() {
        read_err::<Root, _>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:A>value</tns:A> \
                 <tns:Unknown>junk</tns:Unknown>",
            is_reader_error,
        );
    }

    // EOF in the middle of a skipped subtree: must error, but neither hang
    // nor panic.
    #[test]
    fn v5_eof_mid_skip() {
        read_err::<Root, _>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:A>value</tns:A> \
                 <tns:Unknown><D><D>",
            is_reader_error,
        );
    }

    // Empty document: must error, but neither hang nor panic.
    #[test]
    fn v5_empty_document() {
        read_err::<Root, _>("", is_reader_error);
    }

    // Unknown element with many attributes (including namespace declarations)
    // must be skipped completely.
    #[test]
    fn v5_unknown_with_many_attributes() {
        let mut attribs = String::new();
        for i in 0..30 {
            attribs += &format!(" a{i}=\"{i}\"");
        }
        for i in 0..10 {
            attribs += &format!(" xmlns:p{i}=\"urn:p{i}\"");
        }

        let mut xml = String::from("<tns:Root xmlns:tns=\"http://example.com\">");
        xml += "<tns:A>value</tns:A>";
        xml += &format!("<tns:Unknown{attribs}><D p0=\"x\">junk</D></tns:Unknown>");
        xml += "<tns:Pos>after</tns:Pos></tns:Root>";

        let obj = read_ok::<Root>(Box::leak(xml.into_boxed_str()));

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    /* Vector 6: is_known_start_tag boundaries */

    // Tag names must match case sensitively: `POS` and `pos` are not known to
    // the content model (only `Pos` is) and must be skipped.
    #[test]
    fn v6_case_must_matter() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:POS>junk</tns:POS> \
                 <tns:A>value</tns:A> \
                 <tns:pos>junk</tns:pos> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    // The same prefix declared twice on the same element (last wins) and on
    // nested elements inside a skipped subtree must not break the namespace
    // scope stack.
    #[test]
    fn v6_duplicate_prefix_declarations() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown xmlns:tns=\"urn:evil1\" xmlns:tns=\"urn:evil2\"> \
                     <tns:Deep xmlns:tns=\"urn:evil3\"><tns:A>junk</tns:A></tns:Deep> \
                 </tns:Unknown> \
                 <tns:A>value</tns:A> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    // Simple sanity check that the generated root type can be deserialized
    // with the in-memory reader used by the tests above.
    #[test]
    fn v0_root_type_still_works() {
        let obj = read_ok::<RootType>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:A>value</tns:A> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos, None);
    }

    // Unknown empty element with a rebound default namespace: no subtree is
    // skipped, but the namespace scope must still be popped lazily, so that
    // the following known elements resolve correctly.
    #[test]
    fn v2_unknown_empty_with_default_namespace_rebind() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <Unknown xmlns=\"urn:evil\"/> \
                 <tns:A>value</tns:A> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    // An unknown element nested inside a complex child element, whose tag
    // matches a known but already exhausted element of the root type, is a
    // content model violation and must still be rejected: the event surfaces
    // unconsumed at the driver and the `is_known_start_tag` check of the root
    // type rejects it.
    #[test]
    fn v2_nested_unknown_colliding_with_root_model_rejected() {
        read_err::<Outer, _>(
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>inner</tns:Inner> \
                     <tns:Child>junk</tns:Child> \
                 </tns:Child> \
                 <tns:B>outer</tns:B> \
             </tns:Outer>",
            is_unexpected_event,
        );
    }

    // Upstream event bubbling semantics: an unknown element inside a complex
    // child, whose tag matches a still open slot of the root type, is claimed
    // by the state machine of the root type (the event is consumed and never
    // surfaces at the driver). This behavior is independent of the skip logic
    // and must not be changed by it.
    #[test]
    fn v2_nested_unknown_claimed_by_open_root_slot() {
        let obj = read_ok::<Outer>(
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>inner</tns:Inner> \
                     <tns:B>collide</tns:B> \
                 </tns:Child> \
             </tns:Outer>",
        );

        assert_eq!(obj.child.inner, "inner");
        assert_eq!(obj.child.extra, None);
        assert_eq!(obj.b.as_deref(), Some("collide"));
    }

    // Unknown element that surfaces from a nested child deserializer (two
    // levels below the root) while the child deserializer is staged in the
    // fallback slot: after the skip, the child must still complete correctly.
    #[test]
    fn v3_unknown_from_nested_child_completes_child() {
        let obj = read_ok::<Outer>(
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>inner</tns:Inner> \
                     <tns:Extra>extra</tns:Extra> \
                     <tns:Inner>junk</tns:Inner> \
                 </tns:Child> \
                 <tns:B>after</tns:B> \
             </tns:Outer>",
        );

        assert_eq!(obj.child.inner, "inner");
        assert_eq!(obj.child.extra.as_deref(), Some("extra"));
        assert_eq!(obj.b.as_deref(), Some("after"));
    }

    // An attribute that merely starts with `xmlns` (e.g. `xmlnsx`) is not a
    // namespace declaration: inside a skipped subtree it must neither break
    // the namespace scope stack nor the parse.
    #[test]
    fn v2_xmlns_lookalike_attribute_inside_skip() {
        let obj = read_ok::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown xmlnsx=\"junk\"> \
                     <D xmlnsx2=\"junk\"/>
                 </tns:Unknown> \
                 <tns:A>value</tns:A> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
        );

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));
    }

    /* Async driver */
    // The async driver shares the `handle_event` implementation with the
    // sync driver: unknown elements must be skipped as well.
    #[tokio::test]
    async fn v7_async_path_skips_unknown() {
        use xsd_parser_types::quick_xml::DeserializeAsync;

        async fn read<T>(xml: &'static str) -> Result<T, Error>
        where
            T: DeserializeAsync<'static, SliceReader<'static>, Error = Error>,
        {
            T::deserialize_async(&mut SliceReader::new(xml)).await
        }

        let obj = read::<Root>(
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:A>value</tns:A> \
                 <tns:Unknown><D><D>junk</D></D></tns:Unknown> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
        )
        .await
        .unwrap_or_else(|e| panic!("expected successful deserialization: {e:?}"));

        assert_eq!(obj.a, "value");
        assert_eq!(obj.pos.as_deref(), Some("after"));

        read::<Root>("<tns:Root xmlns:tns=\"http://example.com\"><tns:A>x</tns:A><tns:A>y</tns:A></tns:Root>")
            .await
            .expect_err("duplicate known element must be rejected");
    }
}
