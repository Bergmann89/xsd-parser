#![allow(missing_docs, clippy::wildcard_imports)]

use anyhow::Error;
use benchmark::{schemas::*, Args, Runner};
use clap::Parser;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let mut runner = Runner::new(args)?;

    /* XMLSchema */

    runner.add_test_quick_xml_serialize_file::<xs_quick_xml::Schema, _>(
        "XMLSchema",
        "XMLSchema",
        "../xsd-parser/schema/XMLSchema.xsd",
    )?;
    runner.add_test_quick_xml_deserialize_file::<xs_quick_xml::Schema, _>(
        "XMLSchema",
        "XMLSchema",
        "../xsd-parser/schema/XMLSchema.xsd",
    )?;
    runner.add_test_quick_xml_deserialize_file::<xs_quick_xml_boxed::Schema, _>(
        "XMLSchema (boxed)",
        "XMLSchema",
        "../xsd-parser/schema/XMLSchema.xsd",
    )?;

    /* ONIX_BookProduct_3.1 */

    runner.add_test_quick_xml_serialize_file::<xs_quick_xml::Schema, _>(
        "XMLSchema",
        "ONIX_BookProduct_3.1",
        "../xsd-parser/tests/schema/onix/schema/ONIX_BookProduct_3.1_reference.xsd",
    )?;
    runner.add_test_quick_xml_deserialize_file::<xs_quick_xml::Schema, _>(
        "XMLSchema",
        "ONIX_BookProduct_3.1",
        "../xsd-parser/tests/schema/onix/schema/ONIX_BookProduct_3.1_reference.xsd",
    )?;
    runner.add_test_quick_xml_deserialize_file::<xs_quick_xml_boxed::Schema, _>(
        "XMLSchema (boxed)",
        "ONIX_BookProduct_3.1",
        "../xsd-parser/tests/schema/onix/schema/ONIX_BookProduct_3.1_reference.xsd",
    )?;

    /* XJustiz 3.4.1 (dabag) */

    runner.add_test_quick_xml_serialize_file::<xs_quick_xml::Schema, _>(
        "XMLSchema",
        "XJustiz 3.4.1 (dabag)",
        "schemas/xjustiz_2900_dabag_3_1.xsd",
    )?;
    runner.add_test_quick_xml_deserialize_file::<xs_quick_xml::Schema, _>(
        "XMLSchema",
        "XJustiz 3.4.1 (dabag)",
        "schemas/xjustiz_2900_dabag_3_1.xsd",
    )?;
    runner.add_test_quick_xml_deserialize_file::<xs_quick_xml_boxed::Schema, _>(
        "XMLSchema (boxed)",
        "XJustiz 3.4.1 (dabag)",
        "schemas/xjustiz_2900_dabag_3_1.xsd",
    )?;

    /* iDEAL 3.3.1 Merchant-Acquirer */

    runner
        .add_test_quick_xml_serialize_file::<ideal_merchant_acquirer_quick_xml::DirectoryReq, _>(
            "iDEAL 3.3.1 Merchant-Acquirer",
            "merchant-acquirer.xml",
            "../xsd-parser/tests/schema/ideal_merchant_acquirer/example/merchant-acquirer.xml",
        )?;
    runner.add_test_serde_quick_xml_serialize_file::<ideal_merchant_acquirer_serde_quick_xml::DirectoryReq, _>(
        "iDEAL 3.3.1 Merchant-Acquirer",
        "merchant-acquirer.xml",
        "../xsd-parser/tests/schema/ideal_merchant_acquirer/example/merchant-acquirer.xml",
    )?;
    let value = runner.serde_xml_rs_v7_deserialize_file(
        "../xsd-parser/tests/schema/ideal_merchant_acquirer/example/merchant-acquirer.xml",
    )?;
    // Serialization is broken in `xml_rs v0.7`
    runner.add_test_serde_xml_rs_serialize_value::<ideal_merchant_acquirer_serde_xml_rs_v7::DirectoryReq>(
        "iDEAL 3.3.1 Merchant-Acquirer",
        "merchant-acquirer.xml",
        &value,
    )?;
    runner
        .add_test_quick_xml_deserialize_file::<ideal_merchant_acquirer_quick_xml::DirectoryReq, _>(
            "iDEAL 3.3.1 Merchant-Acquirer",
            "merchant-acquirer.xml",
            "../xsd-parser/tests/schema/ideal_merchant_acquirer/example/merchant-acquirer.xml",
        )?;
    runner.add_test_quick_xml_deserialize_file::<ideal_merchant_acquirer_quick_xml_boxed::DirectoryReq, _>(
        "iDEAL 3.3.1 Merchant-Acquirer (boxed)",
        "merchant-acquirer.xml",
        "../xsd-parser/tests/schema/ideal_merchant_acquirer/example/merchant-acquirer.xml",
    )?;
    runner
        .add_test_serde_quick_xml_deserialize_file::<ideal_merchant_acquirer_serde_quick_xml::DirectoryReq, _>(
            "iDEAL 3.3.1 Merchant-Acquirer",
            "merchant-acquirer.xml",
            "../xsd-parser/tests/schema/ideal_merchant_acquirer/example/merchant-acquirer.xml",
        )?;
    runner
        .add_test_serde_xml_rs_v7_deserialize_file::<ideal_merchant_acquirer_serde_xml_rs_v7::DirectoryReq, _>(
            "iDEAL 3.3.1 Merchant-Acquirer",
            "merchant-acquirer.xml",
            "../xsd-parser/tests/schema/ideal_merchant_acquirer/example/merchant-acquirer.xml",
        )?;

    /* Summary */

    runner.summary();

    Ok(())
}
