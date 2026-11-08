use xsd_parser_types::misc::{Namespace, NamespacePrefix};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_O: Namespace = Namespace::new_const(b"O");
pub const NS_TNS: Namespace = Namespace::new_const(b"T");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_O: NamespacePrefix = NamespacePrefix::new_const(b"o");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub mod o {
    use xsd_parser_types::quick_xml::{ElementNamespace, Error, WithDeserializer, WithSerializer};
    pub type Wrap = WrapType;
    #[derive(Debug)]
    pub struct WrapType {
        pub foo: super::tns::FooType,
    }
    impl WithSerializer for WrapType {
        type Serializer<'x> = quick_xml_serialize::WrapTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            WithSerializer::serializer_with_namespace(
                self,
                name,
                ElementNamespace::Unknown,
                is_root,
            )
        }
        fn serializer_with_namespace<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            namespace: ElementNamespace,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::WrapTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::WrapTypeSerializerState::Init__),
                name: name.unwrap_or("o:Wrap"),
                namespace,
                is_root,
            })
        }
    }
    impl WithDeserializer for WrapType {
        type Deserializer = quick_xml_deserialize::WrapTypeDeserializer;
    }
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser_types::quick_xml::{
            BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
            DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
            RawByteStr, WithDeserializer,
        };
        #[derive(Debug)]
        pub struct WrapTypeDeserializer {
            foo: Option<super::super::tns::FooType>,
            state__: Box<WrapTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum WrapTypeDeserializerState {
            Init__,
            Foo(Option<<super::super::tns::FooType as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl WrapTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
                Ok(Self {
                    foo: None,
                    state__: Box::new(WrapTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: WrapTypeDeserializerState,
            ) -> Result<(), Error> {
                use WrapTypeDeserializerState as S;
                match state {
                    S::Foo(Some(deserializer)) => self.store_foo(deserializer.finish(helper)?)?,
                    _ => (),
                }
                Ok(())
            }
            fn store_foo(&mut self, value: super::super::tns::FooType) -> Result<(), Error> {
                if self.foo.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Foo")))?;
                }
                self.foo = Some(value);
                Ok(())
            }
            fn handle_foo<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::super::tns::FooType>,
                fallback: &mut Option<WrapTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use WrapTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Foo(None));
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
                        self.store_foo(data)?;
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Foo(Some(deserializer)));
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::WrapType> for WrapTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::WrapType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::WrapType> {
                use WrapTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let mut allow_any_element = false;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Foo(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_foo(helper, output, &mut fallback)? {
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
                            *self.state__ = S::Foo(None);
                            event
                        }
                        (S::Foo(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"Foo",
                                false,
                            )?;
                            match self.handle_foo(helper, output, &mut fallback)? {
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
                let event = event.into_break();
                if let Some(fallback) = fallback {
                    *self.state__ = fallback;
                }
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::WrapType, Error> {
                let state = replace(&mut *self.state__, WrapTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::WrapType {
                    foo: helper.finish_element("Foo", self.foo)?,
                })
            }
        }
    }
    pub mod quick_xml_serialize {
        use xsd_parser_types::quick_xml::{
            BytesEnd, BytesStart, ElementNamespace, Error, Event, SerializeHelper, Serializer,
            WithSerializer,
        };
        #[derive(Debug)]
        pub struct WrapTypeSerializer<'ser> {
            pub(super) value: &'ser super::WrapType,
            pub(super) state: Box<WrapTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) namespace: ElementNamespace,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum WrapTypeSerializerState<'ser> {
            Init__,
            Foo(<super::super::tns::FooType as WithSerializer>::Serializer<'ser>),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> WrapTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        WrapTypeSerializerState::Init__ => {
                            *self.state = WrapTypeSerializerState::Foo(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.foo,
                                    Some("tns:Foo"),
                                    ElementNamespace::Namespace(super::super::NS_TNS),
                                    false,
                                )?,
                            );
                            let namespace = if self.is_root {
                                ElementNamespace::Namespace(super::super::NS_O)
                            } else {
                                self.namespace.clone()
                            };
                            self.name = helper.element_name(self.name, &namespace);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                            if self.is_root {
                                helper.write_default_namespace(&mut bytes, self.name, &namespace);
                            }
                            return Ok(Some(Event::Start(bytes)));
                        }
                        WrapTypeSerializerState::Foo(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = WrapTypeSerializerState::End__,
                        },
                        WrapTypeSerializerState::End__ => {
                            *self.state = WrapTypeSerializerState::Done__;
                            helper.end_ns_scope();
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        WrapTypeSerializerState::Done__ => return Ok(None),
                        WrapTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for WrapTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = WrapTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
pub mod tns {
    use core::fmt::Debug;
    use xsd_parser_types::{
        quick_xml::{
            BoxedSerializer, ElementNamespace, Error, WithBoxedSerializer, WithDeserializer,
            WithSerializer, XsiTypeSerializer,
        },
        AsAny,
    };
    pub type Foo = FooType;
    #[derive(Debug)]
    pub struct FooType {
        pub bar: BarType,
        pub baz: String,
        pub q: QType,
        pub animal: Option<AnimalType>,
    }
    impl WithSerializer for FooType {
        type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            WithSerializer::serializer_with_namespace(
                self,
                name,
                ElementNamespace::Unknown,
                is_root,
            )
        }
        fn serializer_with_namespace<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            namespace: ElementNamespace,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::FooTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::FooTypeSerializerState::Init__),
                name: name.unwrap_or("tns:Foo"),
                namespace,
                is_root,
            })
        }
    }
    impl WithDeserializer for FooType {
        type Deserializer = quick_xml_deserialize::FooTypeDeserializer;
    }
    #[derive(Debug)]
    pub struct BarType {
        pub a: i32,
        pub content: String,
    }
    impl WithSerializer for BarType {
        type Serializer<'x> = quick_xml_serialize::BarTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            WithSerializer::serializer_with_namespace(
                self,
                name,
                ElementNamespace::Unknown,
                is_root,
            )
        }
        fn serializer_with_namespace<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            namespace: ElementNamespace,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::BarTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::BarTypeSerializerState::Init__),
                name: name.unwrap_or("tns:BarType"),
                namespace,
                is_root,
            })
        }
    }
    impl WithDeserializer for BarType {
        type Deserializer = quick_xml_deserialize::BarTypeDeserializer;
    }
    #[derive(Debug)]
    pub struct QType {
        pub inner: String,
        pub pet: Option<AnimalType>,
    }
    impl WithSerializer for QType {
        type Serializer<'x> = quick_xml_serialize::QTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            WithSerializer::serializer_with_namespace(
                self,
                name,
                ElementNamespace::Unknown,
                is_root,
            )
        }
        fn serializer_with_namespace<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            namespace: ElementNamespace,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::QTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::QTypeSerializerState::Init__),
                name: name.unwrap_or("tns:QType"),
                namespace,
                is_root,
            })
        }
    }
    impl WithDeserializer for QType {
        type Deserializer = quick_xml_deserialize::QTypeDeserializer;
    }
    #[derive(Debug)]
    pub struct AnimalType(pub Box<dyn AnimalTypeTrait>);
    pub trait AnimalTypeTrait: Debug + AsAny + WithBoxedSerializer {}
    impl AnimalType {
        pub fn new<T: AnimalTypeTrait + 'static>(value: T) -> Self {
            Self(Box::new(value))
        }
    }
    impl WithSerializer for AnimalType {
        type Serializer<'x> = BoxedSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            WithSerializer::serializer_with_namespace(
                self,
                name,
                ElementNamespace::Unknown,
                is_root,
            )
        }
        fn serializer_with_namespace<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            namespace: ElementNamespace,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            if (*self.0).as_any().is::<DogType>() {
                return Ok(Box::new(
                    XsiTypeSerializer::new(
                        self.0.serializer(Some("tns:Dog"), is_root)?,
                        name.unwrap_or("tns:Animal"),
                        is_root,
                    )
                    .with_namespace(namespace)
                    .with_type_namespace(super::NS_TNS),
                ));
            }
            self.0.serializer_with_namespace(name, namespace, is_root)
        }
    }
    impl WithDeserializer for AnimalType {
        type Deserializer = quick_xml_deserialize::AnimalTypeDeserializer;
    }
    #[derive(Debug)]
    pub struct DogType {
        pub name: String,
    }
    impl AnimalTypeTrait for DogType {}
    impl WithSerializer for DogType {
        type Serializer<'x> = quick_xml_serialize::DogTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            WithSerializer::serializer_with_namespace(
                self,
                name,
                ElementNamespace::Unknown,
                is_root,
            )
        }
        fn serializer_with_namespace<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            namespace: ElementNamespace,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::DogTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::DogTypeSerializerState::Init__),
                name: name.unwrap_or("tns:Dog"),
                namespace,
                is_root,
            })
        }
    }
    impl WithDeserializer for DogType {
        type Deserializer = quick_xml_deserialize::DogTypeDeserializer;
    }
    pub type Zoo = ZooType;
    #[derive(Debug)]
    pub struct ZooType {
        pub animal: AnimalType,
    }
    impl WithSerializer for ZooType {
        type Serializer<'x> = quick_xml_serialize::ZooTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            WithSerializer::serializer_with_namespace(
                self,
                name,
                ElementNamespace::Unknown,
                is_root,
            )
        }
        fn serializer_with_namespace<'ser>(
            &'ser self,
            name: Option<&'ser str>,
            namespace: ElementNamespace,
            is_root: bool,
        ) -> Result<Self::Serializer<'ser>, Error> {
            Ok(quick_xml_serialize::ZooTypeSerializer {
                value: self,
                state: Box::new(quick_xml_serialize::ZooTypeSerializerState::Init__),
                name: name.unwrap_or("tns:Zoo"),
                namespace,
                is_root,
            })
        }
    }
    impl WithDeserializer for ZooType {
        type Deserializer = quick_xml_deserialize::ZooTypeDeserializer;
    }
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser_types::quick_xml::{
            BytesStart, ContentDeserializer, DeserializeHelper, Deserializer, DeserializerArtifact,
            DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
            ErrorKind, Event, QName, RawByteStr, WithDeserializer,
        };
        #[derive(Debug)]
        pub struct FooTypeDeserializer {
            bar: Option<super::BarType>,
            baz: Option<String>,
            q: Option<super::QType>,
            animal: Option<super::AnimalType>,
            state__: Box<FooTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum FooTypeDeserializerState {
            Init__,
            Bar(Option<<super::BarType as WithDeserializer>::Deserializer>),
            Baz(Option<<String as WithDeserializer>::Deserializer>),
            Q(Option<<super::QType as WithDeserializer>::Deserializer>),
            Animal(Option<<super::AnimalType as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl FooTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
                Ok(Self {
                    bar: None,
                    baz: None,
                    q: None,
                    animal: None,
                    state__: Box::new(FooTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: FooTypeDeserializerState,
            ) -> Result<(), Error> {
                use FooTypeDeserializerState as S;
                match state {
                    S::Bar(Some(deserializer)) => self.store_bar(deserializer.finish(helper)?)?,
                    S::Baz(Some(deserializer)) => self.store_baz(deserializer.finish(helper)?)?,
                    S::Q(Some(deserializer)) => self.store_q(deserializer.finish(helper)?)?,
                    S::Animal(Some(deserializer)) => {
                        self.store_animal(deserializer.finish(helper)?)?
                    }
                    _ => (),
                }
                Ok(())
            }
            fn store_bar(&mut self, value: super::BarType) -> Result<(), Error> {
                if self.bar.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"bar")))?;
                }
                self.bar = Some(value);
                Ok(())
            }
            fn store_baz(&mut self, value: String) -> Result<(), Error> {
                if self.baz.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"baz")))?;
                }
                self.baz = Some(value);
                Ok(())
            }
            fn store_q(&mut self, value: super::QType) -> Result<(), Error> {
                if self.q.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"q")))?;
                }
                self.q = Some(value);
                Ok(())
            }
            fn store_animal(&mut self, value: super::AnimalType) -> Result<(), Error> {
                if self.animal.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"animal",
                    )))?;
                }
                self.animal = Some(value);
                Ok(())
            }
            fn handle_bar<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::BarType>,
                fallback: &mut Option<FooTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use FooTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Bar(None));
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
                        self.store_bar(data)?;
                        *self.state__ = S::Baz(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Bar(Some(deserializer)));
                        *self.state__ = S::Baz(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
            fn handle_baz<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, String>,
                fallback: &mut Option<FooTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use FooTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Baz(None));
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
                        self.store_baz(data)?;
                        *self.state__ = S::Q(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Baz(Some(deserializer)));
                        *self.state__ = S::Q(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
            fn handle_q<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::QType>,
                fallback: &mut Option<FooTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use FooTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Q(None));
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
                        self.store_q(data)?;
                        *self.state__ = S::Animal(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Q(Some(deserializer)));
                        *self.state__ = S::Animal(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
            fn handle_animal<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::AnimalType>,
                fallback: &mut Option<FooTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use FooTypeDeserializerState as S;
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
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Animal(Some(deserializer)));
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
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
                use FooTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let mut allow_any_element = false;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Bar(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_bar(helper, output, &mut fallback)? {
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
                            let output = deserializer.next(helper, event)?;
                            match self.handle_baz(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        }
                        (S::Q(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_q(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        }
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
                            *self.state__ = S::Bar(None);
                            event
                        }
                        (S::Bar(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"bar",
                                false,
                            )?;
                            match self.handle_bar(helper, output, &mut fallback)? {
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
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"baz",
                                false,
                            )?;
                            match self.handle_baz(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        }
                        (S::Q(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"q",
                                false,
                            )?;
                            match self.handle_q(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        }
                        (S::Animal(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output =
                                <super::AnimalType as WithDeserializer>::init(helper, event)?;
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
                let event = event.into_break();
                if let Some(fallback) = fallback {
                    *self.state__ = fallback;
                }
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::FooType, Error> {
                let state = replace(&mut *self.state__, FooTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::FooType {
                    bar: helper.finish_element("bar", self.bar)?,
                    baz: helper.finish_element("baz", self.baz)?,
                    q: helper.finish_element("q", self.q)?,
                    animal: self.animal,
                })
            }
        }
        #[derive(Debug)]
        pub struct BarTypeDeserializer {
            a: i32,
            content: Option<String>,
            state__: Box<BarTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum BarTypeDeserializerState {
            Init__,
            Content__(<String as WithDeserializer>::Deserializer),
            Unknown__,
        }
        impl BarTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                let mut a: Option<i32> = None;
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        helper.resolve_local_name(attrib.key, &super::super::NS_TNS),
                        Some(b"a")
                    ) {
                        helper.read_attrib(&mut a, b"a", &attrib.value)?;
                    } else {
                        helper.raise_unexpected_attrib_checked(&attrib)?;
                    }
                }
                Ok(Self {
                    a: a.ok_or_else(|| ErrorKind::MissingAttribute("a".into()))?,
                    content: None,
                    state__: Box::new(BarTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: BarTypeDeserializerState,
            ) -> Result<(), Error> {
                if let BarTypeDeserializerState::Content__(deserializer) = state {
                    self.store_content(deserializer.finish(helper)?)?;
                }
                Ok(())
            }
            fn store_content(&mut self, value: String) -> Result<(), Error> {
                if self.content.is_some() {
                    Err(ErrorKind::DuplicateContent)?;
                }
                self.content = Some(value);
                Ok(())
            }
            fn handle_content<'de>(
                mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, String>,
            ) -> DeserializerResult<'de, super::BarType> {
                use BarTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                match artifact {
                    DeserializerArtifact::None => Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::None,
                        event,
                        allow_any,
                    }),
                    DeserializerArtifact::Data(data) => {
                        self.store_content(data)?;
                        let data = self.finish(helper)?;
                        Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(data),
                            event,
                            allow_any,
                        })
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = S::Content__(deserializer);
                        Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Deserializer(self),
                            event,
                            allow_any,
                        })
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::BarType> for BarTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::BarType> {
                let (Event::Start(x) | Event::Empty(x)) = &event else {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::None,
                        event: DeserializerEvent::Break(event),
                        allow_any: false,
                    });
                };
                Self::from_bytes_start(helper, x)?.next(helper, event)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::BarType> {
                use BarTypeDeserializerState as S;
                match replace(&mut *self.state__, S::Unknown__) {
                    S::Unknown__ => unreachable!(),
                    S::Init__ => {
                        let output = ContentDeserializer::init(helper, event)?;
                        self.handle_content(helper, output)
                    }
                    S::Content__(deserializer) => {
                        let output = deserializer.next(helper, event)?;
                        self.handle_content(helper, output)
                    }
                }
            }
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::BarType, Error> {
                let state = replace(&mut *self.state__, BarTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::BarType {
                    a: self.a,
                    content: helper.finish_content(self.content)?,
                })
            }
        }
        #[derive(Debug)]
        pub struct QTypeDeserializer {
            inner: Option<String>,
            pet: Option<super::AnimalType>,
            state__: Box<QTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum QTypeDeserializerState {
            Init__,
            Inner(Option<<String as WithDeserializer>::Deserializer>),
            Pet(Option<<super::AnimalType as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl QTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
                Ok(Self {
                    inner: None,
                    pet: None,
                    state__: Box::new(QTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: QTypeDeserializerState,
            ) -> Result<(), Error> {
                use QTypeDeserializerState as S;
                match state {
                    S::Inner(Some(deserializer)) => {
                        self.store_inner(deserializer.finish(helper)?)?
                    }
                    S::Pet(Some(deserializer)) => self.store_pet(deserializer.finish(helper)?)?,
                    _ => (),
                }
                Ok(())
            }
            fn store_inner(&mut self, value: String) -> Result<(), Error> {
                if self.inner.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"inner",
                    )))?;
                }
                self.inner = Some(value);
                Ok(())
            }
            fn store_pet(&mut self, value: super::AnimalType) -> Result<(), Error> {
                if self.pet.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pet")))?;
                }
                self.pet = Some(value);
                Ok(())
            }
            fn handle_inner<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, String>,
                fallback: &mut Option<QTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use QTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Inner(None));
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
                        self.store_inner(data)?;
                        *self.state__ = S::Pet(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Inner(Some(deserializer)));
                        *self.state__ = S::Pet(None);
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
            fn handle_pet<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::AnimalType>,
                fallback: &mut Option<QTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use QTypeDeserializerState as S;
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
                        self.store_pet(data)?;
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Pet(Some(deserializer)));
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::QType> for QTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::QType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::QType> {
                use QTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let mut allow_any_element = false;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Inner(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_inner(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        }
                        (S::Pet(Some(deserializer)), event) => {
                            let output = deserializer.next(helper, event)?;
                            match self.handle_pet(helper, output, &mut fallback)? {
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
                            *self.state__ = S::Inner(None);
                            event
                        }
                        (S::Inner(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"inner",
                                false,
                            )?;
                            match self.handle_inner(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        }
                        (S::Pet(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output =
                                <super::AnimalType as WithDeserializer>::init(helper, event)?;
                            match self.handle_pet(helper, output, &mut fallback)? {
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
                let event = event.into_break();
                if let Some(fallback) = fallback {
                    *self.state__ = fallback;
                }
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::QType, Error> {
                let state = replace(&mut *self.state__, QTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::QType {
                    inner: helper.finish_element("inner", self.inner)?,
                    pet: self.pet,
                })
            }
        }
        #[derive(Debug)]
        pub enum AnimalTypeDeserializer {
            Dog(<super::DogType as WithDeserializer>::Deserializer),
        }
        impl<'de> Deserializer<'de, super::AnimalType> for AnimalTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::AnimalType> {
                let Some(type_name) = helper.get_dynamic_type_from_tag(&event) else {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::None,
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                };
                let type_name = type_name.into_owned();
                if matches!(
                    helper.resolve_local_name(QName(&type_name), &super::super::NS_TNS),
                    Some(b"Animal")
                ) {
                    if let Some(type_name) = helper.get_dynamic_type_from_attrib(&event)? {
                        let type_name = type_name.into_owned();
                        if matches!(
                            helper.resolve_local_name(QName(&type_name), &super::super::NS_TNS),
                            Some(b"Dog")
                        ) {
                            let DeserializerOutput {
                                artifact,
                                event,
                                allow_any,
                            } = <super::DogType as WithDeserializer>::init(helper, event)?;
                            return Ok(DeserializerOutput {
                                artifact: artifact
                                    .map(|x| super::AnimalType(Box::new(x)), |x| Self::Dog(x)),
                                event,
                                allow_any,
                            });
                        }
                    }
                }
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
            fn next(
                self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::AnimalType> {
                match self {
                    Self::Dog(x) => {
                        let DeserializerOutput {
                            artifact,
                            event,
                            allow_any,
                        } = x.next(helper, event)?;
                        Ok(DeserializerOutput {
                            artifact: artifact
                                .map(|x| super::AnimalType(Box::new(x)), |x| Self::Dog(x)),
                            event,
                            allow_any,
                        })
                    }
                }
            }
            fn finish(self, helper: &mut DeserializeHelper) -> Result<super::AnimalType, Error> {
                match self {
                    Self::Dog(x) => Ok(super::AnimalType(Box::new(x.finish(helper)?))),
                }
            }
        }
        #[derive(Debug)]
        pub struct DogTypeDeserializer {
            name: Option<String>,
            state__: Box<DogTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum DogTypeDeserializerState {
            Init__,
            Name(Option<<String as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl DogTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
                Ok(Self {
                    name: None,
                    state__: Box::new(DogTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: DogTypeDeserializerState,
            ) -> Result<(), Error> {
                use DogTypeDeserializerState as S;
                match state {
                    S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
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
            fn handle_name<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, String>,
                fallback: &mut Option<DogTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use DogTypeDeserializerState as S;
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
                use DogTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let mut allow_any_element = false;
                let (event, allow_any) = loop {
                    let state = replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
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
                            *self.state__ = S::Name(None);
                            event
                        }
                        (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                            let output = helper.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"name",
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
                let event = event.into_break();
                if let Some(fallback) = fallback {
                    *self.state__ = fallback;
                }
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::DogType, Error> {
                let state = replace(&mut *self.state__, DogTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::DogType {
                    name: helper.finish_element("name", self.name)?,
                })
            }
        }
        #[derive(Debug)]
        pub struct ZooTypeDeserializer {
            animal: Option<super::AnimalType>,
            state__: Box<ZooTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum ZooTypeDeserializerState {
            Init__,
            Animal(Option<<super::AnimalType as WithDeserializer>::Deserializer>),
            Done__,
            Unknown__,
        }
        impl ZooTypeDeserializer {
            fn from_bytes_start(
                helper: &mut DeserializeHelper,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error> {
                for attrib in helper.filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
                Ok(Self {
                    animal: None,
                    state__: Box::new(ZooTypeDeserializerState::Init__),
                })
            }
            fn finish_state(
                &mut self,
                helper: &mut DeserializeHelper,
                state: ZooTypeDeserializerState,
            ) -> Result<(), Error> {
                use ZooTypeDeserializerState as S;
                match state {
                    S::Animal(Some(deserializer)) => {
                        self.store_animal(deserializer.finish(helper)?)?
                    }
                    _ => (),
                }
                Ok(())
            }
            fn store_animal(&mut self, value: super::AnimalType) -> Result<(), Error> {
                if self.animal.is_some() {
                    Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                        b"animal",
                    )))?;
                }
                self.animal = Some(value);
                Ok(())
            }
            fn handle_animal<'de>(
                &mut self,
                helper: &mut DeserializeHelper,
                output: DeserializerOutput<'de, super::AnimalType>,
                fallback: &mut Option<ZooTypeDeserializerState>,
            ) -> Result<ElementHandlerOutput<'de>, Error> {
                use ZooTypeDeserializerState as S;
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    fallback.get_or_insert(S::Animal(None));
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
                        self.store_animal(data)?;
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                    DeserializerArtifact::Deserializer(deserializer) => {
                        fallback.get_or_insert(S::Animal(Some(deserializer)));
                        *self.state__ = S::Done__;
                        Ok(ElementHandlerOutput::from_event(event, allow_any))
                    }
                }
            }
        }
        impl<'de> Deserializer<'de, super::ZooType> for ZooTypeDeserializer {
            fn init(
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::ZooType> {
                helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next(
                mut self,
                helper: &mut DeserializeHelper,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::ZooType> {
                use ZooTypeDeserializerState as S;
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
                            let output =
                                <super::AnimalType as WithDeserializer>::init(helper, event)?;
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
                let event = event.into_break();
                if let Some(fallback) = fallback {
                    *self.state__ = fallback;
                }
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
            fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ZooType, Error> {
                let state = replace(&mut *self.state__, ZooTypeDeserializerState::Unknown__);
                self.finish_state(helper, state)?;
                Ok(super::ZooType {
                    animal: helper.finish_element("animal", self.animal)?,
                })
            }
        }
    }
    pub mod quick_xml_serialize {
        use xsd_parser_types::quick_xml::{
            BytesEnd, BytesStart, ElementNamespace, Error, Event, IterSerializer, SerializeHelper,
            Serializer, WithSerializer,
        };
        #[derive(Debug)]
        pub struct FooTypeSerializer<'ser> {
            pub(super) value: &'ser super::FooType,
            pub(super) state: Box<FooTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) namespace: ElementNamespace,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum FooTypeSerializerState<'ser> {
            Init__,
            Bar(<super::BarType as WithSerializer>::Serializer<'ser>),
            Baz(<String as WithSerializer>::Serializer<'ser>),
            Q(<super::QType as WithSerializer>::Serializer<'ser>),
            Animal(IterSerializer<'ser, Option<&'ser super::AnimalType>, super::AnimalType>),
            End__,
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
                            *self.state = FooTypeSerializerState::Bar(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.bar,
                                    Some("tns:bar"),
                                    ElementNamespace::Namespace(super::super::NS_TNS),
                                    false,
                                )?,
                            );
                            let namespace = if self.is_root {
                                ElementNamespace::Namespace(super::super::NS_TNS)
                            } else {
                                self.namespace.clone()
                            };
                            self.name = helper.element_name(self.name, &namespace);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                            if self.is_root {
                                helper.write_default_namespace(&mut bytes, self.name, &namespace);
                            }
                            return Ok(Some(Event::Start(bytes)));
                        }
                        FooTypeSerializerState::Bar(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = FooTypeSerializerState::Baz(
                                    WithSerializer::serializer_with_namespace(
                                        &self.value.baz,
                                        Some("tns:baz"),
                                        ElementNamespace::Namespace(super::super::NS_TNS),
                                        false,
                                    )?,
                                )
                            }
                        },
                        FooTypeSerializerState::Baz(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = FooTypeSerializerState::Q(
                                    WithSerializer::serializer_with_namespace(
                                        &self.value.q,
                                        Some("q"),
                                        ElementNamespace::None,
                                        false,
                                    )?,
                                )
                            }
                        },
                        FooTypeSerializerState::Q(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = FooTypeSerializerState::Animal(
                                    IterSerializer::new(
                                        self.value.animal.as_ref(),
                                        Some("tns:animal"),
                                        false,
                                    )
                                    .with_namespace(
                                        ElementNamespace::Namespace(super::super::NS_TNS),
                                    ),
                                )
                            }
                        },
                        FooTypeSerializerState::Animal(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = FooTypeSerializerState::End__,
                        },
                        FooTypeSerializerState::End__ => {
                            *self.state = FooTypeSerializerState::Done__;
                            helper.end_ns_scope();
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
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
        #[derive(Debug)]
        pub struct BarTypeSerializer<'ser> {
            pub(super) value: &'ser super::BarType,
            pub(super) state: Box<BarTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) namespace: ElementNamespace,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum BarTypeSerializerState<'ser> {
            Init__,
            Content__(<String as WithSerializer>::Serializer<'ser>),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> BarTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        BarTypeSerializerState::Init__ => {
                            *self.state = BarTypeSerializerState::Content__(
                                WithSerializer::serializer(&self.value.content, None, false)?,
                            );
                            let namespace = if self.is_root {
                                ElementNamespace::Namespace(super::super::NS_TNS)
                            } else {
                                self.namespace.clone()
                            };
                            self.name = helper.element_name(self.name, &namespace);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                            if self.is_root {
                                helper.write_default_namespace(&mut bytes, self.name, &namespace);
                            }
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::super::PREFIX_TNS),
                                &super::super::NS_TNS,
                            );
                            helper.write_attrib(&mut bytes, "tns:a", &self.value.a)?;
                            return Ok(Some(Event::Start(bytes)));
                        }
                        BarTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = BarTypeSerializerState::End__,
                        },
                        BarTypeSerializerState::End__ => {
                            *self.state = BarTypeSerializerState::Done__;
                            helper.end_ns_scope();
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        BarTypeSerializerState::Done__ => return Ok(None),
                        BarTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for BarTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = BarTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct QTypeSerializer<'ser> {
            pub(super) value: &'ser super::QType,
            pub(super) state: Box<QTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) namespace: ElementNamespace,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum QTypeSerializerState<'ser> {
            Init__,
            Inner(<String as WithSerializer>::Serializer<'ser>),
            Pet(IterSerializer<'ser, Option<&'ser super::AnimalType>, super::AnimalType>),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> QTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        QTypeSerializerState::Init__ => {
                            *self.state = QTypeSerializerState::Inner(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.inner,
                                    Some("tns:inner"),
                                    ElementNamespace::Namespace(super::super::NS_TNS),
                                    false,
                                )?,
                            );
                            let namespace = if self.is_root {
                                ElementNamespace::Namespace(super::super::NS_TNS)
                            } else {
                                self.namespace.clone()
                            };
                            self.name = helper.element_name(self.name, &namespace);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                            if self.is_root {
                                helper.write_default_namespace(&mut bytes, self.name, &namespace);
                            }
                            return Ok(Some(Event::Start(bytes)));
                        }
                        QTypeSerializerState::Inner(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = QTypeSerializerState::Pet(
                                    IterSerializer::new(
                                        self.value.pet.as_ref(),
                                        Some("pet"),
                                        false,
                                    )
                                    .with_namespace(ElementNamespace::None),
                                )
                            }
                        },
                        QTypeSerializerState::Pet(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = QTypeSerializerState::End__,
                        },
                        QTypeSerializerState::End__ => {
                            *self.state = QTypeSerializerState::Done__;
                            helper.end_ns_scope();
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        QTypeSerializerState::Done__ => return Ok(None),
                        QTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for QTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = QTypeSerializerState::Done__;
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
            pub(super) namespace: ElementNamespace,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum DogTypeSerializerState<'ser> {
            Init__,
            Name(<String as WithSerializer>::Serializer<'ser>),
            End__,
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
                            *self.state = DogTypeSerializerState::Name(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.name,
                                    Some("tns:name"),
                                    ElementNamespace::Namespace(super::super::NS_TNS),
                                    false,
                                )?,
                            );
                            let namespace = if self.is_root {
                                ElementNamespace::Namespace(super::super::NS_TNS)
                            } else {
                                self.namespace.clone()
                            };
                            self.name = helper.element_name(self.name, &namespace);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                            if self.is_root {
                                helper.write_default_namespace(&mut bytes, self.name, &namespace);
                            }
                            return Ok(Some(Event::Start(bytes)));
                        }
                        DogTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DogTypeSerializerState::End__,
                        },
                        DogTypeSerializerState::End__ => {
                            *self.state = DogTypeSerializerState::Done__;
                            helper.end_ns_scope();
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
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
        pub struct ZooTypeSerializer<'ser> {
            pub(super) value: &'ser super::ZooType,
            pub(super) state: Box<ZooTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) namespace: ElementNamespace,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum ZooTypeSerializerState<'ser> {
            Init__,
            Animal(<super::AnimalType as WithSerializer>::Serializer<'ser>),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> ZooTypeSerializer<'ser> {
            fn next_event(
                &mut self,
                helper: &mut SerializeHelper,
            ) -> Result<Option<Event<'ser>>, Error> {
                loop {
                    match &mut *self.state {
                        ZooTypeSerializerState::Init__ => {
                            *self.state = ZooTypeSerializerState::Animal(
                                WithSerializer::serializer_with_namespace(
                                    &self.value.animal,
                                    Some("tns:animal"),
                                    ElementNamespace::Namespace(super::super::NS_TNS),
                                    false,
                                )?,
                            );
                            let namespace = if self.is_root {
                                ElementNamespace::Namespace(super::super::NS_TNS)
                            } else {
                                self.namespace.clone()
                            };
                            self.name = helper.element_name(self.name, &namespace);
                            let mut bytes = BytesStart::new(self.name);
                            helper.begin_ns_scope();
                            helper.write_xmlns_for_element(&mut bytes, self.name, &namespace);
                            if self.is_root {
                                helper.write_default_namespace(&mut bytes, self.name, &namespace);
                            }
                            return Ok(Some(Event::Start(bytes)));
                        }
                        ZooTypeSerializerState::Animal(x) => match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ZooTypeSerializerState::End__,
                        },
                        ZooTypeSerializerState::End__ => {
                            *self.state = ZooTypeSerializerState::Done__;
                            helper.end_ns_scope();
                            return Ok(Some(Event::End(BytesEnd::new(self.name))));
                        }
                        ZooTypeSerializerState::Done__ => return Ok(None),
                        ZooTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> Serializer<'ser> for ZooTypeSerializer<'ser> {
            fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
                match self.next_event(helper) {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = ZooTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
