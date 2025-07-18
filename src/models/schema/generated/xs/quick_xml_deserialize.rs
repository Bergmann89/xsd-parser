use crate::{
    models::schema::{MaxOccurs, QName},
    quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    },
    xml::AnyElement,
};
use core::mem::replace;
#[derive(Debug)]
pub struct SchemaDeserializer {
    target_namespace: Option<String>,
    version: Option<String>,
    final_default: super::FullDerivationSetType,
    block_default: super::BlockSetType,
    attribute_form_default: super::FormChoiceType,
    element_form_default: super::FormChoiceType,
    default_attributes: Option<QName>,
    xpath_default_namespace: super::XpathDefaultNamespaceType,
    id: Option<String>,
    lang: Option<String>,
    content: Vec<super::SchemaContent>,
    state: Box<SchemaDeserializerState>,
}
#[derive(Debug)]
enum SchemaDeserializerState {
    Init__,
    Next__,
    Content__(<super::SchemaContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl SchemaDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut target_namespace: Option<String> = None;
        let mut version: Option<String> = None;
        let mut final_default: Option<super::FullDerivationSetType> = None;
        let mut block_default: Option<super::BlockSetType> = None;
        let mut attribute_form_default: Option<super::FormChoiceType> = None;
        let mut element_form_default: Option<super::FormChoiceType> = None;
        let mut default_attributes: Option<QName> = None;
        let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
        let mut id: Option<String> = None;
        let mut lang: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"targetNamespace")
            ) {
                reader.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"version")
            ) {
                reader.read_attrib(&mut version, b"version", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"finalDefault")
            ) {
                reader.read_attrib(&mut final_default, b"finalDefault", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"blockDefault")
            ) {
                reader.read_attrib(&mut block_default, b"blockDefault", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"attributeFormDefault")
            ) {
                reader.read_attrib(
                    &mut attribute_form_default,
                    b"attributeFormDefault",
                    &attrib.value,
                )?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"elementFormDefault")
            ) {
                reader.read_attrib(
                    &mut element_form_default,
                    b"elementFormDefault",
                    &attrib.value,
                )?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"defaultAttributes")
            ) {
                reader.read_attrib(&mut default_attributes, b"defaultAttributes", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"xpathDefaultNamespace")
            ) {
                reader.read_attrib(
                    &mut xpath_default_namespace,
                    b"xpathDefaultNamespace",
                    &attrib.value,
                )?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"lang")
            ) {
                reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            target_namespace: target_namespace,
            version: version,
            final_default: final_default.unwrap_or_else(super::Schema::default_final_default),
            block_default: block_default.unwrap_or_else(super::Schema::default_block_default),
            attribute_form_default: attribute_form_default
                .unwrap_or_else(super::Schema::default_attribute_form_default),
            element_form_default: element_form_default
                .unwrap_or_else(super::Schema::default_element_form_default),
            default_attributes: default_attributes,
            xpath_default_namespace: xpath_default_namespace
                .unwrap_or_else(super::Schema::default_xpath_default_namespace),
            id: id,
            lang: lang,
            content: Vec::new(),
            state: Box::new(SchemaDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: SchemaDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let SchemaDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::SchemaContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::SchemaContent>,
        fallback: &mut Option<SchemaDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback.take().unwrap_or(SchemaDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = SchemaDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = SchemaDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(SchemaDeserializerState::Content__(deserializer));
                        *self.state = SchemaDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Schema> for Box<SchemaDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Schema>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, SchemaDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Schema>
    where
        R: DeserializeReader,
    {
        use SchemaDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output = <super::SchemaContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Schema, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, SchemaDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Schema {
            target_namespace: self.target_namespace,
            version: self.version,
            final_default: self.final_default,
            block_default: self.block_default,
            attribute_form_default: self.attribute_form_default,
            element_form_default: self.element_form_default,
            default_attributes: self.default_attributes,
            xpath_default_namespace: self.xpath_default_namespace,
            id: self.id,
            lang: self.lang,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct SchemaContentDeserializer {
    state: Box<SchemaContentDeserializerState>,
}
#[derive(Debug)]
pub enum SchemaContentDeserializerState {
    Init__,
    Include(
        Option<super::Include>,
        Option<<super::Include as WithDeserializer>::Deserializer>,
    ),
    Import(
        Option<super::Import>,
        Option<<super::Import as WithDeserializer>::Deserializer>,
    ),
    Redefine(
        Option<super::Redefine>,
        Option<<super::Redefine as WithDeserializer>::Deserializer>,
    ),
    Override(
        Option<super::Override>,
        Option<<super::Override as WithDeserializer>::Deserializer>,
    ),
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    DefaultOpenContent(
        Option<super::DefaultOpenContent>,
        Option<<super::DefaultOpenContent as WithDeserializer>::Deserializer>,
    ),
    SimpleType(
        Option<super::SimpleBaseType>,
        Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
    ),
    ComplexType(
        Option<super::ComplexBaseType>,
        Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
    ),
    Group(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    AttributeGroup(
        Option<super::AttributeGroupType>,
        Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
    ),
    Element(
        Option<super::ElementType>,
        Option<<super::ElementType as WithDeserializer>::Deserializer>,
    ),
    Attribute(
        Option<super::AttributeType>,
        Option<<super::AttributeType as WithDeserializer>::Deserializer>,
    ),
    Notation(
        Option<super::Notation>,
        Option<<super::Notation as WithDeserializer>::Deserializer>,
    ),
    Done__(super::SchemaContent),
    Unknown__,
}
impl SchemaContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"include")
            ) {
                let output =
                    <super::Include as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_include(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"import")
            ) {
                let output =
                    <super::Import as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_import(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"redefine")
            ) {
                let output =
                    <super::Redefine as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_redefine(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"override")
            ) {
                let output =
                    <super::Override as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_override_(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"defaultOpenContent")
            ) {
                let output = <super::DefaultOpenContent as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_default_open_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"complexType")
            ) {
                let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_complex_type(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"element")
            ) {
                let output =
                    <super::ElementType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_element(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"notation")
            ) {
                let output =
                    <super::Notation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_notation(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(SchemaContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: SchemaContentDeserializerState,
    ) -> Result<super::SchemaContent, Error>
    where
        R: DeserializeReader,
    {
        use SchemaContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Include(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_include(&mut values, value)?;
                }
                Ok(super::SchemaContent::Include(values.ok_or_else(|| {
                    ErrorKind::MissingElement("include".into())
                })?))
            }
            S::Import(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_import(&mut values, value)?;
                }
                Ok(super::SchemaContent::Import(values.ok_or_else(|| {
                    ErrorKind::MissingElement("import".into())
                })?))
            }
            S::Redefine(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_redefine(&mut values, value)?;
                }
                Ok(super::SchemaContent::Redefine(values.ok_or_else(|| {
                    ErrorKind::MissingElement("redefine".into())
                })?))
            }
            S::Override(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_override_(&mut values, value)?;
                }
                Ok(super::SchemaContent::Override(values.ok_or_else(|| {
                    ErrorKind::MissingElement("override".into())
                })?))
            }
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::SchemaContent::Annotation(values.ok_or_else(
                    || ErrorKind::MissingElement("annotation".into()),
                )?))
            }
            S::DefaultOpenContent(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_default_open_content(&mut values, value)?;
                }
                Ok(super::SchemaContent::DefaultOpenContent(
                    values.ok_or_else(|| ErrorKind::MissingElement("defaultOpenContent".into()))?,
                ))
            }
            S::SimpleType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_simple_type(&mut values, value)?;
                }
                Ok(super::SchemaContent::SimpleType(values.ok_or_else(
                    || ErrorKind::MissingElement("simpleType".into()),
                )?))
            }
            S::ComplexType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_complex_type(&mut values, value)?;
                }
                Ok(super::SchemaContent::ComplexType(values.ok_or_else(
                    || ErrorKind::MissingElement("complexType".into()),
                )?))
            }
            S::Group(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_group(&mut values, value)?;
                }
                Ok(super::SchemaContent::Group(values.ok_or_else(|| {
                    ErrorKind::MissingElement("group".into())
                })?))
            }
            S::AttributeGroup(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_attribute_group(&mut values, value)?;
                }
                Ok(super::SchemaContent::AttributeGroup(values.ok_or_else(
                    || ErrorKind::MissingElement("attributeGroup".into()),
                )?))
            }
            S::Element(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_element(&mut values, value)?;
                }
                Ok(super::SchemaContent::Element(values.ok_or_else(|| {
                    ErrorKind::MissingElement("element".into())
                })?))
            }
            S::Attribute(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_attribute(&mut values, value)?;
                }
                Ok(super::SchemaContent::Attribute(values.ok_or_else(
                    || ErrorKind::MissingElement("attribute".into()),
                )?))
            }
            S::Notation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_notation(&mut values, value)?;
                }
                Ok(super::SchemaContent::Notation(values.ok_or_else(|| {
                    ErrorKind::MissingElement("notation".into())
                })?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_include(
        values: &mut Option<super::Include>,
        value: super::Include,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"include",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_import(values: &mut Option<super::Import>, value: super::Import) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"import",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_redefine(
        values: &mut Option<super::Redefine>,
        value: super::Redefine,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"redefine",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_override_(
        values: &mut Option<super::Override>,
        value: super::Override,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"override",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_default_open_content(
        values: &mut Option<super::DefaultOpenContent>,
        value: super::DefaultOpenContent,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"defaultOpenContent",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_simple_type(
        values: &mut Option<super::SimpleBaseType>,
        value: super::SimpleBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"simpleType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_complex_type(
        values: &mut Option<super::ComplexBaseType>,
        value: super::ComplexBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"complexType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_group(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"group",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute_group(
        values: &mut Option<super::AttributeGroupType>,
        value: super::AttributeGroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attributeGroup",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_element(
        values: &mut Option<super::ElementType>,
        value: super::ElementType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"element",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute(
        values: &mut Option<super::AttributeType>,
        value: super::AttributeType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attribute",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_notation(
        values: &mut Option<super::Notation>,
        value: super::Notation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"notation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_include<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Include>,
        output: DeserializerOutput<'de, super::Include>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::Include(values, None),
                Some(SchemaContentDeserializerState::Include(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Include(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Include(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_include(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_include(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Include(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Include(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_import<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Import>,
        output: DeserializerOutput<'de, super::Import>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::Import(values, None),
                Some(SchemaContentDeserializerState::Import(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Import(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Import(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_import(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_import(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Import(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Import(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_redefine<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Redefine>,
        output: DeserializerOutput<'de, super::Redefine>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::Redefine(values, None),
                Some(SchemaContentDeserializerState::Redefine(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Redefine(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Redefine(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_redefine(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_redefine(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Redefine(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Redefine(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_override_<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Override>,
        output: DeserializerOutput<'de, super::Override>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::Override(values, None),
                Some(SchemaContentDeserializerState::Override(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Override(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Override(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_override_(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_override_(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Override(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Override(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::Annotation(values, None),
                Some(SchemaContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_annotation(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SchemaContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_default_open_content<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::DefaultOpenContent>,
        output: DeserializerOutput<'de, super::DefaultOpenContent>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::DefaultOpenContent(values, None),
                Some(SchemaContentDeserializerState::DefaultOpenContent(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::DefaultOpenContent(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::DefaultOpenContent(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_default_open_content(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_default_open_content(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::DefaultOpenContent(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SchemaContentDeserializerState::DefaultOpenContent(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_simple_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::SimpleBaseType>,
        output: DeserializerOutput<'de, super::SimpleBaseType>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::SimpleType(values, None),
                Some(SchemaContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::SimpleType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_simple_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_simple_type(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::SimpleType(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SchemaContentDeserializerState::SimpleType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_complex_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ComplexBaseType>,
        output: DeserializerOutput<'de, super::ComplexBaseType>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::ComplexType(values, None),
                Some(SchemaContentDeserializerState::ComplexType(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::ComplexType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::ComplexType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_complex_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_complex_type(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::ComplexType(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SchemaContentDeserializerState::ComplexType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::Group(values, None),
                Some(SchemaContentDeserializerState::Group(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Group(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Group(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_group(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Group(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Group(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeGroupType>,
        output: DeserializerOutput<'de, super::AttributeGroupType>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::AttributeGroup(values, None),
                Some(SchemaContentDeserializerState::AttributeGroup(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::AttributeGroup(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::AttributeGroup(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_attribute_group(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::AttributeGroup(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SchemaContentDeserializerState::AttributeGroup(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_element<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ElementType>,
        output: DeserializerOutput<'de, super::ElementType>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::Element(values, None),
                Some(SchemaContentDeserializerState::Element(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Element(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Element(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_element(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_element(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Element(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Element(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeType>,
        output: DeserializerOutput<'de, super::AttributeType>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::Attribute(values, None),
                Some(SchemaContentDeserializerState::Attribute(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Attribute(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Attribute(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_attribute(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_attribute(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Attribute(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Attribute(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_notation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Notation>,
        output: DeserializerOutput<'de, super::Notation>,
        fallback: &mut Option<SchemaContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SchemaContentDeserializerState::Notation(values, None),
                Some(SchemaContentDeserializerState::Notation(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Notation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Notation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_notation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_notation(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Notation(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Notation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::SchemaContent> for Box<SchemaContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SchemaContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(SchemaContentDeserializer {
            state: Box::new(SchemaContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, SchemaContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::SchemaContent>
    where
        R: DeserializeReader,
    {
        use SchemaContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Include(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_include(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Import(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_import(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Redefine(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_redefine(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Override(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_override_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::DefaultOpenContent(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_default_open_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_complex_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Element(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_element(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Notation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_notation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            SchemaContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Include(values, None), event) => {
                    let output =
                        <super::Include as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_include(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Import(values, None), event) => {
                    let output =
                        <super::Import as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_import(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Redefine(values, None), event) => {
                    let output =
                        <super::Redefine as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_redefine(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Override(values, None), event) => {
                    let output =
                        <super::Override as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_override_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::DefaultOpenContent(values, None), event) => {
                    let output =
                        <super::DefaultOpenContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_default_open_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, None), event) => {
                    let output = <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexType(values, None), event) => {
                    let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_complex_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, None), event) => {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Element(values, None), event) => {
                    let output = <super::ElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_element(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, None), event) => {
                    let output = <super::AttributeType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Notation(values, None), event) => {
                    let output =
                        <super::Notation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_notation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::SchemaContent, Error>
    where
        R: DeserializeReader,
    {
        SchemaContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct IncludeDeserializer {
    id: Option<String>,
    schema_location: String,
    annotation: Option<super::Annotation>,
    state: Box<IncludeDeserializerState>,
}
#[derive(Debug)]
enum IncludeDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl IncludeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut schema_location: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"schemaLocation")
            ) {
                reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            schema_location: schema_location.ok_or_else(|| {
                reader.map_error(ErrorKind::MissingAttribute("schemaLocation".into()))
            })?,
            annotation: None,
            state: Box::new(IncludeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: IncludeDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use IncludeDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<IncludeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(IncludeDeserializerState::Annotation(None));
            *self.state = IncludeDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = IncludeDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(IncludeDeserializerState::Annotation(Some(
                            deserializer,
                        )));
                        *self.state = IncludeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = IncludeDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Include> for Box<IncludeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Include>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, IncludeDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Include>
    where
        R: DeserializeReader,
    {
        use IncludeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = IncludeDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Include, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, IncludeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Include {
            id: self.id,
            schema_location: self.schema_location,
            annotation: self.annotation,
        })
    }
}
#[derive(Debug)]
pub struct ImportDeserializer {
    id: Option<String>,
    namespace: Option<String>,
    schema_location: Option<String>,
    annotation: Option<super::Annotation>,
    state: Box<ImportDeserializerState>,
}
#[derive(Debug)]
enum ImportDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl ImportDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut namespace: Option<String> = None;
        let mut schema_location: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"namespace")
            ) {
                reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"schemaLocation")
            ) {
                reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            namespace: namespace,
            schema_location: schema_location,
            annotation: None,
            state: Box::new(ImportDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: ImportDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use ImportDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<ImportDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(ImportDeserializerState::Annotation(None));
            *self.state = ImportDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = ImportDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(ImportDeserializerState::Annotation(Some(deserializer)));
                        *self.state = ImportDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = ImportDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Import> for Box<ImportDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Import>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, ImportDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Import>
    where
        R: DeserializeReader,
    {
        use ImportDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = ImportDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Import, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, ImportDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Import {
            id: self.id,
            namespace: self.namespace,
            schema_location: self.schema_location,
            annotation: self.annotation,
        })
    }
}
#[derive(Debug)]
pub struct RedefineDeserializer {
    schema_location: String,
    id: Option<String>,
    content: Vec<super::RedefineContent>,
    state: Box<RedefineDeserializerState>,
}
#[derive(Debug)]
enum RedefineDeserializerState {
    Init__,
    Next__,
    Content__(<super::RedefineContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl RedefineDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut schema_location: Option<String> = None;
        let mut id: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"schemaLocation")
            ) {
                reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            schema_location: schema_location.ok_or_else(|| {
                reader.map_error(ErrorKind::MissingAttribute("schemaLocation".into()))
            })?,
            id: id,
            content: Vec::new(),
            state: Box::new(RedefineDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: RedefineDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let RedefineDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::RedefineContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::RedefineContent>,
        fallback: &mut Option<RedefineDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback.take().unwrap_or(RedefineDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = RedefineDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = RedefineDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(RedefineDeserializerState::Content__(deserializer));
                        *self.state = RedefineDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Redefine> for Box<RedefineDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Redefine>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, RedefineDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Redefine>
    where
        R: DeserializeReader,
    {
        use RedefineDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output = <super::RedefineContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Redefine, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, RedefineDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Redefine {
            schema_location: self.schema_location,
            id: self.id,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct RedefineContentDeserializer {
    state: Box<RedefineContentDeserializerState>,
}
#[derive(Debug)]
pub enum RedefineContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    SimpleType(
        Option<super::SimpleBaseType>,
        Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
    ),
    ComplexType(
        Option<super::ComplexBaseType>,
        Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
    ),
    Group(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    AttributeGroup(
        Option<super::AttributeGroupType>,
        Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
    ),
    Done__(super::RedefineContent),
    Unknown__,
}
impl RedefineContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<RedefineContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"complexType")
            ) {
                let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_complex_type(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(RedefineContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: RedefineContentDeserializerState,
    ) -> Result<super::RedefineContent, Error>
    where
        R: DeserializeReader,
    {
        use RedefineContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RedefineContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::RedefineContent::Annotation(values.ok_or_else(
                    || ErrorKind::MissingElement("annotation".into()),
                )?))
            }
            S::SimpleType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RedefineContentDeserializer::store_simple_type(&mut values, value)?;
                }
                Ok(super::RedefineContent::SimpleType(values.ok_or_else(
                    || ErrorKind::MissingElement("simpleType".into()),
                )?))
            }
            S::ComplexType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RedefineContentDeserializer::store_complex_type(&mut values, value)?;
                }
                Ok(super::RedefineContent::ComplexType(values.ok_or_else(
                    || ErrorKind::MissingElement("complexType".into()),
                )?))
            }
            S::Group(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RedefineContentDeserializer::store_group(&mut values, value)?;
                }
                Ok(super::RedefineContent::Group(values.ok_or_else(|| {
                    ErrorKind::MissingElement("group".into())
                })?))
            }
            S::AttributeGroup(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RedefineContentDeserializer::store_attribute_group(&mut values, value)?;
                }
                Ok(super::RedefineContent::AttributeGroup(values.ok_or_else(
                    || ErrorKind::MissingElement("attributeGroup".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_simple_type(
        values: &mut Option<super::SimpleBaseType>,
        value: super::SimpleBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"simpleType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_complex_type(
        values: &mut Option<super::ComplexBaseType>,
        value: super::ComplexBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"complexType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_group(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"group",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute_group(
        values: &mut Option<super::AttributeGroupType>,
        value: super::AttributeGroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attributeGroup",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<RedefineContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RedefineContentDeserializerState::Annotation(values, None),
                Some(RedefineContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    RedefineContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RedefineContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RedefineContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RedefineContentDeserializer::store_annotation(&mut values, data)?;
                let data = RedefineContentDeserializer::finish_state(
                    reader,
                    RedefineContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = RedefineContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RedefineContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_simple_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::SimpleBaseType>,
        output: DeserializerOutput<'de, super::SimpleBaseType>,
        fallback: &mut Option<RedefineContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RedefineContentDeserializerState::SimpleType(values, None),
                Some(RedefineContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                    RedefineContentDeserializerState::SimpleType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RedefineContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RedefineContentDeserializer::store_simple_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RedefineContentDeserializer::store_simple_type(&mut values, data)?;
                let data = RedefineContentDeserializer::finish_state(
                    reader,
                    RedefineContentDeserializerState::SimpleType(values, None),
                )?;
                *self.state = RedefineContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RedefineContentDeserializerState::SimpleType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_complex_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ComplexBaseType>,
        output: DeserializerOutput<'de, super::ComplexBaseType>,
        fallback: &mut Option<RedefineContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RedefineContentDeserializerState::ComplexType(values, None),
                Some(RedefineContentDeserializerState::ComplexType(_, Some(deserializer))) => {
                    RedefineContentDeserializerState::ComplexType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RedefineContentDeserializerState::ComplexType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RedefineContentDeserializer::store_complex_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RedefineContentDeserializer::store_complex_type(&mut values, data)?;
                let data = RedefineContentDeserializer::finish_state(
                    reader,
                    RedefineContentDeserializerState::ComplexType(values, None),
                )?;
                *self.state = RedefineContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RedefineContentDeserializerState::ComplexType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<RedefineContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RedefineContentDeserializerState::Group(values, None),
                Some(RedefineContentDeserializerState::Group(_, Some(deserializer))) => {
                    RedefineContentDeserializerState::Group(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RedefineContentDeserializerState::Group(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RedefineContentDeserializer::store_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RedefineContentDeserializer::store_group(&mut values, data)?;
                let data = RedefineContentDeserializer::finish_state(
                    reader,
                    RedefineContentDeserializerState::Group(values, None),
                )?;
                *self.state = RedefineContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = RedefineContentDeserializerState::Group(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeGroupType>,
        output: DeserializerOutput<'de, super::AttributeGroupType>,
        fallback: &mut Option<RedefineContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RedefineContentDeserializerState::AttributeGroup(values, None),
                Some(RedefineContentDeserializerState::AttributeGroup(_, Some(deserializer))) => {
                    RedefineContentDeserializerState::AttributeGroup(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RedefineContentDeserializerState::AttributeGroup(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RedefineContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RedefineContentDeserializer::store_attribute_group(&mut values, data)?;
                let data = RedefineContentDeserializer::finish_state(
                    reader,
                    RedefineContentDeserializerState::AttributeGroup(values, None),
                )?;
                *self.state = RedefineContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RedefineContentDeserializerState::AttributeGroup(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::RedefineContent> for Box<RedefineContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RedefineContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(RedefineContentDeserializer {
            state: Box::new(RedefineContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, RedefineContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::RedefineContent>
    where
        R: DeserializeReader,
    {
        use RedefineContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_complex_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            RedefineContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, None), event) => {
                    let output = <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexType(values, None), event) => {
                    let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_complex_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, None), event) => {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::RedefineContent, Error>
    where
        R: DeserializeReader,
    {
        RedefineContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct OverrideDeserializer {
    schema_location: String,
    id: Option<String>,
    content: Vec<super::OverrideContent>,
    state: Box<OverrideDeserializerState>,
}
#[derive(Debug)]
enum OverrideDeserializerState {
    Init__,
    Next__,
    Content__(<super::OverrideContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl OverrideDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut schema_location: Option<String> = None;
        let mut id: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"schemaLocation")
            ) {
                reader.read_attrib(&mut schema_location, b"schemaLocation", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            schema_location: schema_location.ok_or_else(|| {
                reader.map_error(ErrorKind::MissingAttribute("schemaLocation".into()))
            })?,
            id: id,
            content: Vec::new(),
            state: Box::new(OverrideDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: OverrideDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let OverrideDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::OverrideContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::OverrideContent>,
        fallback: &mut Option<OverrideDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback.take().unwrap_or(OverrideDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = OverrideDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = OverrideDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(OverrideDeserializerState::Content__(deserializer));
                        *self.state = OverrideDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Override> for Box<OverrideDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Override>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, OverrideDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Override>
    where
        R: DeserializeReader,
    {
        use OverrideDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output = <super::OverrideContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Override, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, OverrideDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Override {
            schema_location: self.schema_location,
            id: self.id,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct OverrideContentDeserializer {
    state: Box<OverrideContentDeserializerState>,
}
#[derive(Debug)]
pub enum OverrideContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    SimpleType(
        Option<super::SimpleBaseType>,
        Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
    ),
    ComplexType(
        Option<super::ComplexBaseType>,
        Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
    ),
    Group(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    AttributeGroup(
        Option<super::AttributeGroupType>,
        Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
    ),
    Element(
        Option<super::ElementType>,
        Option<<super::ElementType as WithDeserializer>::Deserializer>,
    ),
    Attribute(
        Option<super::AttributeType>,
        Option<<super::AttributeType as WithDeserializer>::Deserializer>,
    ),
    Notation(
        Option<super::Notation>,
        Option<<super::Notation as WithDeserializer>::Deserializer>,
    ),
    Done__(super::OverrideContent),
    Unknown__,
}
impl OverrideContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<OverrideContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"complexType")
            ) {
                let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_complex_type(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"element")
            ) {
                let output =
                    <super::ElementType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_element(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"notation")
            ) {
                let output =
                    <super::Notation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_notation(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(OverrideContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: OverrideContentDeserializerState,
    ) -> Result<super::OverrideContent, Error>
    where
        R: DeserializeReader,
    {
        use OverrideContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    OverrideContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::OverrideContent::Annotation(values.ok_or_else(
                    || ErrorKind::MissingElement("annotation".into()),
                )?))
            }
            S::SimpleType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    OverrideContentDeserializer::store_simple_type(&mut values, value)?;
                }
                Ok(super::OverrideContent::SimpleType(values.ok_or_else(
                    || ErrorKind::MissingElement("simpleType".into()),
                )?))
            }
            S::ComplexType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    OverrideContentDeserializer::store_complex_type(&mut values, value)?;
                }
                Ok(super::OverrideContent::ComplexType(values.ok_or_else(
                    || ErrorKind::MissingElement("complexType".into()),
                )?))
            }
            S::Group(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    OverrideContentDeserializer::store_group(&mut values, value)?;
                }
                Ok(super::OverrideContent::Group(values.ok_or_else(|| {
                    ErrorKind::MissingElement("group".into())
                })?))
            }
            S::AttributeGroup(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    OverrideContentDeserializer::store_attribute_group(&mut values, value)?;
                }
                Ok(super::OverrideContent::AttributeGroup(values.ok_or_else(
                    || ErrorKind::MissingElement("attributeGroup".into()),
                )?))
            }
            S::Element(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    OverrideContentDeserializer::store_element(&mut values, value)?;
                }
                Ok(super::OverrideContent::Element(values.ok_or_else(
                    || ErrorKind::MissingElement("element".into()),
                )?))
            }
            S::Attribute(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    OverrideContentDeserializer::store_attribute(&mut values, value)?;
                }
                Ok(super::OverrideContent::Attribute(values.ok_or_else(
                    || ErrorKind::MissingElement("attribute".into()),
                )?))
            }
            S::Notation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    OverrideContentDeserializer::store_notation(&mut values, value)?;
                }
                Ok(super::OverrideContent::Notation(values.ok_or_else(
                    || ErrorKind::MissingElement("notation".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_simple_type(
        values: &mut Option<super::SimpleBaseType>,
        value: super::SimpleBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"simpleType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_complex_type(
        values: &mut Option<super::ComplexBaseType>,
        value: super::ComplexBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"complexType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_group(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"group",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute_group(
        values: &mut Option<super::AttributeGroupType>,
        value: super::AttributeGroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attributeGroup",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_element(
        values: &mut Option<super::ElementType>,
        value: super::ElementType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"element",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute(
        values: &mut Option<super::AttributeType>,
        value: super::AttributeType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attribute",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_notation(
        values: &mut Option<super::Notation>,
        value: super::Notation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"notation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<OverrideContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => OverrideContentDeserializerState::Annotation(values, None),
                Some(OverrideContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    OverrideContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(OverrideContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                OverrideContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                OverrideContentDeserializer::store_annotation(&mut values, data)?;
                let data = OverrideContentDeserializer::finish_state(
                    reader,
                    OverrideContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = OverrideContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    OverrideContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_simple_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::SimpleBaseType>,
        output: DeserializerOutput<'de, super::SimpleBaseType>,
        fallback: &mut Option<OverrideContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => OverrideContentDeserializerState::SimpleType(values, None),
                Some(OverrideContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                    OverrideContentDeserializerState::SimpleType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(OverrideContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                OverrideContentDeserializer::store_simple_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                OverrideContentDeserializer::store_simple_type(&mut values, data)?;
                let data = OverrideContentDeserializer::finish_state(
                    reader,
                    OverrideContentDeserializerState::SimpleType(values, None),
                )?;
                *self.state = OverrideContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    OverrideContentDeserializerState::SimpleType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_complex_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ComplexBaseType>,
        output: DeserializerOutput<'de, super::ComplexBaseType>,
        fallback: &mut Option<OverrideContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => OverrideContentDeserializerState::ComplexType(values, None),
                Some(OverrideContentDeserializerState::ComplexType(_, Some(deserializer))) => {
                    OverrideContentDeserializerState::ComplexType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(OverrideContentDeserializerState::ComplexType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                OverrideContentDeserializer::store_complex_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                OverrideContentDeserializer::store_complex_type(&mut values, data)?;
                let data = OverrideContentDeserializer::finish_state(
                    reader,
                    OverrideContentDeserializerState::ComplexType(values, None),
                )?;
                *self.state = OverrideContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    OverrideContentDeserializerState::ComplexType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<OverrideContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => OverrideContentDeserializerState::Group(values, None),
                Some(OverrideContentDeserializerState::Group(_, Some(deserializer))) => {
                    OverrideContentDeserializerState::Group(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(OverrideContentDeserializerState::Group(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                OverrideContentDeserializer::store_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                OverrideContentDeserializer::store_group(&mut values, data)?;
                let data = OverrideContentDeserializer::finish_state(
                    reader,
                    OverrideContentDeserializerState::Group(values, None),
                )?;
                *self.state = OverrideContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = OverrideContentDeserializerState::Group(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeGroupType>,
        output: DeserializerOutput<'de, super::AttributeGroupType>,
        fallback: &mut Option<OverrideContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => OverrideContentDeserializerState::AttributeGroup(values, None),
                Some(OverrideContentDeserializerState::AttributeGroup(_, Some(deserializer))) => {
                    OverrideContentDeserializerState::AttributeGroup(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(OverrideContentDeserializerState::AttributeGroup(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                OverrideContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                OverrideContentDeserializer::store_attribute_group(&mut values, data)?;
                let data = OverrideContentDeserializer::finish_state(
                    reader,
                    OverrideContentDeserializerState::AttributeGroup(values, None),
                )?;
                *self.state = OverrideContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    OverrideContentDeserializerState::AttributeGroup(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_element<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ElementType>,
        output: DeserializerOutput<'de, super::ElementType>,
        fallback: &mut Option<OverrideContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => OverrideContentDeserializerState::Element(values, None),
                Some(OverrideContentDeserializerState::Element(_, Some(deserializer))) => {
                    OverrideContentDeserializerState::Element(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(OverrideContentDeserializerState::Element(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                OverrideContentDeserializer::store_element(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                OverrideContentDeserializer::store_element(&mut values, data)?;
                let data = OverrideContentDeserializer::finish_state(
                    reader,
                    OverrideContentDeserializerState::Element(values, None),
                )?;
                *self.state = OverrideContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = OverrideContentDeserializerState::Element(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeType>,
        output: DeserializerOutput<'de, super::AttributeType>,
        fallback: &mut Option<OverrideContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => OverrideContentDeserializerState::Attribute(values, None),
                Some(OverrideContentDeserializerState::Attribute(_, Some(deserializer))) => {
                    OverrideContentDeserializerState::Attribute(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(OverrideContentDeserializerState::Attribute(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                OverrideContentDeserializer::store_attribute(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                OverrideContentDeserializer::store_attribute(&mut values, data)?;
                let data = OverrideContentDeserializer::finish_state(
                    reader,
                    OverrideContentDeserializerState::Attribute(values, None),
                )?;
                *self.state = OverrideContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    OverrideContentDeserializerState::Attribute(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_notation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Notation>,
        output: DeserializerOutput<'de, super::Notation>,
        fallback: &mut Option<OverrideContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => OverrideContentDeserializerState::Notation(values, None),
                Some(OverrideContentDeserializerState::Notation(_, Some(deserializer))) => {
                    OverrideContentDeserializerState::Notation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(OverrideContentDeserializerState::Notation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                OverrideContentDeserializer::store_notation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                OverrideContentDeserializer::store_notation(&mut values, data)?;
                let data = OverrideContentDeserializer::finish_state(
                    reader,
                    OverrideContentDeserializerState::Notation(values, None),
                )?;
                *self.state = OverrideContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    OverrideContentDeserializerState::Notation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::OverrideContent> for Box<OverrideContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::OverrideContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(OverrideContentDeserializer {
            state: Box::new(OverrideContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, OverrideContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::OverrideContent>
    where
        R: DeserializeReader,
    {
        use OverrideContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_complex_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Element(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_element(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Notation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_notation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            OverrideContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, None), event) => {
                    let output = <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexType(values, None), event) => {
                    let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_complex_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, None), event) => {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Element(values, None), event) => {
                    let output = <super::ElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_element(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, None), event) => {
                    let output = <super::AttributeType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Notation(values, None), event) => {
                    let output =
                        <super::Notation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_notation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::OverrideContent, Error>
    where
        R: DeserializeReader,
    {
        OverrideContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct AnnotationDeserializer {
    id: Option<String>,
    content: Vec<super::AnnotationContent>,
    state: Box<AnnotationDeserializerState>,
}
#[derive(Debug)]
enum AnnotationDeserializerState {
    Init__,
    Next__,
    Content__(<super::AnnotationContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl AnnotationDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            content: Vec::new(),
            state: Box::new(AnnotationDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: AnnotationDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let AnnotationDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::AnnotationContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::AnnotationContent>,
        fallback: &mut Option<AnnotationDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(AnnotationDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = AnnotationDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = AnnotationDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(AnnotationDeserializerState::Content__(deserializer));
                        *self.state = AnnotationDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Annotation> for Box<AnnotationDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Annotation>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, AnnotationDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::Annotation>
    where
        R: DeserializeReader,
    {
        use AnnotationDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::AnnotationContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Annotation, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, AnnotationDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Annotation {
            id: self.id,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct AnnotationContentDeserializer {
    state: Box<AnnotationContentDeserializerState>,
}
#[derive(Debug)]
pub enum AnnotationContentDeserializerState {
    Init__,
    Appinfo(
        Option<AnyElement>,
        Option<<AnyElement as WithDeserializer>::Deserializer>,
    ),
    Documentation(
        Option<AnyElement>,
        Option<<AnyElement as WithDeserializer>::Deserializer>,
    ),
    Done__(super::AnnotationContent),
    Unknown__,
}
impl AnnotationContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<AnnotationContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"appinfo")
            ) {
                let output = <AnyElement as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_appinfo(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"documentation")
            ) {
                let output = <AnyElement as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_documentation(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(AnnotationContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: AnnotationContentDeserializerState,
    ) -> Result<super::AnnotationContent, Error>
    where
        R: DeserializeReader,
    {
        use AnnotationContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Appinfo(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AnnotationContentDeserializer::store_appinfo(&mut values, value)?;
                }
                Ok(super::AnnotationContent::Appinfo(values.ok_or_else(
                    || ErrorKind::MissingElement("appinfo".into()),
                )?))
            }
            S::Documentation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AnnotationContentDeserializer::store_documentation(&mut values, value)?;
                }
                Ok(super::AnnotationContent::Documentation(values.ok_or_else(
                    || ErrorKind::MissingElement("documentation".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_appinfo(values: &mut Option<AnyElement>, value: AnyElement) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"appinfo",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_documentation(
        values: &mut Option<AnyElement>,
        value: AnyElement,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"documentation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_appinfo<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<AnyElement>,
        output: DeserializerOutput<'de, AnyElement>,
        fallback: &mut Option<AnnotationContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => AnnotationContentDeserializerState::Appinfo(values, None),
                Some(AnnotationContentDeserializerState::Appinfo(_, Some(deserializer))) => {
                    AnnotationContentDeserializerState::Appinfo(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AnnotationContentDeserializerState::Appinfo(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AnnotationContentDeserializer::store_appinfo(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AnnotationContentDeserializer::store_appinfo(&mut values, data)?;
                let data = AnnotationContentDeserializer::finish_state(
                    reader,
                    AnnotationContentDeserializerState::Appinfo(values, None),
                )?;
                *self.state = AnnotationContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    AnnotationContentDeserializerState::Appinfo(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_documentation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<AnyElement>,
        output: DeserializerOutput<'de, AnyElement>,
        fallback: &mut Option<AnnotationContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => AnnotationContentDeserializerState::Documentation(values, None),
                Some(AnnotationContentDeserializerState::Documentation(_, Some(deserializer))) => {
                    AnnotationContentDeserializerState::Documentation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AnnotationContentDeserializerState::Documentation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AnnotationContentDeserializer::store_documentation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AnnotationContentDeserializer::store_documentation(&mut values, data)?;
                let data = AnnotationContentDeserializer::finish_state(
                    reader,
                    AnnotationContentDeserializerState::Documentation(values, None),
                )?;
                *self.state = AnnotationContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    AnnotationContentDeserializerState::Documentation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::AnnotationContent> for Box<AnnotationContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AnnotationContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(AnnotationContentDeserializer {
            state: Box::new(AnnotationContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, AnnotationContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::AnnotationContent>
    where
        R: DeserializeReader,
    {
        use AnnotationContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Appinfo(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_appinfo(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Documentation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_documentation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            AnnotationContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Appinfo(values, None), event) => {
                    let output =
                        <AnyElement as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_appinfo(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Documentation(values, None), event) => {
                    let output =
                        <AnyElement as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_documentation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::AnnotationContent, Error>
    where
        R: DeserializeReader,
    {
        AnnotationContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct DefaultOpenContentDeserializer {
    id: Option<String>,
    applies_to_empty: bool,
    mode: super::DefaultOpenContentModeType,
    annotation: Option<super::Annotation>,
    any: Option<super::WildcardType>,
    state: Box<DefaultOpenContentDeserializerState>,
}
#[derive(Debug)]
enum DefaultOpenContentDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Any(Option<<super::WildcardType as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl DefaultOpenContentDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut applies_to_empty: Option<bool> = None;
        let mut mode: Option<super::DefaultOpenContentModeType> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"appliesToEmpty")
            ) {
                reader.read_attrib(&mut applies_to_empty, b"appliesToEmpty", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"mode")
            ) {
                reader.read_attrib(&mut mode, b"mode", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            applies_to_empty: applies_to_empty
                .unwrap_or_else(super::DefaultOpenContent::default_applies_to_empty),
            mode: mode.unwrap_or_else(super::DefaultOpenContent::default_mode),
            annotation: None,
            any: None,
            state: Box::new(DefaultOpenContentDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: DefaultOpenContentDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use DefaultOpenContentDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            S::Any(Some(deserializer)) => self.store_any(deserializer.finish(reader)?)?,
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn store_any(&mut self, value: super::WildcardType) -> Result<(), Error> {
        if self.any.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
        }
        self.any = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<DefaultOpenContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(DefaultOpenContentDeserializerState::Annotation(None));
            *self.state = DefaultOpenContentDeserializerState::Any(None);
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = DefaultOpenContentDeserializerState::Any(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(DefaultOpenContentDeserializerState::Annotation(
                            Some(deserializer),
                        ));
                        *self.state = DefaultOpenContentDeserializerState::Any(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state =
                            DefaultOpenContentDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
    fn handle_any<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::WildcardType>,
        fallback: &mut Option<DefaultOpenContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            if self.any.is_some() {
                fallback.get_or_insert(DefaultOpenContentDeserializerState::Any(None));
                *self.state = DefaultOpenContentDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            } else {
                *self.state = DefaultOpenContentDeserializerState::Any(None);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_any(data)?;
                *self.state = DefaultOpenContentDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(DefaultOpenContentDeserializerState::Any(Some(
                            deserializer,
                        )));
                        *self.state = DefaultOpenContentDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = DefaultOpenContentDeserializerState::Any(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::DefaultOpenContent> for Box<DefaultOpenContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DefaultOpenContent>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(
            event,
            DefaultOpenContentDeserializer::from_bytes_start,
        )
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::DefaultOpenContent>
    where
        R: DeserializeReader,
    {
        use DefaultOpenContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (S::Any(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_any(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = DefaultOpenContentDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Any(None);
                        event
                    }
                }
                (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(&event, Some(&super::super::NS_XS), b"any") {
                        let output = <super::WildcardType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_any(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::DefaultOpenContent, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(
            &mut *self.state,
            DefaultOpenContentDeserializerState::Unknown__,
        );
        self.finish_state(reader, state)?;
        Ok(super::DefaultOpenContent {
            id: self.id,
            applies_to_empty: self.applies_to_empty,
            mode: self.mode,
            annotation: self.annotation,
            any: self
                .any
                .ok_or_else(|| ErrorKind::MissingElement("any".into()))?,
        })
    }
}
#[derive(Debug)]
pub struct SimpleBaseTypeDeserializer {
    id: Option<String>,
    final_: Option<super::SimpleDerivationSetType>,
    name: Option<String>,
    content: Vec<super::SimpleBaseTypeContent>,
    state: Box<SimpleBaseTypeDeserializerState>,
}
#[derive(Debug)]
enum SimpleBaseTypeDeserializerState {
    Init__,
    Next__,
    Content__(<super::SimpleBaseTypeContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl SimpleBaseTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut final_: Option<super::SimpleDerivationSetType> = None;
        let mut name: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"final")
            ) {
                reader.read_attrib(&mut final_, b"final", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            final_: final_,
            name: name,
            content: Vec::new(),
            state: Box::new(SimpleBaseTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: SimpleBaseTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let SimpleBaseTypeDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::SimpleBaseTypeContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::SimpleBaseTypeContent>,
        fallback: &mut Option<SimpleBaseTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(SimpleBaseTypeDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = SimpleBaseTypeDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let can_have_more = self.content.len().saturating_add(1) < 2usize;
                let ret = if can_have_more {
                    ElementHandlerOutput::from_event(event, allow_any)
                } else {
                    ElementHandlerOutput::from_event_end(event, allow_any)
                };
                match (can_have_more, &ret) {
                    (true, ElementHandlerOutput::Continue { .. }) => {
                        fallback.get_or_insert(SimpleBaseTypeDeserializerState::Content__(
                            deserializer,
                        ));
                        *self.state = SimpleBaseTypeDeserializerState::Next__;
                    }
                    (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                        *self.state = SimpleBaseTypeDeserializerState::Content__(deserializer);
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::SimpleBaseType> for Box<SimpleBaseTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SimpleBaseType>
    where
        R: DeserializeReader,
    {
        reader
            .init_deserializer_from_start_event(event, SimpleBaseTypeDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::SimpleBaseType>
    where
        R: DeserializeReader,
    {
        use SimpleBaseTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::SimpleBaseTypeContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::SimpleBaseType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, SimpleBaseTypeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::SimpleBaseType {
            id: self.id,
            final_: self.final_,
            name: self.name,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct SimpleBaseTypeContentDeserializer {
    state: Box<SimpleBaseTypeContentDeserializerState>,
}
#[derive(Debug)]
pub enum SimpleBaseTypeContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    Restriction(
        Option<super::Restriction>,
        Option<<super::Restriction as WithDeserializer>::Deserializer>,
    ),
    List(
        Option<super::List>,
        Option<<super::List as WithDeserializer>::Deserializer>,
    ),
    Union(
        Option<super::Union>,
        Option<<super::Union as WithDeserializer>::Deserializer>,
    ),
    Done__(super::SimpleBaseTypeContent),
    Unknown__,
}
impl SimpleBaseTypeContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<SimpleBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"restriction")
            ) {
                let output =
                    <super::Restriction as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_restriction(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"list")
            ) {
                let output = <super::List as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_list(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"union")
            ) {
                let output = <super::Union as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_union_(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(SimpleBaseTypeContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: SimpleBaseTypeContentDeserializerState,
    ) -> Result<super::SimpleBaseTypeContent, Error>
    where
        R: DeserializeReader,
    {
        use SimpleBaseTypeContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SimpleBaseTypeContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::SimpleBaseTypeContent::Annotation(
                    values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                ))
            }
            S::Restriction(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SimpleBaseTypeContentDeserializer::store_restriction(&mut values, value)?;
                }
                Ok(super::SimpleBaseTypeContent::Restriction(
                    values.ok_or_else(|| ErrorKind::MissingElement("restriction".into()))?,
                ))
            }
            S::List(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SimpleBaseTypeContentDeserializer::store_list(&mut values, value)?;
                }
                Ok(super::SimpleBaseTypeContent::List(
                    values.ok_or_else(|| ErrorKind::MissingElement("list".into()))?,
                ))
            }
            S::Union(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SimpleBaseTypeContentDeserializer::store_union_(&mut values, value)?;
                }
                Ok(super::SimpleBaseTypeContent::Union(values.ok_or_else(
                    || ErrorKind::MissingElement("union".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_restriction(
        values: &mut Option<super::Restriction>,
        value: super::Restriction,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"restriction",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_list(values: &mut Option<super::List>, value: super::List) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"list")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_union_(values: &mut Option<super::Union>, value: super::Union) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"union",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<SimpleBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SimpleBaseTypeContentDeserializerState::Annotation(values, None),
                Some(SimpleBaseTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    SimpleBaseTypeContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SimpleBaseTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SimpleBaseTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SimpleBaseTypeContentDeserializer::store_annotation(&mut values, data)?;
                let data = SimpleBaseTypeContentDeserializer::finish_state(
                    reader,
                    SimpleBaseTypeContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = SimpleBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SimpleBaseTypeContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_restriction<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Restriction>,
        output: DeserializerOutput<'de, super::Restriction>,
        fallback: &mut Option<SimpleBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SimpleBaseTypeContentDeserializerState::Restriction(values, None),
                Some(SimpleBaseTypeContentDeserializerState::Restriction(
                    _,
                    Some(deserializer),
                )) => {
                    SimpleBaseTypeContentDeserializerState::Restriction(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SimpleBaseTypeContentDeserializerState::Restriction(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SimpleBaseTypeContentDeserializer::store_restriction(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SimpleBaseTypeContentDeserializer::store_restriction(&mut values, data)?;
                let data = SimpleBaseTypeContentDeserializer::finish_state(
                    reader,
                    SimpleBaseTypeContentDeserializerState::Restriction(values, None),
                )?;
                *self.state = SimpleBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SimpleBaseTypeContentDeserializerState::Restriction(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_list<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::List>,
        output: DeserializerOutput<'de, super::List>,
        fallback: &mut Option<SimpleBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SimpleBaseTypeContentDeserializerState::List(values, None),
                Some(SimpleBaseTypeContentDeserializerState::List(_, Some(deserializer))) => {
                    SimpleBaseTypeContentDeserializerState::List(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SimpleBaseTypeContentDeserializerState::List(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SimpleBaseTypeContentDeserializer::store_list(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SimpleBaseTypeContentDeserializer::store_list(&mut values, data)?;
                let data = SimpleBaseTypeContentDeserializer::finish_state(
                    reader,
                    SimpleBaseTypeContentDeserializerState::List(values, None),
                )?;
                *self.state = SimpleBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SimpleBaseTypeContentDeserializerState::List(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_union_<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Union>,
        output: DeserializerOutput<'de, super::Union>,
        fallback: &mut Option<SimpleBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SimpleBaseTypeContentDeserializerState::Union(values, None),
                Some(SimpleBaseTypeContentDeserializerState::Union(_, Some(deserializer))) => {
                    SimpleBaseTypeContentDeserializerState::Union(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SimpleBaseTypeContentDeserializerState::Union(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SimpleBaseTypeContentDeserializer::store_union_(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SimpleBaseTypeContentDeserializer::store_union_(&mut values, data)?;
                let data = SimpleBaseTypeContentDeserializer::finish_state(
                    reader,
                    SimpleBaseTypeContentDeserializerState::Union(values, None),
                )?;
                *self.state = SimpleBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SimpleBaseTypeContentDeserializerState::Union(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::SimpleBaseTypeContent>
    for Box<SimpleBaseTypeContentDeserializer>
{
    fn init<R>(
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::SimpleBaseTypeContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(SimpleBaseTypeContentDeserializer {
            state: Box::new(SimpleBaseTypeContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, SimpleBaseTypeContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::SimpleBaseTypeContent>
    where
        R: DeserializeReader,
    {
        use SimpleBaseTypeContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Restriction(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_restriction(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::List(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_list(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Union(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_union_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            SimpleBaseTypeContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Restriction(values, None), event) => {
                    let output = <super::Restriction as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_restriction(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::List(values, None), event) => {
                    let output =
                        <super::List as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_list(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Union(values, None), event) => {
                    let output =
                        <super::Union as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_union_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::SimpleBaseTypeContent, Error>
    where
        R: DeserializeReader,
    {
        SimpleBaseTypeContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct ComplexBaseTypeDeserializer {
    id: Option<String>,
    name: Option<String>,
    mixed: Option<bool>,
    abstract_: bool,
    final_: Option<super::DerivationSetType>,
    block: Option<super::DerivationSetType>,
    default_attributes_apply: bool,
    content: Vec<super::ComplexBaseTypeContent>,
    state: Box<ComplexBaseTypeDeserializerState>,
}
#[derive(Debug)]
enum ComplexBaseTypeDeserializerState {
    Init__,
    Next__,
    Content__(<super::ComplexBaseTypeContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl ComplexBaseTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;
        let mut mixed: Option<bool> = None;
        let mut abstract_: Option<bool> = None;
        let mut final_: Option<super::DerivationSetType> = None;
        let mut block: Option<super::DerivationSetType> = None;
        let mut default_attributes_apply: Option<bool> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"mixed")
            ) {
                reader.read_attrib(&mut mixed, b"mixed", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"abstract")
            ) {
                reader.read_attrib(&mut abstract_, b"abstract", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"final")
            ) {
                reader.read_attrib(&mut final_, b"final", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"block")
            ) {
                reader.read_attrib(&mut block, b"block", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"defaultAttributesApply")
            ) {
                reader.read_attrib(
                    &mut default_attributes_apply,
                    b"defaultAttributesApply",
                    &attrib.value,
                )?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            name: name,
            mixed: mixed,
            abstract_: abstract_.unwrap_or_else(super::ComplexBaseType::default_abstract_),
            final_: final_,
            block: block,
            default_attributes_apply: default_attributes_apply
                .unwrap_or_else(super::ComplexBaseType::default_default_attributes_apply),
            content: Vec::new(),
            state: Box::new(ComplexBaseTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: ComplexBaseTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let ComplexBaseTypeDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::ComplexBaseTypeContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::ComplexBaseTypeContent>,
        fallback: &mut Option<ComplexBaseTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(ComplexBaseTypeDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = ComplexBaseTypeDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = ComplexBaseTypeDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(ComplexBaseTypeDeserializerState::Content__(
                            deserializer,
                        ));
                        *self.state = ComplexBaseTypeDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::ComplexBaseType> for Box<ComplexBaseTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ComplexBaseType>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(
            event,
            ComplexBaseTypeDeserializer::from_bytes_start,
        )
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ComplexBaseType>
    where
        R: DeserializeReader,
    {
        use ComplexBaseTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::ComplexBaseTypeContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::ComplexBaseType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(
            &mut *self.state,
            ComplexBaseTypeDeserializerState::Unknown__,
        );
        self.finish_state(reader, state)?;
        Ok(super::ComplexBaseType {
            id: self.id,
            name: self.name,
            mixed: self.mixed,
            abstract_: self.abstract_,
            final_: self.final_,
            block: self.block,
            default_attributes_apply: self.default_attributes_apply,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct ComplexBaseTypeContentDeserializer {
    state: Box<ComplexBaseTypeContentDeserializerState>,
}
#[derive(Debug)]
pub enum ComplexBaseTypeContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    SimpleContent(
        Option<super::SimpleContent>,
        Option<<super::SimpleContent as WithDeserializer>::Deserializer>,
    ),
    ComplexContent(
        Option<super::ComplexContent>,
        Option<<super::ComplexContent as WithDeserializer>::Deserializer>,
    ),
    OpenContent(
        Option<super::OpenContent>,
        Option<<super::OpenContent as WithDeserializer>::Deserializer>,
    ),
    Group(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    All(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Choice(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Sequence(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Attribute(
        Option<super::AttributeType>,
        Option<<super::AttributeType as WithDeserializer>::Deserializer>,
    ),
    AttributeGroup(
        Option<super::AttributeGroupType>,
        Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
    ),
    AnyAttribute(
        Option<super::AnyAttribute>,
        Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
    ),
    Assert(
        Option<super::AssertionType>,
        Option<<super::AssertionType as WithDeserializer>::Deserializer>,
    ),
    Done__(super::ComplexBaseTypeContent),
    Unknown__,
}
impl ComplexBaseTypeContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"simpleContent")
            ) {
                let output =
                    <super::SimpleContent as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"complexContent")
            ) {
                let output =
                    <super::ComplexContent as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_complex_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"openContent")
            ) {
                let output =
                    <super::OpenContent as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_open_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"all")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_all(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"choice")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_choice(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"sequence")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_sequence(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"anyAttribute")
            ) {
                let output =
                    <super::AnyAttribute as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_any_attribute(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"assert")
            ) {
                let output =
                    <super::AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_assert(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(ComplexBaseTypeContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: ComplexBaseTypeContentDeserializerState,
    ) -> Result<super::ComplexBaseTypeContent, Error>
    where
        R: DeserializeReader,
    {
        use ComplexBaseTypeContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::Annotation(
                    values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                ))
            }
            S::SimpleContent(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_simple_content(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::SimpleContent(
                    values.ok_or_else(|| ErrorKind::MissingElement("simpleContent".into()))?,
                ))
            }
            S::ComplexContent(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_complex_content(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::ComplexContent(
                    values.ok_or_else(|| ErrorKind::MissingElement("complexContent".into()))?,
                ))
            }
            S::OpenContent(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_open_content(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::OpenContent(
                    values.ok_or_else(|| ErrorKind::MissingElement("openContent".into()))?,
                ))
            }
            S::Group(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_group(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::Group(values.ok_or_else(
                    || ErrorKind::MissingElement("group".into()),
                )?))
            }
            S::All(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_all(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::All(
                    values.ok_or_else(|| ErrorKind::MissingElement("all".into()))?,
                ))
            }
            S::Choice(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_choice(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::Choice(values.ok_or_else(
                    || ErrorKind::MissingElement("choice".into()),
                )?))
            }
            S::Sequence(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_sequence(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::Sequence(values.ok_or_else(
                    || ErrorKind::MissingElement("sequence".into()),
                )?))
            }
            S::Attribute(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_attribute(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::Attribute(
                    values.ok_or_else(|| ErrorKind::MissingElement("attribute".into()))?,
                ))
            }
            S::AttributeGroup(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_attribute_group(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::AttributeGroup(
                    values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                ))
            }
            S::AnyAttribute(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_any_attribute(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::AnyAttribute(
                    values.ok_or_else(|| ErrorKind::MissingElement("anyAttribute".into()))?,
                ))
            }
            S::Assert(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexBaseTypeContentDeserializer::store_assert(&mut values, value)?;
                }
                Ok(super::ComplexBaseTypeContent::Assert(values.ok_or_else(
                    || ErrorKind::MissingElement("assert".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_simple_content(
        values: &mut Option<super::SimpleContent>,
        value: super::SimpleContent,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"simpleContent",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_complex_content(
        values: &mut Option<super::ComplexContent>,
        value: super::ComplexContent,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"complexContent",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_open_content(
        values: &mut Option<super::OpenContent>,
        value: super::OpenContent,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"openContent",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_group(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"group",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_all(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"all")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_choice(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"choice",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_sequence(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"sequence",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute(
        values: &mut Option<super::AttributeType>,
        value: super::AttributeType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attribute",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute_group(
        values: &mut Option<super::AttributeGroupType>,
        value: super::AttributeGroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attributeGroup",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_any_attribute(
        values: &mut Option<super::AnyAttribute>,
        value: super::AnyAttribute,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"anyAttribute",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_assert(
        values: &mut Option<super::AssertionType>,
        value: super::AssertionType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"assert",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::Annotation(values, None),
                Some(ComplexBaseTypeContentDeserializerState::Annotation(
                    _,
                    Some(deserializer),
                )) => {
                    ComplexBaseTypeContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_annotation(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ComplexBaseTypeContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_simple_content<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::SimpleContent>,
        output: DeserializerOutput<'de, super::SimpleContent>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::SimpleContent(values, None),
                Some(ComplexBaseTypeContentDeserializerState::SimpleContent(
                    _,
                    Some(deserializer),
                )) => ComplexBaseTypeContentDeserializerState::SimpleContent(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::SimpleContent(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_simple_content(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_simple_content(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::SimpleContent(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ComplexBaseTypeContentDeserializerState::SimpleContent(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_complex_content<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ComplexContent>,
        output: DeserializerOutput<'de, super::ComplexContent>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::ComplexContent(values, None),
                Some(ComplexBaseTypeContentDeserializerState::ComplexContent(
                    _,
                    Some(deserializer),
                )) => ComplexBaseTypeContentDeserializerState::ComplexContent(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::ComplexContent(
                _,
                Some(deserializer),
            )) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_complex_content(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_complex_content(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::ComplexContent(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ComplexBaseTypeContentDeserializerState::ComplexContent(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_open_content<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::OpenContent>,
        output: DeserializerOutput<'de, super::OpenContent>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::OpenContent(values, None),
                Some(ComplexBaseTypeContentDeserializerState::OpenContent(
                    _,
                    Some(deserializer),
                )) => {
                    ComplexBaseTypeContentDeserializerState::OpenContent(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::OpenContent(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_open_content(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_open_content(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::OpenContent(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ComplexBaseTypeContentDeserializerState::OpenContent(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::Group(values, None),
                Some(ComplexBaseTypeContentDeserializerState::Group(_, Some(deserializer))) => {
                    ComplexBaseTypeContentDeserializerState::Group(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::Group(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_group(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::Group(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ComplexBaseTypeContentDeserializerState::Group(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_all<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::All(values, None),
                Some(ComplexBaseTypeContentDeserializerState::All(_, Some(deserializer))) => {
                    ComplexBaseTypeContentDeserializerState::All(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::All(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_all(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_all(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::All(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ComplexBaseTypeContentDeserializerState::All(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_choice<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::Choice(values, None),
                Some(ComplexBaseTypeContentDeserializerState::Choice(_, Some(deserializer))) => {
                    ComplexBaseTypeContentDeserializerState::Choice(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::Choice(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_choice(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_choice(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::Choice(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ComplexBaseTypeContentDeserializerState::Choice(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_sequence<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::Sequence(values, None),
                Some(ComplexBaseTypeContentDeserializerState::Sequence(_, Some(deserializer))) => {
                    ComplexBaseTypeContentDeserializerState::Sequence(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::Sequence(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_sequence(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_sequence(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::Sequence(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ComplexBaseTypeContentDeserializerState::Sequence(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeType>,
        output: DeserializerOutput<'de, super::AttributeType>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::Attribute(values, None),
                Some(ComplexBaseTypeContentDeserializerState::Attribute(_, Some(deserializer))) => {
                    ComplexBaseTypeContentDeserializerState::Attribute(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::Attribute(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_attribute(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_attribute(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::Attribute(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ComplexBaseTypeContentDeserializerState::Attribute(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeGroupType>,
        output: DeserializerOutput<'de, super::AttributeGroupType>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::AttributeGroup(values, None),
                Some(ComplexBaseTypeContentDeserializerState::AttributeGroup(
                    _,
                    Some(deserializer),
                )) => ComplexBaseTypeContentDeserializerState::AttributeGroup(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::AttributeGroup(
                _,
                Some(deserializer),
            )) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_attribute_group(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::AttributeGroup(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ComplexBaseTypeContentDeserializerState::AttributeGroup(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_any_attribute<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AnyAttribute>,
        output: DeserializerOutput<'de, super::AnyAttribute>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::AnyAttribute(values, None),
                Some(ComplexBaseTypeContentDeserializerState::AnyAttribute(
                    _,
                    Some(deserializer),
                )) => ComplexBaseTypeContentDeserializerState::AnyAttribute(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::AnyAttribute(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_any_attribute(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_any_attribute(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::AnyAttribute(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ComplexBaseTypeContentDeserializerState::AnyAttribute(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_assert<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AssertionType>,
        output: DeserializerOutput<'de, super::AssertionType>,
        fallback: &mut Option<ComplexBaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexBaseTypeContentDeserializerState::Assert(values, None),
                Some(ComplexBaseTypeContentDeserializerState::Assert(_, Some(deserializer))) => {
                    ComplexBaseTypeContentDeserializerState::Assert(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexBaseTypeContentDeserializerState::Assert(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexBaseTypeContentDeserializer::store_assert(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexBaseTypeContentDeserializer::store_assert(&mut values, data)?;
                let data = ComplexBaseTypeContentDeserializer::finish_state(
                    reader,
                    ComplexBaseTypeContentDeserializerState::Assert(values, None),
                )?;
                *self.state = ComplexBaseTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ComplexBaseTypeContentDeserializerState::Assert(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::ComplexBaseTypeContent>
    for Box<ComplexBaseTypeContentDeserializer>
{
    fn init<R>(
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ComplexBaseTypeContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(ComplexBaseTypeContentDeserializer {
            state: Box::new(ComplexBaseTypeContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, ComplexBaseTypeContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ComplexBaseTypeContent>
    where
        R: DeserializeReader,
    {
        use ComplexBaseTypeContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleContent(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexContent(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_complex_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::OpenContent(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_open_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::All(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_all(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Choice(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_choice(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Sequence(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_sequence(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AnyAttribute(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Assert(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_assert(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            ComplexBaseTypeContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleContent(values, None), event) => {
                    let output = <super::SimpleContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_simple_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexContent(values, None), event) => {
                    let output = <super::ComplexContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_complex_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::OpenContent(values, None), event) => {
                    let output = <super::OpenContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_open_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::All(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_all(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Choice(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_choice(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Sequence(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_sequence(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, None), event) => {
                    let output = <super::AttributeType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, None), event) => {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AnyAttribute(values, None), event) => {
                    let output = <super::AnyAttribute as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Assert(values, None), event) => {
                    let output = <super::AssertionType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_assert(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::ComplexBaseTypeContent, Error>
    where
        R: DeserializeReader,
    {
        ComplexBaseTypeContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct GroupTypeDeserializer {
    id: Option<String>,
    name: Option<String>,
    ref_: Option<QName>,
    min_occurs: usize,
    max_occurs: MaxOccurs,
    content: Vec<super::GroupTypeContent>,
    state: Box<GroupTypeDeserializerState>,
}
#[derive(Debug)]
enum GroupTypeDeserializerState {
    Init__,
    Next__,
    Content__(<super::GroupTypeContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl GroupTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;
        let mut ref_: Option<QName> = None;
        let mut min_occurs: Option<usize> = None;
        let mut max_occurs: Option<MaxOccurs> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"ref")
            ) {
                reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"minOccurs")
            ) {
                reader.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"maxOccurs")
            ) {
                reader.read_attrib(&mut max_occurs, b"maxOccurs", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            name: name,
            ref_: ref_,
            min_occurs: min_occurs.unwrap_or_else(super::GroupType::default_min_occurs),
            max_occurs: max_occurs.unwrap_or_else(super::GroupType::default_max_occurs),
            content: Vec::new(),
            state: Box::new(GroupTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: GroupTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let GroupTypeDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::GroupTypeContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::GroupTypeContent>,
        fallback: &mut Option<GroupTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(GroupTypeDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = GroupTypeDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = GroupTypeDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(GroupTypeDeserializerState::Content__(deserializer));
                        *self.state = GroupTypeDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::GroupType> for Box<GroupTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::GroupType>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, GroupTypeDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::GroupType>
    where
        R: DeserializeReader,
    {
        use GroupTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output = <super::GroupTypeContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::GroupType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, GroupTypeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::GroupType {
            id: self.id,
            name: self.name,
            ref_: self.ref_,
            min_occurs: self.min_occurs,
            max_occurs: self.max_occurs,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct GroupTypeContentDeserializer {
    state: Box<GroupTypeContentDeserializerState>,
}
#[derive(Debug)]
pub enum GroupTypeContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    Element(
        Option<super::ElementType>,
        Option<<super::ElementType as WithDeserializer>::Deserializer>,
    ),
    Group(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    All(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Choice(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Sequence(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Any(
        Option<super::Any>,
        Option<<super::Any as WithDeserializer>::Deserializer>,
    ),
    Done__(super::GroupTypeContent),
    Unknown__,
}
impl GroupTypeContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<GroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"element")
            ) {
                let output =
                    <super::ElementType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_element(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"all")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_all(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"choice")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_choice(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"sequence")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_sequence(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"any")
            ) {
                let output = <super::Any as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_any(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(GroupTypeContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: GroupTypeContentDeserializerState,
    ) -> Result<super::GroupTypeContent, Error>
    where
        R: DeserializeReader,
    {
        use GroupTypeContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    GroupTypeContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::GroupTypeContent::Annotation(values.ok_or_else(
                    || ErrorKind::MissingElement("annotation".into()),
                )?))
            }
            S::Element(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    GroupTypeContentDeserializer::store_element(&mut values, value)?;
                }
                Ok(super::GroupTypeContent::Element(values.ok_or_else(
                    || ErrorKind::MissingElement("element".into()),
                )?))
            }
            S::Group(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    GroupTypeContentDeserializer::store_group(&mut values, value)?;
                }
                Ok(super::GroupTypeContent::Group(values.ok_or_else(|| {
                    ErrorKind::MissingElement("group".into())
                })?))
            }
            S::All(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    GroupTypeContentDeserializer::store_all(&mut values, value)?;
                }
                Ok(super::GroupTypeContent::All(
                    values.ok_or_else(|| ErrorKind::MissingElement("all".into()))?,
                ))
            }
            S::Choice(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    GroupTypeContentDeserializer::store_choice(&mut values, value)?;
                }
                Ok(super::GroupTypeContent::Choice(values.ok_or_else(
                    || ErrorKind::MissingElement("choice".into()),
                )?))
            }
            S::Sequence(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    GroupTypeContentDeserializer::store_sequence(&mut values, value)?;
                }
                Ok(super::GroupTypeContent::Sequence(values.ok_or_else(
                    || ErrorKind::MissingElement("sequence".into()),
                )?))
            }
            S::Any(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    GroupTypeContentDeserializer::store_any(&mut values, value)?;
                }
                Ok(super::GroupTypeContent::Any(
                    values.ok_or_else(|| ErrorKind::MissingElement("any".into()))?,
                ))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_element(
        values: &mut Option<super::ElementType>,
        value: super::ElementType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"element",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_group(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"group",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_all(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"all")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_choice(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"choice",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_sequence(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"sequence",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_any(values: &mut Option<super::Any>, value: super::Any) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<GroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => GroupTypeContentDeserializerState::Annotation(values, None),
                Some(GroupTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    GroupTypeContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(GroupTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                GroupTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                GroupTypeContentDeserializer::store_annotation(&mut values, data)?;
                let data = GroupTypeContentDeserializer::finish_state(
                    reader,
                    GroupTypeContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = GroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    GroupTypeContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_element<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ElementType>,
        output: DeserializerOutput<'de, super::ElementType>,
        fallback: &mut Option<GroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => GroupTypeContentDeserializerState::Element(values, None),
                Some(GroupTypeContentDeserializerState::Element(_, Some(deserializer))) => {
                    GroupTypeContentDeserializerState::Element(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(GroupTypeContentDeserializerState::Element(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                GroupTypeContentDeserializer::store_element(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                GroupTypeContentDeserializer::store_element(&mut values, data)?;
                let data = GroupTypeContentDeserializer::finish_state(
                    reader,
                    GroupTypeContentDeserializerState::Element(values, None),
                )?;
                *self.state = GroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    GroupTypeContentDeserializerState::Element(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<GroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => GroupTypeContentDeserializerState::Group(values, None),
                Some(GroupTypeContentDeserializerState::Group(_, Some(deserializer))) => {
                    GroupTypeContentDeserializerState::Group(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(GroupTypeContentDeserializerState::Group(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                GroupTypeContentDeserializer::store_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                GroupTypeContentDeserializer::store_group(&mut values, data)?;
                let data = GroupTypeContentDeserializer::finish_state(
                    reader,
                    GroupTypeContentDeserializerState::Group(values, None),
                )?;
                *self.state = GroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = GroupTypeContentDeserializerState::Group(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_all<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<GroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => GroupTypeContentDeserializerState::All(values, None),
                Some(GroupTypeContentDeserializerState::All(_, Some(deserializer))) => {
                    GroupTypeContentDeserializerState::All(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(GroupTypeContentDeserializerState::All(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                GroupTypeContentDeserializer::store_all(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                GroupTypeContentDeserializer::store_all(&mut values, data)?;
                let data = GroupTypeContentDeserializer::finish_state(
                    reader,
                    GroupTypeContentDeserializerState::All(values, None),
                )?;
                *self.state = GroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = GroupTypeContentDeserializerState::All(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_choice<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<GroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => GroupTypeContentDeserializerState::Choice(values, None),
                Some(GroupTypeContentDeserializerState::Choice(_, Some(deserializer))) => {
                    GroupTypeContentDeserializerState::Choice(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(GroupTypeContentDeserializerState::Choice(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                GroupTypeContentDeserializer::store_choice(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                GroupTypeContentDeserializer::store_choice(&mut values, data)?;
                let data = GroupTypeContentDeserializer::finish_state(
                    reader,
                    GroupTypeContentDeserializerState::Choice(values, None),
                )?;
                *self.state = GroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = GroupTypeContentDeserializerState::Choice(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_sequence<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<GroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => GroupTypeContentDeserializerState::Sequence(values, None),
                Some(GroupTypeContentDeserializerState::Sequence(_, Some(deserializer))) => {
                    GroupTypeContentDeserializerState::Sequence(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(GroupTypeContentDeserializerState::Sequence(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                GroupTypeContentDeserializer::store_sequence(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                GroupTypeContentDeserializer::store_sequence(&mut values, data)?;
                let data = GroupTypeContentDeserializer::finish_state(
                    reader,
                    GroupTypeContentDeserializerState::Sequence(values, None),
                )?;
                *self.state = GroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    GroupTypeContentDeserializerState::Sequence(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_any<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Any>,
        output: DeserializerOutput<'de, super::Any>,
        fallback: &mut Option<GroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => GroupTypeContentDeserializerState::Any(values, None),
                Some(GroupTypeContentDeserializerState::Any(_, Some(deserializer))) => {
                    GroupTypeContentDeserializerState::Any(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(GroupTypeContentDeserializerState::Any(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                GroupTypeContentDeserializer::store_any(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                GroupTypeContentDeserializer::store_any(&mut values, data)?;
                let data = GroupTypeContentDeserializer::finish_state(
                    reader,
                    GroupTypeContentDeserializerState::Any(values, None),
                )?;
                *self.state = GroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = GroupTypeContentDeserializerState::Any(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::GroupTypeContent> for Box<GroupTypeContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::GroupTypeContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(GroupTypeContentDeserializer {
            state: Box::new(GroupTypeContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, GroupTypeContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::GroupTypeContent>
    where
        R: DeserializeReader,
    {
        use GroupTypeContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Element(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_element(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::All(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_all(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Choice(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_choice(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Sequence(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_sequence(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Any(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_any(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            GroupTypeContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Element(values, None), event) => {
                    let output = <super::ElementType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_element(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::All(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_all(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Choice(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_choice(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Sequence(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_sequence(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Any(values, None), event) => {
                    let output =
                        <super::Any as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_any(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::GroupTypeContent, Error>
    where
        R: DeserializeReader,
    {
        GroupTypeContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct AttributeGroupTypeDeserializer {
    id: Option<String>,
    name: Option<String>,
    ref_: Option<QName>,
    content: Vec<super::AttributeGroupTypeContent>,
    state: Box<AttributeGroupTypeDeserializerState>,
}
#[derive(Debug)]
enum AttributeGroupTypeDeserializerState {
    Init__,
    Next__,
    Content__(<super::AttributeGroupTypeContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl AttributeGroupTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;
        let mut ref_: Option<QName> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"ref")
            ) {
                reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            name: name,
            ref_: ref_,
            content: Vec::new(),
            state: Box::new(AttributeGroupTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: AttributeGroupTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let AttributeGroupTypeDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::AttributeGroupTypeContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::AttributeGroupTypeContent>,
        fallback: &mut Option<AttributeGroupTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(AttributeGroupTypeDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = AttributeGroupTypeDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = AttributeGroupTypeDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(AttributeGroupTypeDeserializerState::Content__(
                            deserializer,
                        ));
                        *self.state = AttributeGroupTypeDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::AttributeGroupType> for Box<AttributeGroupTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AttributeGroupType>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(
            event,
            AttributeGroupTypeDeserializer::from_bytes_start,
        )
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::AttributeGroupType>
    where
        R: DeserializeReader,
    {
        use AttributeGroupTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::AttributeGroupTypeContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::AttributeGroupType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(
            &mut *self.state,
            AttributeGroupTypeDeserializerState::Unknown__,
        );
        self.finish_state(reader, state)?;
        Ok(super::AttributeGroupType {
            id: self.id,
            name: self.name,
            ref_: self.ref_,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct AttributeGroupTypeContentDeserializer {
    state: Box<AttributeGroupTypeContentDeserializerState>,
}
#[derive(Debug)]
pub enum AttributeGroupTypeContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    Attribute(
        Option<super::AttributeType>,
        Option<<super::AttributeType as WithDeserializer>::Deserializer>,
    ),
    AttributeGroup(
        Option<super::AttributeGroupType>,
        Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
    ),
    AnyAttribute(
        Option<super::AnyAttribute>,
        Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
    ),
    Done__(super::AttributeGroupTypeContent),
    Unknown__,
}
impl AttributeGroupTypeContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<AttributeGroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"anyAttribute")
            ) {
                let output =
                    <super::AnyAttribute as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_any_attribute(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(AttributeGroupTypeContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: AttributeGroupTypeContentDeserializerState,
    ) -> Result<super::AttributeGroupTypeContent, Error>
    where
        R: DeserializeReader,
    {
        use AttributeGroupTypeContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AttributeGroupTypeContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::AttributeGroupTypeContent::Annotation(
                    values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                ))
            }
            S::Attribute(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AttributeGroupTypeContentDeserializer::store_attribute(&mut values, value)?;
                }
                Ok(super::AttributeGroupTypeContent::Attribute(
                    values.ok_or_else(|| ErrorKind::MissingElement("attribute".into()))?,
                ))
            }
            S::AttributeGroup(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AttributeGroupTypeContentDeserializer::store_attribute_group(
                        &mut values,
                        value,
                    )?;
                }
                Ok(super::AttributeGroupTypeContent::AttributeGroup(
                    values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                ))
            }
            S::AnyAttribute(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AttributeGroupTypeContentDeserializer::store_any_attribute(&mut values, value)?;
                }
                Ok(super::AttributeGroupTypeContent::AnyAttribute(
                    values.ok_or_else(|| ErrorKind::MissingElement("anyAttribute".into()))?,
                ))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute(
        values: &mut Option<super::AttributeType>,
        value: super::AttributeType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attribute",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute_group(
        values: &mut Option<super::AttributeGroupType>,
        value: super::AttributeGroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attributeGroup",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_any_attribute(
        values: &mut Option<super::AnyAttribute>,
        value: super::AnyAttribute,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"anyAttribute",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<AttributeGroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => AttributeGroupTypeContentDeserializerState::Annotation(values, None),
                Some(AttributeGroupTypeContentDeserializerState::Annotation(
                    _,
                    Some(deserializer),
                )) => AttributeGroupTypeContentDeserializerState::Annotation(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AttributeGroupTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AttributeGroupTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AttributeGroupTypeContentDeserializer::store_annotation(&mut values, data)?;
                let data = AttributeGroupTypeContentDeserializer::finish_state(
                    reader,
                    AttributeGroupTypeContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = AttributeGroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = AttributeGroupTypeContentDeserializerState::Annotation(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeType>,
        output: DeserializerOutput<'de, super::AttributeType>,
        fallback: &mut Option<AttributeGroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => AttributeGroupTypeContentDeserializerState::Attribute(values, None),
                Some(AttributeGroupTypeContentDeserializerState::Attribute(
                    _,
                    Some(deserializer),
                )) => AttributeGroupTypeContentDeserializerState::Attribute(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AttributeGroupTypeContentDeserializerState::Attribute(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AttributeGroupTypeContentDeserializer::store_attribute(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AttributeGroupTypeContentDeserializer::store_attribute(&mut values, data)?;
                let data = AttributeGroupTypeContentDeserializer::finish_state(
                    reader,
                    AttributeGroupTypeContentDeserializerState::Attribute(values, None),
                )?;
                *self.state = AttributeGroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = AttributeGroupTypeContentDeserializerState::Attribute(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeGroupType>,
        output: DeserializerOutput<'de, super::AttributeGroupType>,
        fallback: &mut Option<AttributeGroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => AttributeGroupTypeContentDeserializerState::AttributeGroup(values, None),
                Some(AttributeGroupTypeContentDeserializerState::AttributeGroup(
                    _,
                    Some(deserializer),
                )) => AttributeGroupTypeContentDeserializerState::AttributeGroup(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AttributeGroupTypeContentDeserializerState::AttributeGroup(
                _,
                Some(deserializer),
            )) => {
                let data = deserializer.finish(reader)?;
                AttributeGroupTypeContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AttributeGroupTypeContentDeserializer::store_attribute_group(&mut values, data)?;
                let data = AttributeGroupTypeContentDeserializer::finish_state(
                    reader,
                    AttributeGroupTypeContentDeserializerState::AttributeGroup(values, None),
                )?;
                *self.state = AttributeGroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = AttributeGroupTypeContentDeserializerState::AttributeGroup(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_any_attribute<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AnyAttribute>,
        output: DeserializerOutput<'de, super::AnyAttribute>,
        fallback: &mut Option<AttributeGroupTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => AttributeGroupTypeContentDeserializerState::AnyAttribute(values, None),
                Some(AttributeGroupTypeContentDeserializerState::AnyAttribute(
                    _,
                    Some(deserializer),
                )) => AttributeGroupTypeContentDeserializerState::AnyAttribute(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AttributeGroupTypeContentDeserializerState::AnyAttribute(
                _,
                Some(deserializer),
            )) => {
                let data = deserializer.finish(reader)?;
                AttributeGroupTypeContentDeserializer::store_any_attribute(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AttributeGroupTypeContentDeserializer::store_any_attribute(&mut values, data)?;
                let data = AttributeGroupTypeContentDeserializer::finish_state(
                    reader,
                    AttributeGroupTypeContentDeserializerState::AnyAttribute(values, None),
                )?;
                *self.state = AttributeGroupTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = AttributeGroupTypeContentDeserializerState::AnyAttribute(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::AttributeGroupTypeContent>
    for Box<AttributeGroupTypeContentDeserializer>
{
    fn init<R>(
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::AttributeGroupTypeContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(AttributeGroupTypeContentDeserializer {
            state: Box::new(AttributeGroupTypeContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(
                    &*x.state,
                    AttributeGroupTypeContentDeserializerState::Init__
                ) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::AttributeGroupTypeContent>
    where
        R: DeserializeReader,
    {
        use AttributeGroupTypeContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AnyAttribute(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            AttributeGroupTypeContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, None), event) => {
                    let output = <super::AttributeType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, None), event) => {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AnyAttribute(values, None), event) => {
                    let output = <super::AnyAttribute as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::AttributeGroupTypeContent, Error>
    where
        R: DeserializeReader,
    {
        AttributeGroupTypeContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct ElementTypeDeserializer {
    id: Option<String>,
    name: Option<String>,
    ref_: Option<QName>,
    type_: Option<QName>,
    substitution_group: Option<super::QNameList>,
    min_occurs: usize,
    max_occurs: MaxOccurs,
    default: Option<String>,
    fixed: Option<String>,
    nillable: Option<bool>,
    abstract_: bool,
    final_: Option<super::DerivationSetType>,
    block: Option<super::BlockSetType>,
    form: Option<super::FormChoiceType>,
    target_namespace: Option<String>,
    content: Vec<super::ElementTypeContent>,
    state: Box<ElementTypeDeserializerState>,
}
#[derive(Debug)]
enum ElementTypeDeserializerState {
    Init__,
    Next__,
    Content__(<super::ElementTypeContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl ElementTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;
        let mut ref_: Option<QName> = None;
        let mut type_: Option<QName> = None;
        let mut substitution_group: Option<super::QNameList> = None;
        let mut min_occurs: Option<usize> = None;
        let mut max_occurs: Option<MaxOccurs> = None;
        let mut default: Option<String> = None;
        let mut fixed: Option<String> = None;
        let mut nillable: Option<bool> = None;
        let mut abstract_: Option<bool> = None;
        let mut final_: Option<super::DerivationSetType> = None;
        let mut block: Option<super::BlockSetType> = None;
        let mut form: Option<super::FormChoiceType> = None;
        let mut target_namespace: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"ref")
            ) {
                reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"type")
            ) {
                reader.read_attrib(&mut type_, b"type", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"substitutionGroup")
            ) {
                reader.read_attrib(&mut substitution_group, b"substitutionGroup", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"minOccurs")
            ) {
                reader.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"maxOccurs")
            ) {
                reader.read_attrib(&mut max_occurs, b"maxOccurs", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"default")
            ) {
                reader.read_attrib(&mut default, b"default", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"fixed")
            ) {
                reader.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"nillable")
            ) {
                reader.read_attrib(&mut nillable, b"nillable", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"abstract")
            ) {
                reader.read_attrib(&mut abstract_, b"abstract", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"final")
            ) {
                reader.read_attrib(&mut final_, b"final", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"block")
            ) {
                reader.read_attrib(&mut block, b"block", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"form")
            ) {
                reader.read_attrib(&mut form, b"form", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"targetNamespace")
            ) {
                reader.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            name: name,
            ref_: ref_,
            type_: type_,
            substitution_group: substitution_group,
            min_occurs: min_occurs.unwrap_or_else(super::ElementType::default_min_occurs),
            max_occurs: max_occurs.unwrap_or_else(super::ElementType::default_max_occurs),
            default: default,
            fixed: fixed,
            nillable: nillable,
            abstract_: abstract_.unwrap_or_else(super::ElementType::default_abstract_),
            final_: final_,
            block: block,
            form: form,
            target_namespace: target_namespace,
            content: Vec::new(),
            state: Box::new(ElementTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: ElementTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let ElementTypeDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::ElementTypeContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::ElementTypeContent>,
        fallback: &mut Option<ElementTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(ElementTypeDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = ElementTypeDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = ElementTypeDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(ElementTypeDeserializerState::Content__(deserializer));
                        *self.state = ElementTypeDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::ElementType> for Box<ElementTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ElementType>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, ElementTypeDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ElementType>
    where
        R: DeserializeReader,
    {
        use ElementTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::ElementTypeContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::ElementType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, ElementTypeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::ElementType {
            id: self.id,
            name: self.name,
            ref_: self.ref_,
            type_: self.type_,
            substitution_group: self.substitution_group,
            min_occurs: self.min_occurs,
            max_occurs: self.max_occurs,
            default: self.default,
            fixed: self.fixed,
            nillable: self.nillable,
            abstract_: self.abstract_,
            final_: self.final_,
            block: self.block,
            form: self.form,
            target_namespace: self.target_namespace,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct ElementTypeContentDeserializer {
    state: Box<ElementTypeContentDeserializerState>,
}
#[derive(Debug)]
pub enum ElementTypeContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    SimpleType(
        Option<super::SimpleBaseType>,
        Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
    ),
    ComplexType(
        Option<super::ComplexBaseType>,
        Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
    ),
    Alternative(
        Option<super::AltType>,
        Option<<super::AltType as WithDeserializer>::Deserializer>,
    ),
    Unique(
        Option<super::KeybaseType>,
        Option<<super::KeybaseType as WithDeserializer>::Deserializer>,
    ),
    Key(
        Option<super::KeybaseType>,
        Option<<super::KeybaseType as WithDeserializer>::Deserializer>,
    ),
    Keyref(
        Option<super::Keyref>,
        Option<<super::Keyref as WithDeserializer>::Deserializer>,
    ),
    Done__(super::ElementTypeContent),
    Unknown__,
}
impl ElementTypeContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<ElementTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"complexType")
            ) {
                let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_complex_type(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"alternative")
            ) {
                let output =
                    <super::AltType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_alternative(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"unique")
            ) {
                let output =
                    <super::KeybaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_unique(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"key")
            ) {
                let output =
                    <super::KeybaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_key(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"keyref")
            ) {
                let output =
                    <super::Keyref as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_keyref(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(ElementTypeContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: ElementTypeContentDeserializerState,
    ) -> Result<super::ElementTypeContent, Error>
    where
        R: DeserializeReader,
    {
        use ElementTypeContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ElementTypeContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::ElementTypeContent::Annotation(values.ok_or_else(
                    || ErrorKind::MissingElement("annotation".into()),
                )?))
            }
            S::SimpleType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ElementTypeContentDeserializer::store_simple_type(&mut values, value)?;
                }
                Ok(super::ElementTypeContent::SimpleType(values.ok_or_else(
                    || ErrorKind::MissingElement("simpleType".into()),
                )?))
            }
            S::ComplexType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ElementTypeContentDeserializer::store_complex_type(&mut values, value)?;
                }
                Ok(super::ElementTypeContent::ComplexType(values.ok_or_else(
                    || ErrorKind::MissingElement("complexType".into()),
                )?))
            }
            S::Alternative(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ElementTypeContentDeserializer::store_alternative(&mut values, value)?;
                }
                Ok(super::ElementTypeContent::Alternative(values.ok_or_else(
                    || ErrorKind::MissingElement("alternative".into()),
                )?))
            }
            S::Unique(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ElementTypeContentDeserializer::store_unique(&mut values, value)?;
                }
                Ok(super::ElementTypeContent::Unique(values.ok_or_else(
                    || ErrorKind::MissingElement("unique".into()),
                )?))
            }
            S::Key(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ElementTypeContentDeserializer::store_key(&mut values, value)?;
                }
                Ok(super::ElementTypeContent::Key(
                    values.ok_or_else(|| ErrorKind::MissingElement("key".into()))?,
                ))
            }
            S::Keyref(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ElementTypeContentDeserializer::store_keyref(&mut values, value)?;
                }
                Ok(super::ElementTypeContent::Keyref(values.ok_or_else(
                    || ErrorKind::MissingElement("keyref".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_simple_type(
        values: &mut Option<super::SimpleBaseType>,
        value: super::SimpleBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"simpleType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_complex_type(
        values: &mut Option<super::ComplexBaseType>,
        value: super::ComplexBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"complexType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_alternative(
        values: &mut Option<super::AltType>,
        value: super::AltType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"alternative",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_unique(
        values: &mut Option<super::KeybaseType>,
        value: super::KeybaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"unique",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_key(
        values: &mut Option<super::KeybaseType>,
        value: super::KeybaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"key")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_keyref(values: &mut Option<super::Keyref>, value: super::Keyref) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"keyref",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<ElementTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ElementTypeContentDeserializerState::Annotation(values, None),
                Some(ElementTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    ElementTypeContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ElementTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ElementTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ElementTypeContentDeserializer::store_annotation(&mut values, data)?;
                let data = ElementTypeContentDeserializer::finish_state(
                    reader,
                    ElementTypeContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = ElementTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ElementTypeContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_simple_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::SimpleBaseType>,
        output: DeserializerOutput<'de, super::SimpleBaseType>,
        fallback: &mut Option<ElementTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ElementTypeContentDeserializerState::SimpleType(values, None),
                Some(ElementTypeContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                    ElementTypeContentDeserializerState::SimpleType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ElementTypeContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ElementTypeContentDeserializer::store_simple_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ElementTypeContentDeserializer::store_simple_type(&mut values, data)?;
                let data = ElementTypeContentDeserializer::finish_state(
                    reader,
                    ElementTypeContentDeserializerState::SimpleType(values, None),
                )?;
                *self.state = ElementTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ElementTypeContentDeserializerState::SimpleType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_complex_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ComplexBaseType>,
        output: DeserializerOutput<'de, super::ComplexBaseType>,
        fallback: &mut Option<ElementTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ElementTypeContentDeserializerState::ComplexType(values, None),
                Some(ElementTypeContentDeserializerState::ComplexType(_, Some(deserializer))) => {
                    ElementTypeContentDeserializerState::ComplexType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ElementTypeContentDeserializerState::ComplexType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ElementTypeContentDeserializer::store_complex_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ElementTypeContentDeserializer::store_complex_type(&mut values, data)?;
                let data = ElementTypeContentDeserializer::finish_state(
                    reader,
                    ElementTypeContentDeserializerState::ComplexType(values, None),
                )?;
                *self.state = ElementTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ElementTypeContentDeserializerState::ComplexType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_alternative<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AltType>,
        output: DeserializerOutput<'de, super::AltType>,
        fallback: &mut Option<ElementTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ElementTypeContentDeserializerState::Alternative(values, None),
                Some(ElementTypeContentDeserializerState::Alternative(_, Some(deserializer))) => {
                    ElementTypeContentDeserializerState::Alternative(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ElementTypeContentDeserializerState::Alternative(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ElementTypeContentDeserializer::store_alternative(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ElementTypeContentDeserializer::store_alternative(&mut values, data)?;
                let data = ElementTypeContentDeserializer::finish_state(
                    reader,
                    ElementTypeContentDeserializerState::Alternative(values, None),
                )?;
                *self.state = ElementTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ElementTypeContentDeserializerState::Alternative(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_unique<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::KeybaseType>,
        output: DeserializerOutput<'de, super::KeybaseType>,
        fallback: &mut Option<ElementTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ElementTypeContentDeserializerState::Unique(values, None),
                Some(ElementTypeContentDeserializerState::Unique(_, Some(deserializer))) => {
                    ElementTypeContentDeserializerState::Unique(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ElementTypeContentDeserializerState::Unique(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ElementTypeContentDeserializer::store_unique(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ElementTypeContentDeserializer::store_unique(&mut values, data)?;
                let data = ElementTypeContentDeserializer::finish_state(
                    reader,
                    ElementTypeContentDeserializerState::Unique(values, None),
                )?;
                *self.state = ElementTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ElementTypeContentDeserializerState::Unique(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_key<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::KeybaseType>,
        output: DeserializerOutput<'de, super::KeybaseType>,
        fallback: &mut Option<ElementTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ElementTypeContentDeserializerState::Key(values, None),
                Some(ElementTypeContentDeserializerState::Key(_, Some(deserializer))) => {
                    ElementTypeContentDeserializerState::Key(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ElementTypeContentDeserializerState::Key(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ElementTypeContentDeserializer::store_key(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ElementTypeContentDeserializer::store_key(&mut values, data)?;
                let data = ElementTypeContentDeserializer::finish_state(
                    reader,
                    ElementTypeContentDeserializerState::Key(values, None),
                )?;
                *self.state = ElementTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ElementTypeContentDeserializerState::Key(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_keyref<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Keyref>,
        output: DeserializerOutput<'de, super::Keyref>,
        fallback: &mut Option<ElementTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ElementTypeContentDeserializerState::Keyref(values, None),
                Some(ElementTypeContentDeserializerState::Keyref(_, Some(deserializer))) => {
                    ElementTypeContentDeserializerState::Keyref(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ElementTypeContentDeserializerState::Keyref(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ElementTypeContentDeserializer::store_keyref(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ElementTypeContentDeserializer::store_keyref(&mut values, data)?;
                let data = ElementTypeContentDeserializer::finish_state(
                    reader,
                    ElementTypeContentDeserializerState::Keyref(values, None),
                )?;
                *self.state = ElementTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ElementTypeContentDeserializerState::Keyref(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::ElementTypeContent> for Box<ElementTypeContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ElementTypeContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(ElementTypeContentDeserializer {
            state: Box::new(ElementTypeContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, ElementTypeContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ElementTypeContent>
    where
        R: DeserializeReader,
    {
        use ElementTypeContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_complex_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Alternative(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_alternative(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unique(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_unique(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Key(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_key(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Keyref(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_keyref(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            ElementTypeContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, None), event) => {
                    let output = <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexType(values, None), event) => {
                    let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_complex_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Alternative(values, None), event) => {
                    let output =
                        <super::AltType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_alternative(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unique(values, None), event) => {
                    let output = <super::KeybaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_unique(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Key(values, None), event) => {
                    let output = <super::KeybaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_key(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Keyref(values, None), event) => {
                    let output =
                        <super::Keyref as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_keyref(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::ElementTypeContent, Error>
    where
        R: DeserializeReader,
    {
        ElementTypeContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct AttributeTypeDeserializer {
    id: Option<String>,
    name: Option<String>,
    ref_: Option<QName>,
    type_: Option<QName>,
    use_: super::AttributeUseType,
    default: Option<String>,
    fixed: Option<String>,
    form: Option<super::FormChoiceType>,
    target_namespace: Option<String>,
    inheritable: Option<bool>,
    annotation: Option<super::Annotation>,
    simple_type: Option<super::SimpleBaseType>,
    state: Box<AttributeTypeDeserializerState>,
}
#[derive(Debug)]
enum AttributeTypeDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    SimpleType(Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl AttributeTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;
        let mut ref_: Option<QName> = None;
        let mut type_: Option<QName> = None;
        let mut use_: Option<super::AttributeUseType> = None;
        let mut default: Option<String> = None;
        let mut fixed: Option<String> = None;
        let mut form: Option<super::FormChoiceType> = None;
        let mut target_namespace: Option<String> = None;
        let mut inheritable: Option<bool> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"ref")
            ) {
                reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"type")
            ) {
                reader.read_attrib(&mut type_, b"type", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"use")
            ) {
                reader.read_attrib(&mut use_, b"use", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"default")
            ) {
                reader.read_attrib(&mut default, b"default", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"fixed")
            ) {
                reader.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"form")
            ) {
                reader.read_attrib(&mut form, b"form", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"targetNamespace")
            ) {
                reader.read_attrib(&mut target_namespace, b"targetNamespace", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"inheritable")
            ) {
                reader.read_attrib(&mut inheritable, b"inheritable", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            name: name,
            ref_: ref_,
            type_: type_,
            use_: use_.unwrap_or_else(super::AttributeType::default_use_),
            default: default,
            fixed: fixed,
            form: form,
            target_namespace: target_namespace,
            inheritable: inheritable,
            annotation: None,
            simple_type: None,
            state: Box::new(AttributeTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: AttributeTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use AttributeTypeDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            S::SimpleType(Some(deserializer)) => {
                self.store_simple_type(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn store_simple_type(&mut self, value: super::SimpleBaseType) -> Result<(), Error> {
        if self.simple_type.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"simpleType",
            )))?;
        }
        self.simple_type = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<AttributeTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(AttributeTypeDeserializerState::Annotation(None));
            *self.state = AttributeTypeDeserializerState::SimpleType(None);
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = AttributeTypeDeserializerState::SimpleType(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(AttributeTypeDeserializerState::Annotation(Some(
                            deserializer,
                        )));
                        *self.state = AttributeTypeDeserializerState::SimpleType(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state =
                            AttributeTypeDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
    fn handle_simple_type<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::SimpleBaseType>,
        fallback: &mut Option<AttributeTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(AttributeTypeDeserializerState::SimpleType(None));
            *self.state = AttributeTypeDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_simple_type(data)?;
                *self.state = AttributeTypeDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(AttributeTypeDeserializerState::SimpleType(Some(
                            deserializer,
                        )));
                        *self.state = AttributeTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state =
                            AttributeTypeDeserializerState::SimpleType(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::AttributeType> for Box<AttributeTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AttributeType>
    where
        R: DeserializeReader,
    {
        reader
            .init_deserializer_from_start_event(event, AttributeTypeDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::AttributeType>
    where
        R: DeserializeReader,
    {
        use AttributeTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (S::SimpleType(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_type(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = AttributeTypeDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::SimpleType(None);
                        event
                    }
                }
                (S::SimpleType(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"simpleType",
                    ) {
                        let output =
                            <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_simple_type(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        allow_any_element = true;
                        fallback.get_or_insert(S::SimpleType(None));
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::AttributeType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, AttributeTypeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::AttributeType {
            id: self.id,
            name: self.name,
            ref_: self.ref_,
            type_: self.type_,
            use_: self.use_,
            default: self.default,
            fixed: self.fixed,
            form: self.form,
            target_namespace: self.target_namespace,
            inheritable: self.inheritable,
            annotation: self.annotation,
            simple_type: self.simple_type,
        })
    }
}
#[derive(Debug)]
pub struct NotationDeserializer {
    id: Option<String>,
    name: String,
    public: Option<String>,
    system: Option<String>,
    annotation: Option<super::Annotation>,
    state: Box<NotationDeserializerState>,
}
#[derive(Debug)]
enum NotationDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl NotationDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;
        let mut public: Option<String> = None;
        let mut system: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"public")
            ) {
                reader.read_attrib(&mut public, b"public", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"system")
            ) {
                reader.read_attrib(&mut system, b"system", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            name: name
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
            public: public,
            system: system,
            annotation: None,
            state: Box::new(NotationDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: NotationDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use NotationDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<NotationDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(NotationDeserializerState::Annotation(None));
            *self.state = NotationDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = NotationDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(NotationDeserializerState::Annotation(Some(
                            deserializer,
                        )));
                        *self.state = NotationDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = NotationDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Notation> for Box<NotationDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Notation>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, NotationDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Notation>
    where
        R: DeserializeReader,
    {
        use NotationDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = NotationDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Notation, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, NotationDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Notation {
            id: self.id,
            name: self.name,
            public: self.public,
            system: self.system,
            annotation: self.annotation,
        })
    }
}
#[derive(Debug)]
pub struct WildcardTypeDeserializer {
    id: Option<String>,
    namespace: Option<super::NamespaceListType>,
    not_namespace: Option<super::BasicNamespaceListType>,
    process_contents: super::ProcessContentsType,
    annotation: Option<super::Annotation>,
    state: Box<WildcardTypeDeserializerState>,
}
#[derive(Debug)]
enum WildcardTypeDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl WildcardTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut namespace: Option<super::NamespaceListType> = None;
        let mut not_namespace: Option<super::BasicNamespaceListType> = None;
        let mut process_contents: Option<super::ProcessContentsType> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"namespace")
            ) {
                reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"notNamespace")
            ) {
                reader.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"processContents")
            ) {
                reader.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            namespace: namespace,
            not_namespace: not_namespace,
            process_contents: process_contents
                .unwrap_or_else(super::WildcardType::default_process_contents),
            annotation: None,
            state: Box::new(WildcardTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: WildcardTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use WildcardTypeDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<WildcardTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(WildcardTypeDeserializerState::Annotation(None));
            *self.state = WildcardTypeDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = WildcardTypeDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(WildcardTypeDeserializerState::Annotation(Some(
                            deserializer,
                        )));
                        *self.state = WildcardTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = WildcardTypeDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::WildcardType> for Box<WildcardTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::WildcardType>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, WildcardTypeDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::WildcardType>
    where
        R: DeserializeReader,
    {
        use WildcardTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = WildcardTypeDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::WildcardType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, WildcardTypeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::WildcardType {
            id: self.id,
            namespace: self.namespace,
            not_namespace: self.not_namespace,
            process_contents: self.process_contents,
            annotation: self.annotation,
        })
    }
}
#[derive(Debug)]
pub struct RestrictionDeserializer {
    id: Option<String>,
    base: Option<QName>,
    content: Vec<super::RestrictionContent>,
    state: Box<RestrictionDeserializerState>,
}
#[derive(Debug)]
enum RestrictionDeserializerState {
    Init__,
    Next__,
    Content__(<super::RestrictionContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl RestrictionDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut base: Option<QName> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"base")
            ) {
                reader.read_attrib(&mut base, b"base", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            base: base,
            content: Vec::new(),
            state: Box::new(RestrictionDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: RestrictionDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let RestrictionDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::RestrictionContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::RestrictionContent>,
        fallback: &mut Option<RestrictionDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(RestrictionDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = RestrictionDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = RestrictionDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(RestrictionDeserializerState::Content__(deserializer));
                        *self.state = RestrictionDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Restriction> for Box<RestrictionDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Restriction>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, RestrictionDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::Restriction>
    where
        R: DeserializeReader,
    {
        use RestrictionDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::RestrictionContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Restriction, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, RestrictionDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Restriction {
            id: self.id,
            base: self.base,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct RestrictionContentDeserializer {
    state: Box<RestrictionContentDeserializerState>,
}
#[derive(Debug)]
pub enum RestrictionContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    SimpleType(
        Option<super::SimpleBaseType>,
        Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
    ),
    Facet(
        Option<super::Facet>,
        Option<<super::Facet as WithDeserializer>::Deserializer>,
    ),
    Done__(super::RestrictionContent),
    Unknown__,
}
impl RestrictionContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<RestrictionContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let mut event = event;
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            event = {
                let output = <super::Facet as WithDeserializer>::Deserializer::init(reader, event)?;
                match self.handle_facet(reader, Default::default(), output, &mut *fallback)? {
                    ElementHandlerOutput::Continue { event, .. } => event,
                    output => {
                        return Ok(output);
                    }
                }
            };
        }
        *self.state = fallback
            .take()
            .unwrap_or(RestrictionContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: RestrictionContentDeserializerState,
    ) -> Result<super::RestrictionContent, Error>
    where
        R: DeserializeReader,
    {
        use RestrictionContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::RestrictionContent::Annotation(values.ok_or_else(
                    || ErrorKind::MissingElement("annotation".into()),
                )?))
            }
            S::SimpleType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionContentDeserializer::store_simple_type(&mut values, value)?;
                }
                Ok(super::RestrictionContent::SimpleType(values.ok_or_else(
                    || ErrorKind::MissingElement("simpleType".into()),
                )?))
            }
            S::Facet(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionContentDeserializer::store_facet(&mut values, value)?;
                }
                Ok(super::RestrictionContent::Facet(values.ok_or_else(
                    || ErrorKind::MissingElement("facet".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_simple_type(
        values: &mut Option<super::SimpleBaseType>,
        value: super::SimpleBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"simpleType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_facet(values: &mut Option<super::Facet>, value: super::Facet) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"facet",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<RestrictionContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionContentDeserializerState::Annotation(values, None),
                Some(RestrictionContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    RestrictionContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionContentDeserializer::store_annotation(&mut values, data)?;
                let data = RestrictionContentDeserializer::finish_state(
                    reader,
                    RestrictionContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = RestrictionContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_simple_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::SimpleBaseType>,
        output: DeserializerOutput<'de, super::SimpleBaseType>,
        fallback: &mut Option<RestrictionContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionContentDeserializerState::SimpleType(values, None),
                Some(RestrictionContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                    RestrictionContentDeserializerState::SimpleType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionContentDeserializer::store_simple_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionContentDeserializer::store_simple_type(&mut values, data)?;
                let data = RestrictionContentDeserializer::finish_state(
                    reader,
                    RestrictionContentDeserializerState::SimpleType(values, None),
                )?;
                *self.state = RestrictionContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionContentDeserializerState::SimpleType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_facet<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Facet>,
        output: DeserializerOutput<'de, super::Facet>,
        fallback: &mut Option<RestrictionContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionContentDeserializerState::Facet(values, None),
                Some(RestrictionContentDeserializerState::Facet(_, Some(deserializer))) => {
                    RestrictionContentDeserializerState::Facet(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionContentDeserializerState::Facet(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionContentDeserializer::store_facet(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionContentDeserializer::store_facet(&mut values, data)?;
                let data = RestrictionContentDeserializer::finish_state(
                    reader,
                    RestrictionContentDeserializerState::Facet(values, None),
                )?;
                *self.state = RestrictionContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionContentDeserializerState::Facet(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::RestrictionContent> for Box<RestrictionContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RestrictionContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(RestrictionContentDeserializer {
            state: Box::new(RestrictionContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, RestrictionContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::RestrictionContent>
    where
        R: DeserializeReader,
    {
        use RestrictionContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Facet(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_facet(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            RestrictionContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, None), event) => {
                    let output = <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Facet(values, None), event) => {
                    let output =
                        <super::Facet as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_facet(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::RestrictionContent, Error>
    where
        R: DeserializeReader,
    {
        RestrictionContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct ListDeserializer {
    id: Option<String>,
    item_type: Option<QName>,
    annotation: Option<super::Annotation>,
    simple_type: Option<super::SimpleBaseType>,
    state: Box<ListDeserializerState>,
}
#[derive(Debug)]
enum ListDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    SimpleType(Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl ListDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut item_type: Option<QName> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"itemType")
            ) {
                reader.read_attrib(&mut item_type, b"itemType", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            item_type: item_type,
            annotation: None,
            simple_type: None,
            state: Box::new(ListDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: ListDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use ListDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            S::SimpleType(Some(deserializer)) => {
                self.store_simple_type(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn store_simple_type(&mut self, value: super::SimpleBaseType) -> Result<(), Error> {
        if self.simple_type.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"simpleType",
            )))?;
        }
        self.simple_type = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<ListDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(ListDeserializerState::Annotation(None));
            *self.state = ListDeserializerState::SimpleType(None);
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = ListDeserializerState::SimpleType(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(ListDeserializerState::Annotation(Some(deserializer)));
                        *self.state = ListDeserializerState::SimpleType(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = ListDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
    fn handle_simple_type<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::SimpleBaseType>,
        fallback: &mut Option<ListDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(ListDeserializerState::SimpleType(None));
            *self.state = ListDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_simple_type(data)?;
                *self.state = ListDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(ListDeserializerState::SimpleType(Some(deserializer)));
                        *self.state = ListDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = ListDeserializerState::SimpleType(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::List> for Box<ListDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::List>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, ListDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::List>
    where
        R: DeserializeReader,
    {
        use ListDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (S::SimpleType(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_type(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = ListDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::SimpleType(None);
                        event
                    }
                }
                (S::SimpleType(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"simpleType",
                    ) {
                        let output =
                            <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_simple_type(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        allow_any_element = true;
                        fallback.get_or_insert(S::SimpleType(None));
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::List, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, ListDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::List {
            id: self.id,
            item_type: self.item_type,
            annotation: self.annotation,
            simple_type: self.simple_type,
        })
    }
}
#[derive(Debug)]
pub struct UnionDeserializer {
    id: Option<String>,
    member_types: Option<super::QNameList>,
    annotation: Option<super::Annotation>,
    simple_type: Vec<super::SimpleBaseType>,
    state: Box<UnionDeserializerState>,
}
#[derive(Debug)]
enum UnionDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    SimpleType(Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl UnionDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut member_types: Option<super::QNameList> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"memberTypes")
            ) {
                reader.read_attrib(&mut member_types, b"memberTypes", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            member_types: member_types,
            annotation: None,
            simple_type: Vec::new(),
            state: Box::new(UnionDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: UnionDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use UnionDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            S::SimpleType(Some(deserializer)) => {
                self.store_simple_type(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn store_simple_type(&mut self, value: super::SimpleBaseType) -> Result<(), Error> {
        self.simple_type.push(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<UnionDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(UnionDeserializerState::Annotation(None));
            *self.state = UnionDeserializerState::SimpleType(None);
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = UnionDeserializerState::SimpleType(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(UnionDeserializerState::Annotation(Some(deserializer)));
                        *self.state = UnionDeserializerState::SimpleType(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = UnionDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
    fn handle_simple_type<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::SimpleBaseType>,
        fallback: &mut Option<UnionDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(UnionDeserializerState::SimpleType(None));
            *self.state = UnionDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_simple_type(data)?;
                *self.state = UnionDeserializerState::SimpleType(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(UnionDeserializerState::SimpleType(Some(deserializer)));
                        *self.state = UnionDeserializerState::SimpleType(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = UnionDeserializerState::SimpleType(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Union> for Box<UnionDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Union>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, UnionDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Union>
    where
        R: DeserializeReader,
    {
        use UnionDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (S::SimpleType(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_type(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = UnionDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::SimpleType(None);
                        event
                    }
                }
                (S::SimpleType(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"simpleType",
                    ) {
                        let output =
                            <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_simple_type(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        allow_any_element = true;
                        fallback.get_or_insert(S::SimpleType(None));
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Union, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, UnionDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Union {
            id: self.id,
            member_types: self.member_types,
            annotation: self.annotation,
            simple_type: self.simple_type,
        })
    }
}
#[derive(Debug)]
pub struct SimpleContentDeserializer {
    id: Option<String>,
    content: Vec<super::SimpleContentContent>,
    state: Box<SimpleContentDeserializerState>,
}
#[derive(Debug)]
enum SimpleContentDeserializerState {
    Init__,
    Next__,
    Content__(<super::SimpleContentContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl SimpleContentDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            content: Vec::new(),
            state: Box::new(SimpleContentDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: SimpleContentDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let SimpleContentDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::SimpleContentContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::SimpleContentContent>,
        fallback: &mut Option<SimpleContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(SimpleContentDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = SimpleContentDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let can_have_more = self.content.len().saturating_add(1) < 2usize;
                let ret = if can_have_more {
                    ElementHandlerOutput::from_event(event, allow_any)
                } else {
                    ElementHandlerOutput::from_event_end(event, allow_any)
                };
                match (can_have_more, &ret) {
                    (true, ElementHandlerOutput::Continue { .. }) => {
                        fallback
                            .get_or_insert(SimpleContentDeserializerState::Content__(deserializer));
                        *self.state = SimpleContentDeserializerState::Next__;
                    }
                    (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                        *self.state = SimpleContentDeserializerState::Content__(deserializer);
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::SimpleContent> for Box<SimpleContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::SimpleContent>
    where
        R: DeserializeReader,
    {
        reader
            .init_deserializer_from_start_event(event, SimpleContentDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::SimpleContent>
    where
        R: DeserializeReader,
    {
        use SimpleContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::SimpleContentContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::SimpleContent, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, SimpleContentDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::SimpleContent {
            id: self.id,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct SimpleContentContentDeserializer {
    state: Box<SimpleContentContentDeserializerState>,
}
#[derive(Debug)]
pub enum SimpleContentContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    Restriction(
        Option<super::RestrictionType>,
        Option<<super::RestrictionType as WithDeserializer>::Deserializer>,
    ),
    Extension(
        Option<super::ExtensionType>,
        Option<<super::ExtensionType as WithDeserializer>::Deserializer>,
    ),
    Done__(super::SimpleContentContent),
    Unknown__,
}
impl SimpleContentContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<SimpleContentContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"restriction")
            ) {
                let output = <super::RestrictionType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_restriction(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"extension")
            ) {
                let output =
                    <super::ExtensionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_extension(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(SimpleContentContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: SimpleContentContentDeserializerState,
    ) -> Result<super::SimpleContentContent, Error>
    where
        R: DeserializeReader,
    {
        use SimpleContentContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SimpleContentContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::SimpleContentContent::Annotation(values.ok_or_else(
                    || ErrorKind::MissingElement("annotation".into()),
                )?))
            }
            S::Restriction(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SimpleContentContentDeserializer::store_restriction(&mut values, value)?;
                }
                Ok(super::SimpleContentContent::Restriction(
                    values.ok_or_else(|| ErrorKind::MissingElement("restriction".into()))?,
                ))
            }
            S::Extension(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SimpleContentContentDeserializer::store_extension(&mut values, value)?;
                }
                Ok(super::SimpleContentContent::Extension(values.ok_or_else(
                    || ErrorKind::MissingElement("extension".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_restriction(
        values: &mut Option<super::RestrictionType>,
        value: super::RestrictionType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"restriction",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_extension(
        values: &mut Option<super::ExtensionType>,
        value: super::ExtensionType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"extension",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<SimpleContentContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SimpleContentContentDeserializerState::Annotation(values, None),
                Some(SimpleContentContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    SimpleContentContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SimpleContentContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SimpleContentContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SimpleContentContentDeserializer::store_annotation(&mut values, data)?;
                let data = SimpleContentContentDeserializer::finish_state(
                    reader,
                    SimpleContentContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = SimpleContentContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SimpleContentContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_restriction<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::RestrictionType>,
        output: DeserializerOutput<'de, super::RestrictionType>,
        fallback: &mut Option<SimpleContentContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SimpleContentContentDeserializerState::Restriction(values, None),
                Some(SimpleContentContentDeserializerState::Restriction(_, Some(deserializer))) => {
                    SimpleContentContentDeserializerState::Restriction(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SimpleContentContentDeserializerState::Restriction(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SimpleContentContentDeserializer::store_restriction(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SimpleContentContentDeserializer::store_restriction(&mut values, data)?;
                let data = SimpleContentContentDeserializer::finish_state(
                    reader,
                    SimpleContentContentDeserializerState::Restriction(values, None),
                )?;
                *self.state = SimpleContentContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SimpleContentContentDeserializerState::Restriction(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_extension<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ExtensionType>,
        output: DeserializerOutput<'de, super::ExtensionType>,
        fallback: &mut Option<SimpleContentContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => SimpleContentContentDeserializerState::Extension(values, None),
                Some(SimpleContentContentDeserializerState::Extension(_, Some(deserializer))) => {
                    SimpleContentContentDeserializerState::Extension(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SimpleContentContentDeserializerState::Extension(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SimpleContentContentDeserializer::store_extension(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SimpleContentContentDeserializer::store_extension(&mut values, data)?;
                let data = SimpleContentContentDeserializer::finish_state(
                    reader,
                    SimpleContentContentDeserializerState::Extension(values, None),
                )?;
                *self.state = SimpleContentContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SimpleContentContentDeserializerState::Extension(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::SimpleContentContent> for Box<SimpleContentContentDeserializer> {
    fn init<R>(
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::SimpleContentContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(SimpleContentContentDeserializer {
            state: Box::new(SimpleContentContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, SimpleContentContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::SimpleContentContent>
    where
        R: DeserializeReader,
    {
        use SimpleContentContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Restriction(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_restriction(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Extension(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_extension(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            SimpleContentContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Restriction(values, None), event) => {
                    let output = <super::RestrictionType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_restriction(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Extension(values, None), event) => {
                    let output = <super::ExtensionType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_extension(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::SimpleContentContent, Error>
    where
        R: DeserializeReader,
    {
        SimpleContentContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct ComplexContentDeserializer {
    id: Option<String>,
    mixed: Option<bool>,
    content: Vec<super::ComplexContentContent>,
    state: Box<ComplexContentDeserializerState>,
}
#[derive(Debug)]
enum ComplexContentDeserializerState {
    Init__,
    Next__,
    Content__(<super::ComplexContentContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl ComplexContentDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut mixed: Option<bool> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"mixed")
            ) {
                reader.read_attrib(&mut mixed, b"mixed", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            mixed: mixed,
            content: Vec::new(),
            state: Box::new(ComplexContentDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: ComplexContentDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let ComplexContentDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::ComplexContentContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::ComplexContentContent>,
        fallback: &mut Option<ComplexContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(ComplexContentDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = ComplexContentDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let can_have_more = self.content.len().saturating_add(1) < 2usize;
                let ret = if can_have_more {
                    ElementHandlerOutput::from_event(event, allow_any)
                } else {
                    ElementHandlerOutput::from_event_end(event, allow_any)
                };
                match (can_have_more, &ret) {
                    (true, ElementHandlerOutput::Continue { .. }) => {
                        fallback.get_or_insert(ComplexContentDeserializerState::Content__(
                            deserializer,
                        ));
                        *self.state = ComplexContentDeserializerState::Next__;
                    }
                    (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                        *self.state = ComplexContentDeserializerState::Content__(deserializer);
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::ComplexContent> for Box<ComplexContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ComplexContent>
    where
        R: DeserializeReader,
    {
        reader
            .init_deserializer_from_start_event(event, ComplexContentDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ComplexContent>
    where
        R: DeserializeReader,
    {
        use ComplexContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::ComplexContentContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::ComplexContent, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, ComplexContentDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::ComplexContent {
            id: self.id,
            mixed: self.mixed,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct ComplexContentContentDeserializer {
    state: Box<ComplexContentContentDeserializerState>,
}
#[derive(Debug)]
pub enum ComplexContentContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    Restriction(
        Option<super::RestrictionType>,
        Option<<super::RestrictionType as WithDeserializer>::Deserializer>,
    ),
    Extension(
        Option<super::ExtensionType>,
        Option<<super::ExtensionType as WithDeserializer>::Deserializer>,
    ),
    Done__(super::ComplexContentContent),
    Unknown__,
}
impl ComplexContentContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<ComplexContentContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"restriction")
            ) {
                let output = <super::RestrictionType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_restriction(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"extension")
            ) {
                let output =
                    <super::ExtensionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_extension(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(ComplexContentContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: ComplexContentContentDeserializerState,
    ) -> Result<super::ComplexContentContent, Error>
    where
        R: DeserializeReader,
    {
        use ComplexContentContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexContentContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::ComplexContentContent::Annotation(
                    values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                ))
            }
            S::Restriction(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexContentContentDeserializer::store_restriction(&mut values, value)?;
                }
                Ok(super::ComplexContentContent::Restriction(
                    values.ok_or_else(|| ErrorKind::MissingElement("restriction".into()))?,
                ))
            }
            S::Extension(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ComplexContentContentDeserializer::store_extension(&mut values, value)?;
                }
                Ok(super::ComplexContentContent::Extension(values.ok_or_else(
                    || ErrorKind::MissingElement("extension".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_restriction(
        values: &mut Option<super::RestrictionType>,
        value: super::RestrictionType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"restriction",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_extension(
        values: &mut Option<super::ExtensionType>,
        value: super::ExtensionType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"extension",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<ComplexContentContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexContentContentDeserializerState::Annotation(values, None),
                Some(ComplexContentContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    ComplexContentContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexContentContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexContentContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexContentContentDeserializer::store_annotation(&mut values, data)?;
                let data = ComplexContentContentDeserializer::finish_state(
                    reader,
                    ComplexContentContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = ComplexContentContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ComplexContentContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_restriction<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::RestrictionType>,
        output: DeserializerOutput<'de, super::RestrictionType>,
        fallback: &mut Option<ComplexContentContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexContentContentDeserializerState::Restriction(values, None),
                Some(ComplexContentContentDeserializerState::Restriction(
                    _,
                    Some(deserializer),
                )) => {
                    ComplexContentContentDeserializerState::Restriction(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexContentContentDeserializerState::Restriction(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexContentContentDeserializer::store_restriction(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexContentContentDeserializer::store_restriction(&mut values, data)?;
                let data = ComplexContentContentDeserializer::finish_state(
                    reader,
                    ComplexContentContentDeserializerState::Restriction(values, None),
                )?;
                *self.state = ComplexContentContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ComplexContentContentDeserializerState::Restriction(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_extension<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ExtensionType>,
        output: DeserializerOutput<'de, super::ExtensionType>,
        fallback: &mut Option<ComplexContentContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ComplexContentContentDeserializerState::Extension(values, None),
                Some(ComplexContentContentDeserializerState::Extension(_, Some(deserializer))) => {
                    ComplexContentContentDeserializerState::Extension(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ComplexContentContentDeserializerState::Extension(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ComplexContentContentDeserializer::store_extension(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ComplexContentContentDeserializer::store_extension(&mut values, data)?;
                let data = ComplexContentContentDeserializer::finish_state(
                    reader,
                    ComplexContentContentDeserializerState::Extension(values, None),
                )?;
                *self.state = ComplexContentContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ComplexContentContentDeserializerState::Extension(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::ComplexContentContent>
    for Box<ComplexContentContentDeserializer>
{
    fn init<R>(
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ComplexContentContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(ComplexContentContentDeserializer {
            state: Box::new(ComplexContentContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, ComplexContentContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ComplexContentContent>
    where
        R: DeserializeReader,
    {
        use ComplexContentContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Restriction(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_restriction(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Extension(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_extension(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            ComplexContentContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Restriction(values, None), event) => {
                    let output = <super::RestrictionType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_restriction(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Extension(values, None), event) => {
                    let output = <super::ExtensionType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_extension(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::ComplexContentContent, Error>
    where
        R: DeserializeReader,
    {
        ComplexContentContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct OpenContentDeserializer {
    id: Option<String>,
    mode: super::OpenContentModeType,
    annotation: Option<super::Annotation>,
    any: Option<super::WildcardType>,
    state: Box<OpenContentDeserializerState>,
}
#[derive(Debug)]
enum OpenContentDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Any(Option<<super::WildcardType as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl OpenContentDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut mode: Option<super::OpenContentModeType> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"mode")
            ) {
                reader.read_attrib(&mut mode, b"mode", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            mode: mode.unwrap_or_else(super::OpenContent::default_mode),
            annotation: None,
            any: None,
            state: Box::new(OpenContentDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: OpenContentDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use OpenContentDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            S::Any(Some(deserializer)) => self.store_any(deserializer.finish(reader)?)?,
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn store_any(&mut self, value: super::WildcardType) -> Result<(), Error> {
        if self.any.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"any")))?;
        }
        self.any = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<OpenContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(OpenContentDeserializerState::Annotation(None));
            *self.state = OpenContentDeserializerState::Any(None);
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = OpenContentDeserializerState::Any(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(OpenContentDeserializerState::Annotation(Some(
                            deserializer,
                        )));
                        *self.state = OpenContentDeserializerState::Any(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = OpenContentDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
    fn handle_any<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::WildcardType>,
        fallback: &mut Option<OpenContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(OpenContentDeserializerState::Any(None));
            *self.state = OpenContentDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_any(data)?;
                *self.state = OpenContentDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(OpenContentDeserializerState::Any(Some(deserializer)));
                        *self.state = OpenContentDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = OpenContentDeserializerState::Any(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::OpenContent> for Box<OpenContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::OpenContent>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, OpenContentDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::OpenContent>
    where
        R: DeserializeReader,
    {
        use OpenContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (S::Any(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_any(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = OpenContentDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Any(None);
                        event
                    }
                }
                (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(&event, Some(&super::super::NS_XS), b"any") {
                        let output = <super::WildcardType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_any(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::OpenContent, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, OpenContentDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::OpenContent {
            id: self.id,
            mode: self.mode,
            annotation: self.annotation,
            any: self.any,
        })
    }
}
#[derive(Debug)]
pub struct AnyAttributeDeserializer {
    id: Option<String>,
    namespace: Option<super::NamespaceListType>,
    not_namespace: Option<super::BasicNamespaceListType>,
    process_contents: super::ProcessContentsType,
    not_q_name: Option<super::QnameListAType>,
    annotation: Option<super::Annotation>,
    state: Box<AnyAttributeDeserializerState>,
}
#[derive(Debug)]
enum AnyAttributeDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl AnyAttributeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut namespace: Option<super::NamespaceListType> = None;
        let mut not_namespace: Option<super::BasicNamespaceListType> = None;
        let mut process_contents: Option<super::ProcessContentsType> = None;
        let mut not_q_name: Option<super::QnameListAType> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"namespace")
            ) {
                reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"notNamespace")
            ) {
                reader.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"processContents")
            ) {
                reader.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"notQName")
            ) {
                reader.read_attrib(&mut not_q_name, b"notQName", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            namespace: namespace,
            not_namespace: not_namespace,
            process_contents: process_contents
                .unwrap_or_else(super::AnyAttribute::default_process_contents),
            not_q_name: not_q_name,
            annotation: None,
            state: Box::new(AnyAttributeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: AnyAttributeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use AnyAttributeDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<AnyAttributeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(AnyAttributeDeserializerState::Annotation(None));
            *self.state = AnyAttributeDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = AnyAttributeDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(AnyAttributeDeserializerState::Annotation(Some(
                            deserializer,
                        )));
                        *self.state = AnyAttributeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = AnyAttributeDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::AnyAttribute> for Box<AnyAttributeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AnyAttribute>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, AnyAttributeDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::AnyAttribute>
    where
        R: DeserializeReader,
    {
        use AnyAttributeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = AnyAttributeDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::AnyAttribute, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, AnyAttributeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::AnyAttribute {
            id: self.id,
            namespace: self.namespace,
            not_namespace: self.not_namespace,
            process_contents: self.process_contents,
            not_q_name: self.not_q_name,
            annotation: self.annotation,
        })
    }
}
#[derive(Debug)]
pub struct AssertionTypeDeserializer {
    id: Option<String>,
    test: Option<String>,
    xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
    annotation: Option<super::Annotation>,
    state: Box<AssertionTypeDeserializerState>,
}
#[derive(Debug)]
enum AssertionTypeDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl AssertionTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut test: Option<String> = None;
        let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"test")
            ) {
                reader.read_attrib(&mut test, b"test", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"xpathDefaultNamespace")
            ) {
                reader.read_attrib(
                    &mut xpath_default_namespace,
                    b"xpathDefaultNamespace",
                    &attrib.value,
                )?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            test: test,
            xpath_default_namespace: xpath_default_namespace,
            annotation: None,
            state: Box::new(AssertionTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: AssertionTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use AssertionTypeDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<AssertionTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(AssertionTypeDeserializerState::Annotation(None));
            *self.state = AssertionTypeDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = AssertionTypeDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(AssertionTypeDeserializerState::Annotation(Some(
                            deserializer,
                        )));
                        *self.state = AssertionTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state =
                            AssertionTypeDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::AssertionType> for Box<AssertionTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AssertionType>
    where
        R: DeserializeReader,
    {
        reader
            .init_deserializer_from_start_event(event, AssertionTypeDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::AssertionType>
    where
        R: DeserializeReader,
    {
        use AssertionTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = AssertionTypeDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::AssertionType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, AssertionTypeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::AssertionType {
            id: self.id,
            test: self.test,
            xpath_default_namespace: self.xpath_default_namespace,
            annotation: self.annotation,
        })
    }
}
#[derive(Debug)]
pub struct AnyDeserializer {
    id: Option<String>,
    namespace: Option<super::NamespaceListType>,
    not_namespace: Option<super::BasicNamespaceListType>,
    process_contents: super::ProcessContentsType,
    not_q_name: Option<super::QnameListType>,
    min_occurs: usize,
    max_occurs: MaxOccurs,
    annotation: Option<super::Annotation>,
    state: Box<AnyDeserializerState>,
}
#[derive(Debug)]
enum AnyDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl AnyDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut namespace: Option<super::NamespaceListType> = None;
        let mut not_namespace: Option<super::BasicNamespaceListType> = None;
        let mut process_contents: Option<super::ProcessContentsType> = None;
        let mut not_q_name: Option<super::QnameListType> = None;
        let mut min_occurs: Option<usize> = None;
        let mut max_occurs: Option<MaxOccurs> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"namespace")
            ) {
                reader.read_attrib(&mut namespace, b"namespace", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"notNamespace")
            ) {
                reader.read_attrib(&mut not_namespace, b"notNamespace", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"processContents")
            ) {
                reader.read_attrib(&mut process_contents, b"processContents", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"notQName")
            ) {
                reader.read_attrib(&mut not_q_name, b"notQName", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"minOccurs")
            ) {
                reader.read_attrib(&mut min_occurs, b"minOccurs", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"maxOccurs")
            ) {
                reader.read_attrib(&mut max_occurs, b"maxOccurs", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            namespace: namespace,
            not_namespace: not_namespace,
            process_contents: process_contents.unwrap_or_else(super::Any::default_process_contents),
            not_q_name: not_q_name,
            min_occurs: min_occurs.unwrap_or_else(super::Any::default_min_occurs),
            max_occurs: max_occurs.unwrap_or_else(super::Any::default_max_occurs),
            annotation: None,
            state: Box::new(AnyDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: AnyDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use AnyDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<AnyDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(AnyDeserializerState::Annotation(None));
            *self.state = AnyDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = AnyDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(AnyDeserializerState::Annotation(Some(deserializer)));
                        *self.state = AnyDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = AnyDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Any> for Box<AnyDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Any>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, AnyDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Any>
    where
        R: DeserializeReader,
    {
        use AnyDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = AnyDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Any, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, AnyDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Any {
            id: self.id,
            namespace: self.namespace,
            not_namespace: self.not_namespace,
            process_contents: self.process_contents,
            not_q_name: self.not_q_name,
            min_occurs: self.min_occurs,
            max_occurs: self.max_occurs,
            annotation: self.annotation,
        })
    }
}
#[derive(Debug)]
pub struct AltTypeDeserializer {
    id: Option<String>,
    test: Option<String>,
    type_: Option<QName>,
    xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
    content: Vec<super::AltTypeContent>,
    state: Box<AltTypeDeserializerState>,
}
#[derive(Debug)]
enum AltTypeDeserializerState {
    Init__,
    Next__,
    Content__(<super::AltTypeContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl AltTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut test: Option<String> = None;
        let mut type_: Option<QName> = None;
        let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"test")
            ) {
                reader.read_attrib(&mut test, b"test", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"type")
            ) {
                reader.read_attrib(&mut type_, b"type", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"xpathDefaultNamespace")
            ) {
                reader.read_attrib(
                    &mut xpath_default_namespace,
                    b"xpathDefaultNamespace",
                    &attrib.value,
                )?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            test: test,
            type_: type_,
            xpath_default_namespace: xpath_default_namespace,
            content: Vec::new(),
            state: Box::new(AltTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: AltTypeDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let AltTypeDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::AltTypeContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::AltTypeContent>,
        fallback: &mut Option<AltTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback.take().unwrap_or(AltTypeDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = AltTypeDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let can_have_more = self.content.len().saturating_add(1) < 2usize;
                let ret = if can_have_more {
                    ElementHandlerOutput::from_event(event, allow_any)
                } else {
                    ElementHandlerOutput::from_event_end(event, allow_any)
                };
                match (can_have_more, &ret) {
                    (true, ElementHandlerOutput::Continue { .. }) => {
                        fallback.get_or_insert(AltTypeDeserializerState::Content__(deserializer));
                        *self.state = AltTypeDeserializerState::Next__;
                    }
                    (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                        *self.state = AltTypeDeserializerState::Content__(deserializer);
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::AltType> for Box<AltTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AltType>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, AltTypeDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AltType>
    where
        R: DeserializeReader,
    {
        use AltTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output = <super::AltTypeContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::AltType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, AltTypeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::AltType {
            id: self.id,
            test: self.test,
            type_: self.type_,
            xpath_default_namespace: self.xpath_default_namespace,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct AltTypeContentDeserializer {
    state: Box<AltTypeContentDeserializerState>,
}
#[derive(Debug)]
pub enum AltTypeContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    SimpleType(
        Option<super::SimpleBaseType>,
        Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
    ),
    ComplexType(
        Option<super::ComplexBaseType>,
        Option<<super::ComplexBaseType as WithDeserializer>::Deserializer>,
    ),
    Done__(super::AltTypeContent),
    Unknown__,
}
impl AltTypeContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<AltTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"complexType")
            ) {
                let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_complex_type(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(AltTypeContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: AltTypeContentDeserializerState,
    ) -> Result<super::AltTypeContent, Error>
    where
        R: DeserializeReader,
    {
        use AltTypeContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AltTypeContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::AltTypeContent::Annotation(values.ok_or_else(
                    || ErrorKind::MissingElement("annotation".into()),
                )?))
            }
            S::SimpleType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AltTypeContentDeserializer::store_simple_type(&mut values, value)?;
                }
                Ok(super::AltTypeContent::SimpleType(values.ok_or_else(
                    || ErrorKind::MissingElement("simpleType".into()),
                )?))
            }
            S::ComplexType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AltTypeContentDeserializer::store_complex_type(&mut values, value)?;
                }
                Ok(super::AltTypeContent::ComplexType(values.ok_or_else(
                    || ErrorKind::MissingElement("complexType".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_simple_type(
        values: &mut Option<super::SimpleBaseType>,
        value: super::SimpleBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"simpleType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_complex_type(
        values: &mut Option<super::ComplexBaseType>,
        value: super::ComplexBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"complexType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<AltTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => AltTypeContentDeserializerState::Annotation(values, None),
                Some(AltTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    AltTypeContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AltTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AltTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AltTypeContentDeserializer::store_annotation(&mut values, data)?;
                let data = AltTypeContentDeserializer::finish_state(
                    reader,
                    AltTypeContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = AltTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    AltTypeContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_simple_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::SimpleBaseType>,
        output: DeserializerOutput<'de, super::SimpleBaseType>,
        fallback: &mut Option<AltTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => AltTypeContentDeserializerState::SimpleType(values, None),
                Some(AltTypeContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                    AltTypeContentDeserializerState::SimpleType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AltTypeContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AltTypeContentDeserializer::store_simple_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AltTypeContentDeserializer::store_simple_type(&mut values, data)?;
                let data = AltTypeContentDeserializer::finish_state(
                    reader,
                    AltTypeContentDeserializerState::SimpleType(values, None),
                )?;
                *self.state = AltTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    AltTypeContentDeserializerState::SimpleType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_complex_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ComplexBaseType>,
        output: DeserializerOutput<'de, super::ComplexBaseType>,
        fallback: &mut Option<AltTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => AltTypeContentDeserializerState::ComplexType(values, None),
                Some(AltTypeContentDeserializerState::ComplexType(_, Some(deserializer))) => {
                    AltTypeContentDeserializerState::ComplexType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AltTypeContentDeserializerState::ComplexType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AltTypeContentDeserializer::store_complex_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AltTypeContentDeserializer::store_complex_type(&mut values, data)?;
                let data = AltTypeContentDeserializer::finish_state(
                    reader,
                    AltTypeContentDeserializerState::ComplexType(values, None),
                )?;
                *self.state = AltTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    AltTypeContentDeserializerState::ComplexType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::AltTypeContent> for Box<AltTypeContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AltTypeContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(AltTypeContentDeserializer {
            state: Box::new(AltTypeContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, AltTypeContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::AltTypeContent>
    where
        R: DeserializeReader,
    {
        use AltTypeContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_complex_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            AltTypeContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, None), event) => {
                    let output = <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ComplexType(values, None), event) => {
                    let output = <super::ComplexBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_complex_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::AltTypeContent, Error>
    where
        R: DeserializeReader,
    {
        AltTypeContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct KeybaseTypeDeserializer {
    id: Option<String>,
    name: Option<String>,
    ref_: Option<QName>,
    content: Option<super::KeybaseTypeContent>,
    state: Box<KeybaseTypeDeserializerState>,
}
#[derive(Debug)]
enum KeybaseTypeDeserializerState {
    Init__,
    Next__,
    Content__(<super::KeybaseTypeContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl KeybaseTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;
        let mut ref_: Option<QName> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"ref")
            ) {
                reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            name: name,
            ref_: ref_,
            content: None,
            state: Box::new(KeybaseTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: KeybaseTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let KeybaseTypeDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::KeybaseTypeContent) -> Result<(), Error> {
        if self.content.is_some() {
            Err(ErrorKind::DuplicateContent)?;
        }
        self.content = Some(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::KeybaseTypeContent>,
        fallback: &mut Option<KeybaseTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(KeybaseTypeDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = KeybaseTypeDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = KeybaseTypeDeserializerState::Content__(deserializer);
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::KeybaseType> for Box<KeybaseTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::KeybaseType>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, KeybaseTypeDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::KeybaseType>
    where
        R: DeserializeReader,
    {
        use KeybaseTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::KeybaseTypeContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::KeybaseType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, KeybaseTypeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::KeybaseType {
            id: self.id,
            name: self.name,
            ref_: self.ref_,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct KeybaseTypeContentDeserializer {
    annotation: Option<super::Annotation>,
    selector: Option<super::Field>,
    field: Vec<super::Field>,
    state: Box<KeybaseTypeContentDeserializerState>,
}
#[derive(Debug)]
enum KeybaseTypeContentDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Selector(Option<<super::Field as WithDeserializer>::Deserializer>),
    Field(Option<<super::Field as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl KeybaseTypeContentDeserializer {
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: KeybaseTypeContentDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use KeybaseTypeContentDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            S::Selector(Some(deserializer)) => self.store_selector(deserializer.finish(reader)?)?,
            S::Field(Some(deserializer)) => self.store_field(deserializer.finish(reader)?)?,
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn store_selector(&mut self, value: super::Field) -> Result<(), Error> {
        if self.selector.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"selector",
            )))?;
        }
        self.selector = Some(value);
        Ok(())
    }
    fn store_field(&mut self, value: super::Field) -> Result<(), Error> {
        self.field.push(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<KeybaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(KeybaseTypeContentDeserializerState::Annotation(None));
            *self.state = KeybaseTypeContentDeserializerState::Selector(None);
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = KeybaseTypeContentDeserializerState::Selector(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(KeybaseTypeContentDeserializerState::Annotation(
                            Some(deserializer),
                        ));
                        *self.state = KeybaseTypeContentDeserializerState::Selector(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state =
                            KeybaseTypeContentDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
    fn handle_selector<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Field>,
        fallback: &mut Option<KeybaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            if self.selector.is_some() {
                fallback.get_or_insert(KeybaseTypeContentDeserializerState::Selector(None));
                *self.state = KeybaseTypeContentDeserializerState::Field(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            } else {
                *self.state = KeybaseTypeContentDeserializerState::Selector(None);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_selector(data)?;
                *self.state = KeybaseTypeContentDeserializerState::Field(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(KeybaseTypeContentDeserializerState::Selector(
                            Some(deserializer),
                        ));
                        *self.state = KeybaseTypeContentDeserializerState::Field(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state =
                            KeybaseTypeContentDeserializerState::Selector(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
    fn handle_field<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Field>,
        fallback: &mut Option<KeybaseTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            if self.field.len() < 1usize {
                *self.state = KeybaseTypeContentDeserializerState::Field(None);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            } else {
                fallback.get_or_insert(KeybaseTypeContentDeserializerState::Field(None));
                *self.state = KeybaseTypeContentDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_field(data)?;
                *self.state = KeybaseTypeContentDeserializerState::Field(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(KeybaseTypeContentDeserializerState::Field(Some(
                            deserializer,
                        )));
                        if self.field.len().saturating_add(1) < 1usize {
                            *self.state = KeybaseTypeContentDeserializerState::Field(None);
                        } else {
                            *self.state = KeybaseTypeContentDeserializerState::Done__;
                        }
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state =
                            KeybaseTypeContentDeserializerState::Field(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::KeybaseTypeContent> for Box<KeybaseTypeContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::KeybaseTypeContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(KeybaseTypeContentDeserializer {
            annotation: None,
            selector: None,
            field: Vec::new(),
            state: Box::new(KeybaseTypeContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, KeybaseTypeContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::KeybaseTypeContent>
    where
        R: DeserializeReader,
    {
        use KeybaseTypeContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (S::Selector(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_selector(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (S::Field(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_field(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, event @ Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = KeybaseTypeContentDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Selector(None);
                        event
                    }
                }
                (S::Selector(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(&event, Some(&super::super::NS_XS), b"selector")
                    {
                        let output =
                            <super::Field as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_selector(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Field(None);
                        event
                    }
                }
                (S::Field(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(&event, Some(&super::super::NS_XS), b"field") {
                        let output =
                            <super::Field as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_field(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::KeybaseTypeContent, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(
            &mut *self.state,
            KeybaseTypeContentDeserializerState::Unknown__,
        );
        self.finish_state(reader, state)?;
        Ok(super::KeybaseTypeContent {
            annotation: self.annotation,
            selector: self
                .selector
                .ok_or_else(|| ErrorKind::MissingElement("selector".into()))?,
            field: self.field,
        })
    }
}
#[derive(Debug)]
pub struct KeyrefDeserializer {
    id: Option<String>,
    name: Option<String>,
    ref_: Option<QName>,
    refer: Option<QName>,
    content: Option<super::KeyrefContent>,
    state: Box<KeyrefDeserializerState>,
}
#[derive(Debug)]
enum KeyrefDeserializerState {
    Init__,
    Next__,
    Content__(<super::KeyrefContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl KeyrefDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut name: Option<String> = None;
        let mut ref_: Option<QName> = None;
        let mut refer: Option<QName> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"ref")
            ) {
                reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"refer")
            ) {
                reader.read_attrib(&mut refer, b"refer", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            name: name,
            ref_: ref_,
            refer: refer,
            content: None,
            state: Box::new(KeyrefDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: KeyrefDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let KeyrefDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::KeyrefContent) -> Result<(), Error> {
        if self.content.is_some() {
            Err(ErrorKind::DuplicateContent)?;
        }
        self.content = Some(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::KeyrefContent>,
        fallback: &mut Option<KeyrefDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback.take().unwrap_or(KeyrefDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = KeyrefDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = KeyrefDeserializerState::Content__(deserializer);
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Keyref> for Box<KeyrefDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Keyref>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, KeyrefDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Keyref>
    where
        R: DeserializeReader,
    {
        use KeyrefDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output = <super::KeyrefContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Keyref, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, KeyrefDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Keyref {
            id: self.id,
            name: self.name,
            ref_: self.ref_,
            refer: self.refer,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct KeyrefContentDeserializer {
    annotation: Option<super::Annotation>,
    selector: Option<super::Field>,
    field: Vec<super::Field>,
    state: Box<KeyrefContentDeserializerState>,
}
#[derive(Debug)]
enum KeyrefContentDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Selector(Option<<super::Field as WithDeserializer>::Deserializer>),
    Field(Option<<super::Field as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl KeyrefContentDeserializer {
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: KeyrefContentDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use KeyrefContentDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            S::Selector(Some(deserializer)) => self.store_selector(deserializer.finish(reader)?)?,
            S::Field(Some(deserializer)) => self.store_field(deserializer.finish(reader)?)?,
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn store_selector(&mut self, value: super::Field) -> Result<(), Error> {
        if self.selector.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"selector",
            )))?;
        }
        self.selector = Some(value);
        Ok(())
    }
    fn store_field(&mut self, value: super::Field) -> Result<(), Error> {
        self.field.push(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<KeyrefContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(KeyrefContentDeserializerState::Annotation(None));
            *self.state = KeyrefContentDeserializerState::Selector(None);
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = KeyrefContentDeserializerState::Selector(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(KeyrefContentDeserializerState::Annotation(Some(
                            deserializer,
                        )));
                        *self.state = KeyrefContentDeserializerState::Selector(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state =
                            KeyrefContentDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
    fn handle_selector<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Field>,
        fallback: &mut Option<KeyrefContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            if self.selector.is_some() {
                fallback.get_or_insert(KeyrefContentDeserializerState::Selector(None));
                *self.state = KeyrefContentDeserializerState::Field(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            } else {
                *self.state = KeyrefContentDeserializerState::Selector(None);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_selector(data)?;
                *self.state = KeyrefContentDeserializerState::Field(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(KeyrefContentDeserializerState::Selector(Some(
                            deserializer,
                        )));
                        *self.state = KeyrefContentDeserializerState::Field(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = KeyrefContentDeserializerState::Selector(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
    fn handle_field<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Field>,
        fallback: &mut Option<KeyrefContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            if self.field.len() < 1usize {
                *self.state = KeyrefContentDeserializerState::Field(None);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            } else {
                fallback.get_or_insert(KeyrefContentDeserializerState::Field(None));
                *self.state = KeyrefContentDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_field(data)?;
                *self.state = KeyrefContentDeserializerState::Field(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(KeyrefContentDeserializerState::Field(Some(
                            deserializer,
                        )));
                        if self.field.len().saturating_add(1) < 1usize {
                            *self.state = KeyrefContentDeserializerState::Field(None);
                        } else {
                            *self.state = KeyrefContentDeserializerState::Done__;
                        }
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = KeyrefContentDeserializerState::Field(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::KeyrefContent> for Box<KeyrefContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::KeyrefContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(KeyrefContentDeserializer {
            annotation: None,
            selector: None,
            field: Vec::new(),
            state: Box::new(KeyrefContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, KeyrefContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::KeyrefContent>
    where
        R: DeserializeReader,
    {
        use KeyrefContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (S::Selector(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_selector(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (S::Field(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_field(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, event @ Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = KeyrefContentDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Selector(None);
                        event
                    }
                }
                (S::Selector(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(&event, Some(&super::super::NS_XS), b"selector")
                    {
                        let output =
                            <super::Field as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_selector(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Field(None);
                        event
                    }
                }
                (S::Field(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(&event, Some(&super::super::NS_XS), b"field") {
                        let output =
                            <super::Field as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_field(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::KeyrefContent, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, KeyrefContentDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::KeyrefContent {
            annotation: self.annotation,
            selector: self
                .selector
                .ok_or_else(|| ErrorKind::MissingElement("selector".into()))?,
            field: self.field,
        })
    }
}
#[derive(Debug)]
pub struct FacetDeserializer {
    state: Box<FacetDeserializerState>,
}
#[derive(Debug)]
pub enum FacetDeserializerState {
    Init__,
    MinExclusive(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    MinInclusive(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    MaxExclusive(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    MaxInclusive(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    TotalDigits(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    FractionDigits(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    Length(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    MinLength(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    MaxLength(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    Enumeration(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    WhiteSpace(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    Pattern(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    Assertion(
        Option<super::AssertionType>,
        Option<<super::AssertionType as WithDeserializer>::Deserializer>,
    ),
    ExplicitTimezone(
        Option<super::FacetType>,
        Option<<super::FacetType as WithDeserializer>::Deserializer>,
    ),
    Done__(super::Facet),
    Unknown__,
}
impl FacetDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"minExclusive")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_min_exclusive(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"minInclusive")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_min_inclusive(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"maxExclusive")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_max_exclusive(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"maxInclusive")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_max_inclusive(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"totalDigits")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_total_digits(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"fractionDigits")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_fraction_digits(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"length")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_length(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"minLength")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_min_length(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"maxLength")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_max_length(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"enumeration")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_enumeration(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"whiteSpace")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_white_space(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"pattern")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_pattern(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"assertion")
            ) {
                let output =
                    <super::AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_assertion(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"explicitTimezone")
            ) {
                let output =
                    <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_explicit_timezone(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
        }
        *self.state = fallback.take().unwrap_or(FacetDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(reader: &R, state: FacetDeserializerState) -> Result<super::Facet, Error>
    where
        R: DeserializeReader,
    {
        use FacetDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::MinExclusive(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_min_exclusive(&mut values, value)?;
                }
                Ok(super::Facet::MinExclusive(values.ok_or_else(|| {
                    ErrorKind::MissingElement("minExclusive".into())
                })?))
            }
            S::MinInclusive(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_min_inclusive(&mut values, value)?;
                }
                Ok(super::Facet::MinInclusive(values.ok_or_else(|| {
                    ErrorKind::MissingElement("minInclusive".into())
                })?))
            }
            S::MaxExclusive(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_max_exclusive(&mut values, value)?;
                }
                Ok(super::Facet::MaxExclusive(values.ok_or_else(|| {
                    ErrorKind::MissingElement("maxExclusive".into())
                })?))
            }
            S::MaxInclusive(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_max_inclusive(&mut values, value)?;
                }
                Ok(super::Facet::MaxInclusive(values.ok_or_else(|| {
                    ErrorKind::MissingElement("maxInclusive".into())
                })?))
            }
            S::TotalDigits(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_total_digits(&mut values, value)?;
                }
                Ok(super::Facet::TotalDigits(values.ok_or_else(|| {
                    ErrorKind::MissingElement("totalDigits".into())
                })?))
            }
            S::FractionDigits(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_fraction_digits(&mut values, value)?;
                }
                Ok(super::Facet::FractionDigits(values.ok_or_else(|| {
                    ErrorKind::MissingElement("fractionDigits".into())
                })?))
            }
            S::Length(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_length(&mut values, value)?;
                }
                Ok(super::Facet::Length(values.ok_or_else(|| {
                    ErrorKind::MissingElement("length".into())
                })?))
            }
            S::MinLength(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_min_length(&mut values, value)?;
                }
                Ok(super::Facet::MinLength(values.ok_or_else(|| {
                    ErrorKind::MissingElement("minLength".into())
                })?))
            }
            S::MaxLength(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_max_length(&mut values, value)?;
                }
                Ok(super::Facet::MaxLength(values.ok_or_else(|| {
                    ErrorKind::MissingElement("maxLength".into())
                })?))
            }
            S::Enumeration(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_enumeration(&mut values, value)?;
                }
                Ok(super::Facet::Enumeration(values.ok_or_else(|| {
                    ErrorKind::MissingElement("enumeration".into())
                })?))
            }
            S::WhiteSpace(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_white_space(&mut values, value)?;
                }
                Ok(super::Facet::WhiteSpace(values.ok_or_else(|| {
                    ErrorKind::MissingElement("whiteSpace".into())
                })?))
            }
            S::Pattern(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_pattern(&mut values, value)?;
                }
                Ok(super::Facet::Pattern(values.ok_or_else(|| {
                    ErrorKind::MissingElement("pattern".into())
                })?))
            }
            S::Assertion(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_assertion(&mut values, value)?;
                }
                Ok(super::Facet::Assertion(values.ok_or_else(|| {
                    ErrorKind::MissingElement("assertion".into())
                })?))
            }
            S::ExplicitTimezone(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    FacetDeserializer::store_explicit_timezone(&mut values, value)?;
                }
                Ok(super::Facet::ExplicitTimezone(values.ok_or_else(|| {
                    ErrorKind::MissingElement("explicitTimezone".into())
                })?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_min_exclusive(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"minExclusive",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_min_inclusive(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"minInclusive",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_max_exclusive(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"maxExclusive",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_max_inclusive(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"maxInclusive",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_total_digits(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"totalDigits",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_fraction_digits(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"fractionDigits",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_length(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"length",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_min_length(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"minLength",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_max_length(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"maxLength",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_enumeration(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"enumeration",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_white_space(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"whiteSpace",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_pattern(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"pattern",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_assertion(
        values: &mut Option<super::AssertionType>,
        value: super::AssertionType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"assertion",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_explicit_timezone(
        values: &mut Option<super::FacetType>,
        value: super::FacetType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"explicitTimezone",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_min_exclusive<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::MinExclusive(values, None),
                Some(FacetDeserializerState::MinExclusive(_, Some(deserializer))) => {
                    FacetDeserializerState::MinExclusive(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::MinExclusive(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_min_exclusive(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_min_exclusive(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::MinExclusive(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::MinExclusive(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_min_inclusive<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::MinInclusive(values, None),
                Some(FacetDeserializerState::MinInclusive(_, Some(deserializer))) => {
                    FacetDeserializerState::MinInclusive(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::MinInclusive(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_min_inclusive(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_min_inclusive(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::MinInclusive(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::MinInclusive(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_max_exclusive<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::MaxExclusive(values, None),
                Some(FacetDeserializerState::MaxExclusive(_, Some(deserializer))) => {
                    FacetDeserializerState::MaxExclusive(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::MaxExclusive(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_max_exclusive(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_max_exclusive(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::MaxExclusive(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::MaxExclusive(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_max_inclusive<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::MaxInclusive(values, None),
                Some(FacetDeserializerState::MaxInclusive(_, Some(deserializer))) => {
                    FacetDeserializerState::MaxInclusive(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::MaxInclusive(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_max_inclusive(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_max_inclusive(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::MaxInclusive(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::MaxInclusive(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_total_digits<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::TotalDigits(values, None),
                Some(FacetDeserializerState::TotalDigits(_, Some(deserializer))) => {
                    FacetDeserializerState::TotalDigits(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::TotalDigits(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_total_digits(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_total_digits(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::TotalDigits(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::TotalDigits(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_fraction_digits<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::FractionDigits(values, None),
                Some(FacetDeserializerState::FractionDigits(_, Some(deserializer))) => {
                    FacetDeserializerState::FractionDigits(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::FractionDigits(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_fraction_digits(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_fraction_digits(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::FractionDigits(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::FractionDigits(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_length<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::Length(values, None),
                Some(FacetDeserializerState::Length(_, Some(deserializer))) => {
                    FacetDeserializerState::Length(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::Length(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_length(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_length(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::Length(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::Length(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_min_length<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::MinLength(values, None),
                Some(FacetDeserializerState::MinLength(_, Some(deserializer))) => {
                    FacetDeserializerState::MinLength(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::MinLength(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_min_length(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_min_length(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::MinLength(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::MinLength(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_max_length<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::MaxLength(values, None),
                Some(FacetDeserializerState::MaxLength(_, Some(deserializer))) => {
                    FacetDeserializerState::MaxLength(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::MaxLength(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_max_length(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_max_length(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::MaxLength(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::MaxLength(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_enumeration<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::Enumeration(values, None),
                Some(FacetDeserializerState::Enumeration(_, Some(deserializer))) => {
                    FacetDeserializerState::Enumeration(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::Enumeration(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_enumeration(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_enumeration(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::Enumeration(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::Enumeration(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_white_space<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::WhiteSpace(values, None),
                Some(FacetDeserializerState::WhiteSpace(_, Some(deserializer))) => {
                    FacetDeserializerState::WhiteSpace(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::WhiteSpace(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_white_space(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_white_space(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::WhiteSpace(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::WhiteSpace(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_pattern<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::Pattern(values, None),
                Some(FacetDeserializerState::Pattern(_, Some(deserializer))) => {
                    FacetDeserializerState::Pattern(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::Pattern(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_pattern(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_pattern(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::Pattern(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::Pattern(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_assertion<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AssertionType>,
        output: DeserializerOutput<'de, super::AssertionType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::Assertion(values, None),
                Some(FacetDeserializerState::Assertion(_, Some(deserializer))) => {
                    FacetDeserializerState::Assertion(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::Assertion(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_assertion(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_assertion(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::Assertion(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::Assertion(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_explicit_timezone<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::FacetType>,
        output: DeserializerOutput<'de, super::FacetType>,
        fallback: &mut Option<FacetDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => FacetDeserializerState::ExplicitTimezone(values, None),
                Some(FacetDeserializerState::ExplicitTimezone(_, Some(deserializer))) => {
                    FacetDeserializerState::ExplicitTimezone(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(FacetDeserializerState::ExplicitTimezone(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                FacetDeserializer::store_explicit_timezone(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                FacetDeserializer::store_explicit_timezone(&mut values, data)?;
                let data = FacetDeserializer::finish_state(
                    reader,
                    FacetDeserializerState::ExplicitTimezone(values, None),
                )?;
                *self.state = FacetDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = FacetDeserializerState::ExplicitTimezone(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Facet> for Box<FacetDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Facet>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(FacetDeserializer {
            state: Box::new(FacetDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, FacetDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Facet>
    where
        R: DeserializeReader,
    {
        use FacetDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::MinExclusive(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_min_exclusive(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::MinInclusive(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_min_inclusive(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::MaxExclusive(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_max_exclusive(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::MaxInclusive(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_max_inclusive(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::TotalDigits(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_total_digits(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::FractionDigits(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_fraction_digits(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Length(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_length(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::MinLength(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_min_length(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::MaxLength(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_max_length(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Enumeration(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_enumeration(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::WhiteSpace(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_white_space(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Pattern(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_pattern(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Assertion(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_assertion(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ExplicitTimezone(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_explicit_timezone(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(FacetDeserializer::finish_state(
                            reader, state,
                        )?),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::MinExclusive(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_min_exclusive(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::MinInclusive(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_min_inclusive(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::MaxExclusive(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_max_exclusive(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::MaxInclusive(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_max_inclusive(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::TotalDigits(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_total_digits(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::FractionDigits(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_fraction_digits(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Length(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_length(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::MinLength(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_min_length(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::MaxLength(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_max_length(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Enumeration(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_enumeration(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::WhiteSpace(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_white_space(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Pattern(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_pattern(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Assertion(values, None), event) => {
                    let output = <super::AssertionType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_assertion(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ExplicitTimezone(values, None), event) => {
                    let output =
                        <super::FacetType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_explicit_timezone(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::Facet, Error>
    where
        R: DeserializeReader,
    {
        FacetDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct RestrictionTypeDeserializer {
    id: Option<String>,
    base: QName,
    content: Vec<super::RestrictionTypeContent>,
    state: Box<RestrictionTypeDeserializerState>,
}
#[derive(Debug)]
enum RestrictionTypeDeserializerState {
    Init__,
    Next__,
    Content__(<super::RestrictionTypeContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl RestrictionTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut base: Option<QName> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"base")
            ) {
                reader.read_attrib(&mut base, b"base", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            base: base
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("base".into())))?,
            content: Vec::new(),
            state: Box::new(RestrictionTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: RestrictionTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let RestrictionTypeDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::RestrictionTypeContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::RestrictionTypeContent>,
        fallback: &mut Option<RestrictionTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(RestrictionTypeDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = RestrictionTypeDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = RestrictionTypeDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(RestrictionTypeDeserializerState::Content__(
                            deserializer,
                        ));
                        *self.state = RestrictionTypeDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::RestrictionType> for Box<RestrictionTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RestrictionType>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(
            event,
            RestrictionTypeDeserializer::from_bytes_start,
        )
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::RestrictionType>
    where
        R: DeserializeReader,
    {
        use RestrictionTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::RestrictionTypeContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::RestrictionType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(
            &mut *self.state,
            RestrictionTypeDeserializerState::Unknown__,
        );
        self.finish_state(reader, state)?;
        Ok(super::RestrictionType {
            id: self.id,
            base: self.base,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct RestrictionTypeContentDeserializer {
    state: Box<RestrictionTypeContentDeserializerState>,
}
#[derive(Debug)]
pub enum RestrictionTypeContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    OpenContent(
        Option<super::OpenContent>,
        Option<<super::OpenContent as WithDeserializer>::Deserializer>,
    ),
    Group(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    All(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Choice(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Sequence(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    SimpleType(
        Option<super::SimpleBaseType>,
        Option<<super::SimpleBaseType as WithDeserializer>::Deserializer>,
    ),
    Facet(
        Option<super::Facet>,
        Option<<super::Facet as WithDeserializer>::Deserializer>,
    ),
    Attribute(
        Option<super::AttributeType>,
        Option<<super::AttributeType as WithDeserializer>::Deserializer>,
    ),
    AttributeGroup(
        Option<super::AttributeGroupType>,
        Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
    ),
    AnyAttribute(
        Option<super::AnyAttribute>,
        Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
    ),
    Assert(
        Option<super::AssertionType>,
        Option<<super::AssertionType as WithDeserializer>::Deserializer>,
    ),
    Done__(super::RestrictionTypeContent),
    Unknown__,
}
impl RestrictionTypeContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let mut event = event;
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"openContent")
            ) {
                let output =
                    <super::OpenContent as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_open_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"all")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_all(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"choice")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_choice(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"sequence")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_sequence(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"simpleType")
            ) {
                let output =
                    <super::SimpleBaseType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_simple_type(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"anyAttribute")
            ) {
                let output =
                    <super::AnyAttribute as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_any_attribute(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"assert")
            ) {
                let output =
                    <super::AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_assert(reader, Default::default(), output, &mut *fallback);
            }
            event = {
                let output = <super::Facet as WithDeserializer>::Deserializer::init(reader, event)?;
                match self.handle_facet(reader, Default::default(), output, &mut *fallback)? {
                    ElementHandlerOutput::Continue { event, .. } => event,
                    output => {
                        return Ok(output);
                    }
                }
            };
        }
        *self.state = fallback
            .take()
            .unwrap_or(RestrictionTypeContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: RestrictionTypeContentDeserializerState,
    ) -> Result<super::RestrictionTypeContent, Error>
    where
        R: DeserializeReader,
    {
        use RestrictionTypeContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::Annotation(
                    values.ok_or_else(|| ErrorKind::MissingElement("annotation".into()))?,
                ))
            }
            S::OpenContent(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_open_content(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::OpenContent(
                    values.ok_or_else(|| ErrorKind::MissingElement("openContent".into()))?,
                ))
            }
            S::Group(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_group(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::Group(values.ok_or_else(
                    || ErrorKind::MissingElement("group".into()),
                )?))
            }
            S::All(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_all(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::All(
                    values.ok_or_else(|| ErrorKind::MissingElement("all".into()))?,
                ))
            }
            S::Choice(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_choice(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::Choice(values.ok_or_else(
                    || ErrorKind::MissingElement("choice".into()),
                )?))
            }
            S::Sequence(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_sequence(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::Sequence(values.ok_or_else(
                    || ErrorKind::MissingElement("sequence".into()),
                )?))
            }
            S::SimpleType(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_simple_type(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::SimpleType(
                    values.ok_or_else(|| ErrorKind::MissingElement("simpleType".into()))?,
                ))
            }
            S::Facet(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_facet(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::Facet(values.ok_or_else(
                    || ErrorKind::MissingElement("facet".into()),
                )?))
            }
            S::Attribute(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_attribute(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::Attribute(
                    values.ok_or_else(|| ErrorKind::MissingElement("attribute".into()))?,
                ))
            }
            S::AttributeGroup(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_attribute_group(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::AttributeGroup(
                    values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                ))
            }
            S::AnyAttribute(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_any_attribute(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::AnyAttribute(
                    values.ok_or_else(|| ErrorKind::MissingElement("anyAttribute".into()))?,
                ))
            }
            S::Assert(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RestrictionTypeContentDeserializer::store_assert(&mut values, value)?;
                }
                Ok(super::RestrictionTypeContent::Assert(values.ok_or_else(
                    || ErrorKind::MissingElement("assert".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_open_content(
        values: &mut Option<super::OpenContent>,
        value: super::OpenContent,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"openContent",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_group(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"group",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_all(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"all")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_choice(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"choice",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_sequence(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"sequence",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_simple_type(
        values: &mut Option<super::SimpleBaseType>,
        value: super::SimpleBaseType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"simpleType",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_facet(values: &mut Option<super::Facet>, value: super::Facet) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"facet",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute(
        values: &mut Option<super::AttributeType>,
        value: super::AttributeType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attribute",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute_group(
        values: &mut Option<super::AttributeGroupType>,
        value: super::AttributeGroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attributeGroup",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_any_attribute(
        values: &mut Option<super::AnyAttribute>,
        value: super::AnyAttribute,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"anyAttribute",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_assert(
        values: &mut Option<super::AssertionType>,
        value: super::AssertionType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"assert",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::Annotation(values, None),
                Some(RestrictionTypeContentDeserializerState::Annotation(
                    _,
                    Some(deserializer),
                )) => {
                    RestrictionTypeContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_annotation(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionTypeContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_open_content<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::OpenContent>,
        output: DeserializerOutput<'de, super::OpenContent>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::OpenContent(values, None),
                Some(RestrictionTypeContentDeserializerState::OpenContent(
                    _,
                    Some(deserializer),
                )) => {
                    RestrictionTypeContentDeserializerState::OpenContent(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::OpenContent(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_open_content(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_open_content(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::OpenContent(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = RestrictionTypeContentDeserializerState::OpenContent(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::Group(values, None),
                Some(RestrictionTypeContentDeserializerState::Group(_, Some(deserializer))) => {
                    RestrictionTypeContentDeserializerState::Group(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::Group(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_group(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::Group(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionTypeContentDeserializerState::Group(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_all<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::All(values, None),
                Some(RestrictionTypeContentDeserializerState::All(_, Some(deserializer))) => {
                    RestrictionTypeContentDeserializerState::All(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::All(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_all(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_all(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::All(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionTypeContentDeserializerState::All(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_choice<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::Choice(values, None),
                Some(RestrictionTypeContentDeserializerState::Choice(_, Some(deserializer))) => {
                    RestrictionTypeContentDeserializerState::Choice(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::Choice(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_choice(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_choice(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::Choice(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionTypeContentDeserializerState::Choice(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_sequence<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::Sequence(values, None),
                Some(RestrictionTypeContentDeserializerState::Sequence(_, Some(deserializer))) => {
                    RestrictionTypeContentDeserializerState::Sequence(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::Sequence(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_sequence(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_sequence(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::Sequence(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionTypeContentDeserializerState::Sequence(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_simple_type<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::SimpleBaseType>,
        output: DeserializerOutput<'de, super::SimpleBaseType>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::SimpleType(values, None),
                Some(RestrictionTypeContentDeserializerState::SimpleType(
                    _,
                    Some(deserializer),
                )) => {
                    RestrictionTypeContentDeserializerState::SimpleType(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::SimpleType(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_simple_type(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_simple_type(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::SimpleType(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionTypeContentDeserializerState::SimpleType(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_facet<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Facet>,
        output: DeserializerOutput<'de, super::Facet>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::Facet(values, None),
                Some(RestrictionTypeContentDeserializerState::Facet(_, Some(deserializer))) => {
                    RestrictionTypeContentDeserializerState::Facet(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::Facet(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_facet(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_facet(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::Facet(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionTypeContentDeserializerState::Facet(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeType>,
        output: DeserializerOutput<'de, super::AttributeType>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::Attribute(values, None),
                Some(RestrictionTypeContentDeserializerState::Attribute(_, Some(deserializer))) => {
                    RestrictionTypeContentDeserializerState::Attribute(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::Attribute(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_attribute(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_attribute(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::Attribute(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionTypeContentDeserializerState::Attribute(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeGroupType>,
        output: DeserializerOutput<'de, super::AttributeGroupType>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::AttributeGroup(values, None),
                Some(RestrictionTypeContentDeserializerState::AttributeGroup(
                    _,
                    Some(deserializer),
                )) => RestrictionTypeContentDeserializerState::AttributeGroup(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::AttributeGroup(
                _,
                Some(deserializer),
            )) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_attribute_group(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::AttributeGroup(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = RestrictionTypeContentDeserializerState::AttributeGroup(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_any_attribute<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AnyAttribute>,
        output: DeserializerOutput<'de, super::AnyAttribute>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::AnyAttribute(values, None),
                Some(RestrictionTypeContentDeserializerState::AnyAttribute(
                    _,
                    Some(deserializer),
                )) => RestrictionTypeContentDeserializerState::AnyAttribute(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::AnyAttribute(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_any_attribute(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_any_attribute(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::AnyAttribute(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = RestrictionTypeContentDeserializerState::AnyAttribute(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_assert<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AssertionType>,
        output: DeserializerOutput<'de, super::AssertionType>,
        fallback: &mut Option<RestrictionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => RestrictionTypeContentDeserializerState::Assert(values, None),
                Some(RestrictionTypeContentDeserializerState::Assert(_, Some(deserializer))) => {
                    RestrictionTypeContentDeserializerState::Assert(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RestrictionTypeContentDeserializerState::Assert(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RestrictionTypeContentDeserializer::store_assert(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RestrictionTypeContentDeserializer::store_assert(&mut values, data)?;
                let data = RestrictionTypeContentDeserializer::finish_state(
                    reader,
                    RestrictionTypeContentDeserializerState::Assert(values, None),
                )?;
                *self.state = RestrictionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    RestrictionTypeContentDeserializerState::Assert(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::RestrictionTypeContent>
    for Box<RestrictionTypeContentDeserializer>
{
    fn init<R>(
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::RestrictionTypeContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(RestrictionTypeContentDeserializer {
            state: Box::new(RestrictionTypeContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, RestrictionTypeContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::RestrictionTypeContent>
    where
        R: DeserializeReader,
    {
        use RestrictionTypeContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::OpenContent(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_open_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::All(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_all(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Choice(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_choice(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Sequence(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_sequence(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Facet(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_facet(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AnyAttribute(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Assert(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_assert(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            RestrictionTypeContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::OpenContent(values, None), event) => {
                    let output = <super::OpenContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_open_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::All(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_all(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Choice(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_choice(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Sequence(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_sequence(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::SimpleType(values, None), event) => {
                    let output = <super::SimpleBaseType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_simple_type(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Facet(values, None), event) => {
                    let output =
                        <super::Facet as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_facet(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, None), event) => {
                    let output = <super::AttributeType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, None), event) => {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AnyAttribute(values, None), event) => {
                    let output = <super::AnyAttribute as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Assert(values, None), event) => {
                    let output = <super::AssertionType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_assert(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::RestrictionTypeContent, Error>
    where
        R: DeserializeReader,
    {
        RestrictionTypeContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct ExtensionTypeDeserializer {
    id: Option<String>,
    base: QName,
    content: Vec<super::ExtensionTypeContent>,
    state: Box<ExtensionTypeDeserializerState>,
}
#[derive(Debug)]
enum ExtensionTypeDeserializerState {
    Init__,
    Next__,
    Content__(<super::ExtensionTypeContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl ExtensionTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut base: Option<QName> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"base")
            ) {
                reader.read_attrib(&mut base, b"base", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            base: base
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("base".into())))?,
            content: Vec::new(),
            state: Box::new(ExtensionTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: ExtensionTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let ExtensionTypeDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::ExtensionTypeContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::ExtensionTypeContent>,
        fallback: &mut Option<ExtensionTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = fallback
                .take()
                .unwrap_or(ExtensionTypeDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = ExtensionTypeDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = ExtensionTypeDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(ExtensionTypeDeserializerState::Content__(deserializer));
                        *self.state = ExtensionTypeDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::ExtensionType> for Box<ExtensionTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ExtensionType>
    where
        R: DeserializeReader,
    {
        reader
            .init_deserializer_from_start_event(event, ExtensionTypeDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ExtensionType>
    where
        R: DeserializeReader,
    {
        use ExtensionTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Content__(deserializer), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (_, Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (state @ (S::Init__ | S::Next__), event) => {
                    fallback.get_or_insert(state);
                    let output =
                        <super::ExtensionTypeContent as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_content(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = DeserializerArtifact::Deserializer(self);
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::ExtensionType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, ExtensionTypeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::ExtensionType {
            id: self.id,
            base: self.base,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct ExtensionTypeContentDeserializer {
    state: Box<ExtensionTypeContentDeserializerState>,
}
#[derive(Debug)]
pub enum ExtensionTypeContentDeserializerState {
    Init__,
    Annotation(
        Option<super::Annotation>,
        Option<<super::Annotation as WithDeserializer>::Deserializer>,
    ),
    OpenContent(
        Option<super::OpenContent>,
        Option<<super::OpenContent as WithDeserializer>::Deserializer>,
    ),
    Group(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    All(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Choice(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Sequence(
        Option<super::GroupType>,
        Option<<super::GroupType as WithDeserializer>::Deserializer>,
    ),
    Attribute(
        Option<super::AttributeType>,
        Option<<super::AttributeType as WithDeserializer>::Deserializer>,
    ),
    AttributeGroup(
        Option<super::AttributeGroupType>,
        Option<<super::AttributeGroupType as WithDeserializer>::Deserializer>,
    ),
    AnyAttribute(
        Option<super::AnyAttribute>,
        Option<<super::AnyAttribute as WithDeserializer>::Deserializer>,
    ),
    Assert(
        Option<super::AssertionType>,
        Option<<super::AssertionType as WithDeserializer>::Deserializer>,
    ),
    Done__(super::ExtensionTypeContent),
    Unknown__,
}
impl ExtensionTypeContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"annotation")
            ) {
                let output =
                    <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_annotation(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"openContent")
            ) {
                let output =
                    <super::OpenContent as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_open_content(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"group")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_group(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"all")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_all(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"choice")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_choice(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"sequence")
            ) {
                let output =
                    <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_sequence(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attribute")
            ) {
                let output =
                    <super::AttributeType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_attribute(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"attributeGroup")
            ) {
                let output = <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                    reader, event,
                )?;
                return self.handle_attribute_group(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"anyAttribute")
            ) {
                let output =
                    <super::AnyAttribute as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_any_attribute(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_XS),
                Some(b"assert")
            ) {
                let output =
                    <super::AssertionType as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_assert(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(ExtensionTypeContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, false))
    }
    fn finish_state<R>(
        reader: &R,
        state: ExtensionTypeContentDeserializerState,
    ) -> Result<super::ExtensionTypeContent, Error>
    where
        R: DeserializeReader,
    {
        use ExtensionTypeContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Annotation(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ExtensionTypeContentDeserializer::store_annotation(&mut values, value)?;
                }
                Ok(super::ExtensionTypeContent::Annotation(values.ok_or_else(
                    || ErrorKind::MissingElement("annotation".into()),
                )?))
            }
            S::OpenContent(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ExtensionTypeContentDeserializer::store_open_content(&mut values, value)?;
                }
                Ok(super::ExtensionTypeContent::OpenContent(
                    values.ok_or_else(|| ErrorKind::MissingElement("openContent".into()))?,
                ))
            }
            S::Group(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ExtensionTypeContentDeserializer::store_group(&mut values, value)?;
                }
                Ok(super::ExtensionTypeContent::Group(values.ok_or_else(
                    || ErrorKind::MissingElement("group".into()),
                )?))
            }
            S::All(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ExtensionTypeContentDeserializer::store_all(&mut values, value)?;
                }
                Ok(super::ExtensionTypeContent::All(
                    values.ok_or_else(|| ErrorKind::MissingElement("all".into()))?,
                ))
            }
            S::Choice(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ExtensionTypeContentDeserializer::store_choice(&mut values, value)?;
                }
                Ok(super::ExtensionTypeContent::Choice(values.ok_or_else(
                    || ErrorKind::MissingElement("choice".into()),
                )?))
            }
            S::Sequence(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ExtensionTypeContentDeserializer::store_sequence(&mut values, value)?;
                }
                Ok(super::ExtensionTypeContent::Sequence(values.ok_or_else(
                    || ErrorKind::MissingElement("sequence".into()),
                )?))
            }
            S::Attribute(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ExtensionTypeContentDeserializer::store_attribute(&mut values, value)?;
                }
                Ok(super::ExtensionTypeContent::Attribute(values.ok_or_else(
                    || ErrorKind::MissingElement("attribute".into()),
                )?))
            }
            S::AttributeGroup(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ExtensionTypeContentDeserializer::store_attribute_group(&mut values, value)?;
                }
                Ok(super::ExtensionTypeContent::AttributeGroup(
                    values.ok_or_else(|| ErrorKind::MissingElement("attributeGroup".into()))?,
                ))
            }
            S::AnyAttribute(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ExtensionTypeContentDeserializer::store_any_attribute(&mut values, value)?;
                }
                Ok(super::ExtensionTypeContent::AnyAttribute(
                    values.ok_or_else(|| ErrorKind::MissingElement("anyAttribute".into()))?,
                ))
            }
            S::Assert(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ExtensionTypeContentDeserializer::store_assert(&mut values, value)?;
                }
                Ok(super::ExtensionTypeContent::Assert(values.ok_or_else(
                    || ErrorKind::MissingElement("assert".into()),
                )?))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_annotation(
        values: &mut Option<super::Annotation>,
        value: super::Annotation,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_open_content(
        values: &mut Option<super::OpenContent>,
        value: super::OpenContent,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"openContent",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_group(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"group",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_all(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"all")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_choice(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"choice",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_sequence(
        values: &mut Option<super::GroupType>,
        value: super::GroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"sequence",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute(
        values: &mut Option<super::AttributeType>,
        value: super::AttributeType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attribute",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_attribute_group(
        values: &mut Option<super::AttributeGroupType>,
        value: super::AttributeGroupType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"attributeGroup",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_any_attribute(
        values: &mut Option<super::AnyAttribute>,
        value: super::AnyAttribute,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"anyAttribute",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_assert(
        values: &mut Option<super::AssertionType>,
        value: super::AssertionType,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"assert",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Annotation>,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ExtensionTypeContentDeserializerState::Annotation(values, None),
                Some(ExtensionTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                    ExtensionTypeContentDeserializerState::Annotation(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ExtensionTypeContentDeserializerState::Annotation(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ExtensionTypeContentDeserializer::store_annotation(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ExtensionTypeContentDeserializer::store_annotation(&mut values, data)?;
                let data = ExtensionTypeContentDeserializer::finish_state(
                    reader,
                    ExtensionTypeContentDeserializerState::Annotation(values, None),
                )?;
                *self.state = ExtensionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ExtensionTypeContentDeserializerState::Annotation(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_open_content<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::OpenContent>,
        output: DeserializerOutput<'de, super::OpenContent>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ExtensionTypeContentDeserializerState::OpenContent(values, None),
                Some(ExtensionTypeContentDeserializerState::OpenContent(_, Some(deserializer))) => {
                    ExtensionTypeContentDeserializerState::OpenContent(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ExtensionTypeContentDeserializerState::OpenContent(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ExtensionTypeContentDeserializer::store_open_content(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ExtensionTypeContentDeserializer::store_open_content(&mut values, data)?;
                let data = ExtensionTypeContentDeserializer::finish_state(
                    reader,
                    ExtensionTypeContentDeserializerState::OpenContent(values, None),
                )?;
                *self.state = ExtensionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ExtensionTypeContentDeserializerState::OpenContent(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ExtensionTypeContentDeserializerState::Group(values, None),
                Some(ExtensionTypeContentDeserializerState::Group(_, Some(deserializer))) => {
                    ExtensionTypeContentDeserializerState::Group(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ExtensionTypeContentDeserializerState::Group(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ExtensionTypeContentDeserializer::store_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ExtensionTypeContentDeserializer::store_group(&mut values, data)?;
                let data = ExtensionTypeContentDeserializer::finish_state(
                    reader,
                    ExtensionTypeContentDeserializerState::Group(values, None),
                )?;
                *self.state = ExtensionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ExtensionTypeContentDeserializerState::Group(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_all<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ExtensionTypeContentDeserializerState::All(values, None),
                Some(ExtensionTypeContentDeserializerState::All(_, Some(deserializer))) => {
                    ExtensionTypeContentDeserializerState::All(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ExtensionTypeContentDeserializerState::All(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ExtensionTypeContentDeserializer::store_all(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ExtensionTypeContentDeserializer::store_all(&mut values, data)?;
                let data = ExtensionTypeContentDeserializer::finish_state(
                    reader,
                    ExtensionTypeContentDeserializerState::All(values, None),
                )?;
                *self.state = ExtensionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ExtensionTypeContentDeserializerState::All(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_choice<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ExtensionTypeContentDeserializerState::Choice(values, None),
                Some(ExtensionTypeContentDeserializerState::Choice(_, Some(deserializer))) => {
                    ExtensionTypeContentDeserializerState::Choice(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ExtensionTypeContentDeserializerState::Choice(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ExtensionTypeContentDeserializer::store_choice(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ExtensionTypeContentDeserializer::store_choice(&mut values, data)?;
                let data = ExtensionTypeContentDeserializer::finish_state(
                    reader,
                    ExtensionTypeContentDeserializerState::Choice(values, None),
                )?;
                *self.state = ExtensionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ExtensionTypeContentDeserializerState::Choice(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_sequence<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::GroupType>,
        output: DeserializerOutput<'de, super::GroupType>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ExtensionTypeContentDeserializerState::Sequence(values, None),
                Some(ExtensionTypeContentDeserializerState::Sequence(_, Some(deserializer))) => {
                    ExtensionTypeContentDeserializerState::Sequence(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ExtensionTypeContentDeserializerState::Sequence(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ExtensionTypeContentDeserializer::store_sequence(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ExtensionTypeContentDeserializer::store_sequence(&mut values, data)?;
                let data = ExtensionTypeContentDeserializer::finish_state(
                    reader,
                    ExtensionTypeContentDeserializerState::Sequence(values, None),
                )?;
                *self.state = ExtensionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ExtensionTypeContentDeserializerState::Sequence(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeType>,
        output: DeserializerOutput<'de, super::AttributeType>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ExtensionTypeContentDeserializerState::Attribute(values, None),
                Some(ExtensionTypeContentDeserializerState::Attribute(_, Some(deserializer))) => {
                    ExtensionTypeContentDeserializerState::Attribute(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ExtensionTypeContentDeserializerState::Attribute(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ExtensionTypeContentDeserializer::store_attribute(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ExtensionTypeContentDeserializer::store_attribute(&mut values, data)?;
                let data = ExtensionTypeContentDeserializer::finish_state(
                    reader,
                    ExtensionTypeContentDeserializerState::Attribute(values, None),
                )?;
                *self.state = ExtensionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ExtensionTypeContentDeserializerState::Attribute(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_attribute_group<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AttributeGroupType>,
        output: DeserializerOutput<'de, super::AttributeGroupType>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ExtensionTypeContentDeserializerState::AttributeGroup(values, None),
                Some(ExtensionTypeContentDeserializerState::AttributeGroup(
                    _,
                    Some(deserializer),
                )) => ExtensionTypeContentDeserializerState::AttributeGroup(
                    values,
                    Some(deserializer),
                ),
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ExtensionTypeContentDeserializerState::AttributeGroup(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ExtensionTypeContentDeserializer::store_attribute_group(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ExtensionTypeContentDeserializer::store_attribute_group(&mut values, data)?;
                let data = ExtensionTypeContentDeserializer::finish_state(
                    reader,
                    ExtensionTypeContentDeserializerState::AttributeGroup(values, None),
                )?;
                *self.state = ExtensionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ExtensionTypeContentDeserializerState::AttributeGroup(
                    values,
                    Some(deserializer),
                );
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_any_attribute<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AnyAttribute>,
        output: DeserializerOutput<'de, super::AnyAttribute>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ExtensionTypeContentDeserializerState::AnyAttribute(values, None),
                Some(ExtensionTypeContentDeserializerState::AnyAttribute(
                    _,
                    Some(deserializer),
                )) => {
                    ExtensionTypeContentDeserializerState::AnyAttribute(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ExtensionTypeContentDeserializerState::AnyAttribute(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ExtensionTypeContentDeserializer::store_any_attribute(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ExtensionTypeContentDeserializer::store_any_attribute(&mut values, data)?;
                let data = ExtensionTypeContentDeserializer::finish_state(
                    reader,
                    ExtensionTypeContentDeserializerState::AnyAttribute(values, None),
                )?;
                *self.state = ExtensionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ExtensionTypeContentDeserializerState::AnyAttribute(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_assert<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::AssertionType>,
        output: DeserializerOutput<'de, super::AssertionType>,
        fallback: &mut Option<ExtensionTypeContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            *self.state = match fallback.take() {
                None => ExtensionTypeContentDeserializerState::Assert(values, None),
                Some(ExtensionTypeContentDeserializerState::Assert(_, Some(deserializer))) => {
                    ExtensionTypeContentDeserializerState::Assert(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ExtensionTypeContentDeserializerState::Assert(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ExtensionTypeContentDeserializer::store_assert(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ExtensionTypeContentDeserializer::store_assert(&mut values, data)?;
                let data = ExtensionTypeContentDeserializer::finish_state(
                    reader,
                    ExtensionTypeContentDeserializerState::Assert(values, None),
                )?;
                *self.state = ExtensionTypeContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    ExtensionTypeContentDeserializerState::Assert(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::ExtensionTypeContent> for Box<ExtensionTypeContentDeserializer> {
    fn init<R>(
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ExtensionTypeContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(ExtensionTypeContentDeserializer {
            state: Box::new(ExtensionTypeContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, ExtensionTypeContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::ExtensionTypeContent>
    where
        R: DeserializeReader,
    {
        use ExtensionTypeContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::OpenContent(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_open_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::All(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_all(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Choice(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_choice(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Sequence(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_sequence(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AnyAttribute(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Assert(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_assert(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            ExtensionTypeContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Annotation(values, None), event) => {
                    let output =
                        <super::Annotation as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_annotation(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::OpenContent(values, None), event) => {
                    let output = <super::OpenContent as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_open_content(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Group(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::All(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_all(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Choice(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_choice(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Sequence(values, None), event) => {
                    let output =
                        <super::GroupType as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_sequence(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Attribute(values, None), event) => {
                    let output = <super::AttributeType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AttributeGroup(values, None), event) => {
                    let output =
                        <super::AttributeGroupType as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                    match self.handle_attribute_group(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::AnyAttribute(values, None), event) => {
                    let output = <super::AnyAttribute as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_any_attribute(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Assert(values, None), event) => {
                    let output = <super::AssertionType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_assert(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (s @ S::Done__(_), event) => {
                    *self.state = s;
                    break (DeserializerEvent::Continue(event), false);
                }
                (S::Unknown__, _) => unreachable!(),
            }
        };
        let artifact = if matches!(&*self.state, S::Done__(_)) {
            DeserializerArtifact::Data(self.finish(reader)?)
        } else {
            DeserializerArtifact::Deserializer(self)
        };
        Ok(DeserializerOutput {
            artifact,
            event,
            allow_any,
        })
    }
    fn finish<R>(self, reader: &R) -> Result<super::ExtensionTypeContent, Error>
    where
        R: DeserializeReader,
    {
        ExtensionTypeContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct FieldDeserializer {
    id: Option<String>,
    xpath: String,
    xpath_default_namespace: Option<super::XpathDefaultNamespaceType>,
    annotation: Option<super::Annotation>,
    state: Box<FieldDeserializerState>,
}
#[derive(Debug)]
enum FieldDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl FieldDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut xpath: Option<String> = None;
        let mut xpath_default_namespace: Option<super::XpathDefaultNamespaceType> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"xpath")
            ) {
                reader.read_attrib(&mut xpath, b"xpath", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"xpathDefaultNamespace")
            ) {
                reader.read_attrib(
                    &mut xpath_default_namespace,
                    b"xpathDefaultNamespace",
                    &attrib.value,
                )?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            xpath: xpath
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("xpath".into())))?,
            xpath_default_namespace: xpath_default_namespace,
            annotation: None,
            state: Box::new(FieldDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: FieldDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use FieldDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<FieldDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(FieldDeserializerState::Annotation(None));
            *self.state = FieldDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = FieldDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(FieldDeserializerState::Annotation(Some(deserializer)));
                        *self.state = FieldDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = FieldDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Field> for Box<FieldDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Field>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, FieldDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Field>
    where
        R: DeserializeReader,
    {
        use FieldDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = FieldDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Field, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, FieldDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Field {
            id: self.id,
            xpath: self.xpath,
            xpath_default_namespace: self.xpath_default_namespace,
            annotation: self.annotation,
        })
    }
}
#[derive(Debug)]
pub struct FacetTypeDeserializer {
    id: Option<String>,
    value: String,
    fixed: bool,
    annotation: Option<super::Annotation>,
    state: Box<FacetTypeDeserializerState>,
}
#[derive(Debug)]
enum FacetTypeDeserializerState {
    Init__,
    Annotation(Option<<super::Annotation as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl FacetTypeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut value: Option<String> = None;
        let mut fixed: Option<bool> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"value")
            ) {
                reader.read_attrib(&mut value, b"value", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XS),
                Some(b"fixed")
            ) {
                reader.read_attrib(&mut fixed, b"fixed", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            value: value
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("value".into())))?,
            fixed: fixed.unwrap_or_else(super::FacetType::default_fixed),
            annotation: None,
            state: Box::new(FacetTypeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: FacetTypeDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use FacetTypeDeserializerState as S;
        match state {
            S::Annotation(Some(deserializer)) => {
                self.store_annotation(deserializer.finish(reader)?)?
            }
            _ => (),
        }
        Ok(())
    }
    fn store_annotation(&mut self, value: super::Annotation) -> Result<(), Error> {
        if self.annotation.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"annotation",
            )))?;
        }
        self.annotation = Some(value);
        Ok(())
    }
    fn handle_annotation<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Annotation>,
        fallback: &mut Option<FacetTypeDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        let DeserializerOutput {
            artifact,
            event,
            allow_any,
        } = output;
        if artifact.is_none() {
            fallback.get_or_insert(FacetTypeDeserializerState::Annotation(None));
            *self.state = FacetTypeDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_annotation(data)?;
                *self.state = FacetTypeDeserializerState::Done__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(FacetTypeDeserializerState::Annotation(Some(
                            deserializer,
                        )));
                        *self.state = FacetTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = FacetTypeDeserializerState::Annotation(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::FacetType> for Box<FacetTypeDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FacetType>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, FacetTypeDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::FacetType>
    where
        R: DeserializeReader,
    {
        use FacetTypeDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Annotation(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_annotation(reader, output, &mut fallback)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                    }
                }
                (_, Event::End(_)) => {
                    if let Some(fallback) = fallback.take() {
                        self.finish_state(reader, fallback)?;
                    }
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    });
                }
                (S::Init__, event) => {
                    fallback.get_or_insert(S::Init__);
                    *self.state = FacetTypeDeserializerState::Annotation(None);
                    event
                }
                (S::Annotation(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(
                        &event,
                        Some(&super::super::NS_XS),
                        b"annotation",
                    ) {
                        let output = <super::Annotation as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_annotation(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    } else {
                        *self.state = S::Done__;
                        event
                    }
                }
                (S::Done__, event) => {
                    fallback.get_or_insert(S::Done__);
                    break (DeserializerEvent::Continue(event), allow_any_element);
                }
                (S::Unknown__, _) => unreachable!(),
                (state, event) => {
                    *self.state = state;
                    break (DeserializerEvent::Break(event), false);
                }
            }
        };
        if let Some(fallback) = fallback {
            *self.state = fallback;
        }
        Ok(DeserializerOutput {
            artifact: DeserializerArtifact::Deserializer(self),
            event,
            allow_any,
        })
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::FacetType, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, FacetTypeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::FacetType {
            id: self.id,
            value: self.value,
            fixed: self.fixed,
            annotation: self.annotation,
        })
    }
}
