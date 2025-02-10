pub type List = ListType;
#[derive(Debug)]
pub struct ListType {
    pub base: Vec<Base>,
}
impl xsd_parser::quick_xml::WithSerializer for ListType {
    type Serializer<'x> = quick_xml_serialize::ListTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::ListTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for ListType {
    type Deserializer = quick_xml_deserialize::ListTypeDeserializer;
}
#[derive(Debug)]
pub struct Base(pub Box<dyn BaseTrait>);
pub trait BaseTrait:
    core::fmt::Debug + xsd_parser::quick_xml::WithBoxedSerializer + xsd_parser::AsAny
{
}
impl xsd_parser::quick_xml::WithSerializer for Base {
    type Serializer<'x> = xsd_parser::quick_xml::BoxedSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        let _name = name;
        self.0.serializer(None, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Base {
    type Deserializer = quick_xml_deserialize::BaseDeserializer;
}
#[derive(Debug)]
pub struct IntermediateType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
}
impl BaseTrait for IntermediateType {}
impl IntermediateTrait for IntermediateType {}
impl xsd_parser::quick_xml::WithSerializer for IntermediateType {
    type Serializer<'x> = quick_xml_serialize::IntermediateTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::IntermediateTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for IntermediateType {
    type Deserializer = quick_xml_deserialize::IntermediateTypeDeserializer;
}
#[derive(Debug)]
pub struct FinalType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
    pub final_value: Option<i32>,
}
impl BaseTrait for FinalType {}
impl IntermediateTrait for FinalType {}
impl xsd_parser::quick_xml::WithSerializer for FinalType {
    type Serializer<'x> = quick_xml_serialize::FinalTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        quick_xml_serialize::FinalTypeSerializer::new(self, name, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for FinalType {
    type Deserializer = quick_xml_deserialize::FinalTypeDeserializer;
}
#[derive(Debug)]
pub struct Intermediate(pub Box<dyn IntermediateTrait>);
pub trait IntermediateTrait: BaseTrait {}
impl xsd_parser::quick_xml::WithSerializer for Intermediate {
    type Serializer<'x> = xsd_parser::quick_xml::BoxedSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        let _name = name;
        self.0.serializer(None, is_root)
    }
}
impl xsd_parser::quick_xml::WithDeserializer for Intermediate {
    type Deserializer = quick_xml_deserialize::IntermediateDeserializer;
}
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct ListTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::ListType,
        is_root: bool,
        state: ListTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum ListTypeSerializerState<'ser> {
        Init__,
        Base(xsd_parser::quick_xml::IterSerializer<'ser, Vec<Base>, Base>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ListTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::ListType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("tns:list");
            Ok(Self {
                name,
                value,
                is_root,
                state: ListTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for ListTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            loop {
                match &mut self.state {
                    ListTypeSerializerState::Init__ => {
                        self.state = ListTypeSerializerState::Base(
                            xsd_parser::quick_xml::IterSerializer::new(
                                &self.value.base,
                                Some("tns:base"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Some(Ok(Event::Start(bytes)));
                    }
                    ListTypeSerializerState::Base(x) => match x.next() {
                        Some(Ok(event)) => return Some(Ok(event)),
                        Some(Err(error)) => {
                            self.state = ListTypeSerializerState::Done__;
                            return Some(Err(error));
                        }
                        None => self.state = ListTypeSerializerState::End__,
                    },
                    ListTypeSerializerState::End__ => {
                        self.state = ListTypeSerializerState::Done__;
                        return Some(Ok(Event::End(BytesEnd::new(self.name))));
                    }
                    ListTypeSerializerState::Done__ => return None,
                    ListTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct IntermediateTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::IntermediateType,
        is_root: bool,
        state: IntermediateTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum IntermediateTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> IntermediateTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::IntermediateType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("tns:intermediate");
            Ok(Self {
                name,
                value,
                is_root,
                state: IntermediateTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for IntermediateTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::IntermediateType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = value
                    .base_value
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("tns:baseValue", val));
                }
                if let Some(val) = value
                    .intermediate_value
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("tns:intermediateValue", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    IntermediateTypeSerializerState::Init__ => {
                        self.state = IntermediateTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Empty(bytes))),
                            Err(error) => {
                                self.state = IntermediateTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    IntermediateTypeSerializerState::Done__ => return None,
                    IntermediateTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FinalTypeSerializer<'ser> {
        name: &'ser str,
        value: &'ser super::FinalType,
        is_root: bool,
        state: FinalTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    enum FinalTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FinalTypeSerializer<'ser> {
        pub(super) fn new(
            value: &'ser super::FinalType,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self, xsd_parser::quick_xml::Error> {
            let name = name.unwrap_or("tns:final");
            Ok(Self {
                name,
                value,
                is_root,
                state: FinalTypeSerializerState::Init__,
            })
        }
    }
    impl<'ser> core::iter::Iterator for FinalTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            use xsd_parser::quick_xml::{
                BytesEnd, BytesStart, Error, Event, Serializer, WithSerializer,
            };
            fn build_attributes<'a>(
                mut bytes: BytesStart<'a>,
                value: &'a super::FinalType,
            ) -> Result<BytesStart<'a>, Error> {
                use xsd_parser::quick_xml::SerializeBytes;
                if let Some(val) = value
                    .base_value
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("tns:baseValue", val));
                }
                if let Some(val) = value
                    .intermediate_value
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("tns:intermediateValue", val));
                }
                if let Some(val) = value
                    .final_value
                    .as_ref()
                    .map(SerializeBytes::serialize_bytes)
                    .transpose()?
                    .flatten()
                {
                    bytes.push_attribute(("tns:finalValue", val));
                }
                Ok(bytes)
            }
            loop {
                match &mut self.state {
                    FinalTypeSerializerState::Init__ => {
                        self.state = FinalTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        match build_attributes(bytes, &self.value) {
                            Ok(bytes) => return Some(Ok(Event::Empty(bytes))),
                            Err(error) => {
                                self.state = FinalTypeSerializerState::Done__;
                                return Some(Err(error));
                            }
                        }
                    }
                    FinalTypeSerializerState::Done__ => return None,
                    FinalTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use super::*;
    #[derive(Debug)]
    pub struct ListTypeDeserializer {
        base: Vec<super::Base>,
        state: Box<ListTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ListTypeDeserializerState {
        Base(Option<<Base as xsd_parser::quick_xml::WithDeserializer>::Deserializer>),
        Done__,
    }
    impl ListTypeDeserializer {
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
                base: Vec::new(),
                state: Box::new(ListTypeDeserializerState::Base(None)),
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::ListType> for ListTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ListType, Self>
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
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::ListType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                Deserializer, DeserializerOutput, ErrorKind, Event, RawByteStr, WithDeserializer,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            let mut event = event;
            let mut allow_any_fallback = None;
            loop {
                event = match (
                    core::mem::replace(&mut *self.state, ListTypeDeserializerState::Done__),
                    event,
                ) {
                    (ListTypeDeserializerState::Base(Some(deserializer)), event) => {
                        let DeserializerOutput {
                            data,
                            deserializer,
                            event,
                            allow_any,
                        } = deserializer.next(reader, event)?;
                        if let Some(data) = data {
                            self.base.push(data);
                        }
                        let event = match event {
                            Some(event @ (Event::Start(_) | Event::Empty(_) | Event::End(_))) => {
                                event
                            }
                            event => {
                                *self.state = ListTypeDeserializerState::Base(deserializer);
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
                                .get_or_insert(ListTypeDeserializerState::Base(deserializer));
                        } else if let Some(deserializer) = deserializer {
                            let data = deserializer.finish(reader)?;
                            self.base.push(data);
                        }
                        *self.state = ListTypeDeserializerState::Base(None);
                        event
                    }
                    (ListTypeDeserializerState::Base(None), event) => match &event {
                        Event::Start(_) | Event::Empty(_) => {
                            let DeserializerOutput {
                                data,
                                deserializer,
                                event,
                                allow_any,
                            } = <Base as WithDeserializer>::Deserializer::init(reader, event)?;
                            if let Some(data) = data {
                                self.base.push(data);
                            }
                            *self.state = ListTypeDeserializerState::Base(deserializer);
                            match event {
                                Some(event @ (Event::Start(_) | Event::End(_))) => {
                                    *self.state = ListTypeDeserializerState::Done__;
                                    if allow_any {
                                        allow_any_fallback
                                            .get_or_insert(ListTypeDeserializerState::Base(None));
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
                            *self.state = ListTypeDeserializerState::Base(None);
                            return Ok(DeserializerOutput {
                                data: None,
                                deserializer: Some(self),
                                event: Some(event),
                                allow_any: false,
                            });
                        }
                    },
                    (ListTypeDeserializerState::Done__, event) => {
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
        fn finish<R>(self, _reader: &R) -> Result<super::ListType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::ListType { base: self.base })
        }
    }
    #[derive(Debug)]
    pub enum BaseDeserializer {
        Intermediate(
            <super::IntermediateType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        Final(<super::FinalType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Base> for BaseDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Base, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, Event, Namespace, QName, ResolveResult,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            let (Event::Start(b) | Event::Empty(b)) = &event else {
                return Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: None,
                    allow_any: false,
                });
            };
            let attrib = b
                .attributes()
                .find(|attrib| {
                    let Ok(attrib) = attrib else { return false };
                    let (resolve, name) = reader.resolve(attrib.key, true);
                    matches!(
                        resolve,
                        ResolveResult::Unbound
                            | ResolveResult::Bound(Namespace(
                                b"http://www.w3.org/2001/XMLSchema-instance"
                            ))
                    ) && name.as_ref() == b"type"
                })
                .transpose()?;
            let name = attrib
                .as_ref()
                .map(|attrib| QName(&attrib.value))
                .unwrap_or_else(|| b.name());
            if matches!(
                reader.resolve_local_name(name, NS_TNS),
                Some(b"intermediate")
            ) {
                let DeserializerOutput { data , deserializer , event , allow_any , } = < super :: IntermediateType as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                return Ok(DeserializerOutput {
                    data: data.map(|x| super::Base(Box::new(x))),
                    deserializer: deserializer.map(Self::Intermediate),
                    event,
                    allow_any,
                });
            }
            if matches!(reader.resolve_local_name(name, NS_TNS), Some(b"final")) {
                let DeserializerOutput { data , deserializer , event , allow_any , } = < super :: FinalType as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                return Ok(DeserializerOutput {
                    data: data.map(|x| super::Base(Box::new(x))),
                    deserializer: deserializer.map(Self::Final),
                    event,
                    allow_any,
                });
            }
            Ok(DeserializerOutput {
                data: None,
                deserializer: None,
                event: Some(event),
                allow_any: false,
            })
        }
        fn next<R>(
            self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Base, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::DeserializerOutput;
            match self {
                Self::Intermediate(x) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = x.next(reader, event)?;
                    let data = data.map(|x| Base(Box::new(x)));
                    let deserializer = deserializer.map(|x| Self::Intermediate(x));
                    Ok(DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    })
                }
                Self::Final(x) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = x.next(reader, event)?;
                    let data = data.map(|x| Base(Box::new(x)));
                    let deserializer = deserializer.map(|x| Self::Final(x));
                    Ok(DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish<R>(self, reader: &R) -> Result<super::Base, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            match self {
                Self::Intermediate(x) => Ok(Base(Box::new(x.finish(reader)?))),
                Self::Final(x) => Ok(Base(Box::new(x.finish(reader)?))),
            }
        }
    }
    #[derive(Debug)]
    pub struct IntermediateTypeDeserializer {
        base_value: Option<i32>,
        intermediate_value: Option<i32>,
    }
    impl IntermediateTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_TNS: &[u8] = b"http://example.com";
            let mut base_value: Option<i32> = None;
            let mut intermediate_value: Option<i32> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_TNS),
                    Some(b"baseValue")
                ) {
                    reader.read_attrib(&mut base_value, b"baseValue", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_TNS),
                    Some(b"intermediateValue")
                ) {
                    reader.read_attrib(
                        &mut intermediate_value,
                        b"intermediateValue",
                        &attrib.value,
                    )?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                base_value: base_value,
                intermediate_value: intermediate_value,
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::IntermediateType>
        for IntermediateTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::IntermediateType, Self>
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
            self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::IntermediateType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::End(_) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                _ => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn finish<R>(
            self,
            _reader: &R,
        ) -> Result<super::IntermediateType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::IntermediateType {
                base_value: self.base_value,
                intermediate_value: self.intermediate_value,
            })
        }
    }
    #[derive(Debug)]
    pub struct FinalTypeDeserializer {
        base_value: Option<i32>,
        intermediate_value: Option<i32>,
        final_value: Option<i32>,
    }
    impl FinalTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &xsd_parser::quick_xml::BytesStart<'_>,
        ) -> Result<Self, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            const NS_TNS: &[u8] = b"http://example.com";
            let mut base_value: Option<i32> = None;
            let mut intermediate_value: Option<i32> = None;
            let mut final_value: Option<i32> = None;
            for attrib in bytes_start.attributes() {
                let attrib = attrib?;
                if matches ! (attrib . key . prefix () , Some (x) if x . as_ref () == b"xmlns") {
                    continue;
                }
                if matches!(
                    reader.resolve_local_name(attrib.key, NS_TNS),
                    Some(b"baseValue")
                ) {
                    reader.read_attrib(&mut base_value, b"baseValue", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_TNS),
                    Some(b"intermediateValue")
                ) {
                    reader.read_attrib(
                        &mut intermediate_value,
                        b"intermediateValue",
                        &attrib.value,
                    )?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, NS_TNS),
                    Some(b"finalValue")
                ) {
                    reader.read_attrib(&mut final_value, b"finalValue", &attrib.value)?;
                } else {
                    reader.err(ErrorKind::UnexpectedAttribute(
                        xsd_parser::quick_xml::RawByteStr::from_slice(attrib.key.into_inner()),
                    ))?;
                }
            }
            Ok(Self {
                base_value: base_value,
                intermediate_value: intermediate_value,
                final_value: final_value,
            })
        }
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::FinalType> for FinalTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FinalType, Self>
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
            self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::FinalType, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{DeserializerOutput, Event};
            match event {
                Event::End(_) => {
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        data: Some(data),
                        deserializer: None,
                        event: None,
                        allow_any: false,
                    })
                }
                _ => Ok(DeserializerOutput {
                    data: None,
                    deserializer: Some(self),
                    event: Some(event),
                    allow_any: false,
                }),
            }
        }
        fn finish<R>(self, _reader: &R) -> Result<super::FinalType, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::ErrorKind;
            Ok(super::FinalType {
                base_value: self.base_value,
                intermediate_value: self.intermediate_value,
                final_value: self.final_value,
            })
        }
    }
    #[derive(Debug)]
    pub enum IntermediateDeserializer {
        Intermediate(
            <super::IntermediateType as xsd_parser::quick_xml::WithDeserializer>::Deserializer,
        ),
        Final(<super::FinalType as xsd_parser::quick_xml::WithDeserializer>::Deserializer),
    }
    impl<'de> xsd_parser::quick_xml::Deserializer<'de, super::Intermediate>
        for IntermediateDeserializer
    {
        fn init<R>(
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Intermediate, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::{
                DeserializerOutput, Event, Namespace, QName, ResolveResult,
            };
            const NS_TNS: &[u8] = b"http://example.com";
            let (Event::Start(b) | Event::Empty(b)) = &event else {
                return Ok(DeserializerOutput {
                    data: None,
                    deserializer: None,
                    event: None,
                    allow_any: false,
                });
            };
            let attrib = b
                .attributes()
                .find(|attrib| {
                    let Ok(attrib) = attrib else { return false };
                    let (resolve, name) = reader.resolve(attrib.key, true);
                    matches!(
                        resolve,
                        ResolveResult::Unbound
                            | ResolveResult::Bound(Namespace(
                                b"http://www.w3.org/2001/XMLSchema-instance"
                            ))
                    ) && name.as_ref() == b"type"
                })
                .transpose()?;
            let name = attrib
                .as_ref()
                .map(|attrib| QName(&attrib.value))
                .unwrap_or_else(|| b.name());
            if matches!(
                reader.resolve_local_name(name, NS_TNS),
                Some(b"intermediate")
            ) {
                let DeserializerOutput { data , deserializer , event , allow_any , } = < super :: IntermediateType as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                return Ok(DeserializerOutput {
                    data: data.map(|x| super::Intermediate(Box::new(x))),
                    deserializer: deserializer.map(Self::Intermediate),
                    event,
                    allow_any,
                });
            }
            if matches!(reader.resolve_local_name(name, NS_TNS), Some(b"final")) {
                let DeserializerOutput { data , deserializer , event , allow_any , } = < super :: FinalType as xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                return Ok(DeserializerOutput {
                    data: data.map(|x| super::Intermediate(Box::new(x))),
                    deserializer: deserializer.map(Self::Final),
                    event,
                    allow_any,
                });
            }
            Ok(DeserializerOutput {
                data: None,
                deserializer: None,
                event: Some(event),
                allow_any: false,
            })
        }
        fn next<R>(
            self,
            reader: &R,
            event: xsd_parser::quick_xml::Event<'de>,
        ) -> xsd_parser::quick_xml::DeserializerResult<'de, super::Intermediate, Self>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            use xsd_parser::quick_xml::DeserializerOutput;
            match self {
                Self::Intermediate(x) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = x.next(reader, event)?;
                    let data = data.map(|x| Intermediate(Box::new(x)));
                    let deserializer = deserializer.map(|x| Self::Intermediate(x));
                    Ok(DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    })
                }
                Self::Final(x) => {
                    let DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    } = x.next(reader, event)?;
                    let data = data.map(|x| Intermediate(Box::new(x)));
                    let deserializer = deserializer.map(|x| Self::Final(x));
                    Ok(DeserializerOutput {
                        data,
                        deserializer,
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish<R>(self, reader: &R) -> Result<super::Intermediate, xsd_parser::quick_xml::Error>
        where
            R: xsd_parser::quick_xml::XmlReader,
        {
            match self {
                Self::Intermediate(x) => Ok(Intermediate(Box::new(x.finish(reader)?))),
                Self::Final(x) => Ok(Intermediate(Box::new(x.finish(reader)?))),
            }
        }
    }
}
