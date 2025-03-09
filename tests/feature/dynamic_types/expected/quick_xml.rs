pub type List = ListType;
#[derive(Debug)]
pub struct ListType {
    pub base: Vec<Base>,
}
impl xsd_parser::quick_xml::WithSerializer for ListType {
    type Serializer<'x> = quick_xml_serialize::ListTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        Ok(quick_xml_serialize::ListTypeSerializer {
            value: self,
            state: quick_xml_serialize::ListTypeSerializerState::Init__,
            name: name.unwrap_or("tns:list"),
            is_root,
        })
    }
}
#[derive(Debug)]
pub struct Base(pub Box<dyn BaseTrait>);
pub trait BaseTrait:
    core::fmt::Debug + xsd_parser::quick_xml::WithBoxedSerializer + xsd_parser::AsAny
{
}
impl xsd_parser::quick_xml::WithSerializer for Base {
    type Serializer<'x> = xsd_parser::quick_xml::BoxedSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        let _name = name;
        self.0.serializer(None, is_root)
    }
}
#[derive(Debug)]
pub struct IntermediateType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
}
impl BaseTrait for IntermediateType {}
impl IntermediateTrait for IntermediateType {}
impl xsd_parser::quick_xml::WithSerializer for IntermediateType {
    type Serializer<'x> = quick_xml_serialize::IntermediateTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        Ok(quick_xml_serialize::IntermediateTypeSerializer {
            value: self,
            state: quick_xml_serialize::IntermediateTypeSerializerState::Init__,
            name: name.unwrap_or("tns:intermediate"),
            is_root,
        })
    }
}
#[derive(Debug)]
pub struct FinalType {
    pub base_value: Option<i32>,
    pub intermediate_value: Option<i32>,
    pub final_value: Option<i32>,
}
impl BaseTrait for FinalType {}
impl IntermediateTrait for FinalType {}
impl xsd_parser::quick_xml::WithSerializer for FinalType {
    type Serializer<'x> = quick_xml_serialize::FinalTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        Ok(quick_xml_serialize::FinalTypeSerializer {
            value: self,
            state: quick_xml_serialize::FinalTypeSerializerState::Init__,
            name: name.unwrap_or("tns:final"),
            is_root,
        })
    }
}
#[derive(Debug)]
pub struct Intermediate(pub Box<dyn IntermediateTrait>);
pub trait IntermediateTrait: BaseTrait {}
impl xsd_parser::quick_xml::WithSerializer for Intermediate {
    type Serializer<'x> = xsd_parser::quick_xml::BoxedSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        let _name = name;
        self.0.serializer(None, is_root)
    }
}
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct ListTypeSerializer<'ser> {
        pub(super) value: &'ser super::ListType,
        pub(super) state: ListTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ListTypeSerializerState<'ser> {
        Init__,
        Base(xsd_parser::quick_xml::IterSerializer<'ser, Vec<Base>, Base>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ListTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    ListTypeSerializerState::Init__ => {
                        self.state = ListTypeSerializerState::Base(
                            xsd_parser::quick_xml::IterSerializer::new(
                                &self.value.base,
                                Some("tns:base"),
                                false,
                            ),
                        );
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    ListTypeSerializerState::Base(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = ListTypeSerializerState::End__,
                    },
                    ListTypeSerializerState::End__ => {
                        self.state = ListTypeSerializerState::Done__;
                        return Ok(Some(xsd_parser::quick_xml::Event::End(
                            xsd_parser::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    ListTypeSerializerState::Done__ => return Ok(None),
                    ListTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for ListTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = ListTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct IntermediateTypeSerializer<'ser> {
        pub(super) value: &'ser super::IntermediateType,
        pub(super) state: IntermediateTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum IntermediateTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> IntermediateTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    IntermediateTypeSerializerState::Init__ => {
                        self.state = IntermediateTypeSerializerState::Done__;
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        xsd_parser::quick_xml::write_attrib_opt(
                            &mut bytes,
                            "tns:baseValue",
                            &self.value.base_value,
                        )?;
                        xsd_parser::quick_xml::write_attrib_opt(
                            &mut bytes,
                            "tns:intermediateValue",
                            &self.value.intermediate_value,
                        )?;
                        return Ok(Some(xsd_parser::quick_xml::Event::Empty(bytes)));
                    }
                    IntermediateTypeSerializerState::Done__ => return Ok(None),
                    IntermediateTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for IntermediateTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = IntermediateTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FinalTypeSerializer<'ser> {
        pub(super) value: &'ser super::FinalType,
        pub(super) state: FinalTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FinalTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FinalTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    FinalTypeSerializerState::Init__ => {
                        self.state = FinalTypeSerializerState::Done__;
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute(("xmlns:tns", "http://example.com"));
                        }
                        xsd_parser::quick_xml::write_attrib_opt(
                            &mut bytes,
                            "tns:baseValue",
                            &self.value.base_value,
                        )?;
                        xsd_parser::quick_xml::write_attrib_opt(
                            &mut bytes,
                            "tns:intermediateValue",
                            &self.value.intermediate_value,
                        )?;
                        xsd_parser::quick_xml::write_attrib_opt(
                            &mut bytes,
                            "tns:finalValue",
                            &self.value.final_value,
                        )?;
                        return Ok(Some(xsd_parser::quick_xml::Event::Empty(bytes)));
                    }
                    FinalTypeSerializerState::Done__ => return Ok(None),
                    FinalTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for FinalTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = FinalTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
