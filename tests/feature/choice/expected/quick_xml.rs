pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
use xsd_parser::{
    quick_xml::{deserialize_new::WithDeserializer, Error, WithSerializer},
    schema::Namespace,
};
pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content: FooTypeContent,
}
#[derive(Debug, Clone)]
pub enum FooTypeContent {
    Bar(String),
    Baz(i32),
}
impl WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::FooTypeSerializer {
            value: self,
            state: quick_xml_serialize::FooTypeSerializerState::Init__,
            name: name.unwrap_or("tns:FooType"),
            is_root,
        })
    }
}
impl WithSerializer for FooTypeContent {
    type Serializer<'x> = quick_xml_serialize::FooTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::FooTypeContentSerializer {
            value: self,
            state: quick_xml_serialize::FooTypeContentSerializerState::Init__,
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
impl WithDeserializer for FooTypeContent {
    type Deserializer = quick_xml_deserialize::FooTypeContentDeserializer;
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, WithSerializer};
    #[derive(Debug)]
    pub struct FooTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooType,
        pub(super) state: FooTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeSerializerState<'ser> {
        Init__,
        Content__(<super::FooTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    FooTypeSerializerState::Init__ => {
                        self.state = FooTypeSerializerState::Content__(WithSerializer::serializer(
                            &self.value.content,
                            None,
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::End__ => {
                        self.state = FooTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    FooTypeSerializerState::Done__ => return Ok(None),
                    FooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FooTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = FooTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::FooTypeContent,
        pub(super) state: FooTypeContentSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeContentSerializerState<'ser> {
        Init__,
        Bar(<String as WithSerializer>::Serializer<'ser>),
        Baz(<i32 as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    FooTypeContentSerializerState::Init__ => match self.value {
                        super::FooTypeContent::Bar(x) => {
                            self.state = FooTypeContentSerializerState::Bar(
                                WithSerializer::serializer(x, Some("tns:Bar"), false)?,
                            )
                        }
                        super::FooTypeContent::Baz(x) => {
                            self.state = FooTypeContentSerializerState::Baz(
                                WithSerializer::serializer(x, Some("tns:Baz"), false)?,
                            )
                        }
                    },
                    FooTypeContentSerializerState::Bar(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooTypeContentSerializerState::Done__,
                    },
                    FooTypeContentSerializerState::Baz(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooTypeContentSerializerState::Done__,
                    },
                    FooTypeContentSerializerState::Done__ => return Ok(None),
                    FooTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FooTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = FooTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        deserialize_new::{
            DeserializeReader, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, WithDeserializer,
        },
        filter_xmlns_attributes, BytesStart, Error, ErrorKind, Event, RawByteStr,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        content: Option<super::FooTypeContent>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::FooTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(&bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FooTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let FooTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::FooTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FooTypeContent>,
            fallback: &mut Option<FooTypeDeserializerState>,
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
                *self.state = fallback.take().unwrap_or(FooTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = FooTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(FooTypeDeserializerState::Content__(deserializer));
                            *self.state = FooTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FooType> for FooTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType>
        where
            R: DeserializeReader,
        {
            use FooTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event, allow_any, ..
                            } => break (event, allow_any),
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
                    (old_state @ (S::Init__ | S::Next__), event) => {
                        let output =
                            <super::FooTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event, allow_any, ..
                            } => {
                                if matches!(&*self.state, S::Unknown__) {
                                    *self.state = old_state;
                                }
                                break (event, allow_any);
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::FooType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, FooTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FooType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub enum FooTypeContentDeserializer {
        Init__,
        Bar(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Baz(Option<i32>, Option<<i32 as WithDeserializer>::Deserializer>),
        Unknown__,
    }
    impl FooTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<FooTypeContentDeserializer>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self = Self::Init__;
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Bar")
            ) {
                let output = <String as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_bar(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Baz")
            ) {
                let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_baz(reader, Default::default(), output, &mut *fallback);
            }
            *self = Self::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn store_bar(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_baz(values: &mut Option<i32>, value: i32) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Baz")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_bar<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<String>,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Bar(_, Some(deserializer))) => Self::Bar(values, Some(deserializer)),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Bar(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_bar(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_bar(&mut values, data)?;
                    *self = Self::Bar(values, None);
                    ElementHandlerOutput::Break {
                        event,
                        allow_any,
                        finish: true,
                    }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self = Self::Bar(values, Some(deserializer));
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(Self::Bar(Default::default(), Some(deserializer)));
                            *self = Self::Bar(values, None);
                        }
                    }
                    ret
                }
            })
        }
        fn handle_baz<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<i32>,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<Self>,
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
                *self = match fallback.take() {
                    None => Self::Init__,
                    Some(Self::Baz(_, Some(deserializer))) => Self::Baz(values, Some(deserializer)),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(Self::Baz(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_baz(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_baz(&mut values, data)?;
                    *self = Self::Baz(values, None);
                    ElementHandlerOutput::Break {
                        event,
                        allow_any,
                        finish: true,
                    }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self = Self::Baz(values, Some(deserializer));
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(Self::Baz(Default::default(), Some(deserializer)));
                            *self = Self::Baz(values, None);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FooTypeContent> for FooTypeContentDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FooTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self::Init__;
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(Self::Init__) => DeserializerArtifact::None,
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooTypeContent>
        where
            R: DeserializeReader,
        {
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any, finish) = loop {
                let state = replace(&mut self, Self::Unknown__);
                event = match (state, event) {
                    (Self::Bar(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bar(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Baz(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_baz(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(state.finish(reader)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (Self::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Bar(values, None), event) => {
                        let output =
                            <String as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_bar(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Baz(values, None), event) => {
                        let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_baz(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break {
                                event,
                                allow_any,
                                finish,
                            } => break (event, allow_any, finish),
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (Self::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if finish {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::FooTypeContent, Error>
        where
            R: DeserializeReader,
        {
            match self {
                Self::Init__ => Err(ErrorKind::MissingContent.into()),
                Self::Bar(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_bar(&mut values, value)?;
                    }
                    Ok(super::FooTypeContent::Bar(
                        values.ok_or_else(|| ErrorKind::MissingElement("Bar".into()))?,
                    ))
                }
                Self::Baz(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_baz(&mut values, value)?;
                    }
                    Ok(super::FooTypeContent::Baz(
                        values.ok_or_else(|| ErrorKind::MissingElement("Baz".into()))?,
                    ))
                }
                Self::Unknown__ => unreachable!(),
            }
        }
    }
}
