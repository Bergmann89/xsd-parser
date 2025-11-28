use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
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
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        WithDeserializer,
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
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
            fallback: &mut Option<ArrayTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Item")
                ) {
                    let output = <i32 as WithDeserializer>::Deserializer::init(helper, event)?;
                    return self.handle_item(helper, output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(ArrayTypeDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                item: Vec::new(),
                state__: Box::new(ArrayTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ArrayTypeDeserializerState,
        ) -> Result<(), Error> {
            use ArrayTypeDeserializerState as S;
            match state {
                S::Item(deserializer) => self.store_item(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_item(&mut self, value: i32) -> Result<(), Error> {
            self.item.push(value);
            Ok(())
        }
        fn handle_item<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<ArrayTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
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
                self.finish_state(helper, fallback)?;
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
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ArrayType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ArrayType> {
            use ArrayTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Item(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_item(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(helper, event, &mut fallback)? {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ArrayType, Error> {
            let state = replace(&mut *self.state__, ArrayTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
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
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
    };
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
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ArrayTypeSerializerState::Init__ => {
                        *self.state = ArrayTypeSerializerState::Item(IterSerializer::new(
                            &self.value.item[..],
                            Some("Item"),
                            false,
                        ));
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ArrayTypeSerializerState::Item(x) => match x.next(helper).transpose()? {
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
    impl<'ser> Serializer<'ser> for ArrayTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
