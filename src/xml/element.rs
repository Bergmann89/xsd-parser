use std::borrow::Cow;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::mem::replace;
use std::slice::Iter;
use std::str::from_utf8;

use quick_xml::events::BytesEnd;
use quick_xml::{
    events::{attributes::Attribute, BytesStart},
    name::QName,
};

use crate::misc::format_utf8_slice;
use crate::quick_xml::{
    Deserializer, DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
    Error, Event, WithDeserializer, WithSerializer, XmlReader,
};

use super::{
    attributes::{Key as AttribKey, Value as AttribValue},
    Attributes, NamespacesShared, Value,
};

/// Represents a unstructured XML element.
#[derive(Default, Clone, Eq, PartialEq)]
pub struct Element<'a> {
    /// Name of the element.
    pub name: Cow<'a, [u8]>,

    /// Child values of this element.
    pub values: Vec<Value<'a>>,

    /// Attributes of this element.
    pub attributes: Attributes<'a>,

    /// List of valid namespaces for this element.
    pub namespaces: NamespacesShared<'a>,
}

/// Represents a list of unstructured XML elements.
pub type Elements<'a> = Vec<Element<'a>>;

/// Helper type for an element with static lifetime
pub type AnyElement = Element<'static>;

/// Helper type for elements with static lifetime
pub type AnyElements = Elements<'static>;

impl<'a> Element<'a> {
    /// Create a new [`Element`] instance.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the name of the element as [`QName`].
    #[must_use]
    pub fn qname(&self) -> QName<'_> {
        QName(&self.name)
    }

    /// Set the name of the element.
    #[must_use]
    pub fn name<N>(mut self, name: N) -> Self
    where
        N: Into<Cow<'a, [u8]>>,
    {
        self.name = name.into();

        self
    }

    /// Add an attribute to the element.
    #[must_use]
    pub fn attribute<K, V>(mut self, name: K, value: V) -> Self
    where
        K: Into<Cow<'a, [u8]>>,
        V: Into<Cow<'a, [u8]>>,
    {
        self.attributes.insert(name, value);

        self
    }

    /// Add a child value to the element.
    #[must_use]
    pub fn child(mut self, value: Value<'a>) -> Self {
        self.values.push(value);

        self
    }
}

impl Debug for Element<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        struct Name<'a>(&'a [u8]);

        impl Debug for Name<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                write!(f, "\"")?;
                format_utf8_slice(self.0, f)?;
                write!(f, "\"")?;

                Ok(())
            }
        }

        f.debug_struct("Element")
            .field("name", &Name(&self.name))
            .field("values", &self.values)
            .field("attributes", &self.attributes)
            .field("namespaces", &self.namespaces)
            .finish()
    }
}

impl<'el> WithSerializer for Element<'el> {
    type Serializer<'x>
        = ElementSerializer<'x, 'el>
    where
        'el: 'x;

    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;

        Ok(ElementSerializer::new(self))
    }
}

impl WithDeserializer for Element<'static> {
    type Deserializer = ElementDeserializer;
}

#[derive(Debug)]
pub enum ElementSerializer<'ser, 'el> {
    Start {
        element: &'ser Element<'el>,
    },
    End {
        element: &'ser Element<'el>,
    },
    NextValue {
        element: &'ser Element<'el>,
        values: Iter<'ser, Value<'el>>,
    },
    SubElement {
        element: &'ser Element<'el>,
        values: Iter<'ser, Value<'el>>,
        serializer: Box<ElementSerializer<'ser, 'el>>,
    },
    Done,
}

impl<'ser, 'el> ElementSerializer<'ser, 'el> {
    fn new(element: &'ser Element<'el>) -> Self {
        Self::Start { element }
    }

    fn next_item(&mut self) -> Result<Option<Event<'ser>>, Error> {
        loop {
            match replace(self, Self::Done) {
                Self::Start { element } => {
                    let name = from_utf8(&element.name)?;
                    let attributes = element.attributes.iter().map(|(k, v)| Attribute {
                        key: QName(k),
                        value: Cow::Borrowed(&v.0),
                    });

                    let mut start = BytesStart::new(name);
                    start.extend_attributes(attributes);

                    let event = if element.values.is_empty() {
                        Event::Empty(start)
                    } else {
                        let values = element.values.iter();

                        *self = Self::NextValue { element, values };

                        Event::Start(start)
                    };

                    return Ok(Some(event));
                }
                Self::End { element } => {
                    let name = from_utf8(&element.name)?;
                    let end = BytesEnd::new(name);
                    let event = Event::End(end);

                    return Ok(Some(event));
                }
                Self::NextValue {
                    element,
                    mut values,
                } => match values.next() {
                    None => *self = Self::End { element },
                    Some(Value::Element(sub)) => {
                        let serializer = Box::new(Self::new(sub));

                        *self = Self::SubElement {
                            element,
                            values,
                            serializer,
                        };
                    }
                    Some(Value::Comment(comment)) => {
                        *self = Self::NextValue { element, values };

                        return Ok(Some(Event::Comment(comment.borrow())));
                    }
                    Some(Value::CData(cdata)) => {
                        *self = Self::NextValue { element, values };

                        return Ok(Some(Event::CData(cdata.borrow())));
                    }
                    Some(Value::Text(text)) => {
                        *self = Self::NextValue { element, values };

                        return Ok(Some(Event::Text(text.borrow())));
                    }
                },
                Self::SubElement {
                    element,
                    values,
                    mut serializer,
                } => match serializer.next() {
                    None => *self = Self::NextValue { element, values },
                    Some(event) => {
                        *self = Self::SubElement {
                            element,
                            values,
                            serializer,
                        };

                        return event.map(Some);
                    }
                },
                Self::Done => return Ok(None),
            }
        }
    }
}

impl<'ser> Iterator for ElementSerializer<'ser, '_> {
    type Item = Result<Event<'ser>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().transpose()
    }
}

#[derive(Debug)]
pub struct ElementDeserializer {
    element: Element<'static>,
    sub: Box<Option<ElementDeserializer>>,
}

impl ElementDeserializer {
    fn new(element: Element<'static>) -> Self {
        Self {
            element,
            sub: Box::new(None),
        }
    }

    fn create_element(
        start: &BytesStart<'_>,
        namespaces: NamespacesShared<'static>,
    ) -> Result<Element<'static>, Error> {
        let name = Cow::Owned(start.name().0.to_owned());
        let attributes = start
            .attributes()
            .map(|item| match item {
                Ok(Attribute { key, value }) => {
                    let key = Cow::Owned(key.0.to_owned());
                    let value = Cow::Owned(value.into_owned());

                    Ok((AttribKey(key), AttribValue(value)))
                }
                Err(error) => Err(error),
            })
            .collect::<Result<Attributes<'static>, _>>()?;

        Ok(Element {
            name,
            attributes,
            namespaces,
            values: Vec::new(),
        })
    }
}

impl<'de> Deserializer<'de, Element<'static>> for ElementDeserializer {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, Element<'static>>
    where
        R: XmlReader,
    {
        match event {
            Event::Start(start) => {
                let namespaces = reader.namespaces();
                let element = Self::create_element(&start, namespaces)?;
                let deserializer = Self::new(element);

                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(deserializer),
                    event: DeserializerEvent::None,
                    allow_any: true,
                })
            }
            Event::Empty(start) => {
                let namespaces = reader.namespaces();
                let element = Self::create_element(&start, namespaces)?;

                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(element),
                    event: DeserializerEvent::None,
                    allow_any: true,
                })
            }
            event => Ok(DeserializerOutput {
                artifact: DeserializerArtifact::None,
                event: DeserializerEvent::Continue(event),
                allow_any: true,
            }),
        }
    }

    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, Element<'static>>
    where
        R: XmlReader,
    {
        macro_rules! handle_output {
            ($output:expr) => {{
                let output = $output;

                match output.artifact {
                    DeserializerArtifact::None => (),
                    DeserializerArtifact::Data(element) => {
                        let value = Value::Element(element);

                        self.element.values.push(value);
                    }
                    DeserializerArtifact::Deserializer(sub) => {
                        *self.sub = Some(sub);
                    }
                }

                match output.event {
                    DeserializerEvent::None => None,
                    DeserializerEvent::Break(event) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Deserializer(self),
                            event: DeserializerEvent::Break(event),
                            allow_any: true,
                        })
                    }
                    DeserializerEvent::Continue(event) => Some(event),
                }
            }};
        }

        let event = if let Some(sub) = self.sub.take() {
            let output = sub.next(reader, event)?;

            handle_output!(output)
        } else {
            Some(event)
        };

        let event = match event {
            None => None,
            Some(event @ (Event::Start(_) | Event::Empty(_))) => {
                let output = Self::init(reader, event)?;

                handle_output!(output)
            }
            Some(Event::End(_)) => {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.element),
                    event: DeserializerEvent::None,
                    allow_any: true,
                })
            }
            Some(Event::Text(text)) => {
                let value = Value::Text(text.into_owned());

                self.element.values.push(value);

                None
            }
            Some(Event::CData(cdata)) => {
                let value = Value::CData(cdata.into_owned());

                self.element.values.push(value);

                None
            }
            Some(Event::Comment(comment)) => {
                let value = Value::Comment(comment.into_owned());

                self.element.values.push(value);

                None
            }
            event => event,
        };

        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event: event.map_or(DeserializerEvent::None, DeserializerEvent::Break),
            allow_any: true,
        })
    }

    fn finish<R>(mut self, reader: &R) -> Result<Element<'static>, Error>
    where
        R: XmlReader,
    {
        if let Some(sub) = self.sub.take() {
            let element = sub.finish(reader)?;
            let value = Value::Element(element);

            self.element.values.push(value);
        }

        Ok(self.element)
    }
}

#[cfg(test)]
mod tests {
    use std::{str::from_utf8, sync::Arc};

    use quick_xml::{events::BytesText, Writer};

    use crate::{
        quick_xml::{DeserializeSync, SerializeSync, SliceReader},
        xml::Value,
    };

    use super::Element;

    macro_rules! assert_entry {
        ($map:expr, $key:expr, $val:expr) => {
            assert_eq!($map.get(&$key[..]).unwrap().as_ref(), $val);
        };
    }

    macro_rules! assert_element {
        ($value:expr) => {
            if let Value::Element(element) = $value {
                element
            } else {
                panic!("Unexpected value")
            }
        };
    }

    macro_rules! assert_text {
        ($value:expr, $text:expr) => {
            assert!(matches!($value, Value::Text(x) if &**x == $text));
        };
    }

    #[test]
    fn serialize() {
        let mut root = Element::new();
        root.name = b"names".into();

        root.values.push(Value::Text(BytesText::new("\n    ")));

        let mut element = Element::new();
        element.name = b"name".into();
        element.attributes.insert(b"xmlns:ns", b"test");
        element.attributes.insert(b"ns:first", b"bob");
        element.attributes.insert(b"last", b"jones");
        root.values.push(Value::Element(element));

        root.values.push(Value::Text(BytesText::new("\n    ")));

        let mut element = Element::new();
        element.name = b"name".into();
        element.attributes.insert(b"first", b"elizabeth");
        element.attributes.insert(b"last", b"smith");
        root.values.push(Value::Element(element));

        root.values.push(Value::Text(BytesText::new("\n")));

        let mut buffer = Vec::new();
        let mut writer = Writer::new(&mut buffer);
        root.serialize("", &mut writer).unwrap();

        let xml = from_utf8(&buffer).unwrap();
        assert_eq!(xml, XML.trim());
    }

    #[test]
    fn deserialize() {
        let mut reader = SliceReader::new(XML.trim());
        let root = Element::deserialize(&mut reader).unwrap();

        dbg!(&root);

        assert_eq!(root.name.as_ref(), b"names");
        assert_eq!(root.values.len(), 5);
        assert!(root.attributes.is_empty());
        assert!(root.namespaces.is_empty());

        let mut iter = root.values.iter();

        let value = iter.next().unwrap();
        assert_text!(value, b"\n    ");

        let value = iter.next().unwrap();
        let element = assert_element!(value);
        assert_eq!(element.name.as_ref(), b"name");
        assert!(element.values.is_empty());
        assert_eq!(element.attributes.len(), 3);
        assert_entry!(element.attributes, b"xmlns:ns", b"test");
        assert_entry!(element.attributes, b"ns:first", b"bob");
        assert_entry!(element.attributes, b"last", b"jones");
        assert_eq!(element.namespaces.len(), 1);
        assert_entry!(element.namespaces, b"ns", b"test");

        let value = iter.next().unwrap();
        assert_text!(value, b"\n    ");

        let value = iter.next().unwrap();
        let element = assert_element!(value);
        assert_eq!(element.name.as_ref(), b"name");
        assert!(element.values.is_empty());
        assert_eq!(element.attributes.len(), 2);
        assert_entry!(element.attributes, b"first", b"elizabeth");
        assert_entry!(element.attributes, b"last", b"smith");
        assert!(element.namespaces.is_empty());
        assert!(Arc::ptr_eq(&root.namespaces.0, &element.namespaces.0));

        let value = iter.next().unwrap();
        assert_text!(value, b"\n");
    }

    const XML: &str = r##"
<names>
    <name xmlns:ns="test" ns:first="bob" last="jones"/>
    <name first="elizabeth" last="smith"/>
</names>
"##;
}
