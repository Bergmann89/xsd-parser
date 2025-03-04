pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub content_3: FooContent3Type,
    pub content_6: FooContent6Type,
}
impl xsd_parser::quick_xml::WithSerializer for FooType {
    type Serializer<'x> = quick_xml_serialize::FooTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        Ok(quick_xml_serialize::FooTypeSerializer {
            name: name.unwrap_or("tns:FooType"),
            value: self,
            is_root,
            state: quick_xml_serialize::FooTypeSerializerState::Init__,
        })
    }
}
#[derive(Debug, Clone)]
pub enum FooContent3Type {
    Element1(i32),
    Element2(String),
}
impl xsd_parser::quick_xml::WithSerializer for FooContent3Type {
    type Serializer<'x> = quick_xml_serialize::FooContent3TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::FooContent3TypeSerializer {
            value: self,
            state: quick_xml_serialize::FooContent3TypeSerializerState::Init__,
        })
    }
}
#[derive(Debug, Clone)]
pub enum FooContent6Type {
    Element3(i32),
    Element4(String),
}
impl xsd_parser::quick_xml::WithSerializer for FooContent6Type {
    type Serializer<'x> = quick_xml_serialize::FooContent6TypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::FooContent6TypeSerializer {
            value: self,
            state: quick_xml_serialize::FooContent6TypeSerializerState::Init__,
        })
    }
}
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct FooTypeSerializer<'ser> {
        pub(super) name: &'ser str,
        pub(super) value: &'ser super::FooType,
        pub(super) is_root: bool,
        pub(super) state: FooTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum FooTypeSerializerState<'ser> {
        Init__,
        Content3(<FooContent3Type as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Content6(<FooContent6Type as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
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
                        self.state = FooTypeSerializerState::Content3(
                            xsd_parser::quick_xml::WithSerializer::serializer(
                                &self.value.content_3,
                                Some("tns:Content3"),
                                false,
                            )?,
                        );
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Content3(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = FooTypeSerializerState::Content6(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    &self.value.content_6,
                                    Some("tns:Content6"),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeSerializerState::Content6(x) => match x.next().transpose()? {
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
    pub struct FooContent3TypeSerializer<'ser> {
        pub(super) value: &'ser super::FooContent3Type,
        pub(super) state: FooContent3TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum FooContent3TypeSerializerState<'ser> {
        Init__,
        Element1(<i32 as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Element2(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooContent3TypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    FooContent3TypeSerializerState::Init__ => match self.value {
                        FooContent3Type::Element1(x) => {
                            self.state = FooContent3TypeSerializerState::Element1(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    x,
                                    Some("tns:Element1"),
                                    false,
                                )?,
                            )
                        }
                        FooContent3Type::Element2(x) => {
                            self.state = FooContent3TypeSerializerState::Element2(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    x,
                                    Some("tns:Element2"),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooContent3TypeSerializerState::Element1(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooContent3TypeSerializerState::Done__,
                    },
                    FooContent3TypeSerializerState::Element2(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooContent3TypeSerializerState::Done__,
                    },
                    FooContent3TypeSerializerState::Done__ => return Ok(None),
                    FooContent3TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for FooContent3TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            self.next_event().transpose()
        }
    }
    #[derive(Debug)]
    pub struct FooContent6TypeSerializer<'ser> {
        pub(super) value: &'ser super::FooContent6Type,
        pub(super) state: FooContent6TypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum FooContent6TypeSerializerState<'ser> {
        Init__,
        Element3(<i32 as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Element4(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FooContent6TypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    FooContent6TypeSerializerState::Init__ => match self.value {
                        FooContent6Type::Element3(x) => {
                            self.state = FooContent6TypeSerializerState::Element3(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    x,
                                    Some("tns:Element3"),
                                    false,
                                )?,
                            )
                        }
                        FooContent6Type::Element4(x) => {
                            self.state = FooContent6TypeSerializerState::Element4(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    x,
                                    Some("tns:Element4"),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooContent6TypeSerializerState::Element3(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooContent6TypeSerializerState::Done__,
                    },
                    FooContent6TypeSerializerState::Element4(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = FooContent6TypeSerializerState::Done__,
                    },
                    FooContent6TypeSerializerState::Done__ => return Ok(None),
                    FooContent6TypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for FooContent6TypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            self.next_event().transpose()
        }
    }
}
