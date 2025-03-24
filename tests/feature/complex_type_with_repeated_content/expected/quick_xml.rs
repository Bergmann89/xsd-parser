use xsd_parser::{
    quick_xml::{Error, WithDeserializer, WithSerializer},
    schema::Namespace,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content: Vec<FooTypeContent>,
}
#[derive(Debug, Clone)]
pub struct FooTypeContent {
    pub a: i32,
    pub b: Option<String>,
    pub c: Option<String>,
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
            state: Box::new(quick_xml_serialize::FooTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Foo"),
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
            state: Box::new(quick_xml_serialize::FooTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
impl WithDeserializer for FooTypeContent {
    type Deserializer = quick_xml_deserialize::FooTypeContentDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        content: Vec<super::FooTypeContent>,
        state: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::FooTypeContent as WithDeserializer>::Deserializer),
        Done__,
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
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
            self.content.push(value);
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
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(FooTypeDeserializerState::Content__(deserializer));
                            *self.state = FooTypeDeserializerState::Next__;
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
                            <super::FooTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Done__, event) => {
                        *self.state = S::Done__;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = match &*self.state {
                S::Done__ => DeserializerArtifact::Data(self.finish(reader)?),
                _ => DeserializerArtifact::Deserializer(self),
            };
            Ok(DeserializerOutput {
                artifact,
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
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct FooTypeContentDeserializer {
        a: Option<i32>,
        b: Option<String>,
        c: Option<String>,
        state: Box<FooTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeContentDeserializerState {
        Init__,
        A(Option<<i32 as WithDeserializer>::Deserializer>),
        B(Option<<String as WithDeserializer>::Deserializer>),
        C(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl FooTypeContentDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FooTypeContentDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use FooTypeContentDeserializerState as S;
            match state {
                S::A(Some(deserializer)) => self.store_a(deserializer.finish(reader)?)?,
                S::B(Some(deserializer)) => self.store_b(deserializer.finish(reader)?)?,
                S::C(Some(deserializer)) => self.store_c(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_a(&mut self, value: i32) -> Result<(), Error> {
            if self.a.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"a")))?;
            }
            self.a = Some(value);
            Ok(())
        }
        fn store_b(&mut self, value: String) -> Result<(), Error> {
            if self.b.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"b")))?;
            }
            self.b = Some(value);
            Ok(())
        }
        fn store_c(&mut self, value: String) -> Result<(), Error> {
            if self.c.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"c")))?;
            }
            self.c = Some(value);
            Ok(())
        }
        fn handle_a<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<FooTypeContentDeserializerState>,
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
                if self.a.is_some() {
                    fallback.get_or_insert(FooTypeContentDeserializerState::A(None));
                    *self.state = FooTypeContentDeserializerState::B(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FooTypeContentDeserializerState::A(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_a(data)?;
                    *self.state = FooTypeContentDeserializerState::B(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooTypeContentDeserializerState::A(Some(
                                deserializer,
                            )));
                            *self.state = FooTypeContentDeserializerState::B(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooTypeContentDeserializerState::A(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_b<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooTypeContentDeserializerState>,
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
                fallback.get_or_insert(FooTypeContentDeserializerState::B(None));
                *self.state = FooTypeContentDeserializerState::C(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_b(data)?;
                    *self.state = FooTypeContentDeserializerState::C(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooTypeContentDeserializerState::B(Some(
                                deserializer,
                            )));
                            *self.state = FooTypeContentDeserializerState::C(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooTypeContentDeserializerState::B(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_c<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<FooTypeContentDeserializerState>,
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
                fallback.get_or_insert(FooTypeContentDeserializerState::C(None));
                *self.state = FooTypeContentDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_c(data)?;
                    *self.state = FooTypeContentDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(FooTypeContentDeserializerState::C(Some(
                                deserializer,
                            )));
                            *self.state = FooTypeContentDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FooTypeContentDeserializerState::C(Some(deserializer));
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
            let deserializer = Self {
                a: None,
                b: None,
                c: None,
                state: Box::new(FooTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, FooTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::FooTypeContent>
        where
            R: DeserializeReader,
        {
            use FooTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::A(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_a(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::B(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_b(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::C(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_c(reader, output, &mut fallback)? {
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
                        *self.state = FooTypeContentDeserializerState::A(None);
                        event
                    }
                    (S::A(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"a") {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_a(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::B(None);
                            event
                        }
                    }
                    (S::B(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"b") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_b(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::C(None);
                            event
                        }
                    }
                    (S::C(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"c") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_c(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::FooTypeContent, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, FooTypeContentDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::FooTypeContent {
                a: self
                    .a
                    .ok_or_else(|| ErrorKind::MissingElement("a".into()))?,
                b: self.b,
                c: self.c,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct FooTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooType,
        pub(super) state: Box<FooTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, &'ser [super::FooTypeContent], super::FooTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeSerializerState::Init__ => {
                        *self.state = FooTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::End__ => {
                        *self.state = FooTypeSerializerState::Done__;
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
                    *self.state = FooTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::FooTypeContent,
        pub(super) state: Box<FooTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeContentSerializerState<'ser> {
        Init__,
        A(<i32 as WithSerializer>::Serializer<'ser>),
        B(IterSerializer<'ser, Option<&'ser String>, String>),
        C(IterSerializer<'ser, Option<&'ser String>, String>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeContentSerializerState::Init__ => {
                        *self.state = FooTypeContentSerializerState::A(WithSerializer::serializer(
                            &self.value.a,
                            Some("tns:a"),
                            false,
                        )?);
                    }
                    FooTypeContentSerializerState::A(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeContentSerializerState::B(IterSerializer::new(
                                self.value.b.as_ref(),
                                Some("tns:b"),
                                false,
                            ))
                        }
                    },
                    FooTypeContentSerializerState::B(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = FooTypeContentSerializerState::C(IterSerializer::new(
                                self.value.c.as_ref(),
                                Some("tns:c"),
                                false,
                            ))
                        }
                    },
                    FooTypeContentSerializerState::C(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = FooTypeContentSerializerState::Done__,
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
                    *self.state = FooTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
