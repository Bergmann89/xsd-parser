pub type Shiporder = ShiporderType;
#[derive(Debug, Clone)]
pub struct ShiporderType {
    pub orderid: String,
    pub orderperson: String,
    pub shipto: ShiporderShiptoType,
    pub item: Vec<ShiporderItemType>,
}
impl xsd_parser::quick_xml::WithSerializer for ShiporderType {
    type Serializer<'x> = quick_xml_serialize::ShiporderTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::ShiporderTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for ShiporderType {
    type Deserializer = quick_xml_deserialize::ShiporderTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct ShiporderShiptoType {
    pub name: String,
    pub address: String,
    pub city: String,
    pub country: String,
}
impl xsd_parser::quick_xml::WithSerializer for ShiporderShiptoType {
    type Serializer<'x> = quick_xml_serialize::ShiporderShiptoTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::ShiporderShiptoTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for ShiporderShiptoType {
    type Deserializer = quick_xml_deserialize::ShiporderShiptoTypeDeserializer;
}
#[derive(Debug, Clone)]
pub struct ShiporderItemType {
    pub title: String,
    pub note: Option<String>,
    pub quantity: usize,
    pub price: f64,
}
impl xsd_parser::quick_xml::WithSerializer for ShiporderItemType {
    type Serializer<'x> = quick_xml_serialize::ShiporderItemTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::ShiporderItemTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for ShiporderItemType {
    type Deserializer = quick_xml_deserialize::ShiporderItemTypeDeserializer;
}
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct ShiporderTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::ShiporderType,
        is_root: bool,
        state: ShiporderTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum ShiporderTypeSerializerState<'ser> {
        Init__,
        Orderperson(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Shipto(<ShiporderShiptoType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Item(
            xsd_parser::quick_xml::IterSerializer<'ser, Vec<ShiporderItemType>, ShiporderItemType>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::ShiporderType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("shiporder");
            Ok(Self {
                name,
                value,
                is_root,
                state: ShiporderTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for ShiporderTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::ShiporderType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = SerializeBytes::serialize_bytes(&value.orderid)? {
                    bytes.push_attribute(("orderid", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    ShiporderTypeSerializerState::Init__ => {
                        self.state = ShiporderTypeSerializerState::Orderperson(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.orderperson,
                                Some("orderperson"),
                                false,
                            ),
                        );
                        let bytes = BytesStart::new(self.name);
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Start(bytes))),
                            Err(error) => {
                                self.state = ShiporderTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    ShiporderTypeSerializerState::Orderperson(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => match WithSerializer::serializer(
                            &self.value.shipto,
                            Some("shipto"),
                            false,
                        ) {
                            Ok(serializer) => {
                                self.state = ShiporderTypeSerializerState::Shipto(serializer)
                            }
                            Err(error) => {
                                self.state = ShiporderTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        },
                    },
                    ShiporderTypeSerializerState::Shipto(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = ShiporderTypeSerializerState::Item(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.item,
                                    Some("item"),
                                    false,
                                ),
                            )
                        }
                    },
                    ShiporderTypeSerializerState::Item(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = ShiporderTypeSerializerState::End__,
                    },
                    ShiporderTypeSerializerState::End__ => {
                        self.state = ShiporderTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderTypeSerializerState::Done__ => return None,
                    ShiporderTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ShiporderShiptoTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::ShiporderShiptoType,
        is_root: bool,
        state: ShiporderShiptoTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum ShiporderShiptoTypeSerializerState<'ser> {
        Init__,
        Name(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Address(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        City(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Country(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderShiptoTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::ShiporderShiptoType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ShiporderShipto");
            Ok(Self {
                name,
                value,
                is_root,
                state: ShiporderShiptoTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for ShiporderShiptoTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    ShiporderShiptoTypeSerializerState::Init__ => {
                        self.state = ShiporderShiptoTypeSerializerState::Name(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.name,
                                Some("name"),
                                false,
                            ),
                        );
                        let bytes = BytesStart::new(self.name);
                        return Some(Ok(Event::Start(bytes)));
                    }
                    ShiporderShiptoTypeSerializerState::Name(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderShiptoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = ShiporderShiptoTypeSerializerState::Address(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.address,
                                    Some("address"),
                                    false,
                                ),
                            )
                        }
                    },
                    ShiporderShiptoTypeSerializerState::Address(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderShiptoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = ShiporderShiptoTypeSerializerState::City(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.city,
                                    Some("city"),
                                    false,
                                ),
                            )
                        }
                    },
                    ShiporderShiptoTypeSerializerState::City(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderShiptoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = ShiporderShiptoTypeSerializerState::Country(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.country,
                                    Some("country"),
                                    false,
                                ),
                            )
                        }
                    },
                    ShiporderShiptoTypeSerializerState::Country(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderShiptoTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = ShiporderShiptoTypeSerializerState::End__,
                    },
                    ShiporderShiptoTypeSerializerState::End__ => {
                        self.state = ShiporderShiptoTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderShiptoTypeSerializerState::Done__ => return None,
                    ShiporderShiptoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ShiporderItemTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::ShiporderItemType,
        is_root: bool,
        state: ShiporderItemTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum ShiporderItemTypeSerializerState<'ser> {
        Init__,
        Title(xsd_parser::quick_xml::ContentSerializer<'ser, String>),
        Note(xsd_parser::quick_xml::IterSerializer<'ser, Option<String>, String>),
        Quantity(xsd_parser::quick_xml::ContentSerializer<'ser, usize>),
        Price(xsd_parser::quick_xml::ContentSerializer<'ser, f64>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderItemTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::ShiporderItemType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("ShiporderItem");
            Ok(Self {
                name,
                value,
                is_root,
                state: ShiporderItemTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for ShiporderItemTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    ShiporderItemTypeSerializerState::Init__ => {
                        self.state = ShiporderItemTypeSerializerState::Title(
                            xsd_parser::quick_xml::ContentSerializer::new(
                                &self.value.title,
                                Some("title"),
                                false,
                            ),
                        );
                        let bytes = BytesStart::new(self.name);
                        return Some(Ok(Event::Start(bytes)));
                    }
                    ShiporderItemTypeSerializerState::Title(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderItemTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = ShiporderItemTypeSerializerState::Note(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.note,
                                    Some("note"),
                                    false,
                                ),
                            )
                        }
                    },
                    ShiporderItemTypeSerializerState::Note(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderItemTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = ShiporderItemTypeSerializerState::Quantity(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.quantity,
                                    Some("quantity"),
                                    false,
                                ),
                            )
                        }
                    },
                    ShiporderItemTypeSerializerState::Quantity(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderItemTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => {
                            self.state = ShiporderItemTypeSerializerState::Price(
                                xsd_parser::quick_xml::ContentSerializer::new(
                                    &self.value.price,
                                    Some("price"),
                                    false,
                                ),
                            )
                        }
                    },
                    ShiporderItemTypeSerializerState::Price(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ShiporderItemTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = ShiporderItemTypeSerializerState::End__,
                    },
                    ShiporderItemTypeSerializerState::End__ => {
                        self.state = ShiporderItemTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderItemTypeSerializerState::Done__ => return None,
                    ShiporderItemTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
    #[derive(Debug)]
    pub struct ShiporderTypeDeserializer {
        orderid: String,
        orderperson: Option<String>,
        shipto: Option<super::ShiporderShiptoType>,
        item: Vec<super::ShiporderItemType>,
        state: Box<ShiporderTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ShiporderTypeDeserializerState {
        Orderperson(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Shipto(
            Option<<ShiporderShiptoType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>,
        ),
        Item(Option<<ShiporderItemType as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl ShiporderTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            let mut orderid: Option<String> = None;
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
                state: Box::new(ShiporderTypeDeserializerState::Orderperson(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ShiporderType>
        for ShiporderTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ShiporderType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ShiporderType, Self>
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
                    core::mem::replace(&mut *self.state, ShiporderTypeDeserializerState::Done__),
                    event,
                ) {
                    (ShiporderTypeDeserializerState::Orderperson(Some(deserializer)), event) => {
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
                                *self.state =
                                    ShiporderTypeDeserializerState::Orderperson(deserializer);
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
                                ShiporderTypeDeserializerState::Orderperson(deserializer),
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
                        *self.state = ShiporderTypeDeserializerState::Orderperson(None);
                        event
                    }
                    (ShiporderTypeDeserializerState::Orderperson(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"orderperson" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.orderperson.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"orderperson",
                                    )))?;
                                }
                                self.orderperson = Some(data);
                            }
                            *self.state = ShiporderTypeDeserializerState::Orderperson(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderTypeDeserializerState::Shipto(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderTypeDeserializerState::Orderperson(None),
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
                            *self.state = ShiporderTypeDeserializerState::Shipto(None);
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
                            *self.state = ShiporderTypeDeserializerState::Orderperson(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderTypeDeserializerState::Shipto(Some(deserializer)), event) => {
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
                                *self.state = ShiporderTypeDeserializerState::Shipto(deserializer);
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
                                ShiporderTypeDeserializerState::Shipto(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.shipto.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"shipto",
                                )))?;
                            }
                            self.shipto = Some(data);
                        }
                        *self.state = ShiporderTypeDeserializerState::Shipto(None);
                        event
                    }
                    (ShiporderTypeDeserializerState::Shipto(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"shipto" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <ShiporderShiptoType as WithDeserializer>::Deserializer::init(
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
                            *self.state = ShiporderTypeDeserializerState::Shipto(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderTypeDeserializerState::Item(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderTypeDeserializerState::Shipto(None),
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
                            *self.state = ShiporderTypeDeserializerState::Item(None);
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
                            *self.state = ShiporderTypeDeserializerState::Shipto(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderTypeDeserializerState::Item(Some(deserializer)), event) => {
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
                                *self.state = ShiporderTypeDeserializerState::Item(deserializer);
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
                                .get_or_insert(ShiporderTypeDeserializerState::Item(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            self.item.push(data);
                        }
                        *self.state = ShiporderTypeDeserializerState::Item(None);
                        event
                    }
                    (ShiporderTypeDeserializerState::Item(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"item" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <ShiporderItemType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            if let Some(data) = data {
                                self.item.push(data);
                            }
                            *self.state = ShiporderTypeDeserializerState::Item(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderTypeDeserializerState::Item(None),
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
                            *self.state = ShiporderTypeDeserializerState::Done__;
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
                            *self.state = ShiporderTypeDeserializerState::Item(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderTypeDeserializerState::Done__, event) => {
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
        ) -> Result<super::ShiporderType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ShiporderType {
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
    pub struct ShiporderShiptoTypeDeserializer {
        name: Option<String>,
        address: Option<String>,
        city: Option<String>,
        country: Option<String>,
        state: Box<ShiporderShiptoTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ShiporderShiptoTypeDeserializerState {
        Name(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Address(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        City(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Country(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl ShiporderShiptoTypeDeserializer {
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
                state: Box::new(ShiporderShiptoTypeDeserializerState::Name(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ShiporderShiptoType>
        for ShiporderShiptoTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ShiporderShiptoType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ShiporderShiptoType, Self>
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
                    core::mem::replace(
                        &mut *self.state,
                        ShiporderShiptoTypeDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (ShiporderShiptoTypeDeserializerState::Name(Some(deserializer)), event) => {
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
                                *self.state =
                                    ShiporderShiptoTypeDeserializerState::Name(deserializer);
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
                                ShiporderShiptoTypeDeserializerState::Name(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.name.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"name")))?;
                            }
                            self.name = Some(data);
                        }
                        *self.state = ShiporderShiptoTypeDeserializerState::Name(None);
                        event
                    }
                    (ShiporderShiptoTypeDeserializerState::Name(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"name" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.name.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"name",
                                    )))?;
                                }
                                self.name = Some(data);
                            }
                            *self.state = ShiporderShiptoTypeDeserializerState::Name(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state =
                                        ShiporderShiptoTypeDeserializerState::Address(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderShiptoTypeDeserializerState::Name(None),
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
                            *self.state = ShiporderShiptoTypeDeserializerState::Address(None);
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
                            *self.state = ShiporderShiptoTypeDeserializerState::Name(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderShiptoTypeDeserializerState::Address(Some(deserializer)), event) => {
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
                                    ShiporderShiptoTypeDeserializerState::Address(deserializer);
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
                                ShiporderShiptoTypeDeserializerState::Address(deserializer),
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
                        *self.state = ShiporderShiptoTypeDeserializerState::Address(None);
                        event
                    }
                    (ShiporderShiptoTypeDeserializerState::Address(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"address" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.address.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"address",
                                    )))?;
                                }
                                self.address = Some(data);
                            }
                            *self.state =
                                ShiporderShiptoTypeDeserializerState::Address(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderShiptoTypeDeserializerState::City(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderShiptoTypeDeserializerState::Address(None),
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
                            *self.state = ShiporderShiptoTypeDeserializerState::City(None);
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
                            *self.state = ShiporderShiptoTypeDeserializerState::Address(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderShiptoTypeDeserializerState::City(Some(deserializer)), event) => {
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
                                *self.state =
                                    ShiporderShiptoTypeDeserializerState::City(deserializer);
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
                                ShiporderShiptoTypeDeserializerState::City(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.city.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"city")))?;
                            }
                            self.city = Some(data);
                        }
                        *self.state = ShiporderShiptoTypeDeserializerState::City(None);
                        event
                    }
                    (ShiporderShiptoTypeDeserializerState::City(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"city" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.city.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"city",
                                    )))?;
                                }
                                self.city = Some(data);
                            }
                            *self.state = ShiporderShiptoTypeDeserializerState::City(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state =
                                        ShiporderShiptoTypeDeserializerState::Country(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderShiptoTypeDeserializerState::City(None),
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
                            *self.state = ShiporderShiptoTypeDeserializerState::Country(None);
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
                            *self.state = ShiporderShiptoTypeDeserializerState::City(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderShiptoTypeDeserializerState::Country(Some(deserializer)), event) => {
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
                                    ShiporderShiptoTypeDeserializerState::Country(deserializer);
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
                                ShiporderShiptoTypeDeserializerState::Country(deserializer),
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
                        *self.state = ShiporderShiptoTypeDeserializerState::Country(None);
                        event
                    }
                    (ShiporderShiptoTypeDeserializerState::Country(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"country" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.country.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"country",
                                    )))?;
                                }
                                self.country = Some(data);
                            }
                            *self.state =
                                ShiporderShiptoTypeDeserializerState::Country(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderShiptoTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderShiptoTypeDeserializerState::Country(None),
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
                            *self.state = ShiporderShiptoTypeDeserializerState::Done__;
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
                            *self.state = ShiporderShiptoTypeDeserializerState::Country(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderShiptoTypeDeserializerState::Done__, event) => {
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
        ) -> Result<super::ShiporderShiptoType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ShiporderShiptoType {
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
    pub struct ShiporderItemTypeDeserializer {
        title: Option<String>,
        note: Option<String>,
        quantity: Option<usize>,
        price: Option<f64>,
        state: Box<ShiporderItemTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ShiporderItemTypeDeserializerState {
        Title(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Note(Option<<String as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Quantity(Option<<usize as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Price(Option<<f64 as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl ShiporderItemTypeDeserializer {
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
                state: Box::new(ShiporderItemTypeDeserializerState::Title(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ShiporderItemType>
        for ShiporderItemTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ShiporderItemType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ShiporderItemType, Self>
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
                    core::mem::replace(
                        &mut *self.state,
                        ShiporderItemTypeDeserializerState::Done__,
                    ),
                    event,
                ) {
                    (ShiporderItemTypeDeserializerState::Title(Some(deserializer)), event) => {
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
                                *self.state =
                                    ShiporderItemTypeDeserializerState::Title(deserializer);
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
                                ShiporderItemTypeDeserializerState::Title(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.title.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"title",
                                )))?;
                            }
                            self.title = Some(data);
                        }
                        *self.state = ShiporderItemTypeDeserializerState::Title(None);
                        event
                    }
                    (ShiporderItemTypeDeserializerState::Title(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"title" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.title.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"title",
                                    )))?;
                                }
                                self.title = Some(data);
                            }
                            *self.state = ShiporderItemTypeDeserializerState::Title(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderItemTypeDeserializerState::Note(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderItemTypeDeserializerState::Title(None),
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
                            *self.state = ShiporderItemTypeDeserializerState::Note(None);
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
                            *self.state = ShiporderItemTypeDeserializerState::Title(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderItemTypeDeserializerState::Note(Some(deserializer)), event) => {
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
                                *self.state =
                                    ShiporderItemTypeDeserializerState::Note(deserializer);
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
                                ShiporderItemTypeDeserializerState::Note(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.note.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"note")))?;
                            }
                            self.note = Some(data);
                        }
                        *self.state = ShiporderItemTypeDeserializerState::Note(None);
                        event
                    }
                    (ShiporderItemTypeDeserializerState::Note(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"note" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.note.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"note",
                                    )))?;
                                }
                                self.note = Some(data);
                            }
                            *self.state = ShiporderItemTypeDeserializerState::Note(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state =
                                        ShiporderItemTypeDeserializerState::Quantity(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderItemTypeDeserializerState::Note(None),
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
                            *self.state = ShiporderItemTypeDeserializerState::Quantity(None);
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
                            *self.state = ShiporderItemTypeDeserializerState::Note(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderItemTypeDeserializerState::Quantity(Some(deserializer)), event) => {
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
                                    ShiporderItemTypeDeserializerState::Quantity(deserializer);
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
                                ShiporderItemTypeDeserializerState::Quantity(deserializer),
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
                        *self.state = ShiporderItemTypeDeserializerState::Quantity(None);
                        event
                    }
                    (ShiporderItemTypeDeserializerState::Quantity(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"quantity" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <usize as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.quantity.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"quantity",
                                    )))?;
                                }
                                self.quantity = Some(data);
                            }
                            *self.state =
                                ShiporderItemTypeDeserializerState::Quantity(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderItemTypeDeserializerState::Price(None);
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderItemTypeDeserializerState::Quantity(None),
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
                            *self.state = ShiporderItemTypeDeserializerState::Price(None);
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
                            *self.state = ShiporderItemTypeDeserializerState::Quantity(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderItemTypeDeserializerState::Price(Some(deserializer)), event) => {
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
                                *self.state =
                                    ShiporderItemTypeDeserializerState::Price(deserializer);
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
                                ShiporderItemTypeDeserializerState::Price(deserializer),
                            );
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            if self.price.is_some() {
                                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                    b"price",
                                )))?;
                            }
                            self.price = Some(data);
                        }
                        *self.state = ShiporderItemTypeDeserializerState::Price(None);
                        event
                    }
                    (ShiporderItemTypeDeserializerState::Price(None), event) => match &event {
                        Event::Start(x) | Event::Empty(x)
                            if x.name().local_name().as_ref() == b"price" =>
                        {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <f64 as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                if self.price.is_some() {
                                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                                        b"price",
                                    )))?;
                                }
                                self.price = Some(data);
                            }
                            *self.state = ShiporderItemTypeDeserializerState::Price(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ShiporderItemTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback.get_or_insert(
                                            ShiporderItemTypeDeserializerState::Price(None),
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
                            *self.state = ShiporderItemTypeDeserializerState::Done__;
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
                            *self.state = ShiporderItemTypeDeserializerState::Price(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ShiporderItemTypeDeserializerState::Done__, event) => {
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
        ) -> Result<super::ShiporderItemType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ShiporderItemType {
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
