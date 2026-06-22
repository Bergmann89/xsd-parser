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
    Cat(CatType),
    Dog(DogType),
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
pub struct CatType {
    pub id: i32,
    pub breed: String,
}
impl WithSerializer for CatType {
    type Serializer<'x> = quick_xml_serialize::CatTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CatTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CatTypeSerializerState::Init__),
            name: name.unwrap_or("cat"),
            is_root,
        })
    }
}
impl WithDeserializer for CatType {
    type Deserializer = quick_xml_deserialize::CatTypeDeserializer;
}
#[derive(Debug)]
pub enum DogType {
    Labrador(LabradorType),
    Beagle(BeagleType),
}
impl WithSerializer for DogType {
    type Serializer<'x> = quick_xml_serialize::DogTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        Ok(quick_xml_serialize::DogTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DogTypeSerializerState::Init__),
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
#[derive(Debug)]
pub struct BeagleType {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
impl WithSerializer for BeagleType {
    type Serializer<'x> = quick_xml_serialize::BeagleTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::BeagleTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::BeagleTypeSerializerState::Init__),
            name: name.unwrap_or("beagle"),
            is_root,
        })
    }
}
impl WithDeserializer for BeagleType {
    type Deserializer = quick_xml_deserialize::BeagleTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        QName, RawByteStr, WithDeserializer,
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
        Cat(
            Option<super::CatType>,
            Option<<super::CatType as WithDeserializer>::Deserializer>,
            Option<<super::CatType as WithDeserializer>::Deserializer>,
        ),
        Dog(
            Option<super::DogType>,
            Option<<super::DogType as WithDeserializer>::Deserializer>,
            Option<<super::DogType as WithDeserializer>::Deserializer>,
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
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"cat")
                ) {
                    let output = <super::CatType as WithDeserializer>::init(helper, event)?;
                    return self.handle_cat(helper, Default::default(), None, output);
                }
                event = {
                    let output = <super::DogType as WithDeserializer>::init(helper, event)?;
                    match self.handle_dog(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = AnimalDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: AnimalDeserializerState,
        ) -> Result<super::Animal, Error> {
            use AnimalDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Cat(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_cat(&mut values, value)?;
                    }
                    Ok(super::Animal::Cat(helper.finish_element("cat", values)?))
                }
                S::Dog(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_dog(&mut values, value)?;
                    }
                    Ok(super::Animal::Dog(helper.finish_element("dog", values)?))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_cat(
            values: &mut Option<super::CatType>,
            value: super::CatType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"cat")))?;
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
        fn handle_cat<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::CatType>,
            fallback: Option<<super::CatType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::CatType>,
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
                Self::store_cat(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_cat(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Cat(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Cat(values, None, Some(deserializer));
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
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
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
                    (S::Cat(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_cat(helper, values, fallback, output)? {
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
                        S::Cat(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"cat",
                            false,
                        )?;
                        match self.handle_cat(helper, values, fallback, output)? {
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
                        let output = <super::DogType as WithDeserializer>::init(helper, event)?;
                        match self.handle_dog(helper, values, fallback, output)? {
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
    pub struct CatTypeDeserializer {
        id: i32,
        breed: String,
        state__: Box<CatTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CatTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl CatTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            let mut breed: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"breed")
                ) {
                    helper.read_attrib(&mut breed, b"breed", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                breed: breed.ok_or_else(|| ErrorKind::MissingAttribute("breed".into()))?,
                state__: Box::new(CatTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: CatTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::CatType> for CatTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CatType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CatType> {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::CatType, Error> {
            let state = replace(&mut *self.state__, CatTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::CatType {
                id: self.id,
                breed: self.breed,
            })
        }
    }
    #[derive(Debug)]
    pub struct DogTypeDeserializer {
        state__: Box<DogTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum DogTypeDeserializerState {
        Init__,
        Labrador(
            Option<super::LabradorType>,
            Option<<super::LabradorType as WithDeserializer>::Deserializer>,
            Option<<super::LabradorType as WithDeserializer>::Deserializer>,
        ),
        Beagle(
            Option<super::BeagleType>,
            Option<<super::BeagleType as WithDeserializer>::Deserializer>,
            Option<<super::BeagleType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::DogType),
        Unknown__,
    }
    impl DogTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"dog")
                ) {
                    if let Some(type_name) = helper.get_dynamic_type_from_attrib_bytes(x)? {
                        let type_name = type_name.into_owned();
                        if matches!(
                            helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                            Some(b"labrador")
                        ) {
                            let output =
                                <super::LabradorType as WithDeserializer>::init(helper, event)?;
                            return self.handle_labrador(helper, Default::default(), None, output);
                        }
                        if matches!(
                            helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                            Some(b"beagle")
                        ) {
                            let output =
                                <super::BeagleType as WithDeserializer>::init(helper, event)?;
                            return self.handle_beagle(helper, Default::default(), None, output);
                        }
                    }
                }
            }
            *self.state__ = DogTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: DogTypeDeserializerState,
        ) -> Result<super::DogType, Error> {
            use DogTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Labrador(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_labrador(&mut values, value)?;
                    }
                    Ok(super::DogType::Labrador(
                        helper.finish_element("labrador", values)?,
                    ))
                }
                S::Beagle(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_beagle(&mut values, value)?;
                    }
                    Ok(super::DogType::Beagle(
                        helper.finish_element("beagle", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
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
        fn store_beagle(
            values: &mut Option<super::BeagleType>,
            value: super::BeagleType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"beagle",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_labrador<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::LabradorType>,
            fallback: Option<<super::LabradorType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::LabradorType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DogTypeDeserializerState as S;
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
        fn handle_beagle<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::BeagleType>,
            fallback: Option<<super::BeagleType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::BeagleType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DogTypeDeserializerState as S;
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
                Self::store_beagle(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_beagle(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Beagle(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Beagle(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DogType> for DogTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DogType> {
            let deserializer = Self {
                state__: Box::new(DogTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, DogTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::DogType> {
            use DogTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Labrador(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_labrador(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Beagle(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_beagle(helper, values, fallback, output)? {
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
                    (
                        S::Beagle(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"beagle",
                            false,
                        )?;
                        match self.handle_beagle(helper, values, fallback, output)? {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::DogType, Error> {
            Self::finish_state(helper, *self.state__)
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
    #[derive(Debug)]
    pub struct BeagleTypeDeserializer {
        id: i32,
        name: String,
        age: i32,
        state__: Box<BeagleTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum BeagleTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl BeagleTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            let mut name: Option<String> = None;
            let mut age: Option<i32> = None;
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
                    Some(b"age")
                ) {
                    helper.read_attrib(&mut age, b"age", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                age: age.ok_or_else(|| ErrorKind::MissingAttribute("age".into()))?,
                state__: Box::new(BeagleTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: BeagleTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::BeagleType> for BeagleTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::BeagleType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::BeagleType> {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::BeagleType, Error> {
            let state = replace(&mut *self.state__, BeagleTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::BeagleType {
                id: self.id,
                name: self.name,
                age: self.age,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer, XsiTypeSerializer,
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
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_XSI),
                                &super::NS_XSI,
                            );
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
        Cat(<super::CatType as WithSerializer>::Serializer<'ser>),
        Dog(<super::DogType as WithSerializer>::Serializer<'ser>),
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
                            super::Animal::Cat(x) => {
                                *self.state = AnimalSerializerState::Cat(
                                    WithSerializer::serializer(x, Some("cat"), self.is_root)?,
                                )
                            }
                            super::Animal::Dog(x) => {
                                *self.state = AnimalSerializerState::Dog(
                                    WithSerializer::serializer(x, Some("dog"), self.is_root)?,
                                )
                            }
                        }
                    }
                    AnimalSerializerState::Cat(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AnimalSerializerState::Done__,
                    },
                    AnimalSerializerState::Dog(x) => match x.next(helper).transpose()? {
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
    pub struct CatTypeSerializer<'ser> {
        pub(super) value: &'ser super::CatType,
        pub(super) state: Box<CatTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CatTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CatTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CatTypeSerializerState::Init__ => {
                        *self.state = CatTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                        }
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        helper.write_attrib(&mut bytes, "breed", &self.value.breed)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    CatTypeSerializerState::Done__ => return Ok(None),
                    CatTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for CatTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CatTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DogTypeSerializer<'ser> {
        pub(super) value: &'ser super::DogType,
        pub(super) state: Box<DogTypeSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DogTypeSerializerState<'ser> {
        Init__,
        Labrador(
            XsiTypeSerializer<'ser, <super::LabradorType as WithSerializer>::Serializer<'ser>>,
        ),
        Beagle(XsiTypeSerializer<'ser, <super::BeagleType as WithSerializer>::Serializer<'ser>>),
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
                    DogTypeSerializerState::Init__ => match self.value {
                        super::DogType::Labrador(x) => {
                            *self.state = DogTypeSerializerState::Labrador(XsiTypeSerializer::new(
                                WithSerializer::serializer(x, Some("labrador"), self.is_root)?,
                                "dog",
                                self.is_root,
                            ))
                        }
                        super::DogType::Beagle(x) => {
                            *self.state = DogTypeSerializerState::Beagle(XsiTypeSerializer::new(
                                WithSerializer::serializer(x, Some("beagle"), self.is_root)?,
                                "dog",
                                self.is_root,
                            ))
                        }
                    },
                    DogTypeSerializerState::Labrador(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DogTypeSerializerState::Done__,
                    },
                    DogTypeSerializerState::Beagle(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DogTypeSerializerState::Done__,
                    },
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
    #[derive(Debug)]
    pub struct BeagleTypeSerializer<'ser> {
        pub(super) value: &'ser super::BeagleType,
        pub(super) state: Box<BeagleTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum BeagleTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> BeagleTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    BeagleTypeSerializerState::Init__ => {
                        *self.state = BeagleTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns_for_tag(&mut bytes, self.name, &super::NS_TNS);
                        }
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib(&mut bytes, "age", &self.value.age)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    BeagleTypeSerializerState::Done__ => return Ok(None),
                    BeagleTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for BeagleTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = BeagleTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
