use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_DEFAULT: Namespace = Namespace::new_const(b"http://www.smokexml.org/schema");
pub type Container = ContainerElementType;
#[derive(Debug)]
pub struct ContainerElementType {
    pub id: ::std::string::String,
    pub content: Vec<ContainerElementTypeContent>,
}
#[derive(Debug)]
pub enum ContainerElementTypeContent {
    RootType(RootTypeElementType),
}
impl WithDeserializer for ContainerElementType {
    type Deserializer = quick_xml_deserialize::ContainerElementTypeDeserializer;
}
impl WithDeserializer for ContainerElementTypeContent {
    type Deserializer = quick_xml_deserialize::ContainerElementTypeContentDeserializer;
}
impl WithSerializer for ContainerElementType {
    type Serializer<'x> = quick_xml_serialize::ContainerElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ContainerElementTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ContainerElementTypeSerializerState::Init__),
            name: name.unwrap_or("Container"),
            is_root,
        })
    }
}
impl WithSerializer for ContainerElementTypeContent {
    type Serializer<'x> = quick_xml_serialize::ContainerElementTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::ContainerElementTypeContentSerializer {
            value: self,
            state: Box::new(
                quick_xml_serialize::ContainerElementTypeContentSerializerState::Init__,
            ),
        })
    }
}
pub type Name = ::std::string::String;
pub type RootType = RootTypeElementType;
#[derive(Debug)]
pub struct RootTypeElementType {
    pub id: ::std::string::String,
    pub content: Vec<RootTypeElementTypeContent>,
}
#[derive(Debug)]
pub enum RootTypeElementTypeContent {
    SomeType(SomeTypeElementType),
}
impl WithDeserializer for RootTypeElementType {
    type Deserializer = quick_xml_deserialize::RootTypeElementTypeDeserializer;
}
impl WithDeserializer for RootTypeElementTypeContent {
    type Deserializer = quick_xml_deserialize::RootTypeElementTypeContentDeserializer;
}
impl WithSerializer for RootTypeElementType {
    type Serializer<'x> = quick_xml_serialize::RootTypeElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::RootTypeElementTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RootTypeElementTypeSerializerState::Init__),
            name: name.unwrap_or("RootType"),
            is_root,
        })
    }
}
impl WithSerializer for RootTypeElementTypeContent {
    type Serializer<'x> = quick_xml_serialize::RootTypeElementTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::RootTypeElementTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RootTypeElementTypeContentSerializerState::Init__),
        })
    }
}
pub type SomeType = SomeTypeElementType;
#[derive(Debug)]
pub struct SomeTypeElementType {
    pub id: ::std::string::String,
    pub partner_id_ref: Option<::std::string::String>,
}
impl WithDeserializer for SomeTypeElementType {
    type Deserializer = quick_xml_deserialize::SomeTypeElementTypeDeserializer;
}
impl WithSerializer for SomeTypeElementType {
    type Serializer<'x> = quick_xml_serialize::SomeTypeElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SomeTypeElementTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SomeTypeElementTypeSerializerState::Init__),
            name: name.unwrap_or("SomeType"),
            is_root,
        })
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ContainerElementTypeDeserializer {
        id: ::std::string::String,
        content: Vec<super::ContainerElementTypeContent>,
        state: Box<ContainerElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ContainerElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ContainerElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ContainerElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DEFAULT),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("id".into())))?,
                content: Vec::new(),
                state: Box::new(ContainerElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ContainerElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let ContainerElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::ContainerElementTypeContent,
        ) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ContainerElementTypeContent>,
            fallback: &mut Option<ContainerElementTypeDeserializerState>,
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
                    .unwrap_or(ContainerElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = ContainerElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ContainerElementTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ContainerElementTypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = ContainerElementTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ContainerElementType> for ContainerElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerElementType>
        where
            R: DeserializeReader,
        {
            use ContainerElementTypeDeserializerState as S;
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
                        let output = < super :: ContainerElementTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
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
        fn finish<R>(mut self, reader: &R) -> Result<super::ContainerElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ContainerElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ContainerElementType {
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct ContainerElementTypeContentDeserializer {
        state: Box<ContainerElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ContainerElementTypeContentDeserializerState {
        Init__,
        RootType(
            Option<super::RootTypeElementType>,
            Option<<super::RootTypeElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ContainerElementTypeContent),
        Unknown__,
    }
    impl ContainerElementTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<ContainerElementTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DEFAULT),
                    Some(b"RootType")
                ) {
                    let output =
                        <super::RootTypeElementType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_root_type(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(ContainerElementTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: ContainerElementTypeContentDeserializerState,
        ) -> Result<super::ContainerElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use ContainerElementTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::RootType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_root_type(&mut values, value)?;
                    }
                    Ok(super::ContainerElementTypeContent::RootType(
                        values.ok_or_else(|| ErrorKind::MissingElement("RootType".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
            }
        }
        fn store_root_type(
            values: &mut Option<super::RootTypeElementType>,
            value: super::RootTypeElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"RootType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_root_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::RootTypeElementType>,
            output: DeserializerOutput<'de, super::RootTypeElementType>,
            fallback: &mut Option<ContainerElementTypeContentDeserializerState>,
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
                *self.state = match fallback.take() {
                    None if values.is_none() => {
                        *self.state = ContainerElementTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => ContainerElementTypeContentDeserializerState::RootType(values, None),
                    Some(ContainerElementTypeContentDeserializerState::RootType(
                        _,
                        Some(deserializer),
                    )) => ContainerElementTypeContentDeserializerState::RootType(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(ContainerElementTypeContentDeserializerState::RootType(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_root_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_root_type(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        ContainerElementTypeContentDeserializerState::RootType(values, None),
                    )?;
                    *self.state = ContainerElementTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = ContainerElementTypeContentDeserializerState::RootType(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ContainerElementTypeContent>
        for ContainerElementTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ContainerElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(ContainerElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state,
                        ContainerElementTypeContentDeserializerState::Init__
                    ) =>
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
        ) -> DeserializerResult<'de, super::ContainerElementTypeContent>
        where
            R: DeserializeReader,
        {
            use ContainerElementTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::RootType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_root_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::RootType(values, None), event) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DEFAULT),
                            b"RootType",
                            false,
                        )?;
                        match self.handle_root_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
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
        fn finish<R>(self, reader: &R) -> Result<super::ContainerElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct RootTypeElementTypeDeserializer {
        id: ::std::string::String,
        content: Vec<super::RootTypeElementTypeContent>,
        state: Box<RootTypeElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RootTypeElementTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::RootTypeElementTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl RootTypeElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DEFAULT),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("id".into())))?,
                content: Vec::new(),
                state: Box::new(RootTypeElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RootTypeElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let RootTypeElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::RootTypeElementTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RootTypeElementTypeContent>,
            fallback: &mut Option<RootTypeElementTypeDeserializerState>,
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
                    .unwrap_or(RootTypeElementTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = RootTypeElementTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                RootTypeElementTypeDeserializerState::Content__(deserializer);
                        }
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                RootTypeElementTypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = RootTypeElementTypeDeserializerState::Next__;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RootTypeElementType> for RootTypeElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootTypeElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootTypeElementType>
        where
            R: DeserializeReader,
        {
            use RootTypeElementTypeDeserializerState as S;
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
                        let output = < super :: RootTypeElementTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
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
        fn finish<R>(mut self, reader: &R) -> Result<super::RootTypeElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                RootTypeElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::RootTypeElementType {
                id: self.id,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct RootTypeElementTypeContentDeserializer {
        state: Box<RootTypeElementTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum RootTypeElementTypeContentDeserializerState {
        Init__,
        SomeType(
            Option<super::SomeTypeElementType>,
            Option<<super::SomeTypeElementType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::RootTypeElementTypeContent),
        Unknown__,
    }
    impl RootTypeElementTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<RootTypeElementTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_DEFAULT),
                    Some(b"SomeType")
                ) {
                    let output =
                        <super::SomeTypeElementType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    return self.handle_some_type(
                        reader,
                        Default::default(),
                        output,
                        &mut *fallback,
                    );
                }
            }
            *self.state = fallback
                .take()
                .unwrap_or(RootTypeElementTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: RootTypeElementTypeContentDeserializerState,
        ) -> Result<super::RootTypeElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use RootTypeElementTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::SomeType(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_some_type(&mut values, value)?;
                    }
                    Ok(super::RootTypeElementTypeContent::SomeType(
                        values.ok_or_else(|| ErrorKind::MissingElement("SomeType".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
            }
        }
        fn store_some_type(
            values: &mut Option<super::SomeTypeElementType>,
            value: super::SomeTypeElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SomeType",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_some_type<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SomeTypeElementType>,
            output: DeserializerOutput<'de, super::SomeTypeElementType>,
            fallback: &mut Option<RootTypeElementTypeContentDeserializerState>,
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
                *self.state = match fallback.take() {
                    None if values.is_none() => {
                        *self.state = RootTypeElementTypeContentDeserializerState::Init__;
                        return Ok(ElementHandlerOutput::from_event(event, allow_any));
                    }
                    None => RootTypeElementTypeContentDeserializerState::SomeType(values, None),
                    Some(RootTypeElementTypeContentDeserializerState::SomeType(
                        _,
                        Some(deserializer),
                    )) => RootTypeElementTypeContentDeserializerState::SomeType(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(RootTypeElementTypeContentDeserializerState::SomeType(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_some_type(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_some_type(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        RootTypeElementTypeContentDeserializerState::SomeType(values, None),
                    )?;
                    *self.state = RootTypeElementTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = RootTypeElementTypeContentDeserializerState::SomeType(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RootTypeElementTypeContent>
        for RootTypeElementTypeContentDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RootTypeElementTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(RootTypeElementTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state,
                        RootTypeElementTypeContentDeserializerState::Init__
                    ) =>
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
        ) -> DeserializerResult<'de, super::RootTypeElementTypeContent>
        where
            R: DeserializeReader,
        {
            use RootTypeElementTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::SomeType(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_some_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::SomeType(values, None), event) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_DEFAULT),
                            b"SomeType",
                            false,
                        )?;
                        match self.handle_some_type(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
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
        fn finish<R>(self, reader: &R) -> Result<super::RootTypeElementTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct SomeTypeElementTypeDeserializer {
        id: ::std::string::String,
        partner_id_ref: Option<::std::string::String>,
        state: Box<SomeTypeElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SomeTypeElementTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl SomeTypeElementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<::std::string::String> = None;
            let mut partner_id_ref: Option<::std::string::String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DEFAULT),
                    Some(b"id")
                ) {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_DEFAULT),
                    Some(b"partnerIdRef")
                ) {
                    reader.read_attrib(&mut partner_id_ref, b"partnerIdRef", &attrib.value)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("id".into())))?,
                partner_id_ref: partner_id_ref,
                state: Box::new(SomeTypeElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SomeTypeElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::SomeTypeElementType> for SomeTypeElementTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SomeTypeElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SomeTypeElementType>
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
                    allow_any: false,
                })
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SomeTypeElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SomeTypeElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SomeTypeElementType {
                id: self.id,
                partner_id_ref: self.partner_id_ref,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::{
        models::schema::Namespace,
        quick_xml::{
            write_attrib, write_attrib_opt, BytesEnd, BytesStart, Error, Event, IterSerializer,
            WithSerializer,
        },
    };
    #[derive(Debug)]
    pub struct ContainerElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::ContainerElementType,
        pub(super) state: Box<ContainerElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ContainerElementTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<
                'ser,
                &'ser [super::ContainerElementTypeContent],
                super::ContainerElementTypeContent,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ContainerElementTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ContainerElementTypeSerializerState::Init__ => {
                        *self.state = ContainerElementTypeSerializerState::Content__(
                            IterSerializer::new(&self.value.content[..], None, false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:xsi"[..], &Namespace::XSI[..]));
                        }
                        write_attrib(&mut bytes, "id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ContainerElementTypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ContainerElementTypeSerializerState::End__,
                        }
                    }
                    ContainerElementTypeSerializerState::End__ => {
                        *self.state = ContainerElementTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ContainerElementTypeSerializerState::Done__ => return Ok(None),
                    ContainerElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ContainerElementTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ContainerElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ContainerElementTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::ContainerElementTypeContent,
        pub(super) state: Box<ContainerElementTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ContainerElementTypeContentSerializerState<'ser> {
        Init__,
        RootType(<super::RootTypeElementType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ContainerElementTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ContainerElementTypeContentSerializerState::Init__ => match self.value {
                        super::ContainerElementTypeContent::RootType(x) => {
                            *self.state = ContainerElementTypeContentSerializerState::RootType(
                                WithSerializer::serializer(x, Some("RootType"), false)?,
                            )
                        }
                    },
                    ContainerElementTypeContentSerializerState::RootType(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ContainerElementTypeContentSerializerState::Done__
                            }
                        }
                    }
                    ContainerElementTypeContentSerializerState::Done__ => return Ok(None),
                    ContainerElementTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ContainerElementTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ContainerElementTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RootTypeElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::RootTypeElementType,
        pub(super) state: Box<RootTypeElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RootTypeElementTypeSerializerState<'ser> {
        Init__,
        Content__(
            IterSerializer<
                'ser,
                &'ser [super::RootTypeElementTypeContent],
                super::RootTypeElementTypeContent,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RootTypeElementTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RootTypeElementTypeSerializerState::Init__ => {
                        *self.state = RootTypeElementTypeSerializerState::Content__(
                            IterSerializer::new(&self.value.content[..], None, false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:xsi"[..], &Namespace::XSI[..]));
                        }
                        write_attrib(&mut bytes, "id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RootTypeElementTypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RootTypeElementTypeSerializerState::End__,
                        }
                    }
                    RootTypeElementTypeSerializerState::End__ => {
                        *self.state = RootTypeElementTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RootTypeElementTypeSerializerState::Done__ => return Ok(None),
                    RootTypeElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for RootTypeElementTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RootTypeElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RootTypeElementTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::RootTypeElementTypeContent,
        pub(super) state: Box<RootTypeElementTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum RootTypeElementTypeContentSerializerState<'ser> {
        Init__,
        SomeType(<super::SomeTypeElementType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RootTypeElementTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RootTypeElementTypeContentSerializerState::Init__ => match self.value {
                        super::RootTypeElementTypeContent::SomeType(x) => {
                            *self.state = RootTypeElementTypeContentSerializerState::SomeType(
                                WithSerializer::serializer(x, Some("SomeType"), false)?,
                            )
                        }
                    },
                    RootTypeElementTypeContentSerializerState::SomeType(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RootTypeElementTypeContentSerializerState::Done__,
                        }
                    }
                    RootTypeElementTypeContentSerializerState::Done__ => return Ok(None),
                    RootTypeElementTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for RootTypeElementTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RootTypeElementTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SomeTypeElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::SomeTypeElementType,
        pub(super) state: Box<SomeTypeElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SomeTypeElementTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SomeTypeElementTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SomeTypeElementTypeSerializerState::Init__ => {
                        *self.state = SomeTypeElementTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_DEFAULT[..]));
                            bytes.push_attribute((&b"xmlns:xsi"[..], &Namespace::XSI[..]));
                        }
                        write_attrib(&mut bytes, "id", &self.value.id)?;
                        write_attrib_opt(&mut bytes, "partnerIdRef", &self.value.partner_id_ref)?;
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    SomeTypeElementTypeSerializerState::Done__ => return Ok(None),
                    SomeTypeElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SomeTypeElementTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SomeTypeElementTypeSerializerState::Done__;
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
    pub struct EntitiesType(pub Vec<::std::string::String>);
    impl DeserializeBytes for EntitiesType {
        fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| ::std::string::String::deserialize_bytes(reader, bytes))
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
    pub type IdType = ::std::string::String;
    pub type IdrefType = ::std::string::String;
    pub type IdrefsType = EntitiesType;
    pub type NcNameType = ::std::string::String;
    pub type NmtokenType = ::std::string::String;
    pub type NmtokensType = EntitiesType;
    pub type NotationType = ::std::string::String;
    pub type NameType = ::std::string::String;
    pub type QNameType = ::std::string::String;
    pub type AnySimpleType = ::std::string::String;
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
                name: name.unwrap_or("anyType"),
                is_root,
            })
        }
    }
    pub type AnyUriType = ::std::string::String;
    pub type Base64BinaryType = ::std::string::String;
    pub type BooleanType = ::core::primitive::bool;
    pub type ByteType = ::core::primitive::i8;
    pub type DateType = ::std::string::String;
    pub type DateTimeType = ::std::string::String;
    pub type DecimalType = ::core::primitive::f64;
    pub type DoubleType = ::core::primitive::f64;
    pub type DurationType = ::std::string::String;
    pub type FloatType = ::core::primitive::f32;
    pub type GDayType = ::std::string::String;
    pub type GMonthType = ::std::string::String;
    pub type GMonthDayType = ::std::string::String;
    pub type GYearType = ::std::string::String;
    pub type GYearMonthType = ::std::string::String;
    pub type HexBinaryType = ::std::string::String;
    pub type IntType = ::core::primitive::i32;
    pub type IntegerType = ::core::primitive::i32;
    pub type LanguageType = ::std::string::String;
    pub type LongType = ::core::primitive::i64;
    pub type NegativeIntegerType = ::core::primitive::isize;
    pub type NonNegativeIntegerType = ::core::primitive::usize;
    pub type NonPositiveIntegerType = ::core::primitive::isize;
    pub type NormalizedStringType = ::std::string::String;
    pub type PositiveIntegerType = ::core::primitive::usize;
    pub type ShortType = ::core::primitive::i16;
    pub type StringType = ::std::string::String;
    pub type TimeType = ::std::string::String;
    pub type TokenType = ::std::string::String;
    pub type UnsignedByteType = ::core::primitive::u8;
    pub type UnsignedIntType = ::core::primitive::u32;
    pub type UnsignedLongType = ::core::primitive::u64;
    pub type UnsignedShortType = ::core::primitive::u16;
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
        use xsd_parser::{
            models::schema::Namespace,
            quick_xml::{BytesStart, Error, Event},
        };
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
                            let mut bytes = BytesStart::new(self.name);
                            if self.is_root {
                                bytes
                                    .push_attribute((&b"xmlns"[..], &super::super::NS_DEFAULT[..]));
                                bytes.push_attribute((&b"xmlns:xsi"[..], &Namespace::XSI[..]));
                            }
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
