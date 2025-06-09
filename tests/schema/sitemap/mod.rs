use xsd_parser::{
    config::{GeneratorFlags, NamespaceIdent, SerdeXmlRsVersion},
    Config, IdentType,
};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    Config::test_default()
        .with_generator_flags(GeneratorFlags::FLATTEN_CONTENT | GeneratorFlags::USE_MODULES)
        .with_generate([(
            IdentType::Element,
            NamespaceIdent::namespace(b"http://www.sitemaps.org/schemas/sitemap/0.9"),
            "urlset",
        )])
}

#[test]
fn generate_default() {
    generate_test(
        "tests/schema/sitemap/schema.xsd",
        "tests/schema/sitemap/expected/default.rs",
        config(),
    );
}

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/schema/sitemap/schema.xsd",
        "tests/schema/sitemap/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
fn generate_serde_xml_rs() {
    generate_test(
        "tests/schema/sitemap/schema.xsd",
        "tests/schema/sitemap/expected/serde_xml_rs.rs",
        config().with_serde_xml_rs(SerdeXmlRsVersion::Version08AndAbove),
    );
}

#[test]
fn generate_serde_quick_xml() {
    generate_test(
        "tests/schema/sitemap/schema.xsd",
        "tests/schema/sitemap/expected/serde_quick_xml.rs",
        config().with_serde_quick_xml(),
    );
}

#[cfg(not(feature = "update-expectations"))]
macro_rules! assert_obj {
    ($obj:expr, $changefreq:ident $(, $( $map:tt )+ )?) => {{
        let obj = $obj;
        let mut urls = obj.url.iter();

        let url = urls.next().unwrap();
        assert_eq!("http://www.example.com/", url.loc);
        assert_eq!(Some("2005-01-01"), url.lastmod.as_deref());
        assert!(matches!(url.changefreq $( $( $map )+ )?, Some($changefreq::Monthly)));
        assert_eq!(Some(0.8), url.priority);

        let url = urls.next().unwrap();
        assert_eq!(
            "http://www.example.com/catalog?item=12&desc=vacation_hawaii",
            url.loc
        );
        assert_eq!(None, url.lastmod.as_deref());
        assert!(matches!(url.changefreq $( $( $map )+ )?, Some($changefreq::Weekly)));
        assert_eq!(None, url.priority);

        let url = urls.next().unwrap();
        assert_eq!(
            "http://www.example.com/catalog?item=73&desc=vacation_new_zealand",
            url.loc
        );
        assert_eq!(Some("2004-12-23"), url.lastmod.as_deref());
        assert!(matches!(url.changefreq $( $( $map )+ )?, Some($changefreq::Weekly)));
        assert_eq!(None, url.priority);

        let url = urls.next().unwrap();
        assert_eq!(
            "http://www.example.com/catalog?item=74&desc=vacation_newfoundland",
            url.loc
        );
        assert_eq!(Some("2004-12-23T18:00:15+00:00"), url.lastmod.as_deref());
        assert!(matches!(url.changefreq $( $( $map )+ )?, None));
        assert_eq!(Some(0.3), url.priority);

        let url = urls.next().unwrap();
        assert_eq!(
            "http://www.example.com/catalog?item=83&desc=vacation_usa",
            url.loc
        );
        assert_eq!(Some("2004-11-23"), url.lastmod.as_deref());
        assert!(matches!(url.changefreq $( $( $map )+ )?, None));
        assert_eq!(None, url.priority);

        assert!(urls.next().is_none());
    }};
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::{TChangeFreqType, Urlset};

    let obj =
        crate::utils::quick_xml_read_test::<Urlset, _>("tests/schema/sitemap/example/default.xml");

    assert_obj!(obj, TChangeFreqType);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_xml_rs() {
    use serde_xml_rs::{TChangeFreqValue, Urlset};

    let obj = crate::utils::serde_xml_rs_read_test::<Urlset, _>(
        "tests/schema/sitemap/example/default.xml",
    );

    assert_obj!(obj, TChangeFreqValue, .as_ref().map(|x| &x.value));
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_serde_quick_xml() {
    use serde_quick_xml::{TChangeFreqType, Urlset};

    let obj = crate::utils::serde_quick_xml_read_test::<Urlset, _>(
        "tests/schema/sitemap/example/default.xml",
    );

    assert_obj!(obj, TChangeFreqType);
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    include!("expected/default.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    include!("expected/quick_xml.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_xml_rs {
    include!("expected/serde_xml_rs.rs");
}

#[cfg(not(feature = "update-expectations"))]
mod serde_quick_xml {
    #![allow(unused_imports)]

    include!("expected/serde_quick_xml.rs");
}
