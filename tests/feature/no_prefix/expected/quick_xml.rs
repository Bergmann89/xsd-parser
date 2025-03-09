pub type Foo = FooType;
#[derive(Debug, Clone)]
pub struct FooType {
    pub once: i32,
    pub optional: Option<i32>,
    pub once_specify: i32,
    pub twice_or_more: Vec<i32>,
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
            name: name.unwrap_or("FooType"),
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
        Once(<i32 as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Optional(xsd_parser::quick_xml::IterSerializer<'ser, Option<i32>, i32>),
        OnceSpecify(<i32 as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        TwiceOrMore(xsd_parser::quick_xml::IterSerializer<'ser, Vec<i32>, i32>),
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
                        self.state = FooTypeSerializerState::Once(
                            xsd_parser::quick_xml::WithSerializer::serializer(
                                &self.value.once,
                                Some("Once"),
                                false,
                            )?,
                        );
                        let bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    FooTypeSerializerState::Once(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = FooTypeSerializerState::Optional(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.optional,
                                    Some("Optional"),
                                    false,
                                ),
                            )
                        }
                    },
                    FooTypeSerializerState::Optional(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = FooTypeSerializerState::OnceSpecify(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    &self.value.once_specify,
                                    Some("OnceSpecify"),
                                    false,
                                )?,
                            )
                        }
                    },
                    FooTypeSerializerState::OnceSpecify(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = FooTypeSerializerState::TwiceOrMore(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.twice_or_more,
                                    Some("TwiceOrMore"),
                                    false,
                                ),
                            )
                        }
                    },
                    FooTypeSerializerState::TwiceOrMore(x) => match x.next().transpose()? {
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
