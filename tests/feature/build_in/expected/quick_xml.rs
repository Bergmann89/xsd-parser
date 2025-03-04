#[derive(Debug, Clone, Default)]
pub struct Entitiestype(pub Vec<String>);
impl xsd_parser::quick_xml::SerializeBytes for Entitiestype {
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
#[derive(Debug, Clone, Default)]
pub struct Entitytype(pub Vec<String>);
impl xsd_parser::quick_xml::SerializeBytes for Entitytype {
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
pub type Idtype = String;
pub type Idreftype = String;
#[derive(Debug, Clone, Default)]
pub struct Idrefstype(pub Vec<String>);
impl xsd_parser::quick_xml::SerializeBytes for Idrefstype {
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
pub type NcnameType = String;
pub type Nmtokentype = String;
#[derive(Debug, Clone, Default)]
pub struct Nmtokenstype(pub Vec<String>);
impl xsd_parser::quick_xml::SerializeBytes for Nmtokenstype {
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
pub type Notationtype = String;
pub type NameType = String;
pub type QnameType = String;
#[derive(Debug, Clone)]
pub struct AnyType {}
impl xsd_parser::quick_xml::WithSerializer for AnyType {
    type Serializer<'x> = quick_xml_serialize::AnyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        Ok(quick_xml_serialize::AnyTypeSerializer {
            name: name.unwrap_or("xs:anyType"),
            value: self,
            is_root,
            state: quick_xml_serialize::AnyTypeSerializerState::Init__,
        })
    }
}
pub type AnyURIType = String;
pub type Base64BinaryType = String;
pub type BooleanType = bool;
pub type ByteType = i8;
pub type DateType = String;
pub type DateTimeType = String;
pub type DecimalType = f64;
pub type DoubleType = f64;
pub type DurationType = String;
pub type FloatType = f32;
pub type GdayType = String;
pub type GmonthType = String;
pub type GmonthDayType = String;
pub type GyearType = String;
pub type GyearMonthType = String;
pub type HexBinaryType = String;
pub type IntType = i32;
pub type IntegerType = i32;
pub type LanguageType = String;
pub type LongType = i64;
pub type NegativeIntegerType = isize;
pub type NonNegativeIntegerType = usize;
pub type NonPositiveIntegerType = isize;
pub type NormalizedStringType = String;
pub type PositiveIntegerType = usize;
pub type ShortType = i16;
pub type StringType = String;
pub type TimeType = String;
pub type TokenType = String;
pub type UnsignedByteType = u8;
pub type UnsignedIntType = u32;
pub type UnsignedLongType = u64;
pub type UnsignedShortType = u16;
pub mod quick_xml_serialize {
    use super::*;
    #[derive(Debug)]
    pub struct AnyTypeSerializer<'ser> {
        pub(super) name: &'ser str,
        pub(super) value: &'ser super::AnyType,
        pub(super) is_root: bool,
        pub(super) state: AnyTypeSerializerState<'ser>,
    }
    #[derive(Debug)]
    pub(super) enum AnyTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnyTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    AnyTypeSerializerState::Init__ => {
                        self.state = AnyTypeSerializerState::Done__;
                        let bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        return Ok(Some(xsd_parser::quick_xml::Event::Empty(bytes)));
                    }
                    AnyTypeSerializerState::Done__ => return Ok(None),
                    AnyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for AnyTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = AnyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
