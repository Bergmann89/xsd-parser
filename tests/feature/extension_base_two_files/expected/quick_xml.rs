pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub a: f32,
    pub b: BarType,
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
pub struct BarType {
    pub b: i32,
    pub c: String,
}
impl xsd_parser::quick_xml::WithSerializer for BarType {
    type Serializer<'x> = quick_xml_serialize::BarTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        Ok(quick_xml_serialize::BarTypeSerializer {
            value: self,
            state: quick_xml_serialize::BarTypeSerializerState::Init__,
            name: name.unwrap_or("other:BarType"),
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
        A(<f32 as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        B(<BarType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
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
                        self.state = FooTypeSerializerState::A(
                            xsd_parser::quick_xml::WithSerializer::serializer(
                                &self.value.a,
                                Some("tns:a"),
                                false,
                            )?,
                        );
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                            bytes.push_attribute(("xmlns:other", "http://other.example.com"));
                        }
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    FooTypeSerializerState::A(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = FooTypeSerializerState::B(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    &self.value.b,
                                    Some("tns:b"),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeSerializerState::B(x) => match x.next().transpose()? {
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
    pub struct BarTypeSerializer<'ser> {
        pub(super) value: &'ser super::BarType,
        pub(super) state: BarTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum BarTypeSerializerState<'ser> {
        Init__,
        B(<i32 as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        C(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> BarTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    BarTypeSerializerState::Init__ => {
                        self.state = BarTypeSerializerState::B(
                            xsd_parser::quick_xml::WithSerializer::serializer(
                                &self.value.b,
                                Some("other:b"),
                                false,
                            )?,
                        );
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                            bytes.push_attribute(("xmlns:other", "http://other.example.com"));
                        }
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    BarTypeSerializerState::B(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = BarTypeSerializerState::C(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    &self.value.c,
                                    Some("other:c"),
                                    false,
                                )?,
                            )
                        }
                    },
                    BarTypeSerializerState::C(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = BarTypeSerializerState::End__,
                    },
                    BarTypeSerializerState::End__ => {
                        self.state = BarTypeSerializerState::Done__;
                        return Ok(Some(xsd_parser::quick_xml::Event::End(
                            xsd_parser::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    BarTypeSerializerState::Done__ => return Ok(None),
                    BarTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for BarTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = BarTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
