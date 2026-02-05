#![allow(dead_code, unused, unused_imports, non_camel_case_types)]

mod netex;

use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::error::Error as StdError;

use anyhow::{Context, Error};
use xsd_parser_types::quick_xml::{DeserializeSync, ErrorReader, IoReader, XmlReader};

use self::netex::netex::ne_t_ex_publication::PublicationDelivery;

fn main() -> Result<(), Error> {
    load_example_xml::<PublicationDelivery>("./schema/examples/functions/calendar/NeTEx_Calendar_se_PA1.xml")?;

    Ok(())
}

fn load_example_xml<T>(path: &str) -> Result<(), Error>
where
    T: DeserializeSync<'static, ErrorReader<IoReader<BufReader<File>>>> + Debug,
    <T as DeserializeSync<'static, ErrorReader<IoReader<BufReader<File>>>>>::Error: StdError + Send + Sync + 'static,
{
    println!("Loading example XML from {path}");

    let input = File::open(path).context("Unable to open example XML for reading")?;
    let reader = BufReader::new(input);
    let mut reader = IoReader::new(reader).with_error_info();
    let obj = T::deserialize(&mut reader).context("Error while deserializing XML content")?;

    println!("{obj:?}");

    Ok(())
}
