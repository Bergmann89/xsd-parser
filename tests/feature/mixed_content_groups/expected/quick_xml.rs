use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
    xml::{Mixed, Text},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type NormalElement = NormalType;
#[derive(Debug)]
pub struct NormalType {
    pub group: NormalGroupType,
    pub baz: String,
}
impl WithSerializer for NormalType {
    type Serializer<'x> = quick_xml_serialize::NormalTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::NormalTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NormalTypeSerializerState::Init__),
            name: name.unwrap_or("tns:NormalType"),
            is_root,
        })
    }
}
impl WithDeserializer for NormalType {
    type Deserializer = quick_xml_deserialize::NormalTypeDeserializer;
}
#[derive(Debug)]
pub struct NormalGroupType {
    pub fuu: i32,
    pub bar: String,
}
impl WithSerializer for NormalGroupType {
    type Serializer<'x> = quick_xml_serialize::NormalGroupTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::NormalGroupTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NormalGroupTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for NormalGroupType {
    type Deserializer = quick_xml_deserialize::NormalGroupTypeDeserializer;
}
pub type MixedElement = MixedType;
#[derive(Debug)]
pub struct MixedType {
    pub text_before: Option<Text>,
    pub group: MixedGroupType,
    pub baz: Mixed<String>,
}
impl WithSerializer for MixedType {
    type Serializer<'x> = quick_xml_serialize::MixedTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::MixedTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedTypeSerializerState::Init__),
            name: name.unwrap_or("tns:MixedType"),
            is_root,
        })
    }
}
impl WithDeserializer for MixedType {
    type Deserializer = quick_xml_deserialize::MixedTypeDeserializer;
}
#[derive(Debug)]
pub struct MixedGroupType {
    pub fuu: Mixed<i32>,
    pub bar: Mixed<String>,
}
impl WithSerializer for MixedGroupType {
    type Serializer<'x> = quick_xml_serialize::MixedGroupTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::MixedGroupTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedGroupTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for MixedGroupType {
    type Deserializer = quick_xml_deserialize::MixedGroupTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::{
        quick_xml::{
            filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer,
            DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
            ElementHandlerOutput, Error, ErrorKind, Event, RawByteStr, WithDeserializer,
        },
        xml::{Mixed, Text},
    };
    #[derive(Debug)]
    pub struct NormalTypeDeserializer {
        group: Option<super::NormalGroupType>,
        baz: Option<String>,
        state: Box<NormalTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NormalTypeDeserializerState {
        Init__,
        Group(Option<<super::NormalGroupType as WithDeserializer>::Deserializer>),
        Baz(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NormalTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                group: None,
                baz: None,
                state: Box::new(NormalTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: NormalTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use NormalTypeDeserializerState as S;
            match state {
                S::Group(Some(deserializer)) => self.store_group(deserializer.finish(reader)?)?,
                S::Baz(Some(deserializer)) => self.store_baz(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_group(&mut self, value: super::NormalGroupType) -> Result<(), Error> {
            if self.group.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Group",
                )))?;
            }
            self.group = Some(value);
            Ok(())
        }
        fn store_baz(&mut self, value: String) -> Result<(), Error> {
            if self.baz.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Baz")))?;
            }
            self.baz = Some(value);
            Ok(())
        }
        fn handle_group<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::NormalGroupType>,
            fallback: &mut Option<NormalTypeDeserializerState>,
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
                if self.group.is_some() {
                    fallback.get_or_insert(NormalTypeDeserializerState::Group(None));
                    *self.state = NormalTypeDeserializerState::Baz(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = NormalTypeDeserializerState::Group(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_group(data)?;
                    *self.state = NormalTypeDeserializerState::Baz(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(NormalTypeDeserializerState::Group(Some(
                                deserializer,
                            )));
                            *self.state = NormalTypeDeserializerState::Baz(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = NormalTypeDeserializerState::Group(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_baz<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<NormalTypeDeserializerState>,
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
                if self.baz.is_some() {
                    fallback.get_or_insert(NormalTypeDeserializerState::Baz(None));
                    *self.state = NormalTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = NormalTypeDeserializerState::Baz(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_baz(data)?;
                    *self.state = NormalTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(NormalTypeDeserializerState::Baz(Some(
                                deserializer,
                            )));
                            *self.state = NormalTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = NormalTypeDeserializerState::Baz(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::NormalType> for NormalTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::NormalType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NormalType>
        where
            R: DeserializeReader,
        {
            use NormalTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Group(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_group(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Baz(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_baz(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = NormalTypeDeserializerState::Group(None);
                        event
                    }
                    (S::Group(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::NormalGroupType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_group(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Baz(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Baz") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_baz(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::NormalType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, NormalTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::NormalType {
                group: self
                    .group
                    .ok_or_else(|| ErrorKind::MissingElement("Group".into()))?,
                baz: self
                    .baz
                    .ok_or_else(|| ErrorKind::MissingElement("Baz".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct NormalGroupTypeDeserializer {
        fuu: Option<i32>,
        bar: Option<String>,
        state: Box<NormalGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NormalGroupTypeDeserializerState {
        Init__,
        Fuu(Option<<i32 as WithDeserializer>::Deserializer>),
        Bar(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NormalGroupTypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: NormalGroupTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use NormalGroupTypeDeserializerState as S;
            match state {
                S::Fuu(Some(deserializer)) => self.store_fuu(deserializer.finish(reader)?)?,
                S::Bar(Some(deserializer)) => self.store_bar(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_fuu(&mut self, value: i32) -> Result<(), Error> {
            if self.fuu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Fuu")))?;
            }
            self.fuu = Some(value);
            Ok(())
        }
        fn store_bar(&mut self, value: String) -> Result<(), Error> {
            if self.bar.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            self.bar = Some(value);
            Ok(())
        }
        fn handle_fuu<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<NormalGroupTypeDeserializerState>,
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
                if self.fuu.is_some() {
                    fallback.get_or_insert(NormalGroupTypeDeserializerState::Fuu(None));
                    *self.state = NormalGroupTypeDeserializerState::Bar(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = NormalGroupTypeDeserializerState::Fuu(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_fuu(data)?;
                    *self.state = NormalGroupTypeDeserializerState::Bar(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(NormalGroupTypeDeserializerState::Fuu(Some(
                                deserializer,
                            )));
                            *self.state = NormalGroupTypeDeserializerState::Bar(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = NormalGroupTypeDeserializerState::Fuu(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_bar<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<NormalGroupTypeDeserializerState>,
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
                if self.bar.is_some() {
                    fallback.get_or_insert(NormalGroupTypeDeserializerState::Bar(None));
                    *self.state = NormalGroupTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = NormalGroupTypeDeserializerState::Bar(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_bar(data)?;
                    *self.state = NormalGroupTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(NormalGroupTypeDeserializerState::Bar(Some(
                                deserializer,
                            )));
                            *self.state = NormalGroupTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = NormalGroupTypeDeserializerState::Bar(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::NormalGroupType> for NormalGroupTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::NormalGroupType>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                fuu: None,
                bar: None,
                state: Box::new(NormalGroupTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, NormalGroupTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::NormalGroupType>
        where
            R: DeserializeReader,
        {
            use NormalGroupTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Fuu(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fuu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bar(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bar(reader, output, &mut fallback)? {
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
                        *self.state = NormalGroupTypeDeserializerState::Fuu(None);
                        event
                    }
                    (S::Fuu(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Fuu") {
                            let output =
                                <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_fuu(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Bar(None);
                            event
                        }
                    }
                    (S::Bar(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Bar") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(reader, event)?;
                            match self.handle_bar(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::NormalGroupType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                NormalGroupTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::NormalGroupType {
                fuu: self
                    .fuu
                    .ok_or_else(|| ErrorKind::MissingElement("Fuu".into()))?,
                bar: self
                    .bar
                    .ok_or_else(|| ErrorKind::MissingElement("Bar".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedTypeDeserializer {
        text_before: Option<Text>,
        group: Option<super::MixedGroupType>,
        baz: Option<Mixed<String>>,
        state: Box<MixedTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedTypeDeserializerState {
        Init__,
        TextBefore(Option<<Text as WithDeserializer>::Deserializer>),
        Group(Option<<super::MixedGroupType as WithDeserializer>::Deserializer>),
        Baz(Option<<Mixed<String> as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MixedTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                text_before: None,
                group: None,
                baz: None,
                state: Box::new(MixedTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: MixedTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use MixedTypeDeserializerState as S;
            match state {
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(reader)?)?
                }
                S::Group(Some(deserializer)) => self.store_group(deserializer.finish(reader)?)?,
                S::Baz(Some(deserializer)) => self.store_baz(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_text_before(&mut self, value: Text) -> Result<(), Error> {
            if self.text_before.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"text_before",
                )))?;
            }
            self.text_before = Some(value);
            Ok(())
        }
        fn store_group(&mut self, value: super::MixedGroupType) -> Result<(), Error> {
            if self.group.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Group",
                )))?;
            }
            self.group = Some(value);
            Ok(())
        }
        fn store_baz(&mut self, value: Mixed<String>) -> Result<(), Error> {
            if self.baz.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Baz")))?;
            }
            self.baz = Some(value);
            Ok(())
        }
        fn handle_text_before<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Text>,
            fallback: &mut Option<MixedTypeDeserializerState>,
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
                fallback.get_or_insert(MixedTypeDeserializerState::TextBefore(None));
                *self.state = MixedTypeDeserializerState::Group(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state = MixedTypeDeserializerState::Group(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedTypeDeserializerState::TextBefore(Some(
                                deserializer,
                            )));
                            *self.state = MixedTypeDeserializerState::Group(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                MixedTypeDeserializerState::TextBefore(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_group<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::MixedGroupType>,
            fallback: &mut Option<MixedTypeDeserializerState>,
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
                if self.group.is_some() {
                    fallback.get_or_insert(MixedTypeDeserializerState::Group(None));
                    *self.state = MixedTypeDeserializerState::Baz(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = MixedTypeDeserializerState::Group(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_group(data)?;
                    *self.state = MixedTypeDeserializerState::Baz(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedTypeDeserializerState::Group(Some(
                                deserializer,
                            )));
                            *self.state = MixedTypeDeserializerState::Baz(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = MixedTypeDeserializerState::Group(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_baz<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Mixed<String>>,
            fallback: &mut Option<MixedTypeDeserializerState>,
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
                if self.baz.is_some() {
                    fallback.get_or_insert(MixedTypeDeserializerState::Baz(None));
                    *self.state = MixedTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = MixedTypeDeserializerState::Baz(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_baz(data)?;
                    *self.state = MixedTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(MixedTypeDeserializerState::Baz(Some(deserializer)));
                            *self.state = MixedTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = MixedTypeDeserializerState::Baz(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedType> for MixedTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::MixedType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedType>
        where
            R: DeserializeReader,
        {
            use MixedTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TextBefore(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_text_before(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Group(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_group(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Baz(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_baz(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = MixedTypeDeserializerState::TextBefore(None);
                        event
                    }
                    (S::TextBefore(None), event) => {
                        let output = <Text as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_text_before(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Group(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::MixedGroupType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_group(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Baz(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Baz") {
                            let output = <Mixed<String> as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_baz(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::MixedType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, MixedTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::MixedType {
                text_before: self.text_before,
                group: self
                    .group
                    .ok_or_else(|| ErrorKind::MissingElement("Group".into()))?,
                baz: self
                    .baz
                    .ok_or_else(|| ErrorKind::MissingElement("Baz".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct MixedGroupTypeDeserializer {
        fuu: Option<Mixed<i32>>,
        bar: Option<Mixed<String>>,
        state: Box<MixedGroupTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedGroupTypeDeserializerState {
        Init__,
        Fuu(Option<<Mixed<i32> as WithDeserializer>::Deserializer>),
        Bar(Option<<Mixed<String> as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MixedGroupTypeDeserializer {
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: MixedGroupTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use MixedGroupTypeDeserializerState as S;
            match state {
                S::Fuu(Some(deserializer)) => self.store_fuu(deserializer.finish(reader)?)?,
                S::Bar(Some(deserializer)) => self.store_bar(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_fuu(&mut self, value: Mixed<i32>) -> Result<(), Error> {
            if self.fuu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Fuu")))?;
            }
            self.fuu = Some(value);
            Ok(())
        }
        fn store_bar(&mut self, value: Mixed<String>) -> Result<(), Error> {
            if self.bar.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Bar")))?;
            }
            self.bar = Some(value);
            Ok(())
        }
        fn handle_fuu<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Mixed<i32>>,
            fallback: &mut Option<MixedGroupTypeDeserializerState>,
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
                if self.fuu.is_some() {
                    fallback.get_or_insert(MixedGroupTypeDeserializerState::Fuu(None));
                    *self.state = MixedGroupTypeDeserializerState::Bar(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = MixedGroupTypeDeserializerState::Fuu(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_fuu(data)?;
                    *self.state = MixedGroupTypeDeserializerState::Bar(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedGroupTypeDeserializerState::Fuu(Some(
                                deserializer,
                            )));
                            *self.state = MixedGroupTypeDeserializerState::Bar(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = MixedGroupTypeDeserializerState::Fuu(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_bar<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, Mixed<String>>,
            fallback: &mut Option<MixedGroupTypeDeserializerState>,
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
                if self.bar.is_some() {
                    fallback.get_or_insert(MixedGroupTypeDeserializerState::Bar(None));
                    *self.state = MixedGroupTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = MixedGroupTypeDeserializerState::Bar(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_bar(data)?;
                    *self.state = MixedGroupTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedGroupTypeDeserializerState::Bar(Some(
                                deserializer,
                            )));
                            *self.state = MixedGroupTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = MixedGroupTypeDeserializerState::Bar(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedGroupType> for MixedGroupTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::MixedGroupType>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                fuu: None,
                bar: None,
                state: Box::new(MixedGroupTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, MixedGroupTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::MixedGroupType>
        where
            R: DeserializeReader,
        {
            use MixedGroupTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Fuu(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_fuu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Bar(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_bar(reader, output, &mut fallback)? {
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
                        *self.state = MixedGroupTypeDeserializerState::Fuu(None);
                        event
                    }
                    (S::Fuu(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Fuu") {
                            let output = <Mixed<i32> as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_fuu(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Bar(None);
                            event
                        }
                    }
                    (S::Bar(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_TNS), b"Bar") {
                            let output = <Mixed<String> as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_bar(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::MixedGroupType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, MixedGroupTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::MixedGroupType {
                fuu: self
                    .fuu
                    .ok_or_else(|| ErrorKind::MissingElement("Fuu".into()))?,
                bar: self
                    .bar
                    .ok_or_else(|| ErrorKind::MissingElement("Bar".into()))?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::{
        quick_xml::{BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer},
        xml::{Mixed, Text},
    };
    #[derive(Debug)]
    pub struct NormalTypeSerializer<'ser> {
        pub(super) value: &'ser super::NormalType,
        pub(super) state: Box<NormalTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum NormalTypeSerializerState<'ser> {
        Init__,
        Group(<super::NormalGroupType as WithSerializer>::Serializer<'ser>),
        Baz(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NormalTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NormalTypeSerializerState::Init__ => {
                        *self.state = NormalTypeSerializerState::Group(WithSerializer::serializer(
                            &self.value.group,
                            Some("tns:Group"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    NormalTypeSerializerState::Group(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                NormalTypeSerializerState::Baz(WithSerializer::serializer(
                                    &self.value.baz,
                                    Some("tns:Baz"),
                                    false,
                                )?)
                        }
                    },
                    NormalTypeSerializerState::Baz(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NormalTypeSerializerState::End__,
                    },
                    NormalTypeSerializerState::End__ => {
                        *self.state = NormalTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    NormalTypeSerializerState::Done__ => return Ok(None),
                    NormalTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for NormalTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NormalTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NormalGroupTypeSerializer<'ser> {
        pub(super) value: &'ser super::NormalGroupType,
        pub(super) state: Box<NormalGroupTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum NormalGroupTypeSerializerState<'ser> {
        Init__,
        Fuu(<i32 as WithSerializer>::Serializer<'ser>),
        Bar(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NormalGroupTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NormalGroupTypeSerializerState::Init__ => {
                        *self.state = NormalGroupTypeSerializerState::Fuu(
                            WithSerializer::serializer(&self.value.fuu, Some("tns:Fuu"), false)?,
                        );
                    }
                    NormalGroupTypeSerializerState::Fuu(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                NormalGroupTypeSerializerState::Bar(WithSerializer::serializer(
                                    &self.value.bar,
                                    Some("tns:Bar"),
                                    false,
                                )?)
                        }
                    },
                    NormalGroupTypeSerializerState::Bar(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NormalGroupTypeSerializerState::Done__,
                    },
                    NormalGroupTypeSerializerState::Done__ => return Ok(None),
                    NormalGroupTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for NormalGroupTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NormalGroupTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MixedTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedType,
        pub(super) state: Box<MixedTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MixedTypeSerializerState<'ser> {
        Init__,
        TextBefore(IterSerializer<'ser, Option<&'ser Text>, Text>),
        Group(<super::MixedGroupType as WithSerializer>::Serializer<'ser>),
        Baz(<Mixed<String> as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedTypeSerializerState::Init__ => {
                        *self.state = MixedTypeSerializerState::TextBefore(IterSerializer::new(
                            self.value.text_before.as_ref(),
                            Some(""),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedTypeSerializerState::TextBefore(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                MixedTypeSerializerState::Group(WithSerializer::serializer(
                                    &self.value.group,
                                    Some("tns:Group"),
                                    false,
                                )?)
                        }
                    },
                    MixedTypeSerializerState::Group(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = MixedTypeSerializerState::Baz(WithSerializer::serializer(
                                &self.value.baz,
                                Some("tns:Baz"),
                                false,
                            )?)
                        }
                    },
                    MixedTypeSerializerState::Baz(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedTypeSerializerState::End__,
                    },
                    MixedTypeSerializerState::End__ => {
                        *self.state = MixedTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedTypeSerializerState::Done__ => return Ok(None),
                    MixedTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MixedGroupTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedGroupType,
        pub(super) state: Box<MixedGroupTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum MixedGroupTypeSerializerState<'ser> {
        Init__,
        Fuu(<Mixed<i32> as WithSerializer>::Serializer<'ser>),
        Bar(<Mixed<String> as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedGroupTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedGroupTypeSerializerState::Init__ => {
                        *self.state = MixedGroupTypeSerializerState::Fuu(
                            WithSerializer::serializer(&self.value.fuu, Some("tns:Fuu"), false)?,
                        );
                    }
                    MixedGroupTypeSerializerState::Fuu(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                MixedGroupTypeSerializerState::Bar(WithSerializer::serializer(
                                    &self.value.bar,
                                    Some("tns:Bar"),
                                    false,
                                )?)
                        }
                    },
                    MixedGroupTypeSerializerState::Bar(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedGroupTypeSerializerState::Done__,
                    },
                    MixedGroupTypeSerializerState::Done__ => return Ok(None),
                    MixedGroupTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedGroupTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedGroupTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
