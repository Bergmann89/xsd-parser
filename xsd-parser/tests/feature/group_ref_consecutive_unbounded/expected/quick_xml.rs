use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Record = RecordType;
#[derive(Debug)]
pub struct RecordType {
    pub contact: Vec<RecordContactType>,
    pub address: Vec<RecordAddressType>,
    pub name: String,
}
impl WithSerializer for RecordType {
    type Serializer<'x> = quick_xml_serialize::RecordTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::RecordTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RecordTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Record"),
            is_root,
        })
    }
}
impl WithDeserializer for RecordType {
    type Deserializer = quick_xml_deserialize::RecordTypeDeserializer;
}
#[derive(Debug)]
pub struct RecordContactType {
    pub email: String,
}
impl WithSerializer for RecordContactType {
    type Serializer<'x> = quick_xml_serialize::RecordContactTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::RecordContactTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RecordContactTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for RecordContactType {
    type Deserializer = quick_xml_deserialize::RecordContactTypeDeserializer;
}
#[derive(Debug)]
pub struct RecordAddressType {
    pub street: String,
    pub city: String,
}
impl WithSerializer for RecordAddressType {
    type Serializer<'x> = quick_xml_serialize::RecordAddressTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::RecordAddressTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::RecordAddressTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for RecordAddressType {
    type Deserializer = quick_xml_deserialize::RecordAddressTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct RecordTypeDeserializer {
        contact: Vec<super::RecordContactType>,
        address: Vec<super::RecordAddressType>,
        name: Option<String>,
        state__: Box<RecordTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RecordTypeDeserializerState {
        Init__,
        Contact(Option<<super::RecordContactType as WithDeserializer>::Deserializer>),
        Address(Option<<super::RecordAddressType as WithDeserializer>::Deserializer>),
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RecordTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                contact: Vec::new(),
                address: Vec::new(),
                name: None,
                state__: Box::new(RecordTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RecordTypeDeserializerState,
        ) -> Result<(), Error> {
            use RecordTypeDeserializerState as S;
            match state {
                S::Contact(Some(deserializer)) => {
                    self.store_contact(deserializer.finish(helper)?)?
                }
                S::Address(Some(deserializer)) => {
                    self.store_address(deserializer.finish(helper)?)?
                }
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_contact(&mut self, value: super::RecordContactType) -> Result<(), Error> {
            self.contact.push(value);
            Ok(())
        }
        fn store_address(&mut self, value: super::RecordAddressType) -> Result<(), Error> {
            self.address.push(value);
            Ok(())
        }
        fn store_name(&mut self, value: String) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn handle_contact<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RecordContactType>,
            fallback: &mut Option<RecordTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RecordTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
                if let Some(fallback) = fallback.take() {
                    self.finish_state(helper, fallback)?;
                }
                if self.contact.len() < 1usize {
                    fallback.get_or_insert(S::Contact(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    *self.state__ = S::Address(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_contact(data)?;
                    *self.state__ = S::Contact(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Contact(Some(deserializer)));
                    *self.state__ = S::Contact(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_address<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::RecordAddressType>,
            fallback: &mut Option<RecordTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RecordTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
                if let Some(fallback) = fallback.take() {
                    self.finish_state(helper, fallback)?;
                }
                if self.address.len() < 1usize {
                    fallback.get_or_insert(S::Address(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    *self.state__ = S::Name(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_address(data)?;
                    *self.state__ = S::Address(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Address(Some(deserializer)));
                    *self.state__ = S::Address(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RecordTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RecordTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Name(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_name(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Name(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RecordType> for RecordTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RecordType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RecordType> {
            use RecordTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Contact(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_contact(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Address(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_address(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_name(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Contact(None);
                        event
                    }
                    (S::Contact(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::RecordContactType as WithDeserializer>::init(helper, event)?;
                        match self.handle_contact(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Address(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::RecordAddressType as WithDeserializer>::init(helper, event)?;
                        match self.handle_address(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Name",
                            false,
                        )?;
                        match self.handle_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::RecordType, Error> {
            let state = replace(&mut *self.state__, RecordTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::RecordType {
                contact: helper.finish_vec(1usize, None, self.contact)?,
                address: helper.finish_vec(1usize, None, self.address)?,
                name: helper.finish_element("Name", self.name)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RecordContactTypeDeserializer {
        email: Option<String>,
        state__: Box<RecordContactTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RecordContactTypeDeserializerState {
        Init__,
        Email(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RecordContactTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RecordContactTypeDeserializerState,
        ) -> Result<(), Error> {
            use RecordContactTypeDeserializerState as S;
            match state {
                S::Email(Some(deserializer)) => self.store_email(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_email(&mut self, value: String) -> Result<(), Error> {
            if self.email.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Email",
                )))?;
            }
            self.email = Some(value);
            Ok(())
        }
        fn handle_email<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RecordContactTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RecordContactTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Email(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_email(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Email(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RecordContactType> for RecordContactTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RecordContactType> {
            let deserializer = Self {
                email: None,
                state__: Box::new(RecordContactTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, RecordContactTypeDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RecordContactType> {
            use RecordContactTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Email(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_email(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Email(None);
                        event
                    }
                    (S::Email(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Email",
                            false,
                        )?;
                        match self.handle_email(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RecordContactType, Error> {
            let state = replace(
                &mut *self.state__,
                RecordContactTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::RecordContactType {
                email: helper.finish_element("Email", self.email)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct RecordAddressTypeDeserializer {
        street: Option<String>,
        city: Option<String>,
        state__: Box<RecordAddressTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RecordAddressTypeDeserializerState {
        Init__,
        Street(Option<<String as WithDeserializer>::Deserializer>),
        City(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl RecordAddressTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: RecordAddressTypeDeserializerState,
        ) -> Result<(), Error> {
            use RecordAddressTypeDeserializerState as S;
            match state {
                S::Street(Some(deserializer)) => self.store_street(deserializer.finish(helper)?)?,
                S::City(Some(deserializer)) => self.store_city(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_street(&mut self, value: String) -> Result<(), Error> {
            if self.street.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Street",
                )))?;
            }
            self.street = Some(value);
            Ok(())
        }
        fn store_city(&mut self, value: String) -> Result<(), Error> {
            if self.city.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"City")))?;
            }
            self.city = Some(value);
            Ok(())
        }
        fn handle_street<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RecordAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RecordAddressTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Street(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_street(data)?;
                    *self.state__ = S::City(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Street(Some(deserializer)));
                    *self.state__ = S::City(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_city<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<RecordAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use RecordAddressTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::City(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_city(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::City(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::RecordAddressType> for RecordAddressTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RecordAddressType> {
            let deserializer = Self {
                street: None,
                city: None,
                state__: Box::new(RecordAddressTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, RecordAddressTypeDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RecordAddressType> {
            use RecordAddressTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Street(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_street(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::City(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_city(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Street(None);
                        event
                    }
                    (S::Street(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Street",
                            false,
                        )?;
                        match self.handle_street(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::City(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"City",
                            false,
                        )?;
                        match self.handle_city(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::RecordAddressType, Error> {
            let state = replace(
                &mut *self.state__,
                RecordAddressTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::RecordAddressType {
                street: helper.finish_element("Street", self.street)?,
                city: helper.finish_element("City", self.city)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct RecordTypeSerializer<'ser> {
        pub(super) value: &'ser super::RecordType,
        pub(super) state: Box<RecordTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum RecordTypeSerializerState<'ser> {
        Init__,
        Contact(IterSerializer<'ser, &'ser [super::RecordContactType], super::RecordContactType>),
        Address(IterSerializer<'ser, &'ser [super::RecordAddressType], super::RecordAddressType>),
        Name(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RecordTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RecordTypeSerializerState::Init__ => {
                        *self.state = RecordTypeSerializerState::Contact(IterSerializer::new(
                            &self.value.contact[..],
                            Some("Contact"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    RecordTypeSerializerState::Contact(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = RecordTypeSerializerState::Address(IterSerializer::new(
                                &self.value.address[..],
                                Some("Address"),
                                false,
                            ))
                        }
                    },
                    RecordTypeSerializerState::Address(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                RecordTypeSerializerState::Name(WithSerializer::serializer(
                                    &self.value.name,
                                    Some("tns:Name"),
                                    false,
                                )?)
                        }
                    },
                    RecordTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = RecordTypeSerializerState::End__,
                    },
                    RecordTypeSerializerState::End__ => {
                        *self.state = RecordTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    RecordTypeSerializerState::Done__ => return Ok(None),
                    RecordTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RecordTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RecordTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RecordContactTypeSerializer<'ser> {
        pub(super) value: &'ser super::RecordContactType,
        pub(super) state: Box<RecordContactTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum RecordContactTypeSerializerState<'ser> {
        Init__,
        Email(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RecordContactTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RecordContactTypeSerializerState::Init__ => {
                        *self.state =
                            RecordContactTypeSerializerState::Email(WithSerializer::serializer(
                                &self.value.email,
                                Some("tns:Email"),
                                false,
                            )?);
                    }
                    RecordContactTypeSerializerState::Email(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RecordContactTypeSerializerState::Done__,
                        }
                    }
                    RecordContactTypeSerializerState::Done__ => return Ok(None),
                    RecordContactTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RecordContactTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RecordContactTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct RecordAddressTypeSerializer<'ser> {
        pub(super) value: &'ser super::RecordAddressType,
        pub(super) state: Box<RecordAddressTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum RecordAddressTypeSerializerState<'ser> {
        Init__,
        Street(<String as WithSerializer>::Serializer<'ser>),
        City(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> RecordAddressTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    RecordAddressTypeSerializerState::Init__ => {
                        *self.state =
                            RecordAddressTypeSerializerState::Street(WithSerializer::serializer(
                                &self.value.street,
                                Some("tns:Street"),
                                false,
                            )?);
                    }
                    RecordAddressTypeSerializerState::Street(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = RecordAddressTypeSerializerState::City(
                                    WithSerializer::serializer(
                                        &self.value.city,
                                        Some("tns:City"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    RecordAddressTypeSerializerState::City(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RecordAddressTypeSerializerState::Done__,
                        }
                    }
                    RecordAddressTypeSerializerState::Done__ => return Ok(None),
                    RecordAddressTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for RecordAddressTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = RecordAddressTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
