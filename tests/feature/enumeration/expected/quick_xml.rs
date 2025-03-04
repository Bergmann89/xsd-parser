pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub enum_: EnumType,
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
pub enum EnumType {
    Off,
    On,
    Auto,
}
impl xsd_parser::quick_xml::SerializeBytes for EnumType {
    fn serialize_bytes(
        &self,
    ) -> Result<Option<std::borrow::Cow<'_, str>>, xsd_parser::quick_xml::Error> {
        match self {
            Self::Off => Ok(Some(std::borrow::Cow::Borrowed("OFF"))),
            Self::On => Ok(Some(std::borrow::Cow::Borrowed("ON"))),
            Self::Auto => Ok(Some(std::borrow::Cow::Borrowed("AUTO"))),
        }
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
        Enum(<EnumType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
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
                        self.state = FooTypeSerializerState::Enum(
                            xsd_parser::quick_xml::WithSerializer::serializer(
                                &self.value.enum_,
                                Some("tns:Enum"),
                                false,
                            )?,
                        );
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Enum(x) => match x.next().transpose()? {
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
}
