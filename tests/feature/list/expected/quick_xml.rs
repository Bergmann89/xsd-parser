pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub a_list: ListType,
}
impl FooType {
    #[must_use]
    pub fn default_a_list() -> ListType {
        ListType(Vec::new())
    }
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
#[derive(Debug, Clone, Default)]
pub struct ListType(pub Vec<StringType>);
impl xsd_parser::quick_xml::SerializeBytes for ListType {
    fn serialize_bytes(
        &self,
    ) -> Result<Option<std::borrow::Cow<'_, str>>, xsd_parser::quick_xml::Error> {
        if self.0.is_empty() {
            return Ok(None);
        }
        let mut data = String::new();
        for item in &self.0 {
            if let Some(bytes) = item.serialize_bytes()? {
                if !data.is_empty() {
                    data.push(' ');
                }
                data.push_str(&bytes);
            }
        }
        Ok(Some(std::borrow::Cow::Owned(data)))
    }
}
pub type StringType = String;
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
                        self.state = FooTypeSerializerState::Done__;
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        xsd_parser::quick_xml::write_attrib(
                            &mut bytes,
                            "tns:a-list",
                            &self.value.a_list,
                        )?;
                        return Ok(Some(xsd_parser::quick_xml::Event::Empty(bytes)));
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
