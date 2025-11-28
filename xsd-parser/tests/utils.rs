#![allow(dead_code, missing_docs, unreachable_pub)]

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use quick_xml::{escape::unescape, events::BytesText, Reader};
use serde::{Deserialize, Serialize};

use xsd_parser::{
    config::{Config, Generate, GeneratorFlags, IdentTriple, OptimizerFlags, Schema},
    generate,
};
use xsd_parser_types::quick_xml::{
    DeserializeAsync, DeserializeSync, ErrorReader, Event, IoReader, Serializer, WithSerializer,
    Writer, XmlReader,
};

pub trait ConfigEx {
    fn test_default() -> Self;
}

impl ConfigEx for Config {
    fn test_default() -> Self {
        let mut config = Config::default();

        config.optimizer.flags |= OptimizerFlags::RESOLVE_TYPEDEFS;
        config.optimizer.flags |= OptimizerFlags::USE_UNRESTRICTED_BASE_TYPE_SIMPLE;

        config.generator.generate = Generate::Named;
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
    let input_xsd = input_xsd.as_ref().canonicalize().unwrap();
    let expected_rs = expected_rs.as_ref();

    // For debugging purposes enable the following lines

    // let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into());
    // let target_dir = Path::new(&target_dir);
    // let relative_xsd = input_xsd.strip_prefix(cargo_dir).unwrap();
    // let debug_dir = target_dir
    //     .join("logs")
    //     .join(relative_xsd)
    //     .join(expected_rs.file_name().unwrap());
    // std::fs::create_dir_all(&debug_dir).unwrap();

    // config.parser.debug_output = Some(debug_dir.join("schemas.log"));
    // config.interpreter.debug_output = Some(debug_dir.join("interpreter.log"));
    // config.optimizer.debug_output = Some(debug_dir.join("optimizer.log"));

    config.parser.schemas.push(Schema::File(input_xsd));

    let actual = generate(config).unwrap();
    let actual = actual.to_string();

    generate_test_validate(actual, expected_rs);
}

pub fn generate_test_validate<P1, P2>(actual: P1, expected_rs: P2)
where
    P1: AsRef<str>,
    P2: AsRef<Path>,
{
    let actual = actual.as_ref();
    let actual = fmt_code(actual);

    #[cfg(not(feature = "update-expectations"))]
    {
        use std::fs::read_to_string;
        use std::str::FromStr;

        use proc_macro2::TokenStream;
        use text_diff::print_diff;

        let expected = read_to_string(expected_rs).unwrap();
        let expected = TokenStream::from_str(&expected).unwrap();
        let expected = fmt_code(&expected.to_string());

        if expected != actual {
            println!("=== expected:\n{expected}");
            println!("=== actual:\n{actual}");
            println!("=== diff:\n");

            print_diff(&expected, &actual, "\n");

            panic!("Code does not match!");
        }
    }

    #[cfg(feature = "update-expectations")]
    {
        std::fs::write(expected_rs, actual).unwrap();
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
    config.generator.flags -= GeneratorFlags::USE_MODULES;
    config.renderer.derive = Some(Vec::new());

    let input_xsd = input_xsd.as_ref();

    generate_test(input_xsd, expected_0, config.clone());

    config.optimizer.flags |= flags;

    generate_test(input_xsd, expected_1, config.clone());
}

pub fn quick_xml_read_test<T, P>(path: P) -> T
where
    P: AsRef<Path>,
    T: DeserializeSync<'static, ErrorReader<IoReader<BufReader<File>>>>,
    T::Error: Debug,
{
    quick_xml_read_test_result::<T, _>(path).unwrap()
}

#[allow(clippy::missing_errors_doc)]
pub fn quick_xml_read_test_result<T, P>(path: P) -> Result<T, T::Error>
where
    P: AsRef<Path>,
    T: DeserializeSync<'static, ErrorReader<IoReader<BufReader<File>>>>,
    T::Error: Debug,
{
    let reader = File::open(path).unwrap();
    let reader = BufReader::new(reader);
    let mut reader = IoReader::new(reader).with_error_info();

    T::deserialize(&mut reader)
}

pub async fn quick_xml_read_test_async<T, P>(path: P) -> T
where
    P: AsRef<Path>,
    T: DeserializeAsync<'static, ErrorReader<IoReader<tokio::io::BufReader<tokio::fs::File>>>>,
    T::Error: Debug,
{
    let reader = tokio::fs::File::open(path).await.unwrap();
    let reader = tokio::io::BufReader::new(reader);
    let mut reader = IoReader::new(reader).with_error_info();

    T::deserialize_async(&mut reader).await.unwrap()
}

pub fn quick_xml_write_test<T, P>(value: &T, root: &str, path: P)
where
    P: AsRef<Path>,
    T: WithSerializer,
{
    let mut content = Vec::new();
    let mut writer = Writer::new(&mut content);

    let mut actual = FilteredIter::new(
        value
            .serializer(Some(root), true)
            .unwrap()
            .into_iter()
            .map(|ret| ret.unwrap()),
    );
    let mut expected = FilteredIter::new(IterReader::from_file(path));

    let (actual, expected) = loop {
        let actual = actual.next().unwrap_or(Event::Eof);
        let expected = expected.next().unwrap_or(Event::Eof);

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

    quick_xml::de::from_reader::<_, T>(reader).unwrap()
}

pub fn serde_xml_rs_read_test<T, P>(path: P) -> T
where
    P: AsRef<Path>,
    T: for<'de> Deserialize<'de>,
{
    let reader = File::open(path).unwrap();

    serde_xml_rs::from_reader::<T, _>(reader).unwrap()
}

pub fn serde_xml_rs_v7_read_test<T, P>(path: P) -> T
where
    P: AsRef<Path>,
    T: for<'de> Deserialize<'de>,
{
    let reader = File::open(path).unwrap();

    serde_xml_rs_v7::from_reader::<_, T>(reader).unwrap()
}

pub fn serde_xml_rs_write_test<T, P>(value: &T, path: P)
where
    P: AsRef<Path>,
    T: Serialize,
{
    let mut content = Vec::new();
    serde_xml_rs::to_writer(&mut content, value).unwrap();

    serde_xml_rs_compare(&content, path);
}

pub fn serde_xml_rs_v7_write_test<T, P>(value: &T, path: P)
where
    P: AsRef<Path>,
    T: Serialize,
{
    let mut content = Vec::new();
    serde_xml_rs_v7::to_writer(&mut content, value).unwrap();

    serde_xml_rs_compare(&content, path);
}

fn serde_xml_rs_compare<P>(actual: &[u8], expected: P)
where
    P: AsRef<Path>,
{
    let content = std::str::from_utf8(actual).unwrap();

    let mut actual = FilteredIter::new(IterReader::from_str(content));
    let mut expected = FilteredIter::new(IterReader::from_file(expected));

    let (actual, expected) = loop {
        let expected = expected.next().unwrap_or(Event::Eof);
        let actual = actual.next().unwrap_or(Event::Eof);

        match quick_xml_event_cmp(&actual, &expected) {
            None => return,
            Some(true) => (),
            Some(false) => break (actual, expected),
        }
    };

    println!("=== actual: {actual:?}");
    println!("=== expected: {expected:?}");
    println!("=== content: {content}");

    panic!("Unexpected event")
}

/* Misc */

struct IterReader<R> {
    reader: Option<Reader<R>>,
    buffer: Vec<u8>,
}

impl IterReader<BufReader<File>> {
    fn from_file<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            reader: Some(Reader::from_file(path).unwrap()),
            buffer: Vec::new(),
        }
    }
}

impl<'a> IterReader<&'a [u8]> {
    fn from_str(s: &'a str) -> Self {
        Self {
            reader: Some(Reader::from_str(s)),
            buffer: Vec::new(),
        }
    }
}

impl<R> Iterator for IterReader<R>
where
    R: BufRead,
{
    type Item = Event<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        let reader = self.reader.as_mut()?;
        let event = reader.read_event_into(&mut self.buffer).unwrap();

        if matches!(&event, Event::Eof) {
            self.reader = None;
        }

        Some(event.into_owned())
    }
}

struct FilteredIter<'a, I>
where
    I: IntoIterator<Item = Event<'a>>,
{
    iter: I::IntoIter,
    pending: Option<Event<'static>>,
}

impl<'a, I> FilteredIter<'a, I>
where
    I: IntoIterator<Item = Event<'a>>,
{
    fn new(iter: I) -> Self {
        Self {
            iter: iter.into_iter(),
            pending: None,
        }
    }
}

impl<'a, I> Iterator for FilteredIter<'a, I>
where
    I: IntoIterator<Item = Event<'a>>,
{
    type Item = Event<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut text = String::new();

        loop {
            match self.pending.take().or_else(|| self.iter.next()) {
                Some(Event::Decl(_) | Event::Comment(_)) => (),
                Some(Event::Text(x)) => {
                    let x = x.decode().unwrap();

                    text.push_str(&x);
                }
                Some(Event::GeneralRef(x)) => {
                    use std::fmt::Write;

                    let x = x.decode().unwrap();

                    write!(text, "&{x};").unwrap();
                }
                Some(event) if text.trim().is_empty() => return Some(event.into_owned()),
                Some(event) => {
                    self.pending = Some(event.into_owned());

                    return Some(Event::Text(BytesText::from_escaped(text)));
                }
                None if !text.trim().is_empty() => {
                    return Some(Event::Text(BytesText::from_escaped(text)))
                }
                None => return None,
            }
        }
    }
}

fn fmt_code(s: &str) -> String {
    let mut child = Command::new("rustfmt")
        .args(["--emit", "stdout"])
        .args(["--edition", "2021"])
        .args(["--config", "normalize_doc_attributes=true"])
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
        (Event::Text(a), Event::Text(b)) => Some(quick_xml_bytes_text_cmp(a, b)),
        (Event::GeneralRef(a), Event::GeneralRef(b)) => Some(a.trim_ascii() == b.trim_ascii()),
        (Event::Eof, Event::Eof) => None,
        (_, _) => Some(false),
    }
}

fn quick_xml_bytes_text_cmp(a: &BytesText<'_>, b: &BytesText<'_>) -> bool {
    let a = a.decode().unwrap();
    let b = b.decode().unwrap();

    let a = unescape(&a).unwrap();
    let b = unescape(&b).unwrap();

    a.trim() == b.trim()
}
