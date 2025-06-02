use num::{BigInt, BigUint};
use xsd_parser::{
    quick_xml::{Error, WithDeserializer, WithSerializer},
    schema::Namespace,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
#[derive(Debug)]
pub struct DocumentType {
    pub name: String,
    pub count: Option<u32>,
    pub content: Vec<DocumentTypeContent>,
}
#[derive(Debug)]
pub struct DocumentTypeContent {
    pub info: String,
}
impl WithDeserializer for DocumentType {
    type Deserializer = quick_xml_deserialize::DocumentTypeDeserializer;
}
impl WithDeserializer for DocumentTypeContent {
    type Deserializer = quick_xml_deserialize::DocumentTypeContentDeserializer;
}
impl WithSerializer for DocumentType {
    type Serializer<'x> = quick_xml_serialize::DocumentTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DocumentTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DocumentTypeSerializerState::Init__),
            name: name.unwrap_or("Document"),
            is_root,
        })
    }
}
impl WithSerializer for DocumentTypeContent {
    type Serializer<'x> = quick_xml_serialize::DocumentTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::DocumentTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DocumentTypeContentSerializerState::Init__),
        })
    }
}
pub type Document = DocumentType;
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct DocumentTypeDeserializer {
        name: String,
        count: Option<u32>,
        content: Vec<super::DocumentTypeContent>,
        state: Box<DocumentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::DocumentTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DocumentTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut name: Option<String> = None;
            let mut count: Option<u32> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"name" {
                    reader.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"count" {
                    reader.read_attrib(&mut count, b"count", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                name: name
                    .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
                count: count,
                content: Vec::new(),
                state: Box::new(DocumentTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DocumentTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let DocumentTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::DocumentTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DocumentTypeContent>,
            fallback: &mut Option<DocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(DocumentTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = DocumentTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DocumentTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DocumentTypeDeserializerState::Content__(
                                deserializer,
                            ));
                            *self.state = DocumentTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DocumentType> for DocumentTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DocumentType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentType>
        where
            R: DeserializeReader,
        {
            use DocumentTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::DocumentTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DocumentType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, DocumentTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::DocumentType {
                name: self.name,
                count: self.count,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentTypeContentDeserializer {
        info: Option<String>,
        state: Box<DocumentTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentTypeContentDeserializerState {
        Init__,
        Info(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DocumentTypeContentDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DocumentTypeContentDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DocumentTypeContentDeserializerState as S;
            match state {
                S::Info(Some(deserializer)) => self.store_info(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_info(&mut self, value: String) -> Result<(), Error> {
            if self.info.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"info")))?;
            }
            self.info = Some(value);
            Ok(())
        }
        fn handle_info<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DocumentTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.info.is_some() {
                    fallback.get_or_insert(DocumentTypeContentDeserializerState::Info(None));
                    *self.state = DocumentTypeContentDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DocumentTypeContentDeserializerState::Info(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_info(data)?;
                    *self.state = DocumentTypeContentDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DocumentTypeContentDeserializerState::Info(
                                Some(deserializer),
                            ));
                            *self.state = DocumentTypeContentDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                DocumentTypeContentDeserializerState::Info(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DocumentTypeContent> for DocumentTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                info: None,
                state: Box::new(DocumentTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, DocumentTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentTypeContent>
        where
            R: DeserializeReader,
        {
            use DocumentTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Info(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_info(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, event @ Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = DocumentTypeContentDeserializerState::Info(None);
                        event
                    }
                    (S::Info(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"info") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_info(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DocumentTypeContent, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DocumentTypeContentDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DocumentTypeContent {
                info: self
                    .info
                    .ok_or_else(|| ErrorKind::MissingElement("info".into()))?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{
        write_attrib, write_attrib_opt, BytesEnd, BytesStart, Error, Event, IterSerializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct DocumentTypeSerializer<'ser> {
        pub(super) value: &'ser super::DocumentType,
        pub(super) state: Box<DocumentTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DocumentTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<'ser, &'ser [super::DocumentTypeContent], super::DocumentTypeContent>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DocumentTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocumentTypeSerializerState::Init__ => {
                        *self.state = DocumentTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "name", &self.value.name)?;
                        write_attrib_opt(&mut bytes, "count", &self.value.count)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DocumentTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocumentTypeSerializerState::End__,
                    },
                    DocumentTypeSerializerState::End__ => {
                        *self.state = DocumentTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DocumentTypeSerializerState::Done__ => return Ok(None),
                    DocumentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DocumentTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DocumentTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DocumentTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::DocumentTypeContent,
        pub(super) state: Box<DocumentTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum DocumentTypeContentSerializerState<'ser> {
        Init__,
        Info(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DocumentTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocumentTypeContentSerializerState::Init__ => {
                        *self.state = DocumentTypeContentSerializerState::Info(
                            WithSerializer::serializer(&self.value.info, Some("info"), false)?,
                        );
                    }
                    DocumentTypeContentSerializerState::Info(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocumentTypeContentSerializerState::Done__,
                    },
                    DocumentTypeContentSerializerState::Done__ => return Ok(None),
                    DocumentTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DocumentTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DocumentTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
pub mod xs {
    use std::borrow::Cow;
    use xsd_parser::quick_xml::{
        DeserializeBytes, DeserializeReader, Error, SerializeBytes, WithDeserializer,
        WithSerializer,
    };
    #[derive(Debug, Default)]
    pub struct EntitiesType(pub Vec<String>);
    impl DeserializeBytes for EntitiesType {
        fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| String::deserialize_bytes(reader, bytes))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    impl SerializeBytes for EntitiesType {
        fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
            if self.0.is_empty() {
                return Ok(None);
            }
            let mut data = String::new();
            for item in &self.0 {
                if let Some(bytes) = item.serialize_bytes()? {
                    if !data.is_empty() {
                        data.push(' ');
                    }
                    data.push_str(&bytes);
                }
            }
            Ok(Some(Cow::Owned(data)))
        }
    }
    pub type EntityType = EntitiesType;
    pub type IdType = String;
    pub type IdrefType = String;
    pub type IdrefsType = EntitiesType;
    pub type NcNameType = String;
    pub type NmtokenType = String;
    pub type NmtokensType = EntitiesType;
    pub type NotationType = String;
    pub type NameType = String;
    pub type QNameType = String;
    #[derive(Debug)]
    pub struct AnyType;
    impl WithDeserializer for AnyType {
        type Deserializer = quick_xml_deserialize::AnyTypeDeserializer;
    }
    impl WithSerializer for AnyType {
        type Serializer<'x> = quick_xml_serialize::AnyTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::AnyTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::AnyTypeSerializerState::Init__),
                name: name.unwrap_or("xs:anyType"),
                is_root,
            })
        }
    }
    pub type AnyUriType = String;
    pub type Base64BinaryType = String;
    pub type BooleanType = bool;
    pub type ByteType = i8;
    pub type DateType = String;
    pub type DateTimeType = String;
    pub type DecimalType = f64;
    pub type DoubleType = f64;
    pub type DurationType = String;
    pub type FloatType = f32;
    pub type GDayType = String;
    pub type GMonthType = String;
    pub type GMonthDayType = String;
    pub type GYearType = String;
    pub type GYearMonthType = String;
    pub type HexBinaryType = String;
    pub type IntType = i32;
    pub type IntegerType = super::BigInt;
    pub type LanguageType = String;
    pub type LongType = i64;
    pub type NegativeIntegerType = super::BigInt;
    pub type NonNegativeIntegerType = super::BigUint;
    pub type NonPositiveIntegerType = super::BigInt;
    pub type NormalizedStringType = String;
    pub type PositiveIntegerType = super::BigUint;
    pub type ShortType = i16;
    pub type StringType = String;
    pub type TimeType = String;
    pub type TokenType = String;
    pub type UnsignedByteType = u8;
    pub type UnsignedIntType = u32;
    pub type UnsignedLongType = u64;
    pub type UnsignedShortType = u16;
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser::quick_xml::{
            BytesStart, DeserializeReader, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, Error, Event,
        };
        #[derive(Debug)]
        pub struct AnyTypeDeserializer {
            state: Box<AnyTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum AnyTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl AnyTypeDeserializer {
            fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
            where
                R: DeserializeReader,
            {
                Ok(Self {
                    state: Box::new(AnyTypeDeserializerState::Init__),
                })
            }
            fn finish_state<R>(
                &mut self,
                reader: &R,
                state: AnyTypeDeserializerState,
            ) -> Result<(), Error>
            where
                R: DeserializeReader,
            {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::AnyType> for AnyTypeDeserializer {
            fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AnyType>
            where
                R: DeserializeReader,
            {
                reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next<R>(
                mut self,
                reader: &R,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::AnyType>
            where
                R: DeserializeReader,
            {
                if let Event::End(_) = &event {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    })
                } else {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event: DeserializerEvent::Break(event),
                        allow_any: true,
                    })
                }
            }
            fn finish<R>(mut self, reader: &R) -> Result<super::AnyType, Error>
            where
                R: DeserializeReader,
            {
                let state = replace(&mut *self.state, AnyTypeDeserializerState::Unknown__);
                self.finish_state(reader, state)?;
                Ok(super::AnyType {})
            }
        }
    }
    pub mod quick_xml_serialize {
        use core::iter::Iterator;
        use xsd_parser::quick_xml::{BytesStart, Error, Event};
        #[derive(Debug)]
        pub struct AnyTypeSerializer<'ser> {
            pub(super) value: &'ser super::AnyType,
            pub(super) state: Box<AnyTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum AnyTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> AnyTypeSerializer<'ser> {
            fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        AnyTypeSerializerState::Init__ => {
                            *self.state = AnyTypeSerializerState::Done__;
                            let bytes = BytesStart::new(self.name);
                            return Ok(Some(Event::Empty(bytes)));
                        }
                        AnyTypeSerializerState::Done__ => return Ok(None),
                        AnyTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Iterator for AnyTypeSerializer<'ser> {
            type Item = Result<Event<'ser>, Error>;
            fn next(&mut self) -> Option<Self::Item> {
                match self.next_event() {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = AnyTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
