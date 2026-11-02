use std::path::Path;

use xsd_parser::{config::Schema, generate, Config, Error, IdentType, InterpreterError};

use crate::utils::ConfigEx;

#[test]
fn missing_base_returns_error() {
    let mut config = Config::test_default().with_generate([(IdentType::Element, "tns:Foo")]);
    let xsd = Path::new("tests/feature/missing_base_type/schema.xsd")
        .canonicalize()
        .unwrap();
    config.parser.schemas.push(Schema::File(xsd));

    let err = generate(config).unwrap_err();

    assert!(
        matches!(
            &err,
            Error::InterpreterError(InterpreterError::UnknownType(ident))
                if ident.name.as_str() == "Missing"
        ),
        "{err:?}"
    );
}
