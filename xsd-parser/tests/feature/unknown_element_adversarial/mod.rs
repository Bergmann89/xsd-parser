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
            (IdentType::Element, "tns:Scene"),
            (IdentType::Element, "tns:Pick"),
            (IdentType::Element, "tns:Unordered"),
            (IdentType::Element, "tns:Nothing"),
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
        config()
            .with_quick_xml_serialize()
            .with_quick_xml_deserialize_config(false, true),
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

    use xsd_parser_types::quick_xml::{DeserializeSync, Error, ErrorKind, Event, SliceReader};

    use super::quick_xml::{
        GRoot, Max, Node, Nothing, Outer, Pick, PickType, Root, Scene, ShapeType, Unordered,
    };

    fn read<T>(xml: &'static str) -> Result<T, Error>
    where
        T: DeserializeSync<'static, SliceReader<'static>, Error = Error>,
    {
        T::deserialize(&mut SliceReader::new(xml))
    }

    fn read_ok<T>(xml: &'static str) -> T
    where
        T: DeserializeSync<'static, SliceReader<'static>, Error = Error>,
        T::Error: Debug,
    {
        read(xml).unwrap()
    }

    macro_rules! assert_read_err {
        ($ty:ty, $xml:expr, $kind:pat) => {{
            let err = read::<$ty>($xml).unwrap_err();

            assert!(matches!(err.kind, $kind), "unexpected error: {err:?}");
        }};
    }

    // The unknown element must not "consume" the required `A` slot: A is
    // missing and the document has to be rejected with `MissingElement`.
    #[test]
    fn unknown_element_does_not_satisfy_required_element() {
        assert_read_err!(
            Root,
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown>junk</tns:Unknown> \
             </tns:Root>",
            ErrorKind::MissingElement(_)
        );
    }

    // A known element that appears where the required `A` element was
    // expected is misplaced and must not be silently skipped.
    #[test]
    fn misplaced_known_element_after_unknown_element_rejected() {
        assert_read_err!(
            Root,
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:Pos>after</tns:Pos> \
             </tns:Root>",
            ErrorKind::UnexpectedEvent(_)
        );
    }

    // A known element that exceeds `maxOccurs` must still be rejected, even
    // if unknown elements where skipped in between.
    #[test]
    fn max_occurs_exceeded_after_unknown_element_rejected() {
        assert_read_err!(
            Max,
            "<tns:Max xmlns:tns=\"http://example.com\"> \
                 <tns:A>1</tns:A> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:A>2</tns:A> \
                 <tns:A>3</tns:A> \
             </tns:Max>",
            ErrorKind::UnexpectedEvent(_)
        );
    }

    #[test]
    fn max_occurs_boundary_with_unknown_elements() {
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
    fn duplicate_known_element_after_unknown_element_rejected() {
        assert_read_err!(
            Root,
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:A>first</tns:A> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:A>second</tns:A> \
             </tns:Root>",
            ErrorKind::UnexpectedEvent(_)
        );
    }

    // A foreign namespace tag with the local name of a known child is not
    // known to the content model (`resolve_local_name` returns `None`) and
    // must be skipped; the real element must still be parsed.
    #[test]
    fn foreign_namespace_with_known_local_name_skipped() {
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
    fn default_namespace_rebind_inside_skipped_subtree() {
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
    fn prefix_rebind_inside_skipped_subtree() {
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
    fn nested_prefix_rebind_inside_skipped_subtree() {
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
    fn default_namespace_undeclared_inside_skipped_subtree() {
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

    // Three or more consecutive unknown elements.
    #[test]
    fn consecutive_unknown_elements() {
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
    fn unknown_element_before_root_end() {
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
    fn unknown_element_before_root_end_without_optional() {
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
    fn unknown_element_inside_nested_child() {
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
    fn unknown_element_after_nested_child_content() {
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
    fn unknown_element_between_child_and_sibling() {
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
    fn known_element_after_skipped_subtree_uses_fresh_state() {
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

    // Recursive type with unknown elements inside the recursion: must not
    // loop or overflow, the recursion must still deserialize correctly.
    #[test]
    fn unknown_element_inside_recursive_type() {
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
    fn unknown_element_inside_recursive_group() {
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
    fn duplicate_known_element_inside_recursive_group_rejected() {
        assert_read_err!(
            GRoot,
            "<tns:GRoot xmlns:tns=\"http://example.com\"> \
                 <tns:V>1</tns:V> \
                 <tns:Unknown>junk</tns:Unknown> \
                 <tns:V>2</tns:V> \
             </tns:GRoot>",
            ErrorKind::UnexpectedEvent(_)
        );
    }

    // Deeply nested (1000 levels) unknown element: the skip must be iterative
    // and neither overflow the stack nor lose the state.
    #[test]
    fn deeply_nested_unknown_element() {
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
    fn unknown_element_before_eof_rejected() {
        assert_read_err!(
            Root,
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:A>value</tns:A> \
                 <tns:Unknown>junk</tns:Unknown>",
            ErrorKind::UnexpectedEvent(Event::Eof)
        );
    }

    // EOF in the middle of a skipped subtree: must error, but neither hang
    // nor panic.
    #[test]
    fn eof_inside_skipped_subtree_rejected() {
        assert_read_err!(
            Root,
            "<tns:Root xmlns:tns=\"http://example.com\"> \
                 <tns:A>value</tns:A> \
                 <tns:Unknown><D><D>",
            ErrorKind::UnexpectedEvent(Event::Eof)
        );
    }

    // Unknown element with many attributes (including namespace declarations)
    // must be skipped completely.
    #[test]
    fn unknown_element_with_many_attributes() {
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

    // Tag names must match case sensitively: `POS` and `pos` are not known to
    // the content model (only `Pos` is) and must be skipped.
    #[test]
    fn tag_names_are_case_sensitive() {
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
    fn duplicate_prefix_declarations_inside_skipped_subtree() {
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

    // Unknown empty element with a rebound default namespace: no subtree is
    // skipped, but the namespace scope must still be popped lazily, so that
    // the following known elements resolve correctly.
    #[test]
    fn empty_unknown_element_with_default_namespace_rebind() {
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

    // An element that is unknown to the nested `Child` element is skipped by
    // `Child`, even if its tag is a child element of the root type.
    #[test]
    fn nested_unknown_element_known_to_parent_skipped() {
        let obj = read_ok::<Outer>(
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>inner</tns:Inner> \
                     <tns:Child>junk</tns:Child> \
                 </tns:Child> \
                 <tns:B>outer</tns:B> \
             </tns:Outer>",
        );

        assert_eq!(obj.child.inner, "inner");
        assert_eq!(obj.b.as_deref(), Some("outer"));
    }

    // The deserializer of `Child` decides about its own content, so an element
    // that is only known to the parent is not claimed by an open slot of the
    // parent.
    #[test]
    fn nested_unknown_element_not_claimed_by_parent_slot() {
        let obj = read_ok::<Outer>(
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>inner</tns:Inner> \
                     <tns:B>skipped</tns:B> \
                 </tns:Child> \
             </tns:Outer>",
        );

        assert_eq!(obj.child.inner, "inner");
        assert_eq!(obj.child.extra, None);
        assert_eq!(obj.b, None);
    }

    // A known element duplicated inside of a nested element is still rejected.
    #[test]
    fn nested_duplicate_known_element_rejected() {
        assert_read_err!(
            Outer,
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>a</tns:Inner> \
                     <tns:Inner>b</tns:Inner> \
                 </tns:Child> \
             </tns:Outer>",
            ErrorKind::UnexpectedEvent(_)
        );
    }

    // An unknown element in front of a still expected optional sibling does
    // not close the slot of the sibling.
    #[test]
    fn nested_unknown_element_before_optional_sibling() {
        let obj = read_ok::<Outer>(
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>a</tns:Inner> \
                     <tns:Vendor><tns:X/></tns:Vendor> \
                     <tns:Extra>e</tns:Extra> \
                 </tns:Child> \
             </tns:Outer>",
        );

        assert_eq!(obj.child.inner, "a");
        assert_eq!(obj.child.extra.as_deref(), Some("e"));
    }

    // Elements nested in a skipped subtree are ignored, even if they are known
    // to a type further up.
    #[test]
    fn nested_unknown_subtree_ignored() {
        let obj = read_ok::<Outer>(
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Vendor> \
                         <tns:Inner>x</tns:Inner> \
                         <tns:Child><tns:Deep><tns:Z/></tns:Deep></tns:Child> \
                     </tns:Vendor> \
                     <tns:Inner>a</tns:Inner> \
                 </tns:Child> \
                 <tns:B>b</tns:B> \
             </tns:Outer>",
        );

        assert_eq!(obj.child.inner, "a");
        assert_eq!(obj.b.as_deref(), Some("b"));
    }

    // Members of a substitution group are part of the content model: a
    // misplaced member is rejected, an unknown element is skipped.
    #[test]
    fn misplaced_substitution_member_rejected() {
        assert_read_err!(
            Scene,
            "<tns:Scene xmlns:tns=\"http://example.com\"> \
                 <tns:Circle id=\"1\"/> \
                 <tns:Square id=\"2\"/> \
             </tns:Scene>",
            ErrorKind::UnexpectedEvent(_)
        );
    }

    #[test]
    fn choice_skips_unknown_element_and_rejects_second_member() {
        let obj = read_ok::<Pick>(
            "<tns:Pick xmlns:tns=\"http://example.com\"><tns:Z/><tns:X>x</tns:X><tns:Z/></tns:Pick>",
        );

        assert!(matches!(obj, PickType::X(x) if x == "x"));

        assert_read_err!(Pick, "<tns:Pick xmlns:tns=\"http://example.com\"><tns:X>x</tns:X><tns:Y>y</tns:Y></tns:Pick>", ErrorKind::UnexpectedEvent(_));
    }

    #[test]
    fn all_skips_unknown_element_and_rejects_duplicate() {
        let obj = read_ok::<Unordered>(
            "<tns:Unordered xmlns:tns=\"http://example.com\"><tns:Z><tns:Y/></tns:Z><tns:Y>y</tns:Y><tns:X>x</tns:X></tns:Unordered>",
        );

        assert_eq!(obj.x, "x");
        assert_eq!(obj.y.as_deref(), Some("y"));

        assert_read_err!(Unordered, "<tns:Unordered xmlns:tns=\"http://example.com\"><tns:X>x</tns:X><tns:X>y</tns:X></tns:Unordered>", ErrorKind::DuplicateElement(_));
    }

    #[test]
    fn empty_element_skips_unknown_elements() {
        read_ok::<Nothing>(
            "<tns:Nothing xmlns:tns=\"http://example.com\"><tns:Z><tns:Y/></tns:Z><tns:Z/></tns:Nothing>",
        );
    }

    #[test]
    fn substitution_group_skips_unknown_element() {
        let obj = read_ok::<Scene>(
            "<tns:Scene xmlns:tns=\"http://example.com\"> \
                 <tns:Triangle/> \
                 <tns:Circle id=\"1\"/> \
             </tns:Scene>",
        );

        let shape = obj.shape.expect("shape expected");
        let circle = shape.0.as_any().downcast_ref::<ShapeType>().unwrap();

        assert_eq!(circle.id, Some(1));
    }

    // A known element that appears again after the deserializer of the nested
    // child has moved past its slot is rejected by the child, even if the
    // child already read optional content.
    #[test]
    fn duplicate_known_element_after_nested_optional_rejected() {
        assert_read_err!(
            Outer,
            "<tns:Outer xmlns:tns=\"http://example.com\"> \
                 <tns:Child> \
                     <tns:Inner>inner</tns:Inner> \
                     <tns:Extra>extra</tns:Extra> \
                     <tns:Inner>junk</tns:Inner> \
                 </tns:Child> \
                 <tns:B>after</tns:B> \
             </tns:Outer>",
            ErrorKind::UnexpectedEvent(_)
        );
    }

    // An attribute that merely starts with `xmlns` (e.g. `xmlnsx`) is not a
    // namespace declaration: inside a skipped subtree it must neither break
    // the namespace scope stack nor the parse.
    #[test]
    fn xmlns_lookalike_attribute_inside_skipped_subtree() {
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
}
