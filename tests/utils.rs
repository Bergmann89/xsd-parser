#![allow(dead_code, missing_docs, unreachable_pub)]

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use quick_xml::Reader;
use serde::Deserialize;

use xsd_parser::config::IdentTriple;
use xsd_parser::{
    config::{Config, Generate, OptimizerFlags, Schema},
    generate,
    generator::GeneratorFlags,
    quick_xml::{DeserializeSync, ErrorReader, Event, IoReader, WithSerializer, Writer, XmlReader},
};

pub trait ConfigEx {
    fn test_default() -> Self;
}

impl ConfigEx for Config {
    fn test_default() -> Self {
        let mut config = Config::default();

        config.optimizer.flags |= OptimizerFlags::RESOLVE_TYPEDEFS;

        config.generator.type_postfix.element_type = "Type".into();
        config.generator.flags |= GeneratorFlags::FLATTEN_STRUCT_CONTENT;

        config
    }
}

pub fn generate_test<P1, P2>(input_xsd: P1, expected_rs: P2, mut config: Config)
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    config
        .parser
        .schemas
        .push(Schema::File(input_xsd.as_ref().to_path_buf()));

    // For debugging purposes enable the following lines
    // config.parser.debug_output = Some("schemas.log".into());
    // config.interpreter.debug_output = Some("interpreter.log".into());
    // config.optimizer.debug_output = Some("optimizer.log".into());

    let actual = generate(config).unwrap();
    let actual_str = fmt_code(&actual.to_string());

    #[cfg(not(feature = "update-expectations"))]
    {
        use std::fs::read_to_string;
        use std::str::FromStr;

        use proc_macro2::TokenStream;
        use text_diff::print_diff;

        let expected = read_to_string(expected_rs).unwrap();
        let expected = TokenStream::from_str(&expected).unwrap();
        let expected_str = fmt_code(&expected.to_string());

        if expected_str != actual_str {
            println!("=== expected:\n{expected_str}");
            println!("=== actual:\n{actual_str}");
            println!("=== diff:\n");

            print_diff(&expected_str, &actual_str, "\n");

            panic!("Code does not match!");
        }
    }

    #[cfg(feature = "update-expectations")]
    {
        std::fs::write(expected_rs, actual_str).unwrap();
    }
}

pub fn optimizer_test<P1, P2, P3, T>(
    input_xsd: P1,
    expected_0: P2,
    expected_1: P3,
    types: T,
    flags: OptimizerFlags,
) where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
    T: IntoIterator,
    T::Item: Into<IdentTriple>,
{
    let mut config = Config::test_default();

    config.optimizer.flags = OptimizerFlags::RESOLVE_TYPEDEFS;

    optimizer_test_with_config(input_xsd, expected_0, expected_1, types, flags, config);
}

pub fn optimizer_test_with_config<P1, P2, P3, T>(
    input_xsd: P1,
    expected_0: P2,
    expected_1: P3,
    types: T,
    flags: OptimizerFlags,
    mut config: Config,
) where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
    T: IntoIterator,
    T::Item: Into<IdentTriple>,
{
    config
        .parser
        .schemas
        .push(Schema::File(input_xsd.as_ref().to_path_buf()));
    config.generator.generate = Generate::Types(types.into_iter().map(Into::into).collect());
    config.generator.derive = Some(Vec::new());
    config.generator.flags -= GeneratorFlags::USE_MODULES;

    let input_xsd = input_xsd.as_ref();

    generate_test(&input_xsd, expected_0, config.clone());

    config.optimizer.flags |= flags;

    generate_test(&input_xsd, expected_1, config.clone());
}

pub fn quick_xml_read_test<T, P>(path: P) -> T
where
    P: AsRef<Path>,
    T: DeserializeSync<'static, ErrorReader<IoReader<BufReader<File>>>>,
    T::Error: Debug,
{
    let reader = File::open(path).unwrap();
    let reader = BufReader::new(reader);
    let mut reader = IoReader::new(reader).with_error_info();

    let ret = T::deserialize(&mut reader).unwrap();

    ret
}

pub fn quick_xml_write_test<T, P>(value: &T, root: &str, path: P)
where
    P: AsRef<Path>,
    T: WithSerializer,
{
    let mut content = Vec::new();
    let mut writer = Writer::new(&mut content);

    let mut actual = value.serializer(Some(root), true).unwrap();
    let mut expected = Reader::from_file(path).unwrap();
    let mut buffer = Vec::new();

    let (actual, expected) = loop {
        let expected = match expected.read_event_into(&mut buffer).unwrap() {
            Event::Decl(_) => continue,
            Event::Text(x) if x.trim_ascii_start().trim_ascii_end().is_empty() => continue,
            expected => expected,
        };
        let actual = actual.next().transpose().unwrap().unwrap_or(Event::Eof);

        match quick_xml_event_cmp(&actual, &expected) {
            None => return,
            Some(true) => (),
            Some(false) => break (actual, expected),
        }

        writer.write_event(actual).unwrap();
    };

    println!("=== actual: {actual:?}");
    println!("=== expected: {expected:?}");

    let content = std::str::from_utf8(&content).unwrap();

    println!("=== content: {content}");

    panic!("Unexpected event")
}

pub fn serde_quick_xml_read_test<T, P>(path: P) -> T
where
    P: AsRef<Path>,
    T: for<'de> Deserialize<'de>,
{
    let reader = File::open(path).unwrap();
    let reader = BufReader::new(reader);
    let ret = quick_xml::de::from_reader::<_, T>(reader).unwrap();

    ret
}

pub fn serde_xml_rs_read_test<T, P>(path: P) -> T
where
    P: AsRef<Path>,
    T: for<'de> Deserialize<'de>,
{
    let reader = File::open(path).unwrap();
    let ret = serde_xml_rs::from_reader::<_, T>(reader).unwrap();

    ret
}

fn fmt_code(s: &str) -> String {
    let mut child = Command::new("rustfmt")
        .arg("--emit")
        .arg("stdout")
        .arg("--edition")
        .arg("2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn rustfmt: {}", e))
        .expect("Unable to spawn rustfmt command");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(s.as_bytes())
        .expect("Unable to write data to stdin");
    let output = child
        .wait_with_output()
        .expect("Unable to get formatted output");

    #[cfg(not(feature = "update-expectations"))]
    if !output.status.success() {
        panic!(
            "rustfmt failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        )
    }

    #[cfg(feature = "update-expectations")]
    if !output.status.success() {
        return s.into();
    }

    String::from_utf8(output.stdout).expect("Invalid output")
}

fn quick_xml_event_cmp(a: &Event<'_>, b: &Event<'_>) -> Option<bool> {
    match (a, b) {
        (Event::Start(a), Event::Start(b)) | (Event::Empty(a), Event::Empty(b)) => {
            let mut attribs_a = a.attributes().collect::<Result<Vec<_>, _>>().unwrap();
            let mut attribs_b = b.attributes().collect::<Result<Vec<_>, _>>().unwrap();

            attribs_a.sort_by(|a, b| (&a.key, &a.value).cmp(&(&b.key, &b.value)));
            attribs_b.sort_by(|a, b| (&a.key, &a.value).cmp(&(&b.key, &b.value)));

            Some(a.name() == b.name() && attribs_a == attribs_b)
        }
        (Event::End(a), Event::End(b)) => Some(a.name() == b.name()),
        (Event::Text(a), Event::Text(b)) => Some(a.trim_ascii() == b.trim_ascii()),
        (Event::Eof, Event::Eof) => None,
        (_, _) => Some(false),
    }
}
