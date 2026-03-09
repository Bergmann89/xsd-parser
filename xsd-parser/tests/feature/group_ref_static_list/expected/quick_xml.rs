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
pub type Person = PersonType;
#[derive(Debug)]
pub struct PersonType {
    pub address: Vec<PersonAddressType>,
    pub name: String,
}
impl WithSerializer for PersonType {
    type Serializer<'x> = quick_xml_serialize::PersonTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PersonTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PersonTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Person"),
            is_root,
        })
    }
}
impl WithDeserializer for PersonType {
    type Deserializer = quick_xml_deserialize::PersonTypeDeserializer;
}
#[derive(Debug)]
pub struct PersonAddressType {
    pub street: String,
    pub city: String,
}
impl WithSerializer for PersonAddressType {
    type Serializer<'x> = quick_xml_serialize::PersonAddressTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::PersonAddressTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PersonAddressTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for PersonAddressType {
    type Deserializer = quick_xml_deserialize::PersonAddressTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct PersonTypeDeserializer {
        address: Vec<super::PersonAddressType>,
        name: Option<String>,
        state__: Box<PersonTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PersonTypeDeserializerState {
        Init__,
        Address(Option<<super::PersonAddressType as WithDeserializer>::Deserializer>),
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl PersonTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                address: Vec::new(),
                name: None,
                state__: Box::new(PersonTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PersonTypeDeserializerState,
        ) -> Result<(), Error> {
            use PersonTypeDeserializerState as S;
            match state {
                S::Address(Some(deserializer)) => {
                    self.store_address(deserializer.finish(helper)?)?
                }
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_address(&mut self, value: super::PersonAddressType) -> Result<(), Error> {
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
        fn handle_address<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::PersonAddressType>,
            fallback: &mut Option<PersonTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PersonTypeDeserializerState as S;
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
                    if self.address.len() < 3usize {
                        *self.state__ = S::Address(None);
                    } else {
                        *self.state__ = S::Name(None);
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Address(Some(deserializer)));
                    if self.address.len() < 2usize {
                        *self.state__ = S::Address(None);
                    } else {
                        *self.state__ = S::Name(None);
                    }
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<PersonTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PersonTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::PersonType> for PersonTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PersonType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PersonType> {
            use PersonTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
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
                        *self.state__ = S::Address(None);
                        event
                    }
                    (S::Address(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            <super::PersonAddressType as WithDeserializer>::init(helper, event)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::PersonType, Error> {
            let state = replace(&mut *self.state__, PersonTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::PersonType {
                address: helper.finish_vec(1usize, Some(3usize), self.address)?,
                name: helper.finish_element("Name", self.name)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PersonAddressTypeDeserializer {
        street: Option<String>,
        city: Option<String>,
        state__: Box<PersonAddressTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PersonAddressTypeDeserializerState {
        Init__,
        Street(Option<<String as WithDeserializer>::Deserializer>),
        City(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl PersonAddressTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: PersonAddressTypeDeserializerState,
        ) -> Result<(), Error> {
            use PersonAddressTypeDeserializerState as S;
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
            fallback: &mut Option<PersonAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PersonAddressTypeDeserializerState as S;
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
            fallback: &mut Option<PersonAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use PersonAddressTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::PersonAddressType> for PersonAddressTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PersonAddressType> {
            let deserializer = Self {
                street: None,
                city: None,
                state__: Box::new(PersonAddressTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, PersonAddressTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::PersonAddressType> {
            use PersonAddressTypeDeserializerState as S;
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
        ) -> Result<super::PersonAddressType, Error> {
            let state = replace(
                &mut *self.state__,
                PersonAddressTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PersonAddressType {
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
    pub struct PersonTypeSerializer<'ser> {
        pub(super) value: &'ser super::PersonType,
        pub(super) state: Box<PersonTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PersonTypeSerializerState<'ser> {
        Init__,
        Address(IterSerializer<'ser, &'ser [super::PersonAddressType], super::PersonAddressType>),
        Name(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PersonTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PersonTypeSerializerState::Init__ => {
                        *self.state = PersonTypeSerializerState::Address(IterSerializer::new(
                            &self.value.address[..],
                            Some("Address"),
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
                    PersonTypeSerializerState::Address(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                PersonTypeSerializerState::Name(WithSerializer::serializer(
                                    &self.value.name,
                                    Some("tns:Name"),
                                    false,
                                )?)
                        }
                    },
                    PersonTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PersonTypeSerializerState::End__,
                    },
                    PersonTypeSerializerState::End__ => {
                        *self.state = PersonTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PersonTypeSerializerState::Done__ => return Ok(None),
                    PersonTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PersonTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PersonTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PersonAddressTypeSerializer<'ser> {
        pub(super) value: &'ser super::PersonAddressType,
        pub(super) state: Box<PersonAddressTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum PersonAddressTypeSerializerState<'ser> {
        Init__,
        Street(<String as WithSerializer>::Serializer<'ser>),
        City(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PersonAddressTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PersonAddressTypeSerializerState::Init__ => {
                        *self.state =
                            PersonAddressTypeSerializerState::Street(WithSerializer::serializer(
                                &self.value.street,
                                Some("tns:Street"),
                                false,
                            )?);
                    }
                    PersonAddressTypeSerializerState::Street(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PersonAddressTypeSerializerState::City(
                                    WithSerializer::serializer(
                                        &self.value.city,
                                        Some("tns:City"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    PersonAddressTypeSerializerState::City(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PersonAddressTypeSerializerState::Done__,
                        }
                    }
                    PersonAddressTypeSerializerState::Done__ => return Ok(None),
                    PersonAddressTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for PersonAddressTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PersonAddressTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
