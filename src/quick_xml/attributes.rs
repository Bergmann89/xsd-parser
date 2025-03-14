use quick_xml::events::{
    attributes::{AttrError, Attribute},
    BytesStart,
};

use super::{Error, SerializeBytes};

/// Write the passed `attrib` to the passed `bytes` object.
///
/// # Errors
/// An error is returned of the attribute could not be serialized.
pub fn write_attrib<T>(bytes: &mut BytesStart<'_>, name: &str, attrib: &T) -> Result<(), Error>
where
    T: SerializeBytes,
{
    if let Some(attrib) = SerializeBytes::serialize_bytes(attrib)? {
        bytes.push_attribute((name, attrib));
    }

    Ok(())
}

/// Write the passed `attrib`  to the passed `bytes` object.
///
/// # Errors
/// An error is returned of the attribute could not be serialized.
pub fn write_attrib_opt<T>(
    bytes: &mut BytesStart<'_>,
    name: &str,
    attrib: &Option<T>,
) -> Result<(), Error>
where
    T: SerializeBytes,
{
    let Some(attrib) = attrib else {
        return Ok(());
    };

    write_attrib(bytes, name, attrib)
}

/// Returns an iterator that yields all attributes of the passed `bytes_start`
/// object, except the `xmlns` attributes.
pub fn filter_xmlns_attributes<'a>(
    bytes_start: &'a BytesStart<'_>,
) -> impl Iterator<Item = Result<Attribute<'a>, AttrError>> {
    bytes_start.attributes().filter(|attrib| {
        let Ok(attrib) = attrib else {
            return true;
        };

        attrib.key.0 != b"xmlns" && !attrib.key.0.starts_with(b"xmlns:")
    })
}
