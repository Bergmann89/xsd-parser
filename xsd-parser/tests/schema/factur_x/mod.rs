use xsd_parser::{config::NamespaceIdent, Config, IdentType};

use crate::utils::ConfigEx;

mod basic;
mod basicwl;
mod en16931;
mod extended;
mod minimum;

fn config() -> Config {
    Config::test_default().with_generate([(
        IdentType::Element,
        Some(NamespaceIdent::namespace(
            b"urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100",
        )),
        "CrossIndustryInvoice",
    )])
}
