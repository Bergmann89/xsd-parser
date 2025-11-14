use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub type Shiporder = ShiporderType;
#[derive(Debug)]
pub struct ShiporderType {
    pub orderid: String,
    pub orderperson: String,
    pub shipto: ShiporderShiptoType,
    pub item: Vec<ShiporderItemType>,
}
impl WithSerializer for ShiporderType {
    type Serializer<'x> = quick_xml_serialize::ShiporderTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ShiporderTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ShiporderTypeSerializerState::Init__),
            name: name.unwrap_or("shiporder"),
            is_root,
        })
    }
}
impl WithDeserializer for ShiporderType {
    type Deserializer = quick_xml_deserialize::ShiporderTypeDeserializer;
}
#[derive(Debug)]
pub struct ShiporderShiptoType {
    pub name: String,
    pub address: String,
    pub city: String,
    pub country: String,
}
impl WithSerializer for ShiporderShiptoType {
    type Serializer<'x> = quick_xml_serialize::ShiporderShiptoTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ShiporderShiptoTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ShiporderShiptoTypeSerializerState::Init__),
            name: name.unwrap_or("ShiporderShipto"),
            is_root,
        })
    }
}
impl WithDeserializer for ShiporderShiptoType {
    type Deserializer = quick_xml_deserialize::ShiporderShiptoTypeDeserializer;
}
#[derive(Debug)]
pub struct ShiporderItemType {
    pub title: String,
    pub note: Option<String>,
    pub quantity: usize,
    pub price: f64,
}
impl WithSerializer for ShiporderItemType {
    type Serializer<'x> = quick_xml_serialize::ShiporderItemTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ShiporderItemTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ShiporderItemTypeSerializerState::Init__),
            name: name.unwrap_or("ShiporderItem"),
            is_root,
        })
    }
}
impl WithDeserializer for ShiporderItemType {
    type Deserializer = quick_xml_deserialize::ShiporderItemTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ShiporderTypeDeserializer {
        orderid: String,
        orderperson: Option<String>,
        shipto: Option<super::ShiporderShiptoType>,
        item: Vec<super::ShiporderItemType>,
        state__: Box<ShiporderTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ShiporderTypeDeserializerState {
        Init__,
        Orderperson(Option<<String as WithDeserializer>::Deserializer>),
        Shipto(Option<<super::ShiporderShiptoType as WithDeserializer>::Deserializer>),
        Item(Option<<super::ShiporderItemType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ShiporderTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut orderid: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"orderid" {
                    reader.read_attrib(&mut orderid, b"orderid", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
            }
            Ok(Self {
                orderid: orderid.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("orderid".into()))
                })?,
                orderperson: None,
                shipto: None,
                item: Vec::new(),
                state__: Box::new(ShiporderTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ShiporderTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ShiporderTypeDeserializerState as S;
            match state {
                S::Orderperson(Some(deserializer)) => {
                    self.store_orderperson(deserializer.finish(reader)?)?
                }
                S::Shipto(Some(deserializer)) => self.store_shipto(deserializer.finish(reader)?)?,
                S::Item(Some(deserializer)) => self.store_item(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_orderperson(&mut self, value: String) -> Result<(), Error> {
            if self.orderperson.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"orderperson",
                )))?;
            }
            self.orderperson = Some(value);
            Ok(())
        }
        fn store_shipto(&mut self, value: super::ShiporderShiptoType) -> Result<(), Error> {
            if self.shipto.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"shipto",
                )))?;
            }
            self.shipto = Some(value);
            Ok(())
        }
        fn store_item(&mut self, value: super::ShiporderItemType) -> Result<(), Error> {
            self.item.push(value);
            Ok(())
        }
        fn handle_orderperson<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ShiporderTypeDeserializerState>,
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
                if self.orderperson.is_some() {
                    fallback.get_or_insert(ShiporderTypeDeserializerState::Orderperson(None));
                    *self.state__ = ShiporderTypeDeserializerState::Shipto(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = ShiporderTypeDeserializerState::Orderperson(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_orderperson(data)?;
                    *self.state__ = ShiporderTypeDeserializerState::Shipto(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderTypeDeserializerState::Orderperson(
                                Some(deserializer),
                            ));
                            *self.state__ = ShiporderTypeDeserializerState::Shipto(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderTypeDeserializerState::Orderperson(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_shipto<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ShiporderShiptoType>,
            fallback: &mut Option<ShiporderTypeDeserializerState>,
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
                if self.shipto.is_some() {
                    fallback.get_or_insert(ShiporderTypeDeserializerState::Shipto(None));
                    *self.state__ = ShiporderTypeDeserializerState::Item(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = ShiporderTypeDeserializerState::Shipto(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_shipto(data)?;
                    *self.state__ = ShiporderTypeDeserializerState::Item(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderTypeDeserializerState::Shipto(Some(
                                deserializer,
                            )));
                            *self.state__ = ShiporderTypeDeserializerState::Item(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderTypeDeserializerState::Shipto(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_item<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ShiporderItemType>,
            fallback: &mut Option<ShiporderTypeDeserializerState>,
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
                if self.item.len() < 1usize {
                    *self.state__ = ShiporderTypeDeserializerState::Item(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(ShiporderTypeDeserializerState::Item(None));
                    *self.state__ = ShiporderTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_item(data)?;
                    *self.state__ = ShiporderTypeDeserializerState::Item(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderTypeDeserializerState::Item(Some(
                                deserializer,
                            )));
                            *self.state__ = ShiporderTypeDeserializerState::Item(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderTypeDeserializerState::Item(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ShiporderType> for ShiporderTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ShiporderType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ShiporderType>
        where
            R: DeserializeReader,
        {
            use ShiporderTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Orderperson(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_orderperson(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Shipto(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_shipto(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Item(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_item(reader, output, &mut fallback)? {
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
                        *self.state__ = ShiporderTypeDeserializerState::Orderperson(None);
                        event
                    }
                    (S::Orderperson(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            None,
                            b"orderperson",
                            false,
                        )?;
                        match self.handle_orderperson(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Shipto(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"shipto", false)?;
                        match self.handle_shipto(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Item(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"item", false)?;
                        match self.handle_item(reader, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Done__);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::ShiporderType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state__,
                ShiporderTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ShiporderType {
                orderid: self.orderid,
                orderperson: self
                    .orderperson
                    .ok_or_else(|| ErrorKind::MissingElement("orderperson".into()))?,
                shipto: self
                    .shipto
                    .ok_or_else(|| ErrorKind::MissingElement("shipto".into()))?,
                item: self.item,
            })
        }
    }
    #[derive(Debug)]
    pub struct ShiporderShiptoTypeDeserializer {
        name: Option<String>,
        address: Option<String>,
        city: Option<String>,
        country: Option<String>,
        state__: Box<ShiporderShiptoTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ShiporderShiptoTypeDeserializerState {
        Init__,
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Address(Option<<String as WithDeserializer>::Deserializer>),
        City(Option<<String as WithDeserializer>::Deserializer>),
        Country(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ShiporderShiptoTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                name: None,
                address: None,
                city: None,
                country: None,
                state__: Box::new(ShiporderShiptoTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ShiporderShiptoTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ShiporderShiptoTypeDeserializerState as S;
            match state {
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(reader)?)?,
                S::Address(Some(deserializer)) => {
                    self.store_address(deserializer.finish(reader)?)?
                }
                S::City(Some(deserializer)) => self.store_city(deserializer.finish(reader)?)?,
                S::Country(Some(deserializer)) => {
                    self.store_country(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_name(&mut self, value: String) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_address(&mut self, value: String) -> Result<(), Error> {
            if self.address.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"address",
                )))?;
            }
            self.address = Some(value);
            Ok(())
        }
        fn store_city(&mut self, value: String) -> Result<(), Error> {
            if self.city.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"city")))?;
            }
            self.city = Some(value);
            Ok(())
        }
        fn store_country(&mut self, value: String) -> Result<(), Error> {
            if self.country.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"country",
                )))?;
            }
            self.country = Some(value);
            Ok(())
        }
        fn handle_name<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ShiporderShiptoTypeDeserializerState>,
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
                if self.name.is_some() {
                    fallback.get_or_insert(ShiporderShiptoTypeDeserializerState::Name(None));
                    *self.state__ = ShiporderShiptoTypeDeserializerState::Address(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = ShiporderShiptoTypeDeserializerState::Name(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_name(data)?;
                    *self.state__ = ShiporderShiptoTypeDeserializerState::Address(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderShiptoTypeDeserializerState::Name(
                                Some(deserializer),
                            ));
                            *self.state__ = ShiporderShiptoTypeDeserializerState::Address(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderShiptoTypeDeserializerState::Name(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_address<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ShiporderShiptoTypeDeserializerState>,
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
                if self.address.is_some() {
                    fallback.get_or_insert(ShiporderShiptoTypeDeserializerState::Address(None));
                    *self.state__ = ShiporderShiptoTypeDeserializerState::City(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = ShiporderShiptoTypeDeserializerState::Address(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_address(data)?;
                    *self.state__ = ShiporderShiptoTypeDeserializerState::City(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderShiptoTypeDeserializerState::Address(
                                Some(deserializer),
                            ));
                            *self.state__ = ShiporderShiptoTypeDeserializerState::City(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderShiptoTypeDeserializerState::Address(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_city<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ShiporderShiptoTypeDeserializerState>,
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
                if self.city.is_some() {
                    fallback.get_or_insert(ShiporderShiptoTypeDeserializerState::City(None));
                    *self.state__ = ShiporderShiptoTypeDeserializerState::Country(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = ShiporderShiptoTypeDeserializerState::City(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_city(data)?;
                    *self.state__ = ShiporderShiptoTypeDeserializerState::Country(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderShiptoTypeDeserializerState::City(
                                Some(deserializer),
                            ));
                            *self.state__ = ShiporderShiptoTypeDeserializerState::Country(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderShiptoTypeDeserializerState::City(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_country<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ShiporderShiptoTypeDeserializerState>,
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
                if self.country.is_some() {
                    fallback.get_or_insert(ShiporderShiptoTypeDeserializerState::Country(None));
                    *self.state__ = ShiporderShiptoTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = ShiporderShiptoTypeDeserializerState::Country(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_country(data)?;
                    *self.state__ = ShiporderShiptoTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderShiptoTypeDeserializerState::Country(
                                Some(deserializer),
                            ));
                            *self.state__ = ShiporderShiptoTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderShiptoTypeDeserializerState::Country(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ShiporderShiptoType> for ShiporderShiptoTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ShiporderShiptoType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ShiporderShiptoType>
        where
            R: DeserializeReader,
        {
            use ShiporderShiptoTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_name(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_address(reader, output, &mut fallback)? {
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
                        let output = deserializer.next(reader, event)?;
                        match self.handle_city(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Country(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_country(reader, output, &mut fallback)? {
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
                        *self.state__ = ShiporderShiptoTypeDeserializerState::Name(None);
                        event
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"name", false)?;
                        match self.handle_name(reader, output, &mut fallback)? {
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
                            reader.init_start_tag_deserializer(event, None, b"address", false)?;
                        match self.handle_address(reader, output, &mut fallback)? {
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
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"city", false)?;
                        match self.handle_city(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Country(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"country", false)?;
                        match self.handle_country(reader, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Done__);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::ShiporderShiptoType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state__,
                ShiporderShiptoTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ShiporderShiptoType {
                name: self
                    .name
                    .ok_or_else(|| ErrorKind::MissingElement("name".into()))?,
                address: self
                    .address
                    .ok_or_else(|| ErrorKind::MissingElement("address".into()))?,
                city: self
                    .city
                    .ok_or_else(|| ErrorKind::MissingElement("city".into()))?,
                country: self
                    .country
                    .ok_or_else(|| ErrorKind::MissingElement("country".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ShiporderItemTypeDeserializer {
        title: Option<String>,
        note: Option<String>,
        quantity: Option<usize>,
        price: Option<f64>,
        state__: Box<ShiporderItemTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ShiporderItemTypeDeserializerState {
        Init__,
        Title(Option<<String as WithDeserializer>::Deserializer>),
        Note(Option<<String as WithDeserializer>::Deserializer>),
        Quantity(Option<<usize as WithDeserializer>::Deserializer>),
        Price(Option<<f64 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ShiporderItemTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                title: None,
                note: None,
                quantity: None,
                price: None,
                state__: Box::new(ShiporderItemTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ShiporderItemTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ShiporderItemTypeDeserializerState as S;
            match state {
                S::Title(Some(deserializer)) => self.store_title(deserializer.finish(reader)?)?,
                S::Note(Some(deserializer)) => self.store_note(deserializer.finish(reader)?)?,
                S::Quantity(Some(deserializer)) => {
                    self.store_quantity(deserializer.finish(reader)?)?
                }
                S::Price(Some(deserializer)) => self.store_price(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_title(&mut self, value: String) -> Result<(), Error> {
            if self.title.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"title",
                )))?;
            }
            self.title = Some(value);
            Ok(())
        }
        fn store_note(&mut self, value: String) -> Result<(), Error> {
            if self.note.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"note")))?;
            }
            self.note = Some(value);
            Ok(())
        }
        fn store_quantity(&mut self, value: usize) -> Result<(), Error> {
            if self.quantity.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"quantity",
                )))?;
            }
            self.quantity = Some(value);
            Ok(())
        }
        fn store_price(&mut self, value: f64) -> Result<(), Error> {
            if self.price.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"price",
                )))?;
            }
            self.price = Some(value);
            Ok(())
        }
        fn handle_title<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ShiporderItemTypeDeserializerState>,
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
                if self.title.is_some() {
                    fallback.get_or_insert(ShiporderItemTypeDeserializerState::Title(None));
                    *self.state__ = ShiporderItemTypeDeserializerState::Note(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = ShiporderItemTypeDeserializerState::Title(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_title(data)?;
                    *self.state__ = ShiporderItemTypeDeserializerState::Note(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderItemTypeDeserializerState::Title(
                                Some(deserializer),
                            ));
                            *self.state__ = ShiporderItemTypeDeserializerState::Note(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderItemTypeDeserializerState::Title(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_note<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ShiporderItemTypeDeserializerState>,
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
                fallback.get_or_insert(ShiporderItemTypeDeserializerState::Note(None));
                *self.state__ = ShiporderItemTypeDeserializerState::Quantity(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_note(data)?;
                    *self.state__ = ShiporderItemTypeDeserializerState::Quantity(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderItemTypeDeserializerState::Note(Some(
                                deserializer,
                            )));
                            *self.state__ = ShiporderItemTypeDeserializerState::Quantity(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderItemTypeDeserializerState::Note(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_quantity<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, usize>,
            fallback: &mut Option<ShiporderItemTypeDeserializerState>,
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
                if self.quantity.is_some() {
                    fallback.get_or_insert(ShiporderItemTypeDeserializerState::Quantity(None));
                    *self.state__ = ShiporderItemTypeDeserializerState::Price(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = ShiporderItemTypeDeserializerState::Quantity(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_quantity(data)?;
                    *self.state__ = ShiporderItemTypeDeserializerState::Price(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderItemTypeDeserializerState::Quantity(
                                Some(deserializer),
                            ));
                            *self.state__ = ShiporderItemTypeDeserializerState::Price(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderItemTypeDeserializerState::Quantity(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_price<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, f64>,
            fallback: &mut Option<ShiporderItemTypeDeserializerState>,
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
                if self.price.is_some() {
                    fallback.get_or_insert(ShiporderItemTypeDeserializerState::Price(None));
                    *self.state__ = ShiporderItemTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = ShiporderItemTypeDeserializerState::Price(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_price(data)?;
                    *self.state__ = ShiporderItemTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ShiporderItemTypeDeserializerState::Price(
                                Some(deserializer),
                            ));
                            *self.state__ = ShiporderItemTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                ShiporderItemTypeDeserializerState::Price(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ShiporderItemType> for ShiporderItemTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ShiporderItemType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ShiporderItemType>
        where
            R: DeserializeReader,
        {
            use ShiporderItemTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Title(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_title(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Note(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_note(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Quantity(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_quantity(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Price(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_price(reader, output, &mut fallback)? {
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
                        *self.state__ = ShiporderItemTypeDeserializerState::Title(None);
                        event
                    }
                    (S::Title(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"title", false)?;
                        match self.handle_title(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Note(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"note", false)?;
                        match self.handle_note(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Quantity(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"quantity", false)?;
                        match self.handle_quantity(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Price(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output =
                            reader.init_start_tag_deserializer(event, None, b"price", false)?;
                        match self.handle_price(reader, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Done__);
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
        fn finish<R>(mut self, reader: &R) -> Result<super::ShiporderItemType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state__,
                ShiporderItemTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ShiporderItemType {
                title: self
                    .title
                    .ok_or_else(|| ErrorKind::MissingElement("title".into()))?,
                note: self.note,
                quantity: self
                    .quantity
                    .ok_or_else(|| ErrorKind::MissingElement("quantity".into()))?,
                price: self
                    .price
                    .ok_or_else(|| ErrorKind::MissingElement("price".into()))?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser::quick_xml::{
        write_attrib, BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct ShiporderTypeSerializer<'ser> {
        pub(super) value: &'ser super::ShiporderType,
        pub(super) state: Box<ShiporderTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ShiporderTypeSerializerState<'ser> {
        Init__,
        Orderperson(<String as WithSerializer>::Serializer<'ser>),
        Shipto(<super::ShiporderShiptoType as WithSerializer>::Serializer<'ser>),
        Item(IterSerializer<'ser, &'ser [super::ShiporderItemType], super::ShiporderItemType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ShiporderTypeSerializerState::Init__ => {
                        *self.state =
                            ShiporderTypeSerializerState::Orderperson(WithSerializer::serializer(
                                &self.value.orderperson,
                                Some("orderperson"),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "orderid", &self.value.orderid)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ShiporderTypeSerializerState::Orderperson(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                ShiporderTypeSerializerState::Shipto(WithSerializer::serializer(
                                    &self.value.shipto,
                                    Some("shipto"),
                                    false,
                                )?)
                        }
                    },
                    ShiporderTypeSerializerState::Shipto(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ShiporderTypeSerializerState::Item(
                                    IterSerializer::new(&self.value.item[..], Some("item"), false),
                                )
                            }
                        }
                    }
                    ShiporderTypeSerializerState::Item(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ShiporderTypeSerializerState::End__,
                    },
                    ShiporderTypeSerializerState::End__ => {
                        *self.state = ShiporderTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderTypeSerializerState::Done__ => return Ok(None),
                    ShiporderTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ShiporderTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ShiporderTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ShiporderShiptoTypeSerializer<'ser> {
        pub(super) value: &'ser super::ShiporderShiptoType,
        pub(super) state: Box<ShiporderShiptoTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ShiporderShiptoTypeSerializerState<'ser> {
        Init__,
        Name(<String as WithSerializer>::Serializer<'ser>),
        Address(<String as WithSerializer>::Serializer<'ser>),
        City(<String as WithSerializer>::Serializer<'ser>),
        Country(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderShiptoTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ShiporderShiptoTypeSerializerState::Init__ => {
                        *self.state = ShiporderShiptoTypeSerializerState::Name(
                            WithSerializer::serializer(&self.value.name, Some("name"), false)?,
                        );
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ShiporderShiptoTypeSerializerState::Name(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ShiporderShiptoTypeSerializerState::Address(
                                WithSerializer::serializer(
                                    &self.value.address,
                                    Some("address"),
                                    false,
                                )?,
                            )
                        }
                    },
                    ShiporderShiptoTypeSerializerState::Address(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ShiporderShiptoTypeSerializerState::City(
                                WithSerializer::serializer(&self.value.city, Some("city"), false)?,
                            )
                        }
                    },
                    ShiporderShiptoTypeSerializerState::City(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ShiporderShiptoTypeSerializerState::Country(
                                WithSerializer::serializer(
                                    &self.value.country,
                                    Some("country"),
                                    false,
                                )?,
                            )
                        }
                    },
                    ShiporderShiptoTypeSerializerState::Country(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ShiporderShiptoTypeSerializerState::End__,
                    },
                    ShiporderShiptoTypeSerializerState::End__ => {
                        *self.state = ShiporderShiptoTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderShiptoTypeSerializerState::Done__ => return Ok(None),
                    ShiporderShiptoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ShiporderShiptoTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ShiporderShiptoTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ShiporderItemTypeSerializer<'ser> {
        pub(super) value: &'ser super::ShiporderItemType,
        pub(super) state: Box<ShiporderItemTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ShiporderItemTypeSerializerState<'ser> {
        Init__,
        Title(<String as WithSerializer>::Serializer<'ser>),
        Note(IterSerializer<'ser, Option<&'ser String>, String>),
        Quantity(<usize as WithSerializer>::Serializer<'ser>),
        Price(<f64 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderItemTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ShiporderItemTypeSerializerState::Init__ => {
                        *self.state = ShiporderItemTypeSerializerState::Title(
                            WithSerializer::serializer(&self.value.title, Some("title"), false)?,
                        );
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ShiporderItemTypeSerializerState::Title(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ShiporderItemTypeSerializerState::Note(
                                IterSerializer::new(self.value.note.as_ref(), Some("note"), false),
                            )
                        }
                    },
                    ShiporderItemTypeSerializerState::Note(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ShiporderItemTypeSerializerState::Quantity(
                                WithSerializer::serializer(
                                    &self.value.quantity,
                                    Some("quantity"),
                                    false,
                                )?,
                            )
                        }
                    },
                    ShiporderItemTypeSerializerState::Quantity(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                ShiporderItemTypeSerializerState::Price(WithSerializer::serializer(
                                    &self.value.price,
                                    Some("price"),
                                    false,
                                )?)
                        }
                    },
                    ShiporderItemTypeSerializerState::Price(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ShiporderItemTypeSerializerState::End__,
                    },
                    ShiporderItemTypeSerializerState::End__ => {
                        *self.state = ShiporderItemTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderItemTypeSerializerState::Done__ => return Ok(None),
                    ShiporderItemTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ShiporderItemTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ShiporderItemTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
