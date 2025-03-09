pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub messages: FooTypeMessagesType,
}
impl xsd_parser::quick_xml::WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        Ok(quick_xml_serialize::FooTypeSerializer {
            value: self,
            state: quick_xml_serialize::FooTypeSerializerState::Init__,
            name: name.unwrap_or("tns:FooType"),
            is_root,
        })
    }
}
#[derive(Debug, Clone)]
pub struct FooTypeMessagesType {
    pub aa: i32,
    pub bb: String,
    pub a: String,
}
impl xsd_parser::quick_xml::WithSerializer for FooTypeMessagesType {
    type Serializer<'x> = quick_xml_serialize::FooTypeMessagesTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        Ok(quick_xml_serialize::FooTypeMessagesTypeSerializer {
            value: self,
            state: quick_xml_serialize::FooTypeMessagesTypeSerializerState::Init__,
            name: name.unwrap_or("tns:FooTypeMessages"),
            is_root,
        })
    }
}
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct FooTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooType,
        pub(super) state: FooTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeSerializerState<'ser> {
        Init__,
        Messages(<FooTypeMessagesType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    FooTypeSerializerState::Init__ => {
                        self.state = FooTypeSerializerState::Messages(
                            xsd_parser::quick_xml::WithSerializer::serializer(
                                &self.value.messages,
                                Some("tns:Messages"),
                                false,
                            )?,
                        );
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Messages(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooTypeSerializerState::End__,
                    },
                    FooTypeSerializerState::End__ => {
                        self.state = FooTypeSerializerState::Done__;
                        return Ok(Some(xsd_parser::quick_xml::Event::End(
                            xsd_parser::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    FooTypeSerializerState::Done__ => return Ok(None),
                    FooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for FooTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = FooTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FooTypeMessagesTypeSerializer<'ser> {
        pub(super) value: &'ser super::FooTypeMessagesType,
        pub(super) state: FooTypeMessagesTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeMessagesTypeSerializerState<'ser> {
        Init__,
        Aa(<i32 as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Bb(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        A(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooTypeMessagesTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    FooTypeMessagesTypeSerializerState::Init__ => {
                        self.state = FooTypeMessagesTypeSerializerState::Aa(
                            xsd_parser::quick_xml::WithSerializer::serializer(
                                &self.value.aa,
                                Some("tns:aa"),
                                false,
                            )?,
                        );
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    FooTypeMessagesTypeSerializerState::Aa(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = FooTypeMessagesTypeSerializerState::Bb(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    &self.value.bb,
                                    Some("tns:bb"),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeMessagesTypeSerializerState::Bb(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = FooTypeMessagesTypeSerializerState::A(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    &self.value.a,
                                    Some("tns:a"),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeMessagesTypeSerializerState::A(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooTypeMessagesTypeSerializerState::End__,
                    },
                    FooTypeMessagesTypeSerializerState::End__ => {
                        self.state = FooTypeMessagesTypeSerializerState::Done__;
                        return Ok(Some(xsd_parser::quick_xml::Event::End(
                            xsd_parser::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    FooTypeMessagesTypeSerializerState::Done__ => return Ok(None),
                    FooTypeMessagesTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for FooTypeMessagesTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = FooTypeMessagesTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
