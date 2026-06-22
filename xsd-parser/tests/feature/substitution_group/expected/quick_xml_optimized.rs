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
pub type List = ListType;
#[derive(Debug)]
pub struct ListType {
    pub animal: Vec<Animal>,
}
impl WithSerializer for ListType {
    type Serializer<'x> = quick_xml_serialize::ListTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ListTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ListTypeSerializerState::Init__),
            name: name.unwrap_or("list"),
            is_root,
        })
    }
}
impl WithDeserializer for ListType {
    type Deserializer = quick_xml_deserialize::ListTypeDeserializer;
}
#[derive(Debug)]
pub enum Animal {
    Animal(AnimalType),
    Dog(DogType),
    Labrador(LabradorType),
}
impl WithSerializer for Animal {
    type Serializer<'x> = quick_xml_serialize::AnimalSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        Ok(quick_xml_serialize::AnimalSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AnimalSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for Animal {
    type Deserializer = quick_xml_deserialize::AnimalDeserializer;
}
#[derive(Debug)]
pub struct AnimalType {
    pub id: i32,
}
impl WithSerializer for AnimalType {
    type Serializer<'x> = quick_xml_serialize::AnimalTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::AnimalTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AnimalTypeSerializerState::Init__),
            name: name.unwrap_or("animal"),
            is_root,
        })
    }
}
impl WithDeserializer for AnimalType {
    type Deserializer = quick_xml_deserialize::AnimalTypeDeserializer;
}
#[derive(Debug)]
pub struct DogType {
    pub id: i32,
    pub name: String,
}
impl WithSerializer for DogType {
    type Serializer<'x> = quick_xml_serialize::DogTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DogTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DogTypeSerializerState::Init__),
            name: name.unwrap_or("dog"),
            is_root,
        })
    }
}
impl WithDeserializer for DogType {
    type Deserializer = quick_xml_deserialize::DogTypeDeserializer;
}
#[derive(Debug)]
pub struct LabradorType {
    pub id: i32,
    pub name: String,
    pub color: String,
}
impl WithSerializer for LabradorType {
    type Serializer<'x> = quick_xml_serialize::LabradorTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::LabradorTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::LabradorTypeSerializerState::Init__),
            name: name.unwrap_or("labrador"),
            is_root,
        })
    }
}
impl WithDeserializer for LabradorType {
    type Deserializer = quick_xml_deserialize::LabradorTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ListTypeDeserializer {
        animal: Vec<super::Animal>,
        state__: Box<ListTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ListTypeDeserializerState {
        Init__,
        Animal(Option<<super::Animal as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ListTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                animal: Vec::new(),
                state__: Box::new(ListTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ListTypeDeserializerState,
        ) -> Result<(), Error> {
            use ListTypeDeserializerState as S;
            match state {
                S::Animal(Some(deserializer)) => self.store_animal(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_animal(&mut self, value: super::Animal) -> Result<(), Error> {
            self.animal.push(value);
            Ok(())
        }
        fn handle_animal<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::Animal>,
            fallback: &mut Option<ListTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ListTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_animal(data)?;
                    *self.state__ = S::Animal(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Animal(Some(deserializer)));
                    *self.state__ = S::Animal(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ListType> for ListTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ListType> {
            use ListTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Animal(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_animal(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Animal(None);
                        event
                    }
                    (S::Animal(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::Animal as WithDeserializer>::init(helper, event)?;
                        match self.handle_animal(helper, output, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ListType, Error> {
            let state = replace(&mut *self.state__, ListTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ListType {
                animal: self.animal,
            })
        }
    }
    #[derive(Debug)]
    pub struct AnimalDeserializer {
        state__: Box<AnimalDeserializerState>,
    }
    #[derive(Debug)]
    pub enum AnimalDeserializerState {
        Init__,
        Animal(
            Option<super::AnimalType>,
            Option<<super::AnimalType as WithDeserializer>::Deserializer>,
            Option<<super::AnimalType as WithDeserializer>::Deserializer>,
        ),
        Dog(
            Option<super::DogType>,
            Option<<super::DogType as WithDeserializer>::Deserializer>,
            Option<<super::DogType as WithDeserializer>::Deserializer>,
        ),
        Labrador(
            Option<super::LabradorType>,
            Option<<super::LabradorType as WithDeserializer>::Deserializer>,
            Option<<super::LabradorType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::Animal),
        Unknown__,
    }
    impl AnimalDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"animal")
                ) {
                    let output = <super::AnimalType as WithDeserializer>::init(helper, event)?;
                    return self.handle_animal(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"dog")
                ) {
                    let output = <super::DogType as WithDeserializer>::init(helper, event)?;
                    return self.handle_dog(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"labrador")
                ) {
                    let output = <super::LabradorType as WithDeserializer>::init(helper, event)?;
                    return self.handle_labrador(helper, Default::default(), None, output);
                }
            }
            *self.state__ = AnimalDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: AnimalDeserializerState,
        ) -> Result<super::Animal, Error> {
            use AnimalDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Animal(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_animal(&mut values, value)?;
                    }
                    Ok(super::Animal::Animal(
                        helper.finish_element("animal", values)?,
                    ))
                }
                S::Dog(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_dog(&mut values, value)?;
                    }
                    Ok(super::Animal::Dog(helper.finish_element("dog", values)?))
                }
                S::Labrador(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_labrador(&mut values, value)?;
                    }
                    Ok(super::Animal::Labrador(
                        helper.finish_element("labrador", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_animal(
            values: &mut Option<super::AnimalType>,
            value: super::AnimalType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"animal",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_dog(
            values: &mut Option<super::DogType>,
            value: super::DogType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"dog")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_labrador(
            values: &mut Option<super::LabradorType>,
            value: super::LabradorType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"labrador",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_animal<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnimalType>,
            fallback: Option<<super::AnimalType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnimalType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnimalDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_animal(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_animal(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Animal(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Animal(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_dog<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::DogType>,
            fallback: Option<<super::DogType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::DogType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnimalDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_dog(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_dog(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Dog(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Dog(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_labrador<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::LabradorType>,
            fallback: Option<<super::LabradorType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::LabradorType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnimalDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_labrador(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_labrador(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Labrador(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Labrador(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::Animal> for AnimalDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Animal> {
            let deserializer = Self {
                state__: Box::new(AnimalDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, AnimalDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::Animal> {
            use AnimalDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Animal(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_animal(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Dog(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_dog(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Labrador(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_labrador(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Animal(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"animal",
                            false,
                        )?;
                        match self.handle_animal(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Dog(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"dog",
                            false,
                        )?;
                        match self.handle_dog(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (
                        S::Labrador(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"labrador",
                            false,
                        )?;
                        match self.handle_labrador(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Animal, Error> {
            Self::finish_state(helper, *self.state__)
        }
    }
    #[derive(Debug)]
    pub struct AnimalTypeDeserializer {
        id: i32,
        state__: Box<AnimalTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnimalTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl AnimalTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                state__: Box::new(AnimalTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnimalTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::AnimalType> for AnimalTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnimalType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnimalType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AnimalType, Error> {
            let state = replace(&mut *self.state__, AnimalTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::AnimalType { id: self.id })
        }
    }
    #[derive(Debug)]
    pub struct DogTypeDeserializer {
        id: i32,
        name: String,
        state__: Box<DogTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DogTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl DogTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                state__: Box::new(DogTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DogTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::DogType> for DogTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DogType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DogType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::DogType, Error> {
            let state = replace(&mut *self.state__, DogTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::DogType {
                id: self.id,
                name: self.name,
            })
        }
    }
    #[derive(Debug)]
    pub struct LabradorTypeDeserializer {
        id: i32,
        name: String,
        color: String,
        state__: Box<LabradorTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum LabradorTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl LabradorTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            let mut name: Option<String> = None;
            let mut color: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"color")
                ) {
                    helper.read_attrib(&mut color, b"color", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                color: color.ok_or_else(|| ErrorKind::MissingAttribute("color".into()))?,
                state__: Box::new(LabradorTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: LabradorTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::LabradorType> for LabradorTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LabradorType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LabradorType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::LabradorType, Error> {
            let state = replace(&mut *self.state__, LabradorTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::LabradorType {
                id: self.id,
                name: self.name,
                color: self.color,
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
    pub struct ListTypeSerializer<'ser> {
        pub(super) value: &'ser super::ListType,
        pub(super) state: Box<ListTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ListTypeSerializerState<'ser> {
        Init__,
        Animal(IterSerializer<'ser, &'ser [super::Animal], super::Animal>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ListTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ListTypeSerializerState::Init__ => {
                        *self.state = ListTypeSerializerState::Animal(IterSerializer::new(
                            &self.value.animal[..],
                            Some("animal"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ListTypeSerializerState::Animal(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ListTypeSerializerState::End__,
                    },
                    ListTypeSerializerState::End__ => {
                        *self.state = ListTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ListTypeSerializerState::Done__ => return Ok(None),
                    ListTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ListTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ListTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AnimalSerializer<'ser> {
        pub(super) value: &'ser super::Animal,
        pub(super) state: Box<AnimalSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AnimalSerializerState<'ser> {
        Init__,
        Animal(<super::AnimalType as WithSerializer>::Serializer<'ser>),
        Dog(<super::DogType as WithSerializer>::Serializer<'ser>),
        Labrador(<super::LabradorType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnimalSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AnimalSerializerState::Init__ => {
                        match self.value {
                            super::Animal::Animal(x) => {
                                *self.state = AnimalSerializerState::Animal(
                                    WithSerializer::serializer(x, Some("animal"), self.is_root)?,
                                )
                            }
                            super::Animal::Dog(x) => {
                                *self.state = AnimalSerializerState::Dog(
                                    WithSerializer::serializer(x, Some("dog"), self.is_root)?,
                                )
                            }
                            super::Animal::Labrador(x) => {
                                *self.state = AnimalSerializerState::Labrador(
                                    WithSerializer::serializer(x, Some("labrador"), self.is_root)?,
                                )
                            }
                        }
                    }
                    AnimalSerializerState::Animal(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AnimalSerializerState::Done__,
                    },
                    AnimalSerializerState::Dog(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AnimalSerializerState::Done__,
                    },
                    AnimalSerializerState::Labrador(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AnimalSerializerState::Done__,
                    },
                    AnimalSerializerState::Done__ => return Ok(None),
                    AnimalSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for AnimalSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AnimalSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AnimalTypeSerializer<'ser> {
        pub(super) value: &'ser super::AnimalType,
        pub(super) state: Box<AnimalTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AnimalTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnimalTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AnimalTypeSerializerState::Init__ => {
                        *self.state = AnimalTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                        }
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    AnimalTypeSerializerState::Done__ => return Ok(None),
                    AnimalTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for AnimalTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AnimalTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DogTypeSerializer<'ser> {
        pub(super) value: &'ser super::DogType,
        pub(super) state: Box<DogTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DogTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DogTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DogTypeSerializerState::Init__ => {
                        *self.state = DogTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                        }
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    DogTypeSerializerState::Done__ => return Ok(None),
                    DogTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DogTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DogTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct LabradorTypeSerializer<'ser> {
        pub(super) value: &'ser super::LabradorType,
        pub(super) state: Box<LabradorTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum LabradorTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> LabradorTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    LabradorTypeSerializerState::Init__ => {
                        *self.state = LabradorTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                        }
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib(&mut bytes, "color", &self.value.color)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    LabradorTypeSerializerState::Done__ => return Ok(None),
                    LabradorTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for LabradorTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = LabradorTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
