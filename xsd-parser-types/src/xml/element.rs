use std::borrow::Cow;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::mem::replace;
use std::slice::Iter;
use std::str::from_utf8;

#[cfg(feature = "quick-xml")]
use quick_xml::{
    events::{attributes::Attribute, BytesEnd, BytesStart},
    name::QName,
};

use crate::misc::format_utf8_slice;

#[cfg(feature = "quick-xml")]
use crate::quick_xml::{
    DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent, DeserializerOutput,
    DeserializerResult, Error, Event, WithDeserializer, WithSerializer,
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
    #[cfg(feature = "quick-xml")]
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

#[cfg(feature = "quick-xml")]
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
        let _is_root = is_root;

        Ok(ElementSerializer::new(self, name))
    }
}

#[cfg(feature = "quick-xml")]
impl WithDeserializer for Element<'static> {
    type Deserializer = ElementDeserializer;
}

#[derive(Debug)]
#[cfg(feature = "quick-xml")]
pub enum ElementSerializer<'ser, 'el> {
    Start {
        name: Option<&'ser str>,
        element: &'ser Element<'el>,
    },
    End {
        name: Option<&'ser str>,
        element: &'ser Element<'el>,
    },
    NextValue {
        name: Option<&'ser str>,
        element: &'ser Element<'el>,
        values: Iter<'ser, Value<'el>>,
    },
    SubElement {
        name: Option<&'ser str>,
        element: &'ser Element<'el>,
        values: Iter<'ser, Value<'el>>,
        serializer: Box<ElementSerializer<'ser, 'el>>,
    },
    Done,
}

#[cfg(feature = "quick-xml")]
impl<'ser, 'el> ElementSerializer<'ser, 'el> {
    fn new(element: &'ser Element<'el>, name: Option<&'ser str>) -> Self {
        Self::Start { name, element }
    }

    fn next_item(&mut self) -> Result<Option<Event<'ser>>, Error> {
        loop {
            match replace(self, Self::Done) {
                Self::Start { name, element } => {
                    let element_name = name.map_or_else(|| from_utf8(&element.name), Ok)?;
                    let attributes = element.attributes.iter().map(|(k, v)| Attribute {
                        key: QName(k),
                        value: Cow::Borrowed(&v.0),
                    });

                    let mut start = BytesStart::new(element_name);
                    start.extend_attributes(attributes);

                    let event = if element.values.is_empty() {
                        Event::Empty(start)
                    } else {
                        let values = element.values.iter();

                        *self = Self::NextValue {
                            name,
                            element,
                            values,
                        };

                        Event::Start(start)
                    };

                    return Ok(Some(event));
                }
                Self::End { name, element } => {
                    let element_name = name.map_or_else(|| from_utf8(&element.name), Ok)?;
                    let end = BytesEnd::new(element_name);
                    let event = Event::End(end);

                    return Ok(Some(event));
                }
                Self::NextValue {
                    name,
                    element,
                    mut values,
                } => match values.next() {
                    None => *self = Self::End { name, element },
                    Some(Value::Element(sub)) => {
                        let serializer = Box::new(Self::new(sub, None));

                        *self = Self::SubElement {
                            name,
                            element,
                            values,
                            serializer,
                        };
                    }
                    Some(Value::Comment(comment)) => {
                        *self = Self::NextValue {
                            name,
                            element,
                            values,
                        };

                        return Ok(Some(Event::Comment(comment.borrow())));
                    }
                    Some(Value::CData(cdata)) => {
                        *self = Self::NextValue {
                            name,
                            element,
                            values,
                        };

                        return Ok(Some(Event::CData(cdata.borrow())));
                    }
                    Some(Value::Text(text)) => {
                        *self = Self::NextValue {
                            name,
                            element,
                            values,
                        };

                        return Ok(Some(Event::Text(text.borrow())));
                    }
                },
                Self::SubElement {
                    name,
                    element,
                    values,
                    mut serializer,
                } => match serializer.next() {
                    None => {
                        *self = Self::NextValue {
                            name,
                            element,
                            values,
                        }
                    }
                    Some(event) => {
                        *self = Self::SubElement {
                            name,
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

#[cfg(feature = "quick-xml")]
impl<'ser> Iterator for ElementSerializer<'ser, '_> {
    type Item = Result<Event<'ser>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().transpose()
    }
}

#[derive(Debug)]
#[cfg(feature = "quick-xml")]
pub struct ElementDeserializer {
    element: Element<'static>,
    sub: Box<Option<ElementDeserializer>>,
}

#[cfg(feature = "quick-xml")]
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

#[cfg(feature = "quick-xml")]
impl<'de> Deserializer<'de, Element<'static>> for ElementDeserializer {
    fn init(
        helper: &mut DeserializeHelper,
        event: Event<'de>,
    ) -> DeserializerResult<'de, Element<'static>> {
        match event {
            Event::Start(start) => {
                let namespaces = helper.namespaces();
                let element = Self::create_element(&start, namespaces)?;
                let deserializer = Self::new(element);

                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(deserializer),
                    event: DeserializerEvent::None,
                    allow_any: true,
                })
            }
            Event::Empty(start) => {
                let namespaces = helper.namespaces();
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

    fn next(
        mut self,
        helper: &mut DeserializeHelper,
        event: Event<'de>,
    ) -> DeserializerResult<'de, Element<'static>> {
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
            let output = sub.next(helper, event)?;

            handle_output!(output)
        } else {
            Some(event)
        };

        let event = match event {
            None => None,
            Some(event @ (Event::Start(_) | Event::Empty(_))) => {
                let output = Self::init(helper, event)?;

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

    #[allow(clippy::only_used_in_recursion)]
    fn finish(mut self, helper: &mut DeserializeHelper) -> Result<Element<'static>, Error> {
        if let Some(sub) = self.sub.take() {
            let element = sub.finish(helper)?;
            let value = Value::Element(element);

            self.element.values.push(value);
        }

        Ok(self.element)
    }
}

#[cfg(all(test, feature = "quick-xml"))]
mod tests {
    use std::str::from_utf8;
    use std::sync::Arc;

    use quick_xml::{events::BytesText, Writer};

    use crate::quick_xml::{DeserializeSync, SerializeSync, SliceReader};
    use crate::xml::Value;

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
        root.name = b"root".into();

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
        root.serialize("names", &mut writer).unwrap();

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

    const XML: &str = r#"
<names>
    <name xmlns:ns="test" ns:first="bob" last="jones"/>
    <name first="elizabeth" last="smith"/>
</names>
"#;
}
