use crate::quick_xml::{DeserializeBytes, DeserializeReader, Error, ErrorKind, RawByteStr};
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Space {
    Default,
    Preserve,
}
impl DeserializeBytes for Space {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"default" => Ok(Self::Default),
            b"preserve" => Ok(Self::Preserve),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
