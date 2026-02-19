pub const NS_XS: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: xsd_parser_types::misc::NamespacePrefix =
    xsd_parser_types::misc::NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: xsd_parser_types::misc::NamespacePrefix =
    xsd_parser_types::misc::NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: xsd_parser_types::misc::NamespacePrefix =
    xsd_parser_types::misc::NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: xsd_parser_types::misc::NamespacePrefix =
    xsd_parser_types::misc::NamespacePrefix::new_const(b"tns");
pub mod tns {
    pub mod base {
        #[derive(Debug)]
        pub struct PersonType {
            pub name: ::std::string::String,
            pub gender: GenderType,
        }
        impl ::xsd_parser_types::quick_xml::WithSerializer for PersonType {
            type Serializer<'x> = quick_xml_serialize::PersonTypeSerializer<'x>;
            fn serializer<'ser>(
                &'ser self,
                name: ::core::option::Option<&'ser ::core::primitive::str>,
                is_root: ::core::primitive::bool,
            ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error>
            {
                Ok(quick_xml_serialize::PersonTypeSerializer {
                    value: self,
                    state: ::std::boxed::Box::new(
                        quick_xml_serialize::PersonTypeSerializerState::Init__,
                    ),
                    name: name.unwrap_or("PersonType"),
                    is_root,
                })
            }
        }
        impl ::xsd_parser_types::quick_xml::WithDeserializer for PersonType {
            type Deserializer = quick_xml_deserialize::PersonTypeDeserializer;
        }
        #[derive(Debug)]
        pub enum GenderType {
            Male,
            Female,
        }
        impl ::xsd_parser_types::quick_xml::SerializeBytes for GenderType {
            fn serialize_bytes(
                &self,
                helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
            ) -> ::core::result::Result<
                ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
                ::xsd_parser_types::quick_xml::Error,
            > {
                match self {
                    Self::Male => Ok(Some(::std::borrow::Cow::Borrowed("male"))),
                    Self::Female => Ok(Some(::std::borrow::Cow::Borrowed("female"))),
                }
            }
        }
        impl ::xsd_parser_types::quick_xml::DeserializeBytes for GenderType {
            fn deserialize_bytes(
                helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                bytes: &[::core::primitive::u8],
            ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
                match bytes {
                    b"male" => Ok(Self::Male),
                    b"female" => Ok(Self::Female),
                    x => Err(::xsd_parser_types::quick_xml::Error::from(
                        ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                            ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                        ),
                    )),
                }
            }
        }
        pub mod quick_xml_deserialize {
            use xsd_parser_types::quick_xml::Deserializer as _;
            #[derive(Debug)]
            pub struct PersonTypeDeserializer {
                name: ::core::option::Option<::std::string::String>,
                gender: ::core::option::Option<super::GenderType>,
                state__: ::std::boxed::Box<PersonTypeDeserializerState>,
            }
            #[derive(Debug)]
            enum PersonTypeDeserializerState {
                Init__ , Name (:: core :: option :: Option << :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Gender (:: core :: option :: Option << super :: GenderType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
            impl PersonTypeDeserializer {
                fn from_bytes_start(
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
                ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error>
                {
                    for attrib in helper.filter_xmlns_attributes(bytes_start) {
                        let attrib = attrib?;
                        helper.raise_unexpected_attrib_checked(&attrib)?;
                    }
                    Ok(Self {
                        name: None,
                        gender: None,
                        state__: ::std::boxed::Box::new(PersonTypeDeserializerState::Init__),
                    })
                }
                fn finish_state(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    state: PersonTypeDeserializerState,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    use PersonTypeDeserializerState as S;
                    match state {
                        S::Name(Some(deserializer)) => {
                            self.store_name(deserializer.finish(helper)?)?
                        }
                        S::Gender(Some(deserializer)) => {
                            self.store_gender(deserializer.finish(helper)?)?
                        }
                        _ => (),
                    }
                    Ok(())
                }
                fn store_name(
                    &mut self,
                    value: ::std::string::String,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    if self.name.is_some() {
                        Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                            ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"name"),
                        ))?;
                    }
                    self.name = Some(value);
                    Ok(())
                }
                fn store_gender(
                    &mut self,
                    value: super::GenderType,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    if self.gender.is_some() {
                        Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                            ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"gender"),
                        ))?;
                    }
                    self.gender = Some(value);
                    Ok(())
                }
                fn handle_name<'de>(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                        'de,
                        ::std::string::String,
                    >,
                    fallback: &mut ::core::option::Option<PersonTypeDeserializerState>,
                ) -> ::core::result::Result<
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    use PersonTypeDeserializerState as S;
                    let ::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = output;
                    if artifact.is_none() {
                        fallback.get_or_insert(S::Name(None));
                        if matches!(&fallback, Some(S::Init__)) {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::break_(
                                    event, allow_any,
                                ),
                            );
                        } else {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                                    event, allow_any,
                                ),
                            );
                        }
                    }
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(helper, fallback)?;
                    }
                    match artifact {
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                            self.store_name(data)?;
                            *self.state__ = S::Gender(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            deserializer,
                        ) => {
                            fallback.get_or_insert(S::Name(Some(deserializer)));
                            *self.state__ = S::Gender(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                    }
                }
                fn handle_gender<'de>(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                        'de,
                        super::GenderType,
                    >,
                    fallback: &mut ::core::option::Option<PersonTypeDeserializerState>,
                ) -> ::core::result::Result<
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    use PersonTypeDeserializerState as S;
                    let ::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = output;
                    if artifact.is_none() {
                        fallback.get_or_insert(S::Gender(None));
                        if matches!(&fallback, Some(S::Init__)) {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::break_(
                                    event, allow_any,
                                ),
                            );
                        } else {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                                    event, allow_any,
                                ),
                            );
                        }
                    }
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(helper, fallback)?;
                    }
                    match artifact {
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                            self.store_gender(data)?;
                            *self.state__ = S::Done__;
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            deserializer,
                        ) => {
                            fallback.get_or_insert(S::Gender(Some(deserializer)));
                            *self.state__ = S::Done__;
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                    }
                }
            }
            impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::PersonType>
                for PersonTypeDeserializer
            {
                fn init(
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    event: ::xsd_parser_types::quick_xml::Event<'de>,
                ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PersonType>
                {
                    helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
                }
                fn next(
                    mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    event: ::xsd_parser_types::quick_xml::Event<'de>,
                ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PersonType>
                {
                    use PersonTypeDeserializerState as S;
                    let mut event = event;
                    let mut fallback = None;
                    let mut allow_any_element = false;
                    let (event, allow_any) = loop {
                        let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                        event = match (state, event) {
                            (S::Unknown__, _) => unreachable!(),
                            (S::Name(Some(deserializer)), event) => {
                                let output = deserializer.next(helper, event)?;
                                match self . handle_name (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (S::Gender(Some(deserializer)), event) => {
                                let output = deserializer.next(helper, event)?;
                                match self . handle_gender (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                                if let Some(fallback) = fallback.take() {
                                    self.finish_state(helper, fallback)?;
                                }
                                return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                                    artifact:
                                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                            self.finish(helper)?,
                                        ),
                                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                                    allow_any: false,
                                });
                            }
                            (S::Init__, event) => {
                                fallback.get_or_insert(S::Init__);
                                *self.state__ = S::Name(None);
                                event
                            }
                            (
                                S::Name(None),
                                event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                                | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                            ) => {
                                let output = helper.init_start_tag_deserializer(
                                    event,
                                    Some(&super::super::super::NS_TNS),
                                    b"name",
                                    false,
                                )?;
                                match self . handle_name (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (
                                S::Gender(None),
                                event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                                | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                            ) => {
                                let output = helper.init_start_tag_deserializer(
                                    event,
                                    Some(&super::super::super::NS_TNS),
                                    b"gender",
                                    false,
                                )?;
                                match self . handle_gender (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (S::Done__, event) => {
                                *self.state__ = S::Done__;
                                break (
                                    ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(
                                        event,
                                    ),
                                    allow_any_element,
                                );
                            }
                            (state, event) => {
                                *self.state__ = state;
                                break (
                                    ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                                    false,
                                );
                            }
                        }
                    };
                    if let Some(fallback) = fallback {
                        *self.state__ = fallback;
                    }
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            self,
                        ),
                        event,
                        allow_any,
                    })
                }
                fn finish(
                    mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                ) -> ::core::result::Result<super::PersonType, ::xsd_parser_types::quick_xml::Error>
                {
                    let state = ::core::mem::replace(
                        &mut *self.state__,
                        PersonTypeDeserializerState::Unknown__,
                    );
                    self.finish_state(helper, state)?;
                    Ok(super::PersonType {
                        name: helper.finish_element("name", self.name)?,
                        gender: helper.finish_element("gender", self.gender)?,
                    })
                }
            }
        }
        pub mod quick_xml_serialize {
            use xsd_parser_types::quick_xml::Serializer as _;
            #[derive(Debug)]
            pub struct PersonTypeSerializer<'ser> {
                pub(super) value: &'ser super::PersonType,
                pub(super) state: ::std::boxed::Box<PersonTypeSerializerState<'ser>>,
                pub(super) name: &'ser ::core::primitive::str,
                pub(super) is_root: ::core::primitive::bool,
            }
            #[derive(Debug)]
            pub(super) enum PersonTypeSerializerState<'ser> {
                Init__ , Name (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , Gender (< super :: GenderType as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , End__ , Done__ , Phantom__ (& 'ser ()) , }
            impl<'ser> PersonTypeSerializer<'ser> {
                fn next_event(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
                ) -> ::core::result::Result<
                    ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    loop {
                        match &mut *self.state {
                            PersonTypeSerializerState::Init__ => {
                                *self.state = PersonTypeSerializerState::Name(
                                    ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                        &self.value.name,
                                        Some("name"),
                                        false,
                                    )?,
                                );
                                let mut bytes =
                                    ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                                helper.begin_ns_scope();
                                if self.is_root {
                                    helper.write_xmlns(
                                        &mut bytes,
                                        Some(&super::super::super::PREFIX_XSI),
                                        &super::super::super::NS_XSI,
                                    );
                                }
                                return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(
                                    bytes,
                                )));
                            }
                            PersonTypeSerializerState::Name(x) => {
                                match x.next(helper).transpose()? {
                                    Some(event) => return Ok(Some(event)),
                                    None => *self.state = PersonTypeSerializerState::Gender(
                                        ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                            &self.value.gender,
                                            Some("gender"),
                                            false,
                                        )?,
                                    ),
                                }
                            }
                            PersonTypeSerializerState::Gender(x) => {
                                match x.next(helper).transpose()? {
                                    Some(event) => return Ok(Some(event)),
                                    None => *self.state = PersonTypeSerializerState::End__,
                                }
                            }
                            PersonTypeSerializerState::End__ => {
                                *self.state = PersonTypeSerializerState::Done__;
                                helper.end_ns_scope();
                                return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                                    ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                                )));
                            }
                            PersonTypeSerializerState::Done__ => return Ok(None),
                            PersonTypeSerializerState::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }
            impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for PersonTypeSerializer<'ser> {
                fn next(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
                ) -> ::core::option::Option<
                    ::core::result::Result<
                        ::xsd_parser_types::quick_xml::Event<'ser>,
                        ::xsd_parser_types::quick_xml::Error,
                    >,
                > {
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
        }
    }
    pub mod schema_1 {
        pub type Persons = PersonsType;
        #[derive(Debug)]
        pub struct PersonsType {
            pub person: ::std::vec::Vec<super::base::PersonType>,
        }
        impl ::xsd_parser_types::quick_xml::WithSerializer for PersonsType {
            type Serializer<'x> = quick_xml_serialize::PersonsTypeSerializer<'x>;
            fn serializer<'ser>(
                &'ser self,
                name: ::core::option::Option<&'ser ::core::primitive::str>,
                is_root: ::core::primitive::bool,
            ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error>
            {
                Ok(quick_xml_serialize::PersonsTypeSerializer {
                    value: self,
                    state: ::std::boxed::Box::new(
                        quick_xml_serialize::PersonsTypeSerializerState::Init__,
                    ),
                    name: name.unwrap_or("PersonsType"),
                    is_root,
                })
            }
        }
        impl ::xsd_parser_types::quick_xml::WithDeserializer for PersonsType {
            type Deserializer = quick_xml_deserialize::PersonsTypeDeserializer;
        }
        pub mod quick_xml_deserialize {
            use xsd_parser_types::quick_xml::Deserializer as _;
            #[derive(Debug)]
            pub struct PersonsTypeDeserializer {
                person: ::std::vec::Vec<super::super::base::PersonType>,
                state__: ::std::boxed::Box<PersonsTypeDeserializerState>,
            }
            #[derive(Debug)]
            enum PersonsTypeDeserializerState {
                Init__ , Person (:: core :: option :: Option << super :: super :: base :: PersonType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
            impl PersonsTypeDeserializer {
                fn from_bytes_start(
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
                ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error>
                {
                    for attrib in helper.filter_xmlns_attributes(bytes_start) {
                        let attrib = attrib?;
                        helper.raise_unexpected_attrib_checked(&attrib)?;
                    }
                    Ok(Self {
                        person: ::std::vec::Vec::new(),
                        state__: ::std::boxed::Box::new(PersonsTypeDeserializerState::Init__),
                    })
                }
                fn finish_state(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    state: PersonsTypeDeserializerState,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    use PersonsTypeDeserializerState as S;
                    match state {
                        S::Person(Some(deserializer)) => {
                            self.store_person(deserializer.finish(helper)?)?
                        }
                        _ => (),
                    }
                    Ok(())
                }
                fn store_person(
                    &mut self,
                    value: super::super::base::PersonType,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    self.person.push(value);
                    Ok(())
                }
                fn handle_person<'de>(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                        'de,
                        super::super::base::PersonType,
                    >,
                    fallback: &mut ::core::option::Option<PersonsTypeDeserializerState>,
                ) -> ::core::result::Result<
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    use PersonsTypeDeserializerState as S;
                    let ::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = output;
                    if artifact.is_none() {
                        fallback.get_or_insert(S::Person(None));
                        *self.state__ = S::Done__;
                        return Ok(
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                event, allow_any,
                            ),
                        );
                    }
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(helper, fallback)?;
                    }
                    match artifact {
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                            self.store_person(data)?;
                            *self.state__ = S::Person(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            deserializer,
                        ) => {
                            fallback.get_or_insert(S::Person(Some(deserializer)));
                            *self.state__ = S::Person(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                    }
                }
            }
            impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::PersonsType>
                for PersonsTypeDeserializer
            {
                fn init(
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    event: ::xsd_parser_types::quick_xml::Event<'de>,
                ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PersonsType>
                {
                    helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
                }
                fn next(
                    mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    event: ::xsd_parser_types::quick_xml::Event<'de>,
                ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PersonsType>
                {
                    use PersonsTypeDeserializerState as S;
                    let mut event = event;
                    let mut fallback = None;
                    let mut allow_any_element = false;
                    let (event, allow_any) = loop {
                        let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                        event = match (state, event) {
                            (S::Unknown__, _) => unreachable!(),
                            (S::Person(Some(deserializer)), event) => {
                                let output = deserializer.next(helper, event)?;
                                match self . handle_person (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                                if let Some(fallback) = fallback.take() {
                                    self.finish_state(helper, fallback)?;
                                }
                                return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                                    artifact:
                                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                            self.finish(helper)?,
                                        ),
                                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                                    allow_any: false,
                                });
                            }
                            (S::Init__, event) => {
                                fallback.get_or_insert(S::Init__);
                                *self.state__ = S::Person(None);
                                event
                            }
                            (
                                S::Person(None),
                                event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                                | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                            ) => {
                                let output = helper.init_start_tag_deserializer(
                                    event,
                                    Some(&super::super::super::NS_TNS),
                                    b"Person",
                                    false,
                                )?;
                                match self . handle_person (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (S::Done__, event) => {
                                *self.state__ = S::Done__;
                                break (
                                    ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(
                                        event,
                                    ),
                                    allow_any_element,
                                );
                            }
                            (state, event) => {
                                *self.state__ = state;
                                break (
                                    ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                                    false,
                                );
                            }
                        }
                    };
                    if let Some(fallback) = fallback {
                        *self.state__ = fallback;
                    }
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            self,
                        ),
                        event,
                        allow_any,
                    })
                }
                fn finish(
                    mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                ) -> ::core::result::Result<super::PersonsType, ::xsd_parser_types::quick_xml::Error>
                {
                    let state = ::core::mem::replace(
                        &mut *self.state__,
                        PersonsTypeDeserializerState::Unknown__,
                    );
                    self.finish_state(helper, state)?;
                    Ok(super::PersonsType {
                        person: self.person,
                    })
                }
            }
        }
        pub mod quick_xml_serialize {
            use xsd_parser_types::quick_xml::Serializer as _;
            #[derive(Debug)]
            pub struct PersonsTypeSerializer<'ser> {
                pub(super) value: &'ser super::PersonsType,
                pub(super) state: ::std::boxed::Box<PersonsTypeSerializerState<'ser>>,
                pub(super) name: &'ser ::core::primitive::str,
                pub(super) is_root: ::core::primitive::bool,
            }
            #[derive(Debug)]
            pub(super) enum PersonsTypeSerializerState<'ser> {
                Init__,
                Person(
                    ::xsd_parser_types::quick_xml::IterSerializer<
                        'ser,
                        &'ser [super::super::base::PersonType],
                        super::super::base::PersonType,
                    >,
                ),
                End__,
                Done__,
                Phantom__(&'ser ()),
            }
            impl<'ser> PersonsTypeSerializer<'ser> {
                fn next_event(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
                ) -> ::core::result::Result<
                    ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    loop {
                        match &mut *self.state {
                            PersonsTypeSerializerState::Init__ => {
                                *self.state = PersonsTypeSerializerState::Person(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        &self.value.person[..],
                                        Some("Person"),
                                        false,
                                    ),
                                );
                                let mut bytes =
                                    ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                                helper.begin_ns_scope();
                                if self.is_root {
                                    helper.write_xmlns(
                                        &mut bytes,
                                        Some(&super::super::super::PREFIX_XSI),
                                        &super::super::super::NS_XSI,
                                    );
                                }
                                return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(
                                    bytes,
                                )));
                            }
                            PersonsTypeSerializerState::Person(x) => {
                                match x.next(helper).transpose()? {
                                    Some(event) => return Ok(Some(event)),
                                    None => *self.state = PersonsTypeSerializerState::End__,
                                }
                            }
                            PersonsTypeSerializerState::End__ => {
                                *self.state = PersonsTypeSerializerState::Done__;
                                helper.end_ns_scope();
                                return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                                    ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                                )));
                            }
                            PersonsTypeSerializerState::Done__ => return Ok(None),
                            PersonsTypeSerializerState::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }
            impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for PersonsTypeSerializer<'ser> {
                fn next(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
                ) -> ::core::option::Option<
                    ::core::result::Result<
                        ::xsd_parser_types::quick_xml::Event<'ser>,
                        ::xsd_parser_types::quick_xml::Error,
                    >,
                > {
                    match self.next_event(helper) {
                        Ok(Some(event)) => Some(Ok(event)),
                        Ok(None) => None,
                        Err(error) => {
                            *self.state = PersonsTypeSerializerState::Done__;
                            Some(Err(error))
                        }
                    }
                }
            }
        }
    }
    pub mod schema_2 {
        pub type AdvancedPersons = AdvancedPersonsType;
        #[derive(Debug)]
        pub struct AdvancedPersonsType {
            pub person: ::std::vec::Vec<PersonType>,
        }
        impl ::xsd_parser_types::quick_xml::WithSerializer for AdvancedPersonsType {
            type Serializer<'x> = quick_xml_serialize::AdvancedPersonsTypeSerializer<'x>;
            fn serializer<'ser>(
                &'ser self,
                name: ::core::option::Option<&'ser ::core::primitive::str>,
                is_root: ::core::primitive::bool,
            ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error>
            {
                Ok(quick_xml_serialize::AdvancedPersonsTypeSerializer {
                    value: self,
                    state: ::std::boxed::Box::new(
                        quick_xml_serialize::AdvancedPersonsTypeSerializerState::Init__,
                    ),
                    name: name.unwrap_or("AdvancedPersonsType"),
                    is_root,
                })
            }
        }
        impl ::xsd_parser_types::quick_xml::WithDeserializer for AdvancedPersonsType {
            type Deserializer = quick_xml_deserialize::AdvancedPersonsTypeDeserializer;
        }
        #[derive(Debug)]
        pub struct PersonType {
            pub name: ::std::string::String,
            pub last_name: ::std::string::String,
            pub age: ::core::primitive::i32,
            pub gender: super::base::GenderType,
        }
        impl ::xsd_parser_types::quick_xml::WithSerializer for PersonType {
            type Serializer<'x> = quick_xml_serialize::PersonTypeSerializer<'x>;
            fn serializer<'ser>(
                &'ser self,
                name: ::core::option::Option<&'ser ::core::primitive::str>,
                is_root: ::core::primitive::bool,
            ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error>
            {
                Ok(quick_xml_serialize::PersonTypeSerializer {
                    value: self,
                    state: ::std::boxed::Box::new(
                        quick_xml_serialize::PersonTypeSerializerState::Init__,
                    ),
                    name: name.unwrap_or("PersonType"),
                    is_root,
                })
            }
        }
        impl ::xsd_parser_types::quick_xml::WithDeserializer for PersonType {
            type Deserializer = quick_xml_deserialize::PersonTypeDeserializer;
        }
        pub mod quick_xml_deserialize {
            use xsd_parser_types::quick_xml::Deserializer as _;
            #[derive(Debug)]
            pub struct AdvancedPersonsTypeDeserializer {
                person: ::std::vec::Vec<super::PersonType>,
                state__: ::std::boxed::Box<AdvancedPersonsTypeDeserializerState>,
            }
            #[derive(Debug)]
            enum AdvancedPersonsTypeDeserializerState {
                Init__ , Person (:: core :: option :: Option << super :: PersonType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
            impl AdvancedPersonsTypeDeserializer {
                fn from_bytes_start(
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
                ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error>
                {
                    for attrib in helper.filter_xmlns_attributes(bytes_start) {
                        let attrib = attrib?;
                        helper.raise_unexpected_attrib_checked(&attrib)?;
                    }
                    Ok(Self {
                        person: ::std::vec::Vec::new(),
                        state__: ::std::boxed::Box::new(
                            AdvancedPersonsTypeDeserializerState::Init__,
                        ),
                    })
                }
                fn finish_state(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    state: AdvancedPersonsTypeDeserializerState,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    use AdvancedPersonsTypeDeserializerState as S;
                    match state {
                        S::Person(Some(deserializer)) => {
                            self.store_person(deserializer.finish(helper)?)?
                        }
                        _ => (),
                    }
                    Ok(())
                }
                fn store_person(
                    &mut self,
                    value: super::PersonType,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    self.person.push(value);
                    Ok(())
                }
                fn handle_person<'de>(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                        'de,
                        super::PersonType,
                    >,
                    fallback: &mut ::core::option::Option<AdvancedPersonsTypeDeserializerState>,
                ) -> ::core::result::Result<
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    use AdvancedPersonsTypeDeserializerState as S;
                    let ::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = output;
                    if artifact.is_none() {
                        fallback.get_or_insert(S::Person(None));
                        *self.state__ = S::Done__;
                        return Ok(
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                event, allow_any,
                            ),
                        );
                    }
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(helper, fallback)?;
                    }
                    match artifact {
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                            self.store_person(data)?;
                            *self.state__ = S::Person(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            deserializer,
                        ) => {
                            fallback.get_or_insert(S::Person(Some(deserializer)));
                            *self.state__ = S::Person(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                    }
                }
            }
            impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::AdvancedPersonsType>
                for AdvancedPersonsTypeDeserializer
            {
                fn init(
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    event: ::xsd_parser_types::quick_xml::Event<'de>,
                ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
                    'de,
                    super::AdvancedPersonsType,
                > {
                    helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
                }
                fn next(
                    mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    event: ::xsd_parser_types::quick_xml::Event<'de>,
                ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
                    'de,
                    super::AdvancedPersonsType,
                > {
                    use AdvancedPersonsTypeDeserializerState as S;
                    let mut event = event;
                    let mut fallback = None;
                    let mut allow_any_element = false;
                    let (event, allow_any) = loop {
                        let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                        event = match (state, event) {
                            (S::Unknown__, _) => unreachable!(),
                            (S::Person(Some(deserializer)), event) => {
                                let output = deserializer.next(helper, event)?;
                                match self . handle_person (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                                if let Some(fallback) = fallback.take() {
                                    self.finish_state(helper, fallback)?;
                                }
                                return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                                    artifact:
                                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                            self.finish(helper)?,
                                        ),
                                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                                    allow_any: false,
                                });
                            }
                            (S::Init__, event) => {
                                fallback.get_or_insert(S::Init__);
                                *self.state__ = S::Person(None);
                                event
                            }
                            (
                                S::Person(None),
                                event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                                | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                            ) => {
                                let output = helper.init_start_tag_deserializer(
                                    event,
                                    Some(&super::super::super::NS_TNS),
                                    b"Person",
                                    false,
                                )?;
                                match self . handle_person (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (S::Done__, event) => {
                                *self.state__ = S::Done__;
                                break (
                                    ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(
                                        event,
                                    ),
                                    allow_any_element,
                                );
                            }
                            (state, event) => {
                                *self.state__ = state;
                                break (
                                    ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                                    false,
                                );
                            }
                        }
                    };
                    if let Some(fallback) = fallback {
                        *self.state__ = fallback;
                    }
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            self,
                        ),
                        event,
                        allow_any,
                    })
                }
                fn finish(
                    mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                ) -> ::core::result::Result<
                    super::AdvancedPersonsType,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    let state = ::core::mem::replace(
                        &mut *self.state__,
                        AdvancedPersonsTypeDeserializerState::Unknown__,
                    );
                    self.finish_state(helper, state)?;
                    Ok(super::AdvancedPersonsType {
                        person: self.person,
                    })
                }
            }
            #[derive(Debug)]
            pub struct PersonTypeDeserializer {
                name: ::core::option::Option<::std::string::String>,
                last_name: ::core::option::Option<::std::string::String>,
                age: ::core::option::Option<::core::primitive::i32>,
                gender: ::core::option::Option<super::super::base::GenderType>,
                state__: ::std::boxed::Box<PersonTypeDeserializerState>,
            }
            #[derive(Debug)]
            enum PersonTypeDeserializerState {
                Init__ , Name (:: core :: option :: Option << :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , LastName (:: core :: option :: Option << :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Age (:: core :: option :: Option << :: core :: primitive :: i32 as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Gender (:: core :: option :: Option << super :: super :: base :: GenderType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
            impl PersonTypeDeserializer {
                fn from_bytes_start(
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
                ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error>
                {
                    for attrib in helper.filter_xmlns_attributes(bytes_start) {
                        let attrib = attrib?;
                        helper.raise_unexpected_attrib_checked(&attrib)?;
                    }
                    Ok(Self {
                        name: None,
                        last_name: None,
                        age: None,
                        gender: None,
                        state__: ::std::boxed::Box::new(PersonTypeDeserializerState::Init__),
                    })
                }
                fn finish_state(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    state: PersonTypeDeserializerState,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    use PersonTypeDeserializerState as S;
                    match state {
                        S::Name(Some(deserializer)) => {
                            self.store_name(deserializer.finish(helper)?)?
                        }
                        S::LastName(Some(deserializer)) => {
                            self.store_last_name(deserializer.finish(helper)?)?
                        }
                        S::Age(Some(deserializer)) => {
                            self.store_age(deserializer.finish(helper)?)?
                        }
                        S::Gender(Some(deserializer)) => {
                            self.store_gender(deserializer.finish(helper)?)?
                        }
                        _ => (),
                    }
                    Ok(())
                }
                fn store_name(
                    &mut self,
                    value: ::std::string::String,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    if self.name.is_some() {
                        Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                            ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"name"),
                        ))?;
                    }
                    self.name = Some(value);
                    Ok(())
                }
                fn store_last_name(
                    &mut self,
                    value: ::std::string::String,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    if self.last_name.is_some() {
                        Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                            ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"lastName"),
                        ))?;
                    }
                    self.last_name = Some(value);
                    Ok(())
                }
                fn store_age(
                    &mut self,
                    value: ::core::primitive::i32,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    if self.age.is_some() {
                        Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                            ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"age"),
                        ))?;
                    }
                    self.age = Some(value);
                    Ok(())
                }
                fn store_gender(
                    &mut self,
                    value: super::super::base::GenderType,
                ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error>
                {
                    if self.gender.is_some() {
                        Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                            ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"gender"),
                        ))?;
                    }
                    self.gender = Some(value);
                    Ok(())
                }
                fn handle_name<'de>(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                        'de,
                        ::std::string::String,
                    >,
                    fallback: &mut ::core::option::Option<PersonTypeDeserializerState>,
                ) -> ::core::result::Result<
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    use PersonTypeDeserializerState as S;
                    let ::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = output;
                    if artifact.is_none() {
                        fallback.get_or_insert(S::Name(None));
                        if matches!(&fallback, Some(S::Init__)) {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::break_(
                                    event, allow_any,
                                ),
                            );
                        } else {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                                    event, allow_any,
                                ),
                            );
                        }
                    }
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(helper, fallback)?;
                    }
                    match artifact {
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                            self.store_name(data)?;
                            *self.state__ = S::LastName(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            deserializer,
                        ) => {
                            fallback.get_or_insert(S::Name(Some(deserializer)));
                            *self.state__ = S::LastName(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                    }
                }
                fn handle_last_name<'de>(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                        'de,
                        ::std::string::String,
                    >,
                    fallback: &mut ::core::option::Option<PersonTypeDeserializerState>,
                ) -> ::core::result::Result<
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    use PersonTypeDeserializerState as S;
                    let ::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = output;
                    if artifact.is_none() {
                        fallback.get_or_insert(S::LastName(None));
                        if matches!(&fallback, Some(S::Init__)) {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::break_(
                                    event, allow_any,
                                ),
                            );
                        } else {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                                    event, allow_any,
                                ),
                            );
                        }
                    }
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(helper, fallback)?;
                    }
                    match artifact {
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                            self.store_last_name(data)?;
                            *self.state__ = S::Age(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            deserializer,
                        ) => {
                            fallback.get_or_insert(S::LastName(Some(deserializer)));
                            *self.state__ = S::Age(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                    }
                }
                fn handle_age<'de>(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                        'de,
                        ::core::primitive::i32,
                    >,
                    fallback: &mut ::core::option::Option<PersonTypeDeserializerState>,
                ) -> ::core::result::Result<
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    use PersonTypeDeserializerState as S;
                    let ::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = output;
                    if artifact.is_none() {
                        fallback.get_or_insert(S::Age(None));
                        if matches!(&fallback, Some(S::Init__)) {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::break_(
                                    event, allow_any,
                                ),
                            );
                        } else {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                                    event, allow_any,
                                ),
                            );
                        }
                    }
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(helper, fallback)?;
                    }
                    match artifact {
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                            self.store_age(data)?;
                            *self.state__ = S::Gender(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            deserializer,
                        ) => {
                            fallback.get_or_insert(S::Age(Some(deserializer)));
                            *self.state__ = S::Gender(None);
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                    }
                }
                fn handle_gender<'de>(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                        'de,
                        super::super::base::GenderType,
                    >,
                    fallback: &mut ::core::option::Option<PersonTypeDeserializerState>,
                ) -> ::core::result::Result<
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    use PersonTypeDeserializerState as S;
                    let ::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = output;
                    if artifact.is_none() {
                        fallback.get_or_insert(S::Gender(None));
                        if matches!(&fallback, Some(S::Init__)) {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::break_(
                                    event, allow_any,
                                ),
                            );
                        } else {
                            return Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                                    event, allow_any,
                                ),
                            );
                        }
                    }
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(helper, fallback)?;
                    }
                    match artifact {
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                            self.store_gender(data)?;
                            *self.state__ = S::Done__;
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            deserializer,
                        ) => {
                            fallback.get_or_insert(S::Gender(Some(deserializer)));
                            *self.state__ = S::Done__;
                            Ok(
                                ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                                    event, allow_any,
                                ),
                            )
                        }
                    }
                }
            }
            impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::PersonType>
                for PersonTypeDeserializer
            {
                fn init(
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    event: ::xsd_parser_types::quick_xml::Event<'de>,
                ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PersonType>
                {
                    helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
                }
                fn next(
                    mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                    event: ::xsd_parser_types::quick_xml::Event<'de>,
                ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PersonType>
                {
                    use PersonTypeDeserializerState as S;
                    let mut event = event;
                    let mut fallback = None;
                    let mut allow_any_element = false;
                    let (event, allow_any) = loop {
                        let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                        event = match (state, event) {
                            (S::Unknown__, _) => unreachable!(),
                            (S::Name(Some(deserializer)), event) => {
                                let output = deserializer.next(helper, event)?;
                                match self . handle_name (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (S::LastName(Some(deserializer)), event) => {
                                let output = deserializer.next(helper, event)?;
                                match self . handle_last_name (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (S::Age(Some(deserializer)), event) => {
                                let output = deserializer.next(helper, event)?;
                                match self . handle_age (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (S::Gender(Some(deserializer)), event) => {
                                let output = deserializer.next(helper, event)?;
                                match self . handle_gender (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                                if let Some(fallback) = fallback.take() {
                                    self.finish_state(helper, fallback)?;
                                }
                                return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                                    artifact:
                                        ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                            self.finish(helper)?,
                                        ),
                                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                                    allow_any: false,
                                });
                            }
                            (S::Init__, event) => {
                                fallback.get_or_insert(S::Init__);
                                *self.state__ = S::Name(None);
                                event
                            }
                            (
                                S::Name(None),
                                event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                                | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                            ) => {
                                let output = helper.init_start_tag_deserializer(
                                    event,
                                    Some(&super::super::super::NS_TNS),
                                    b"name",
                                    false,
                                )?;
                                match self . handle_name (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (
                                S::LastName(None),
                                event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                                | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                            ) => {
                                let output = helper.init_start_tag_deserializer(
                                    event,
                                    Some(&super::super::super::NS_TNS),
                                    b"lastName",
                                    false,
                                )?;
                                match self . handle_last_name (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (
                                S::Age(None),
                                event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                                | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                            ) => {
                                let output = helper.init_start_tag_deserializer(
                                    event,
                                    Some(&super::super::super::NS_TNS),
                                    b"age",
                                    false,
                                )?;
                                match self . handle_age (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (
                                S::Gender(None),
                                event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                                | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                            ) => {
                                let output = helper.init_start_tag_deserializer(
                                    event,
                                    Some(&super::super::super::NS_TNS),
                                    b"gender",
                                    false,
                                )?;
                                match self . handle_gender (helper , output , & mut fallback) ? { :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Continue { event , allow_any } => { allow_any_element = allow_any_element || allow_any ; event } , :: xsd_parser_types :: quick_xml :: ElementHandlerOutput :: Break { event , allow_any } => break (event , allow_any) , }
                            }
                            (S::Done__, event) => {
                                *self.state__ = S::Done__;
                                break (
                                    ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(
                                        event,
                                    ),
                                    allow_any_element,
                                );
                            }
                            (state, event) => {
                                *self.state__ = state;
                                break (
                                    ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                                    false,
                                );
                            }
                        }
                    };
                    if let Some(fallback) = fallback {
                        *self.state__ = fallback;
                    }
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            self,
                        ),
                        event,
                        allow_any,
                    })
                }
                fn finish(
                    mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
                ) -> ::core::result::Result<super::PersonType, ::xsd_parser_types::quick_xml::Error>
                {
                    let state = ::core::mem::replace(
                        &mut *self.state__,
                        PersonTypeDeserializerState::Unknown__,
                    );
                    self.finish_state(helper, state)?;
                    Ok(super::PersonType {
                        name: helper.finish_element("name", self.name)?,
                        last_name: helper.finish_element("lastName", self.last_name)?,
                        age: helper.finish_element("age", self.age)?,
                        gender: helper.finish_element("gender", self.gender)?,
                    })
                }
            }
        }
        pub mod quick_xml_serialize {
            use xsd_parser_types::quick_xml::Serializer as _;
            #[derive(Debug)]
            pub struct AdvancedPersonsTypeSerializer<'ser> {
                pub(super) value: &'ser super::AdvancedPersonsType,
                pub(super) state: ::std::boxed::Box<AdvancedPersonsTypeSerializerState<'ser>>,
                pub(super) name: &'ser ::core::primitive::str,
                pub(super) is_root: ::core::primitive::bool,
            }
            #[derive(Debug)]
            pub(super) enum AdvancedPersonsTypeSerializerState<'ser> {
                Init__,
                Person(
                    ::xsd_parser_types::quick_xml::IterSerializer<
                        'ser,
                        &'ser [super::PersonType],
                        super::PersonType,
                    >,
                ),
                End__,
                Done__,
                Phantom__(&'ser ()),
            }
            impl<'ser> AdvancedPersonsTypeSerializer<'ser> {
                fn next_event(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
                ) -> ::core::result::Result<
                    ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    loop {
                        match &mut *self.state {
                            AdvancedPersonsTypeSerializerState::Init__ => {
                                *self.state = AdvancedPersonsTypeSerializerState::Person(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        &self.value.person[..],
                                        Some("Person"),
                                        false,
                                    ),
                                );
                                let mut bytes =
                                    ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                                helper.begin_ns_scope();
                                if self.is_root {
                                    helper.write_xmlns(
                                        &mut bytes,
                                        Some(&super::super::super::PREFIX_XSI),
                                        &super::super::super::NS_XSI,
                                    );
                                }
                                return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(
                                    bytes,
                                )));
                            }
                            AdvancedPersonsTypeSerializerState::Person(x) => {
                                match x.next(helper).transpose()? {
                                    Some(event) => return Ok(Some(event)),
                                    None => *self.state = AdvancedPersonsTypeSerializerState::End__,
                                }
                            }
                            AdvancedPersonsTypeSerializerState::End__ => {
                                *self.state = AdvancedPersonsTypeSerializerState::Done__;
                                helper.end_ns_scope();
                                return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                                    ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                                )));
                            }
                            AdvancedPersonsTypeSerializerState::Done__ => return Ok(None),
                            AdvancedPersonsTypeSerializerState::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }
            impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for AdvancedPersonsTypeSerializer<'ser> {
                fn next(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
                ) -> ::core::option::Option<
                    ::core::result::Result<
                        ::xsd_parser_types::quick_xml::Event<'ser>,
                        ::xsd_parser_types::quick_xml::Error,
                    >,
                > {
                    match self.next_event(helper) {
                        Ok(Some(event)) => Some(Ok(event)),
                        Ok(None) => None,
                        Err(error) => {
                            *self.state = AdvancedPersonsTypeSerializerState::Done__;
                            Some(Err(error))
                        }
                    }
                }
            }
            #[derive(Debug)]
            pub struct PersonTypeSerializer<'ser> {
                pub(super) value: &'ser super::PersonType,
                pub(super) state: ::std::boxed::Box<PersonTypeSerializerState<'ser>>,
                pub(super) name: &'ser ::core::primitive::str,
                pub(super) is_root: ::core::primitive::bool,
            }
            #[derive(Debug)]
            pub(super) enum PersonTypeSerializerState<'ser> {
                Init__ , Name (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , LastName (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , Age (< :: core :: primitive :: i32 as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , Gender (< super :: super :: base :: GenderType as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , End__ , Done__ , Phantom__ (& 'ser ()) , }
            impl<'ser> PersonTypeSerializer<'ser> {
                fn next_event(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
                ) -> ::core::result::Result<
                    ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
                    ::xsd_parser_types::quick_xml::Error,
                > {
                    loop {
                        match &mut *self.state {
                            PersonTypeSerializerState::Init__ => {
                                *self.state = PersonTypeSerializerState::Name(
                                    ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                        &self.value.name,
                                        Some("name"),
                                        false,
                                    )?,
                                );
                                let mut bytes =
                                    ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                                helper.begin_ns_scope();
                                if self.is_root {
                                    helper.write_xmlns(
                                        &mut bytes,
                                        Some(&super::super::super::PREFIX_XSI),
                                        &super::super::super::NS_XSI,
                                    );
                                }
                                return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(
                                    bytes,
                                )));
                            }
                            PersonTypeSerializerState::Name(x) => {
                                match x.next(helper).transpose()? {
                                    Some(event) => return Ok(Some(event)),
                                    None => *self.state = PersonTypeSerializerState::LastName(
                                        ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                            &self.value.last_name,
                                            Some("lastName"),
                                            false,
                                        )?,
                                    ),
                                }
                            }
                            PersonTypeSerializerState::LastName(x) => {
                                match x.next(helper).transpose()? {
                                    Some(event) => return Ok(Some(event)),
                                    None => *self.state = PersonTypeSerializerState::Age(
                                        ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                            &self.value.age,
                                            Some("age"),
                                            false,
                                        )?,
                                    ),
                                }
                            }
                            PersonTypeSerializerState::Age(x) => {
                                match x.next(helper).transpose()? {
                                    Some(event) => return Ok(Some(event)),
                                    None => *self.state = PersonTypeSerializerState::Gender(
                                        ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                            &self.value.gender,
                                            Some("gender"),
                                            false,
                                        )?,
                                    ),
                                }
                            }
                            PersonTypeSerializerState::Gender(x) => {
                                match x.next(helper).transpose()? {
                                    Some(event) => return Ok(Some(event)),
                                    None => *self.state = PersonTypeSerializerState::End__,
                                }
                            }
                            PersonTypeSerializerState::End__ => {
                                *self.state = PersonTypeSerializerState::Done__;
                                helper.end_ns_scope();
                                return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                                    ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                                )));
                            }
                            PersonTypeSerializerState::Done__ => return Ok(None),
                            PersonTypeSerializerState::Phantom__(_) => unreachable!(),
                        }
                    }
                }
            }
            impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for PersonTypeSerializer<'ser> {
                fn next(
                    &mut self,
                    helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
                ) -> ::core::option::Option<
                    ::core::result::Result<
                        ::xsd_parser_types::quick_xml::Event<'ser>,
                        ::xsd_parser_types::quick_xml::Error,
                    >,
                > {
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
        }
    }
}
