use core::ops::Deref;
use std::borrow::Cow;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, SerializeBytes, SerializeHelper, ValidateError,
        WithDeserializer, WithSerializer,
    },
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub a_list: ListType,
}
impl FooType {
    #[must_use]
    pub fn default_a_list() -> ListType {
        ListType(Vec::new())
    }
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
            name: name.unwrap_or("tns:FooType"),
            is_root,
        })
    }
}
impl WithDeserializer for FooType {
    type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
}
#[derive(Debug, Default)]
pub struct ListType(pub Vec<StringType>);
impl SerializeBytes for ListType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes(helper)? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(Cow::Owned(data)))
    }
}
impl DeserializeBytes for ListType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        Ok(Self(helper.deserialize_list(bytes)?))
    }
}
pub type StringType = String;
pub type ListOfMyStrings = ListOfMyStringsType;
#[derive(Debug)]
pub struct ListOfMyStringsType(pub Vec<MyStringType>);
impl ListOfMyStringsType {
    pub fn new(inner: Vec<MyStringType>) -> Result<Self, ValidateError> {
        Self::validate_value(&inner)?;
        Ok(Self(inner))
    }
    #[must_use]
    pub fn into_inner(self) -> Vec<MyStringType> {
        self.0
    }
    pub fn validate_value(value: &Vec<MyStringType>) -> Result<(), ValidateError> {
        if value.is_empty() {
            return Err(ValidateError::MinLength(1usize));
        }
        Ok(())
    }
}
impl From<ListOfMyStringsType> for Vec<MyStringType> {
    fn from(value: ListOfMyStringsType) -> Vec<MyStringType> {
        value.0
    }
}
impl TryFrom<Vec<MyStringType>> for ListOfMyStringsType {
    type Error = ValidateError;
    fn try_from(value: Vec<MyStringType>) -> Result<Self, ValidateError> {
        Self::new(value)
    }
}
impl Deref for ListOfMyStringsType {
    type Target = Vec<MyStringType>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl SerializeBytes for ListOfMyStringsType {
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes(helper)? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(Cow::Owned(data)))
    }
}
impl DeserializeBytes for ListOfMyStringsType {
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        let inner = helper.deserialize_list(bytes)?;
        Ok(Self::new(inner).map_err(|error| (bytes, error))?)
    }
}
pub type MyStringType = String;
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, Error, Event,
    };
    #[derive(Debug)]
    pub struct FooTypeDeserializer {
        a_list: super::ListType,
        state__: Box<FooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FooTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl FooTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut a_list: Option<super::ListType> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"a-list")
                ) {
                    helper.read_attrib(&mut a_list, b"a-list", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                a_list: a_list.unwrap_or_else(super::FooType::default_a_list),
                state__: Box::new(FooTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: FooTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::FooType> for FooTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FooType> {
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FooType, Error> {
            let state = replace(&mut *self.state__, FooTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::FooType {
                a_list: self.a_list,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{BytesStart, Error, Event, SerializeHelper, Serializer};
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
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FooTypeSerializerState::Init__ => {
                        *self.state = FooTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        helper.write_attrib(&mut bytes, "a-list", &self.value.a_list)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    FooTypeSerializerState::Done__ => return Ok(None),
                    FooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for FooTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FooTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
