use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

use quote::quote;

use xsd_parser::{
    config::{Config, IdentTriple, NamespaceIdent},
    models::{
        meta::{CustomMeta, MetaType, ReferenceMeta},
        Ident,
    },
    IdentType,
};
use xsd_parser_types::quick_xml::{DeserializeBytesFromStr, SerializeBytesToString};

use crate::utils::{generate_test, ConfigEx};

fn config() -> Config {
    let mut config = Config::test_default().with_generate([(
        IdentType::Element,
        Some(NamespaceIdent::namespace(b"urn:example:minimal")),
        "Amount",
    )]);

    config.interpreter.types = vec![
        (
            IdentTriple::from((IdentType::Type, "Decimal")),
            MetaType::from(CustomMeta::new("Decimal").with_default(|s: &str| {
                let code = quote! {
                    <Decimal as core::str::FromStr>::from_str(#s).unwrap()
                };

                Some(code)
            })),
        ),
        (
            IdentTriple::from((IdentType::Type, "xs:decimal")),
            MetaType::from(ReferenceMeta::new(Ident::type_("Decimal"))),
        ),
    ];

    config
}

/* default */

#[test]
fn generate_default() {
    generate_test(
        "tests/feature/custom_type/schema.xsd",
        "tests/feature/custom_type/expected/default.rs",
        config(),
    );
}

#[cfg(not(feature = "update-expectations"))]
mod default {
    #![allow(unused_imports)]

    use super::Decimal;

    include!("expected/default.rs");
}

/* quick_xml */

#[test]
fn generate_quick_xml() {
    generate_test(
        "tests/feature/custom_type/schema.xsd",
        "tests/feature/custom_type/expected/quick_xml.rs",
        config().with_quick_xml(),
    );
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn read_quick_xml() {
    use quick_xml::Amount;

    let obj = crate::utils::quick_xml_read_test::<Amount, _>(
        "tests/feature/custom_type/example/default.xml",
    );

    assert_eq!(obj.ccy, "test");
    assert_eq!(f64::try_from(obj.content.0).unwrap(), 1234.0);
}

#[test]
#[cfg(not(feature = "update-expectations"))]
fn write_quick_xml() {
    use quick_xml::Amount;

    let obj = Amount {
        ccy: "test".into(),
        content: Decimal::from_str("1234").unwrap(),
    };

    crate::utils::quick_xml_write_test(
        &obj,
        "Amount",
        "tests/feature/custom_type/example/default.xml",
    );
}

#[cfg(not(feature = "update-expectations"))]
mod quick_xml {
    #![allow(unused_imports)]

    use super::Decimal;

    include!("expected/quick_xml.rs");
}

#[derive(Default, Debug)]
struct Decimal(rust_decimal::Decimal);

impl SerializeBytesToString for Decimal {}
impl DeserializeBytesFromStr for Decimal {}

impl FromStr for Decimal {
    type Err = rust_decimal::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(rust_decimal::Decimal::from_str_exact(s)?))
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}
