#[derive(Debug, Clone)]
pub struct Shiporder {
    pub orderid: StringType,
    pub orderperson: StringType,
    pub shipto: ShiporderShipto,
    pub item: Vec<ShiporderItem>,
}
impl xsd_parser::quick_xml::WithSerializer for Shiporder {
    type Serializer<'x> = quick_xml_serialize::ShiporderSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for Shiporder {
    type Deserializer = quick_xml_deserialize::ShiporderDeserializer;
}
pub type StringType = String;
#[derive(Debug, Clone)]
pub struct ShiporderShipto {
    pub name: StringType,
    pub address: StringType,
    pub city: StringType,
    pub country: StringType,
}
impl xsd_parser::quick_xml::WithSerializer for ShiporderShipto {
    type Serializer<'x> = quick_xml_serialize::ShiporderShiptoSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for ShiporderShipto {
    type Deserializer = quick_xml_deserialize::ShiporderShiptoDeserializer;
}
#[derive(Debug, Clone)]
pub struct ShiporderItem {
    pub title: StringType,
    pub note: Option<StringType>,
    pub quantity: PositiveIntegerType,
    pub price: DecimalType,
}
impl xsd_parser::quick_xml::WithSerializer for ShiporderItem {
    type Serializer<'x> = quick_xml_serialize::ShiporderItemSerializer<'x>;
}
impl xsd_parser::quick_xml::WithDeserializer for ShiporderItem {
    type Deserializer = quick_xml_deserialize::ShiporderItemDeserializer;
}
pub type PositiveIntegerType = usize;
pub type DecimalType = f64;
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct ShiporderSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::Shiporder,
        is_root: bool,
        state: ShiporderSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum ShiporderSerializerState<'ser> {
        Init__,
        Orderperson(xsd_parser::quick_xml::ContentSerializer<'ser, StringType>),
        Shipto(<ShiporderShipto as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Item(xsd_parser::quick_xml::IterSerializer<'ser, Vec<ShiporderItem>, ShiporderItem>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> xsd_parser::quick_xml::Serializer<'ser, super::Shiporder> for ShiporderSerializer<'ser> {
        fn init(
            value: &'ser super::Shiporder,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("shiporder");
            Ok(Self {
                name,
                value,
                is_root,
                state: ShiporderSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for ShiporderSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, Serializer};
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::Shiporder,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = SerializeBytes::serialize_bytes(&value.orderid)? {
                    bytes.push_attribute(("orderid", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    ShiporderSerializerState::Init__ => {
                        match Serializer::init(&self.value.orderperson, Some("orderperson"), false)
                        {
                            Ok(serializer) => {
                                self.state = ShiporderSerializerState::Orderperson(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                        let bytes = BytesStart::new(self.name);
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = ShiporderSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    ShiporderSerializerState::Orderperson(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match Serializer::init(&self.value.shipto, Some("shipto"), false) {
                            Ok(serializer) => {
                                self.state = ShiporderSerializerState::Shipto(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    ShiporderSerializerState::Shipto(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match Serializer::init(&self.value.item, Some("item"), false) {
                            Ok(serializer) => {
                                self.state = ShiporderSerializerState::Item(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    ShiporderSerializerState::Item(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = ShiporderSerializerState::End__,
                    },
                    ShiporderSerializerState::End__ => {
                        self.state = ShiporderSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderSerializerState::Done__ => return None,
                    ShiporderSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ShiporderShiptoSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::ShiporderShipto,
        is_root: bool,
        state: ShiporderShiptoSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum ShiporderShiptoSerializerState<'ser> {
        Init__,
        Name(xsd_parser::quick_xml::ContentSerializer<'ser, StringType>),
        Address(xsd_parser::quick_xml::ContentSerializer<'ser, StringType>),
        City(xsd_parser::quick_xml::ContentSerializer<'ser, StringType>),
        Country(xsd_parser::quick_xml::ContentSerializer<'ser, StringType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> xsd_parser::quick_xml::Serializer<'ser, super::ShiporderShipto>
        for ShiporderShiptoSerializer<'ser>
    {
        fn init(
            value: &'ser super::ShiporderShipto,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ShiporderShipto");
            Ok(Self {
                name,
                value,
                is_root,
                state: ShiporderShiptoSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for ShiporderShiptoSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, Serializer};
            loop {
                match &mut self.state {
                    ShiporderShiptoSerializerState::Init__ => {
                        match Serializer::init(&self.value.name, Some("name"), false) {
                            Ok(serializer) => {
                                self.state = ShiporderShiptoSerializerState::Name(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderShiptoSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                        let bytes = BytesStart::new(self.name);
                        return Some(Ok(Event::Start(bytes)));
                    }
                    ShiporderShiptoSerializerState::Name(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderShiptoSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match Serializer::init(&self.value.address, Some("address"), false)
                        {
                            Ok(serializer) => {
                                self.state = ShiporderShiptoSerializerState::Address(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderShiptoSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    ShiporderShiptoSerializerState::Address(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderShiptoSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match Serializer::init(&self.value.city, Some("city"), false) {
                            Ok(serializer) => {
                                self.state = ShiporderShiptoSerializerState::City(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderShiptoSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    ShiporderShiptoSerializerState::City(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderShiptoSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match Serializer::init(&self.value.country, Some("country"), false)
                        {
                            Ok(serializer) => {
                                self.state = ShiporderShiptoSerializerState::Country(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderShiptoSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    ShiporderShiptoSerializerState::Country(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderShiptoSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = ShiporderShiptoSerializerState::End__,
                    },
                    ShiporderShiptoSerializerState::End__ => {
                        self.state = ShiporderShiptoSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderShiptoSerializerState::Done__ => return None,
                    ShiporderShiptoSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ShiporderItemSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::ShiporderItem,
        is_root: bool,
        state: ShiporderItemSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum ShiporderItemSerializerState<'ser> {
        Init__,
        Title(xsd_parser::quick_xml::ContentSerializer<'ser, StringType>),
        Note(xsd_parser::quick_xml::IterSerializer<'ser, Option<StringType>, StringType>),
        Quantity(xsd_parser::quick_xml::ContentSerializer<'ser, PositiveIntegerType>),
        Price(xsd_parser::quick_xml::ContentSerializer<'ser, DecimalType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> xsd_parser::quick_xml::Serializer<'ser, super::ShiporderItem>
        for ShiporderItemSerializer<'ser>
    {
        fn init(
            value: &'ser super::ShiporderItem,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ShiporderItem");
            Ok(Self {
                name,
                value,
                is_root,
                state: ShiporderItemSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for ShiporderItemSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, Serializer};
            loop {
                match &mut self.state {
                    ShiporderItemSerializerState::Init__ => {
                        match Serializer::init(&self.value.title, Some("title"), false) {
                            Ok(serializer) => {
                                self.state = ShiporderItemSerializerState::Title(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderItemSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                        let bytes = BytesStart::new(self.name);
                        return Some(Ok(Event::Start(bytes)));
                    }
                    ShiporderItemSerializerState::Title(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderItemSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match Serializer::init(&self.value.note, Some("note"), false) {
                            Ok(serializer) => {
                                self.state = ShiporderItemSerializerState::Note(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderItemSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    ShiporderItemSerializerState::Note(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderItemSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            match Serializer::init(&self.value.quantity, Some("quantity"), false) {
                                Ok(serializer) => {
                                    self.state = ShiporderItemSerializerState::Quantity(serializer)
                                }
                                Err(error) => {
                                    self.state = ShiporderItemSerializerState::Done__;
                                    return Some(Err(error));
                                }
                            }
                        }
                    },
                    ShiporderItemSerializerState::Quantity(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderItemSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match Serializer::init(&self.value.price, Some("price"), false) {
                            Ok(serializer) => {
                                self.state = ShiporderItemSerializerState::Price(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderItemSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    ShiporderItemSerializerState::Price(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderItemSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = ShiporderItemSerializerState::End__,
                    },
                    ShiporderItemSerializerState::End__ => {
                        self.state = ShiporderItemSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderItemSerializerState::Done__ => return None,
                    ShiporderItemSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
    #[derive(Debug)]
    pub struct ShiporderDeserializer {
        orderid: super::StringType,
        orderperson: Option<super::StringType>,
        shipto: Option<super::ShiporderShipto>,
        item: Vec<super::ShiporderItem>,
        state: Box<ShiporderDeserializerState>,
    }
    #[derive(Debug)]
    enum ShiporderDeserializerState {
        Orderperson(Option<<StringType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Shipto(Option<<ShiporderShipto as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Item(Option<<ShiporderItem as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl ShiporderDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            let mut orderid: Option<StringType> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if attrib.key.local_name().as_ref() == b"orderid" {
                    reader.read_attrib(&mut orderid, b"orderid", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                orderid: orderid.ok_or(ErrorKind::MissingAttribute("orderid".into()))?,
                orderperson: None,
                shipto: None,
                item: Vec::new(),
                state: Box::new(ShiporderDeserializerState::Orderperson(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Shiporder> for ShiporderDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Shiporder, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Shiporder, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, ShiporderDeserializerState::Done__),
                    event,
                ) {
                    (ShiporderDeserializerState::Orderperson(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.orderperson.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"orderperson",
                                )))?;
                            }
                            self.orderperson = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ShiporderDeserializerState::Orderperson(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                ShiporderDeserializerState::Orderperson(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.orderperson.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"orderperson",
                                )))?;
                            }
                            self.orderperson = Some(data);
                        }
                        *self.state = ShiporderDeserializerState::Orderperson(None);
                        event
                    }
                    (ShiporderDeserializerState::Orderperson(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"orderperson" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <StringType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.orderperson.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"orderperson",
                                    )))?;
                                }
                                self.orderperson = Some(data);
                            }
                            *self.state = ShiporderDeserializerState::Orderperson(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderDeserializerState::Shipto(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderDeserializerState::Orderperson(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderDeserializerState::Shipto(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderDeserializerState::Orderperson(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderDeserializerState::Shipto(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.shipto.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"shipto",
                                )))?;
                            }
                            self.shipto = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ShiporderDeserializerState::Shipto(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback
                                .get_or_insert(ShiporderDeserializerState::Shipto(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.shipto.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"shipto",
                                )))?;
                            }
                            self.shipto = Some(data);
                        }
                        *self.state = ShiporderDeserializerState::Shipto(None);
                        event
                    }
                    (ShiporderDeserializerState::Shipto(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"shipto" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <ShiporderShipto as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.shipto.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"shipto",
                                    )))?;
                                }
                                self.shipto = Some(data);
                            }
                            *self.state = ShiporderDeserializerState::Shipto(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderDeserializerState::Item(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderDeserializerState::Shipto(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderDeserializerState::Item(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderDeserializerState::Shipto(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderDeserializerState::Item(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            self.item.push(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ShiporderDeserializerState::Item(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback
                                .get_or_insert(ShiporderDeserializerState::Item(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            self.item.push(data);
                        }
                        *self.state = ShiporderDeserializerState::Item(None);
                        event
                    }
                    (ShiporderDeserializerState::Item(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"item" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <ShiporderItem as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                self.item.push(data);
                            }
                            *self.state = ShiporderDeserializerState::Item(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(ShiporderDeserializerState::Item(None));
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderDeserializerState::Done__;
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderDeserializerState::Item(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::Shiporder, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::Shiporder {
                orderid: self.orderid,
                orderperson: self
                    .orderperson
                    .ok_or_else(|| ErrorKind::MissingElement("orderperson".into()))?,
                shipto: self
                    .shipto
                    .ok_or_else(|| ErrorKind::MissingElement("shipto".into()))?,
                item: self.item,
            })
        }
    }
    #[derive(Debug)]
    pub struct ShiporderShiptoDeserializer {
        name: Option<super::StringType>,
        address: Option<super::StringType>,
        city: Option<super::StringType>,
        country: Option<super::StringType>,
        state: Box<ShiporderShiptoDeserializerState>,
    }
    #[derive(Debug)]
    enum ShiporderShiptoDeserializerState {
        Name(Option<<StringType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Address(Option<<StringType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        City(Option<<StringType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Country(Option<<StringType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl ShiporderShiptoDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                name: None,
                address: None,
                city: None,
                country: None,
                state: Box::new(ShiporderShiptoDeserializerState::Name(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ShiporderShipto>
        for ShiporderShiptoDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ShiporderShipto, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ShiporderShipto, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, ShiporderShiptoDeserializerState::Done__),
                    event,
                ) {
                    (ShiporderShiptoDeserializerState::Name(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.name.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"name")))?;
                            }
                            self.name = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ShiporderShiptoDeserializerState::Name(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                ShiporderShiptoDeserializerState::Name(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.name.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"name")))?;
                            }
                            self.name = Some(data);
                        }
                        *self.state = ShiporderShiptoDeserializerState::Name(None);
                        event
                    }
                    (ShiporderShiptoDeserializerState::Name(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"name" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <StringType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.name.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"name",
                                    )))?;
                                }
                                self.name = Some(data);
                            }
                            *self.state = ShiporderShiptoDeserializerState::Name(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderShiptoDeserializerState::Address(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderShiptoDeserializerState::Name(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderShiptoDeserializerState::Address(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderShiptoDeserializerState::Name(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderShiptoDeserializerState::Address(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.address.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"address",
                                )))?;
                            }
                            self.address = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    ShiporderShiptoDeserializerState::Address(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                ShiporderShiptoDeserializerState::Address(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.address.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"address",
                                )))?;
                            }
                            self.address = Some(data);
                        }
                        *self.state = ShiporderShiptoDeserializerState::Address(None);
                        event
                    }
                    (ShiporderShiptoDeserializerState::Address(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"address" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <StringType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.address.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"address",
                                    )))?;
                                }
                                self.address = Some(data);
                            }
                            *self.state = ShiporderShiptoDeserializerState::Address(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderShiptoDeserializerState::City(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderShiptoDeserializerState::Address(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderShiptoDeserializerState::City(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderShiptoDeserializerState::Address(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderShiptoDeserializerState::City(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.city.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"city")))?;
                            }
                            self.city = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ShiporderShiptoDeserializerState::City(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                ShiporderShiptoDeserializerState::City(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.city.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"city")))?;
                            }
                            self.city = Some(data);
                        }
                        *self.state = ShiporderShiptoDeserializerState::City(None);
                        event
                    }
                    (ShiporderShiptoDeserializerState::City(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"city" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <StringType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.city.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"city",
                                    )))?;
                                }
                                self.city = Some(data);
                            }
                            *self.state = ShiporderShiptoDeserializerState::City(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderShiptoDeserializerState::Country(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderShiptoDeserializerState::City(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderShiptoDeserializerState::Country(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderShiptoDeserializerState::City(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderShiptoDeserializerState::Country(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.country.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"country",
                                )))?;
                            }
                            self.country = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    ShiporderShiptoDeserializerState::Country(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                ShiporderShiptoDeserializerState::Country(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.country.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"country",
                                )))?;
                            }
                            self.country = Some(data);
                        }
                        *self.state = ShiporderShiptoDeserializerState::Country(None);
                        event
                    }
                    (ShiporderShiptoDeserializerState::Country(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"country" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <StringType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.country.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"country",
                                    )))?;
                                }
                                self.country = Some(data);
                            }
                            *self.state = ShiporderShiptoDeserializerState::Country(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderShiptoDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderShiptoDeserializerState::Country(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderShiptoDeserializerState::Done__;
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderShiptoDeserializerState::Country(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderShiptoDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::ShiporderShipto, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ShiporderShipto {
                name: self
                    .name
                    .ok_or_else(|| ErrorKind::MissingElement("name".into()))?,
                address: self
                    .address
                    .ok_or_else(|| ErrorKind::MissingElement("address".into()))?,
                city: self
                    .city
                    .ok_or_else(|| ErrorKind::MissingElement("city".into()))?,
                country: self
                    .country
                    .ok_or_else(|| ErrorKind::MissingElement("country".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ShiporderItemDeserializer {
        title: Option<super::StringType>,
        note: Option<super::StringType>,
        quantity: Option<super::PositiveIntegerType>,
        price: Option<super::DecimalType>,
        state: Box<ShiporderItemDeserializerState>,
    }
    #[derive(Debug)]
    enum ShiporderItemDeserializerState {
        Title(Option<<StringType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Note(Option<<StringType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Quantity(
            Option<<PositiveIntegerType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        Price(Option<<DecimalType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl ShiporderItemDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                reader.err(ErrorKind::UnexpectedAttribute(
                    xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                ))?;
            }
            Ok(Self {
                title: None,
                note: None,
                quantity: None,
                price: None,
                state: Box::new(ShiporderItemDeserializerState::Title(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ShiporderItem>
        for ShiporderItemDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ShiporderItem, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::Start(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    Ok(DeserializerOutput {
                        data: None,
                        deserializer: Some(deserializer),
                        event: None,
                        allow_any: false,
                    })
                }
                Event::Empty(start) => {
                    let deserializer = Self::from_bytes_start(reader, &start)?;
                    let data = deserializer.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                event => Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ShiporderItem, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, ShiporderItemDeserializerState::Done__),
                    event,
                ) {
                    (ShiporderItemDeserializerState::Title(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.title.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"title",
                                )))?;
                            }
                            self.title = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ShiporderItemDeserializerState::Title(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback
                                .get_or_insert(ShiporderItemDeserializerState::Title(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.title.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"title",
                                )))?;
                            }
                            self.title = Some(data);
                        }
                        *self.state = ShiporderItemDeserializerState::Title(None);
                        event
                    }
                    (ShiporderItemDeserializerState::Title(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"title" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <StringType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.title.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"title",
                                    )))?;
                                }
                                self.title = Some(data);
                            }
                            *self.state = ShiporderItemDeserializerState::Title(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderItemDeserializerState::Note(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderItemDeserializerState::Title(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderItemDeserializerState::Note(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderItemDeserializerState::Title(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderItemDeserializerState::Note(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.note.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"note")))?;
                            }
                            self.note = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ShiporderItemDeserializerState::Note(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback
                                .get_or_insert(ShiporderItemDeserializerState::Note(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.note.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"note")))?;
                            }
                            self.note = Some(data);
                        }
                        *self.state = ShiporderItemDeserializerState::Note(None);
                        event
                    }
                    (ShiporderItemDeserializerState::Note(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"note" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <StringType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.note.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"note",
                                    )))?;
                                }
                                self.note = Some(data);
                            }
                            *self.state = ShiporderItemDeserializerState::Note(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderItemDeserializerState::Quantity(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderItemDeserializerState::Note(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderItemDeserializerState::Quantity(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderItemDeserializerState::Note(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderItemDeserializerState::Quantity(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.quantity.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"quantity",
                                )))?;
                            }
                            self.quantity = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state =
                                    ShiporderItemDeserializerState::Quantity(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback.get_or_insert(
                                ShiporderItemDeserializerState::Quantity(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.quantity.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"quantity",
                                )))?;
                            }
                            self.quantity = Some(data);
                        }
                        *self.state = ShiporderItemDeserializerState::Quantity(None);
                        event
                    }
                    (ShiporderItemDeserializerState::Quantity(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"quantity" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <PositiveIntegerType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.quantity.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"quantity",
                                    )))?;
                                }
                                self.quantity = Some(data);
                            }
                            *self.state = ShiporderItemDeserializerState::Quantity(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderItemDeserializerState::Price(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderItemDeserializerState::Quantity(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderItemDeserializerState::Price(None);
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderItemDeserializerState::Quantity(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderItemDeserializerState::Price(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            if self.price.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"price",
                                )))?;
                            }
                            self.price = Some(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ShiporderItemDeserializerState::Price(deserializer);
                                return Ok(DeserializerOutput {
                                    data: None,
                                    deserializer: Some(self),
                                    event: event,
                                    allow_any: false,
                                });
                            }
                        };
                        if allow_any {
                            allow_any_fallback
                                .get_or_insert(ShiporderItemDeserializerState::Price(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.price.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"price",
                                )))?;
                            }
                            self.price = Some(data);
                        }
                        *self.state = ShiporderItemDeserializerState::Price(None);
                        event
                    }
                    (ShiporderItemDeserializerState::Price(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"price" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <DecimalType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                if self.price.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"price",
                                    )))?;
                                }
                                self.price = Some(data);
                            }
                            *self.state = ShiporderItemDeserializerState::Price(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderItemDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderItemDeserializerState::Price(None),
                                        );
                                    }
                                    event
                                }
                                event @ (None | Some(_)) => {
                                    return Ok(DeserializerOutput {
                                        data: None,
                                        deserializer: Some(self),
                                        event,
                                        allow_any: false,
                                    })
                                }
                            }
                        }
                        Event::Start(_) | Event::Empty(_) => {
                            *self.state = ShiporderItemDeserializerState::Done__;
                            event
                        }
                        Event::End(_) => {
                            let data = self.finish(reader)?;
                            return Ok(DeserializerOutput {
                                data: Some(data),
                                deserializer: None,
                                event: None,
                                allow_any: false,
                            });
                        }
                        _ => {
                            *self.state = ShiporderItemDeserializerState::Price(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderItemDeserializerState::Done__, event) => {
                        let allow_any = if let Some(fallback) = allow_any_fallback {
                            *self.state = fallback;
                            true
                        } else {
                            false
                        };
                        return Ok(DeserializerOutput {
                            data: None,
                            deserializer: Some(self),
                            event: Some(event),
                            allow_any,
                        });
                    }
                }
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::ShiporderItem, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ShiporderItem {
                title: self
                    .title
                    .ok_or_else(|| ErrorKind::MissingElement("title".into()))?,
                note: self.note,
                quantity: self
                    .quantity
                    .ok_or_else(|| ErrorKind::MissingElement("quantity".into()))?,
                price: self
                    .price
                    .ok_or_else(|| ErrorKind::MissingElement("price".into()))?,
            })
        }
    }
}
