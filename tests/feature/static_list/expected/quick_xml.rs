use xsd_parser::schema::Namespace;
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type Array = ArrayType;
#[derive(Debug, Clone)]
pub struct ArrayType {
    pub item: [i32; 5usize],
}
impl xsd_parser::quick_xml::WithSerializer for ArrayType {
    type Serializer<'x> = quick_xml_serialize::ArrayTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        Ok(quick_xml_serialize::ArrayTypeSerializer {
            value: self,
            state: quick_xml_serialize::ArrayTypeSerializerState::Init__,
            name: name.unwrap_or("tns:ArrayType"),
            is_root,
        })
    }
}
pub mod quick_xml_serialize {
    #[derive(Debug)]
    pub struct ArrayTypeSerializer<'ser> {
        pub(super) value: &'ser super::ArrayType,
        pub(super) state: ArrayTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ArrayTypeSerializerState<'ser> {
        Init__,
        Item(xsd_parser::quick_xml::IterSerializer<'ser, [i32; 5usize], i32>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ArrayTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    ArrayTypeSerializerState::Init__ => {
                        self.state = ArrayTypeSerializerState::Item(
                            xsd_parser::quick_xml::IterSerializer::new(
                                &self.value.item,
                                Some("tns:Item"),
                                false,
                            ),
                        );
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    ArrayTypeSerializerState::Item(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = ArrayTypeSerializerState::End__,
                    },
                    ArrayTypeSerializerState::End__ => {
                        self.state = ArrayTypeSerializerState::Done__;
                        return Ok(Some(xsd_parser::quick_xml::Event::End(
                            xsd_parser::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    ArrayTypeSerializerState::Done__ => return Ok(None),
                    ArrayTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for ArrayTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = ArrayTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
