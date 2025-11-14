use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type Array = ArrayType;
#[derive(Debug)]
pub struct ArrayType {
    pub item: [i32; 5usize],
}
impl WithSerializer for ArrayType {
    type Serializer<'x> = quick_xml_serialize::ArrayTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ArrayTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ArrayTypeSerializerState::Init__),
            name: name.unwrap_or("ArrayType"),
            is_root,
        })
    }
}
impl WithDeserializer for ArrayType {
    type Deserializer = quick_xml_deserialize::ArrayTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ArrayTypeDeserializer {
        item: Vec<i32>,
        state__: Box<ArrayTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ArrayTypeDeserializerState {
        Init__,
        Next__,
        Item(<i32 as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ArrayTypeDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<ArrayTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    reader.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Item")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(reader, event)?;
                    return self.handle_item(reader, output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(ArrayTypeDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                item: Vec::new(),
                state__: Box::new(ArrayTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ArrayTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ArrayTypeDeserializerState as S;
            match state {
                S::Item(deserializer) => self.store_item(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_item(&mut self, value: i32) -> Result<(), Error> {
            self.item.push(value);
            Ok(())
        }
        fn handle_item<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<ArrayTypeDeserializerState>,
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
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                *self.state__ = match ret {
                    ElementHandlerOutput::Continue { .. } => ArrayTypeDeserializerState::Next__,
                    ElementHandlerOutput::Break { .. } => fallback
                        .take()
                        .unwrap_or(ArrayTypeDeserializerState::Next__),
                };
                return Ok(ret);
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_item(data)?;
                    *self.state__ = ArrayTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ArrayTypeDeserializerState::Item(deserializer));
                            *self.state__ = ArrayTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = ArrayTypeDeserializerState::Item(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ArrayType> for ArrayTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ArrayType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ArrayType>
        where
            R: DeserializeReader,
        {
            use ArrayTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Item(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_item(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
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
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ArrayType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state__, ArrayTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::ArrayType {
                item: self
                    .item
                    .try_into()
                    .map_err(|vec: Vec<_>| ErrorKind::InsufficientSize {
                        min: 5usize,
                        max: 5usize,
                        actual: vec.len(),
                    })?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, IterSerializer};
    #[derive(Debug)]
    pub struct ArrayTypeSerializer<'ser> {
        pub(super) value: &'ser super::ArrayType,
        pub(super) state: Box<ArrayTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ArrayTypeSerializerState<'ser> {
        Init__,
        Item(IterSerializer<'ser, &'ser [i32], i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ArrayTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ArrayTypeSerializerState::Init__ => {
                        *self.state = ArrayTypeSerializerState::Item(IterSerializer::new(
                            &self.value.item[..],
                            Some("Item"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ArrayTypeSerializerState::Item(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ArrayTypeSerializerState::End__,
                    },
                    ArrayTypeSerializerState::End__ => {
                        *self.state = ArrayTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ArrayTypeSerializerState::Done__ => return Ok(None),
                    ArrayTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ArrayTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ArrayTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
