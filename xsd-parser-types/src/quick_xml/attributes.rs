use quick_xml::events::BytesStart;

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
