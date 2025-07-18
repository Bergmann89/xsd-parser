use crate::quick_xml::{
    filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
    DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
    ErrorKind, Event, RawByteStr, WithDeserializer,
};
use core::mem::replace;
#[derive(Debug)]
pub struct SchemaDeserializer {
    id: Option<String>,
    icon: Option<String>,
    see: Option<String>,
    fpi: Option<String>,
    lang: Option<String>,
    space: Option<super::super::xml::Space>,
    schema_version: Option<String>,
    default_phase: Option<String>,
    query_binding: Option<String>,
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
        let mut id: Option<String> = None;
        let mut icon: Option<String> = None;
        let mut see: Option<String> = None;
        let mut fpi: Option<String> = None;
        let mut lang: Option<String> = None;
        let mut space: Option<super::super::xml::Space> = None;
        let mut schema_version: Option<String> = None;
        let mut default_phase: Option<String> = None;
        let mut query_binding: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"icon")
            ) {
                reader.read_attrib(&mut icon, b"icon", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"see")
            ) {
                reader.read_attrib(&mut see, b"see", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"fpi")
            ) {
                reader.read_attrib(&mut fpi, b"fpi", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"lang")
            ) {
                reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"space")
            ) {
                reader.read_attrib(&mut space, b"space", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"schemaVersion")
            ) {
                reader.read_attrib(&mut schema_version, b"schemaVersion", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"defaultPhase")
            ) {
                reader.read_attrib(&mut default_phase, b"defaultPhase", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"queryBinding")
            ) {
                reader.read_attrib(&mut query_binding, b"queryBinding", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            icon: icon,
            see: see,
            fpi: fpi,
            lang: lang,
            space: space,
            schema_version: schema_version,
            default_phase: default_phase,
            query_binding: query_binding,
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
            id: self.id,
            icon: self.icon,
            see: self.see,
            fpi: self.fpi,
            lang: self.lang,
            space: self.space,
            schema_version: self.schema_version,
            default_phase: self.default_phase,
            query_binding: self.query_binding,
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
    Title(
        Option<super::Title>,
        Option<<super::Title as WithDeserializer>::Deserializer>,
    ),
    Ns(
        Option<super::Ns>,
        Option<<super::Ns as WithDeserializer>::Deserializer>,
    ),
    P(
        Option<super::P>,
        Option<<super::P as WithDeserializer>::Deserializer>,
    ),
    Let(
        Option<super::Let>,
        Option<<super::Let as WithDeserializer>::Deserializer>,
    ),
    Phase(
        Option<super::Phase>,
        Option<<super::Phase as WithDeserializer>::Deserializer>,
    ),
    Pattern(
        Option<super::Pattern>,
        Option<<super::Pattern as WithDeserializer>::Deserializer>,
    ),
    Diagnostics(
        Option<super::Diagnostics>,
        Option<<super::Diagnostics as WithDeserializer>::Deserializer>,
    ),
    Properties(
        Option<super::Properties>,
        Option<<super::Properties as WithDeserializer>::Deserializer>,
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
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"include")
            ) {
                let output =
                    <super::Include as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_include(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"title")
            ) {
                let output = <super::Title as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_title(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"ns")
            ) {
                let output = <super::Ns as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_ns(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"p")
            ) {
                let output = <super::P as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_p(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"let")
            ) {
                let output = <super::Let as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_let_(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"phase")
            ) {
                let output = <super::Phase as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_phase(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"pattern")
            ) {
                let output =
                    <super::Pattern as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_pattern(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"diagnostics")
            ) {
                let output =
                    <super::Diagnostics as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_diagnostics(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"properties")
            ) {
                let output =
                    <super::Properties as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_properties(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(SchemaContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
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
            S::Title(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_title(&mut values, value)?;
                }
                Ok(super::SchemaContent::Title(values.ok_or_else(|| {
                    ErrorKind::MissingElement("title".into())
                })?))
            }
            S::Ns(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_ns(&mut values, value)?;
                }
                Ok(super::SchemaContent::Ns(
                    values.ok_or_else(|| ErrorKind::MissingElement("ns".into()))?,
                ))
            }
            S::P(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_p(&mut values, value)?;
                }
                Ok(super::SchemaContent::P(
                    values.ok_or_else(|| ErrorKind::MissingElement("p".into()))?,
                ))
            }
            S::Let(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_let_(&mut values, value)?;
                }
                Ok(super::SchemaContent::Let(
                    values.ok_or_else(|| ErrorKind::MissingElement("let".into()))?,
                ))
            }
            S::Phase(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_phase(&mut values, value)?;
                }
                Ok(super::SchemaContent::Phase(values.ok_or_else(|| {
                    ErrorKind::MissingElement("phase".into())
                })?))
            }
            S::Pattern(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_pattern(&mut values, value)?;
                }
                Ok(super::SchemaContent::Pattern(values.ok_or_else(|| {
                    ErrorKind::MissingElement("pattern".into())
                })?))
            }
            S::Diagnostics(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_diagnostics(&mut values, value)?;
                }
                Ok(super::SchemaContent::Diagnostics(values.ok_or_else(
                    || ErrorKind::MissingElement("diagnostics".into()),
                )?))
            }
            S::Properties(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    SchemaContentDeserializer::store_properties(&mut values, value)?;
                }
                Ok(super::SchemaContent::Properties(values.ok_or_else(
                    || ErrorKind::MissingElement("properties".into()),
                )?))
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
    fn store_title(values: &mut Option<super::Title>, value: super::Title) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"title",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_ns(values: &mut Option<super::Ns>, value: super::Ns) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ns")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_p(values: &mut Option<super::P>, value: super::P) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"p")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_let_(values: &mut Option<super::Let>, value: super::Let) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"let")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_phase(values: &mut Option<super::Phase>, value: super::Phase) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"phase",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_pattern(
        values: &mut Option<super::Pattern>,
        value: super::Pattern,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"pattern",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_diagnostics(
        values: &mut Option<super::Diagnostics>,
        value: super::Diagnostics,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"diagnostics",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_properties(
        values: &mut Option<super::Properties>,
        value: super::Properties,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"properties",
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
    fn handle_title<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Title>,
        output: DeserializerOutput<'de, super::Title>,
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
                None => SchemaContentDeserializerState::Title(values, None),
                Some(SchemaContentDeserializerState::Title(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Title(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Title(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_title(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_title(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Title(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Title(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_ns<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Ns>,
        output: DeserializerOutput<'de, super::Ns>,
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
                None => SchemaContentDeserializerState::Ns(values, None),
                Some(SchemaContentDeserializerState::Ns(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Ns(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Ns(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_ns(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_ns(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Ns(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Ns(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_p<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::P>,
        output: DeserializerOutput<'de, super::P>,
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
                None => SchemaContentDeserializerState::P(values, None),
                Some(SchemaContentDeserializerState::P(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::P(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::P(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_p(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_p(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::P(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::P(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_let_<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Let>,
        output: DeserializerOutput<'de, super::Let>,
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
                None => SchemaContentDeserializerState::Let(values, None),
                Some(SchemaContentDeserializerState::Let(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Let(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Let(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_let_(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_let_(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Let(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Let(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_phase<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Phase>,
        output: DeserializerOutput<'de, super::Phase>,
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
                None => SchemaContentDeserializerState::Phase(values, None),
                Some(SchemaContentDeserializerState::Phase(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Phase(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Phase(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_phase(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_phase(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Phase(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Phase(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_pattern<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Pattern>,
        output: DeserializerOutput<'de, super::Pattern>,
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
                None => SchemaContentDeserializerState::Pattern(values, None),
                Some(SchemaContentDeserializerState::Pattern(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Pattern(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Pattern(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_pattern(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_pattern(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Pattern(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = SchemaContentDeserializerState::Pattern(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_diagnostics<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Diagnostics>,
        output: DeserializerOutput<'de, super::Diagnostics>,
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
                None => SchemaContentDeserializerState::Diagnostics(values, None),
                Some(SchemaContentDeserializerState::Diagnostics(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Diagnostics(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Diagnostics(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_diagnostics(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_diagnostics(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Diagnostics(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SchemaContentDeserializerState::Diagnostics(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_properties<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Properties>,
        output: DeserializerOutput<'de, super::Properties>,
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
                None => SchemaContentDeserializerState::Properties(values, None),
                Some(SchemaContentDeserializerState::Properties(_, Some(deserializer))) => {
                    SchemaContentDeserializerState::Properties(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(SchemaContentDeserializerState::Properties(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                SchemaContentDeserializer::store_properties(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                SchemaContentDeserializer::store_properties(&mut values, data)?;
                let data = SchemaContentDeserializer::finish_state(
                    reader,
                    SchemaContentDeserializerState::Properties(values, None),
                )?;
                *self.state = SchemaContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    SchemaContentDeserializerState::Properties(values, Some(deserializer));
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
                (S::Title(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_title(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Ns(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_ns(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::P(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_p(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Let(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_let_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Phase(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_phase(reader, values, output, &mut fallback)? {
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
                (S::Diagnostics(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_diagnostics(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Properties(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_properties(reader, values, output, &mut fallback)? {
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
                (S::Title(values, None), event) => {
                    let output =
                        <super::Title as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_title(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Ns(values, None), event) => {
                    let output =
                        <super::Ns as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_ns(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::P(values, None), event) => {
                    let output = <super::P as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_p(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Let(values, None), event) => {
                    let output =
                        <super::Let as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_let_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Phase(values, None), event) => {
                    let output =
                        <super::Phase as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_phase(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Pattern(values, None), event) => {
                    let output =
                        <super::Pattern as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_pattern(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Diagnostics(values, None), event) => {
                    let output = <super::Diagnostics as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                    match self.handle_diagnostics(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Properties(values, None), event) => {
                    let output =
                        <super::Properties as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_properties(reader, values, output, &mut fallback)? {
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
    href: String,
    state: Box<IncludeDeserializerState>,
}
#[derive(Debug)]
enum IncludeDeserializerState {
    Init__,
    Unknown__,
}
impl IncludeDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut href: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"href")
            ) {
                reader.read_attrib(&mut href, b"href", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            href: href
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("href".into())))?,
            state: Box::new(IncludeDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: IncludeDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        Ok(())
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
        if let Event::End(_) = &event {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Data(self.finish(reader)?),
                event: DeserializerEvent::None,
                allow_any: false,
            })
        } else {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: false,
            })
        }
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Include, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, IncludeDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Include { href: self.href })
    }
}
#[derive(Debug)]
pub struct TitleDeserializer {
    dir: Vec<super::Dir>,
    state: Box<TitleDeserializerState>,
}
#[derive(Debug)]
enum TitleDeserializerState {
    Init__,
    Dir(Option<<super::Dir as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl TitleDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            reader.raise_unexpected_attrib_checked(attrib)?;
        }
        Ok(Box::new(Self {
            dir: Vec::new(),
            state: Box::new(TitleDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: TitleDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use TitleDeserializerState as S;
        match state {
            S::Dir(Some(deserializer)) => self.store_dir(deserializer.finish(reader)?)?,
            _ => (),
        }
        Ok(())
    }
    fn store_dir(&mut self, value: super::Dir) -> Result<(), Error> {
        self.dir.push(value);
        Ok(())
    }
    fn handle_dir<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Dir>,
        fallback: &mut Option<TitleDeserializerState>,
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
            fallback.get_or_insert(TitleDeserializerState::Dir(None));
            *self.state = TitleDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_dir(data)?;
                *self.state = TitleDeserializerState::Dir(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(TitleDeserializerState::Dir(Some(deserializer)));
                        *self.state = TitleDeserializerState::Dir(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = TitleDeserializerState::Dir(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Title> for Box<TitleDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Title>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, TitleDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Title>
    where
        R: DeserializeReader,
    {
        use TitleDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Dir(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_dir(reader, output, &mut fallback)? {
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
                    *self.state = TitleDeserializerState::Dir(None);
                    event
                }
                (S::Dir(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(&event, Some(&super::super::NS_SCH), b"dir") {
                        let output =
                            <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_dir(reader, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Dir(None));
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Title, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, TitleDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Title { dir: self.dir })
    }
}
#[derive(Debug)]
pub struct NsDeserializer {
    uri: String,
    prefix: String,
    state: Box<NsDeserializerState>,
}
#[derive(Debug)]
enum NsDeserializerState {
    Init__,
    Unknown__,
}
impl NsDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut uri: Option<String> = None;
        let mut prefix: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"uri")
            ) {
                reader.read_attrib(&mut uri, b"uri", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"prefix")
            ) {
                reader.read_attrib(&mut prefix, b"prefix", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            uri: uri.ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("uri".into())))?,
            prefix: prefix
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("prefix".into())))?,
            state: Box::new(NsDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: NsDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        Ok(())
    }
}
impl<'de> Deserializer<'de, super::Ns> for Box<NsDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Ns>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, NsDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Ns>
    where
        R: DeserializeReader,
    {
        if let Event::End(_) = &event {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Data(self.finish(reader)?),
                event: DeserializerEvent::None,
                allow_any: false,
            })
        } else {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: false,
            })
        }
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Ns, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, NsDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Ns {
            uri: self.uri,
            prefix: self.prefix,
        })
    }
}
#[derive(Debug)]
pub struct PDeserializer {
    id: Option<String>,
    class: Option<String>,
    icon: Option<String>,
    content: Vec<super::PContent>,
    state: Box<PDeserializerState>,
}
#[derive(Debug)]
enum PDeserializerState {
    Init__,
    Next__,
    Content__(<super::PContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl PDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut class: Option<String> = None;
        let mut icon: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"class")
            ) {
                reader.read_attrib(&mut class, b"class", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"icon")
            ) {
                reader.read_attrib(&mut icon, b"icon", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id,
            class: class,
            icon: icon,
            content: Vec::new(),
            state: Box::new(PDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: PDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let PDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::PContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::PContent>,
        fallback: &mut Option<PDeserializerState>,
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
            *self.state = fallback.take().unwrap_or(PDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = PDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = PDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(PDeserializerState::Content__(deserializer));
                        *self.state = PDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::P> for Box<PDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::P>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, PDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::P>
    where
        R: DeserializeReader,
    {
        use PDeserializerState as S;
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
                        <super::PContent as WithDeserializer>::Deserializer::init(reader, event)?;
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
    fn finish<R>(mut self, reader: &R) -> Result<super::P, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, PDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::P {
            id: self.id,
            class: self.class,
            icon: self.icon,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct PContentDeserializer {
    state: Box<PContentDeserializerState>,
}
#[derive(Debug)]
pub enum PContentDeserializerState {
    Init__,
    Dir(
        Option<super::Dir>,
        Option<<super::Dir as WithDeserializer>::Deserializer>,
    ),
    Emph(
        Option<super::Emph>,
        Option<<super::Emph as WithDeserializer>::Deserializer>,
    ),
    Span(
        Option<super::Span>,
        Option<<super::Span as WithDeserializer>::Deserializer>,
    ),
    Done__(super::PContent),
    Unknown__,
}
impl PContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<PContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"dir")
            ) {
                let output = <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_dir(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"emph")
            ) {
                let output = <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_emph(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"span")
            ) {
                let output = <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_span(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback.take().unwrap_or(PContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: PContentDeserializerState,
    ) -> Result<super::PContent, Error>
    where
        R: DeserializeReader,
    {
        use PContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Dir(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PContentDeserializer::store_dir(&mut values, value)?;
                }
                Ok(super::PContent::Dir(
                    values.ok_or_else(|| ErrorKind::MissingElement("dir".into()))?,
                ))
            }
            S::Emph(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PContentDeserializer::store_emph(&mut values, value)?;
                }
                Ok(super::PContent::Emph(
                    values.ok_or_else(|| ErrorKind::MissingElement("emph".into()))?,
                ))
            }
            S::Span(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PContentDeserializer::store_span(&mut values, value)?;
                }
                Ok(super::PContent::Span(
                    values.ok_or_else(|| ErrorKind::MissingElement("span".into()))?,
                ))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_dir(values: &mut Option<super::Dir>, value: super::Dir) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"dir")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_emph(values: &mut Option<super::Emph>, value: super::Emph) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"emph")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_span(values: &mut Option<super::Span>, value: super::Span) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"span")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_dir<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Dir>,
        output: DeserializerOutput<'de, super::Dir>,
        fallback: &mut Option<PContentDeserializerState>,
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
                None => PContentDeserializerState::Dir(values, None),
                Some(PContentDeserializerState::Dir(_, Some(deserializer))) => {
                    PContentDeserializerState::Dir(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PContentDeserializerState::Dir(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PContentDeserializer::store_dir(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PContentDeserializer::store_dir(&mut values, data)?;
                let data = PContentDeserializer::finish_state(
                    reader,
                    PContentDeserializerState::Dir(values, None),
                )?;
                *self.state = PContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PContentDeserializerState::Dir(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_emph<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Emph>,
        output: DeserializerOutput<'de, super::Emph>,
        fallback: &mut Option<PContentDeserializerState>,
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
                None => PContentDeserializerState::Emph(values, None),
                Some(PContentDeserializerState::Emph(_, Some(deserializer))) => {
                    PContentDeserializerState::Emph(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PContentDeserializerState::Emph(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PContentDeserializer::store_emph(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PContentDeserializer::store_emph(&mut values, data)?;
                let data = PContentDeserializer::finish_state(
                    reader,
                    PContentDeserializerState::Emph(values, None),
                )?;
                *self.state = PContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PContentDeserializerState::Emph(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_span<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Span>,
        output: DeserializerOutput<'de, super::Span>,
        fallback: &mut Option<PContentDeserializerState>,
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
                None => PContentDeserializerState::Span(values, None),
                Some(PContentDeserializerState::Span(_, Some(deserializer))) => {
                    PContentDeserializerState::Span(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PContentDeserializerState::Span(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PContentDeserializer::store_span(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PContentDeserializer::store_span(&mut values, data)?;
                let data = PContentDeserializer::finish_state(
                    reader,
                    PContentDeserializerState::Span(values, None),
                )?;
                *self.state = PContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PContentDeserializerState::Span(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::PContent> for Box<PContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(PContentDeserializer {
            state: Box::new(PContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, PContentDeserializerState::Init__) =>
            {
                DeserializerArtifact::None
            }
            artifact => artifact,
        };
        Ok(output)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PContent>
    where
        R: DeserializeReader,
    {
        use PContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Dir(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(PContentDeserializer::finish_state(
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
                (S::Dir(values, None), event) => {
                    let output =
                        <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, None), event) => {
                    let output =
                        <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, None), event) => {
                    let output =
                        <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
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
    fn finish<R>(self, reader: &R) -> Result<super::PContent, Error>
    where
        R: DeserializeReader,
    {
        PContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct LetDeserializer {
    name: String,
    value: Option<String>,
    state: Box<LetDeserializerState>,
}
#[derive(Debug)]
enum LetDeserializerState {
    Init__,
    Unknown__,
}
impl LetDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut name: Option<String> = None;
        let mut value: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"value")
            ) {
                reader.read_attrib(&mut value, b"value", &attrib.value)?;
            } else {
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
        }
        Ok(Box::new(Self {
            name: name
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
            value: value,
            state: Box::new(LetDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: LetDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        Ok(())
    }
}
impl<'de> Deserializer<'de, super::Let> for Box<LetDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Let>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, LetDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Let>
    where
        R: DeserializeReader,
    {
        if let Event::End(_) = &event {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Data(self.finish(reader)?),
                event: DeserializerEvent::None,
                allow_any: false,
            })
        } else {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: true,
            })
        }
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Let, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, LetDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Let {
            name: self.name,
            value: self.value,
        })
    }
}
#[derive(Debug)]
pub struct PhaseDeserializer {
    id: String,
    icon: Option<String>,
    see: Option<String>,
    fpi: Option<String>,
    lang: Option<String>,
    space: Option<super::super::xml::Space>,
    content: Vec<super::PhaseContent>,
    state: Box<PhaseDeserializerState>,
}
#[derive(Debug)]
enum PhaseDeserializerState {
    Init__,
    Next__,
    Content__(<super::PhaseContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl PhaseDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut icon: Option<String> = None;
        let mut see: Option<String> = None;
        let mut fpi: Option<String> = None;
        let mut lang: Option<String> = None;
        let mut space: Option<super::super::xml::Space> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"icon")
            ) {
                reader.read_attrib(&mut icon, b"icon", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"see")
            ) {
                reader.read_attrib(&mut see, b"see", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"fpi")
            ) {
                reader.read_attrib(&mut fpi, b"fpi", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"lang")
            ) {
                reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"space")
            ) {
                reader.read_attrib(&mut space, b"space", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id.ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("id".into())))?,
            icon: icon,
            see: see,
            fpi: fpi,
            lang: lang,
            space: space,
            content: Vec::new(),
            state: Box::new(PhaseDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: PhaseDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let PhaseDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::PhaseContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::PhaseContent>,
        fallback: &mut Option<PhaseDeserializerState>,
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
            *self.state = fallback.take().unwrap_or(PhaseDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = PhaseDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = PhaseDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(PhaseDeserializerState::Content__(deserializer));
                        *self.state = PhaseDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Phase> for Box<PhaseDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Phase>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, PhaseDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Phase>
    where
        R: DeserializeReader,
    {
        use PhaseDeserializerState as S;
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
                    let output = <super::PhaseContent as WithDeserializer>::Deserializer::init(
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Phase, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, PhaseDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Phase {
            id: self.id,
            icon: self.icon,
            see: self.see,
            fpi: self.fpi,
            lang: self.lang,
            space: self.space,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct PhaseContentDeserializer {
    state: Box<PhaseContentDeserializerState>,
}
#[derive(Debug)]
pub enum PhaseContentDeserializerState {
    Init__,
    Include(
        Option<super::Include>,
        Option<<super::Include as WithDeserializer>::Deserializer>,
    ),
    P(
        Option<super::P>,
        Option<<super::P as WithDeserializer>::Deserializer>,
    ),
    Let(
        Option<super::Let>,
        Option<<super::Let as WithDeserializer>::Deserializer>,
    ),
    Active(
        Option<super::Active>,
        Option<<super::Active as WithDeserializer>::Deserializer>,
    ),
    Done__(super::PhaseContent),
    Unknown__,
}
impl PhaseContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<PhaseContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"include")
            ) {
                let output =
                    <super::Include as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_include(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"p")
            ) {
                let output = <super::P as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_p(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"let")
            ) {
                let output = <super::Let as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_let_(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"active")
            ) {
                let output =
                    <super::Active as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_active(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(PhaseContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: PhaseContentDeserializerState,
    ) -> Result<super::PhaseContent, Error>
    where
        R: DeserializeReader,
    {
        use PhaseContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Include(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PhaseContentDeserializer::store_include(&mut values, value)?;
                }
                Ok(super::PhaseContent::Include(values.ok_or_else(|| {
                    ErrorKind::MissingElement("include".into())
                })?))
            }
            S::P(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PhaseContentDeserializer::store_p(&mut values, value)?;
                }
                Ok(super::PhaseContent::P(
                    values.ok_or_else(|| ErrorKind::MissingElement("p".into()))?,
                ))
            }
            S::Let(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PhaseContentDeserializer::store_let_(&mut values, value)?;
                }
                Ok(super::PhaseContent::Let(
                    values.ok_or_else(|| ErrorKind::MissingElement("let".into()))?,
                ))
            }
            S::Active(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PhaseContentDeserializer::store_active(&mut values, value)?;
                }
                Ok(super::PhaseContent::Active(values.ok_or_else(|| {
                    ErrorKind::MissingElement("active".into())
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
    fn store_p(values: &mut Option<super::P>, value: super::P) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"p")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_let_(values: &mut Option<super::Let>, value: super::Let) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"let")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_active(values: &mut Option<super::Active>, value: super::Active) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"active",
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
        fallback: &mut Option<PhaseContentDeserializerState>,
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
                None => PhaseContentDeserializerState::Include(values, None),
                Some(PhaseContentDeserializerState::Include(_, Some(deserializer))) => {
                    PhaseContentDeserializerState::Include(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PhaseContentDeserializerState::Include(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PhaseContentDeserializer::store_include(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PhaseContentDeserializer::store_include(&mut values, data)?;
                let data = PhaseContentDeserializer::finish_state(
                    reader,
                    PhaseContentDeserializerState::Include(values, None),
                )?;
                *self.state = PhaseContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PhaseContentDeserializerState::Include(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_p<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::P>,
        output: DeserializerOutput<'de, super::P>,
        fallback: &mut Option<PhaseContentDeserializerState>,
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
                None => PhaseContentDeserializerState::P(values, None),
                Some(PhaseContentDeserializerState::P(_, Some(deserializer))) => {
                    PhaseContentDeserializerState::P(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PhaseContentDeserializerState::P(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PhaseContentDeserializer::store_p(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PhaseContentDeserializer::store_p(&mut values, data)?;
                let data = PhaseContentDeserializer::finish_state(
                    reader,
                    PhaseContentDeserializerState::P(values, None),
                )?;
                *self.state = PhaseContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PhaseContentDeserializerState::P(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_let_<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Let>,
        output: DeserializerOutput<'de, super::Let>,
        fallback: &mut Option<PhaseContentDeserializerState>,
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
                None => PhaseContentDeserializerState::Let(values, None),
                Some(PhaseContentDeserializerState::Let(_, Some(deserializer))) => {
                    PhaseContentDeserializerState::Let(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PhaseContentDeserializerState::Let(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PhaseContentDeserializer::store_let_(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PhaseContentDeserializer::store_let_(&mut values, data)?;
                let data = PhaseContentDeserializer::finish_state(
                    reader,
                    PhaseContentDeserializerState::Let(values, None),
                )?;
                *self.state = PhaseContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PhaseContentDeserializerState::Let(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_active<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Active>,
        output: DeserializerOutput<'de, super::Active>,
        fallback: &mut Option<PhaseContentDeserializerState>,
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
                None => PhaseContentDeserializerState::Active(values, None),
                Some(PhaseContentDeserializerState::Active(_, Some(deserializer))) => {
                    PhaseContentDeserializerState::Active(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PhaseContentDeserializerState::Active(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PhaseContentDeserializer::store_active(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PhaseContentDeserializer::store_active(&mut values, data)?;
                let data = PhaseContentDeserializer::finish_state(
                    reader,
                    PhaseContentDeserializerState::Active(values, None),
                )?;
                *self.state = PhaseContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PhaseContentDeserializerState::Active(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::PhaseContent> for Box<PhaseContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PhaseContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(PhaseContentDeserializer {
            state: Box::new(PhaseContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, PhaseContentDeserializerState::Init__) =>
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
    ) -> DeserializerResult<'de, super::PhaseContent>
    where
        R: DeserializeReader,
    {
        use PhaseContentDeserializerState as S;
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
                (S::P(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_p(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Let(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_let_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Active(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_active(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            PhaseContentDeserializer::finish_state(reader, state)?,
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
                (S::P(values, None), event) => {
                    let output = <super::P as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_p(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Let(values, None), event) => {
                    let output =
                        <super::Let as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_let_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Active(values, None), event) => {
                    let output =
                        <super::Active as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_active(reader, values, output, &mut fallback)? {
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
    fn finish<R>(self, reader: &R) -> Result<super::PhaseContent, Error>
    where
        R: DeserializeReader,
    {
        PhaseContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct PatternDeserializer {
    documents: Option<String>,
    icon: Option<String>,
    see: Option<String>,
    fpi: Option<String>,
    lang: Option<String>,
    space: Option<super::super::xml::Space>,
    abstract_: Option<super::PatternAbstractType>,
    id: Option<String>,
    is_a: Option<String>,
    content: Vec<super::PatternContent>,
    state: Box<PatternDeserializerState>,
}
#[derive(Debug)]
enum PatternDeserializerState {
    Init__,
    Next__,
    Content__(<super::PatternContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl PatternDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut documents: Option<String> = None;
        let mut icon: Option<String> = None;
        let mut see: Option<String> = None;
        let mut fpi: Option<String> = None;
        let mut lang: Option<String> = None;
        let mut space: Option<super::super::xml::Space> = None;
        let mut abstract_: Option<super::PatternAbstractType> = None;
        let mut id: Option<String> = None;
        let mut is_a: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"documents")
            ) {
                reader.read_attrib(&mut documents, b"documents", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"icon")
            ) {
                reader.read_attrib(&mut icon, b"icon", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"see")
            ) {
                reader.read_attrib(&mut see, b"see", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"fpi")
            ) {
                reader.read_attrib(&mut fpi, b"fpi", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"lang")
            ) {
                reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"space")
            ) {
                reader.read_attrib(&mut space, b"space", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"abstract")
            ) {
                reader.read_attrib(&mut abstract_, b"abstract", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"is-a")
            ) {
                reader.read_attrib(&mut is_a, b"is-a", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            documents: documents,
            icon: icon,
            see: see,
            fpi: fpi,
            lang: lang,
            space: space,
            abstract_: abstract_,
            id: id,
            is_a: is_a,
            content: Vec::new(),
            state: Box::new(PatternDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: PatternDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let PatternDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::PatternContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::PatternContent>,
        fallback: &mut Option<PatternDeserializerState>,
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
            *self.state = fallback.take().unwrap_or(PatternDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = PatternDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = PatternDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(PatternDeserializerState::Content__(deserializer));
                        *self.state = PatternDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Pattern> for Box<PatternDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Pattern>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, PatternDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Pattern>
    where
        R: DeserializeReader,
    {
        use PatternDeserializerState as S;
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
                    let output = <super::PatternContent as WithDeserializer>::Deserializer::init(
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Pattern, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, PatternDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Pattern {
            documents: self.documents,
            icon: self.icon,
            see: self.see,
            fpi: self.fpi,
            lang: self.lang,
            space: self.space,
            abstract_: self.abstract_,
            id: self.id,
            is_a: self.is_a,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct PatternContentDeserializer {
    state: Box<PatternContentDeserializerState>,
}
#[derive(Debug)]
pub enum PatternContentDeserializerState {
    Init__,
    Include(
        Option<super::Include>,
        Option<<super::Include as WithDeserializer>::Deserializer>,
    ),
    Title(
        Option<super::Title>,
        Option<<super::Title as WithDeserializer>::Deserializer>,
    ),
    P(
        Option<super::P>,
        Option<<super::P as WithDeserializer>::Deserializer>,
    ),
    Param(
        Option<super::Param>,
        Option<<super::Param as WithDeserializer>::Deserializer>,
    ),
    Let(
        Option<super::Let>,
        Option<<super::Let as WithDeserializer>::Deserializer>,
    ),
    Rule(
        Option<super::Rule>,
        Option<<super::Rule as WithDeserializer>::Deserializer>,
    ),
    Done__(super::PatternContent),
    Unknown__,
}
impl PatternContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<PatternContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"include")
            ) {
                let output =
                    <super::Include as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_include(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"title")
            ) {
                let output = <super::Title as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_title(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"p")
            ) {
                let output = <super::P as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_p(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"param")
            ) {
                let output = <super::Param as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_param(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"let")
            ) {
                let output = <super::Let as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_let_(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"rule")
            ) {
                let output = <super::Rule as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_rule(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(PatternContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: PatternContentDeserializerState,
    ) -> Result<super::PatternContent, Error>
    where
        R: DeserializeReader,
    {
        use PatternContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Include(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PatternContentDeserializer::store_include(&mut values, value)?;
                }
                Ok(super::PatternContent::Include(values.ok_or_else(|| {
                    ErrorKind::MissingElement("include".into())
                })?))
            }
            S::Title(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PatternContentDeserializer::store_title(&mut values, value)?;
                }
                Ok(super::PatternContent::Title(values.ok_or_else(|| {
                    ErrorKind::MissingElement("title".into())
                })?))
            }
            S::P(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PatternContentDeserializer::store_p(&mut values, value)?;
                }
                Ok(super::PatternContent::P(
                    values.ok_or_else(|| ErrorKind::MissingElement("p".into()))?,
                ))
            }
            S::Param(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PatternContentDeserializer::store_param(&mut values, value)?;
                }
                Ok(super::PatternContent::Param(values.ok_or_else(|| {
                    ErrorKind::MissingElement("param".into())
                })?))
            }
            S::Let(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PatternContentDeserializer::store_let_(&mut values, value)?;
                }
                Ok(super::PatternContent::Let(
                    values.ok_or_else(|| ErrorKind::MissingElement("let".into()))?,
                ))
            }
            S::Rule(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PatternContentDeserializer::store_rule(&mut values, value)?;
                }
                Ok(super::PatternContent::Rule(
                    values.ok_or_else(|| ErrorKind::MissingElement("rule".into()))?,
                ))
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
    fn store_title(values: &mut Option<super::Title>, value: super::Title) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"title",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_p(values: &mut Option<super::P>, value: super::P) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"p")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_param(values: &mut Option<super::Param>, value: super::Param) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"param",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_let_(values: &mut Option<super::Let>, value: super::Let) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"let")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_rule(values: &mut Option<super::Rule>, value: super::Rule) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"rule")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_include<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Include>,
        output: DeserializerOutput<'de, super::Include>,
        fallback: &mut Option<PatternContentDeserializerState>,
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
                None => PatternContentDeserializerState::Include(values, None),
                Some(PatternContentDeserializerState::Include(_, Some(deserializer))) => {
                    PatternContentDeserializerState::Include(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PatternContentDeserializerState::Include(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PatternContentDeserializer::store_include(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PatternContentDeserializer::store_include(&mut values, data)?;
                let data = PatternContentDeserializer::finish_state(
                    reader,
                    PatternContentDeserializerState::Include(values, None),
                )?;
                *self.state = PatternContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PatternContentDeserializerState::Include(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_title<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Title>,
        output: DeserializerOutput<'de, super::Title>,
        fallback: &mut Option<PatternContentDeserializerState>,
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
                None => PatternContentDeserializerState::Title(values, None),
                Some(PatternContentDeserializerState::Title(_, Some(deserializer))) => {
                    PatternContentDeserializerState::Title(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PatternContentDeserializerState::Title(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PatternContentDeserializer::store_title(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PatternContentDeserializer::store_title(&mut values, data)?;
                let data = PatternContentDeserializer::finish_state(
                    reader,
                    PatternContentDeserializerState::Title(values, None),
                )?;
                *self.state = PatternContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PatternContentDeserializerState::Title(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_p<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::P>,
        output: DeserializerOutput<'de, super::P>,
        fallback: &mut Option<PatternContentDeserializerState>,
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
                None => PatternContentDeserializerState::P(values, None),
                Some(PatternContentDeserializerState::P(_, Some(deserializer))) => {
                    PatternContentDeserializerState::P(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PatternContentDeserializerState::P(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PatternContentDeserializer::store_p(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PatternContentDeserializer::store_p(&mut values, data)?;
                let data = PatternContentDeserializer::finish_state(
                    reader,
                    PatternContentDeserializerState::P(values, None),
                )?;
                *self.state = PatternContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PatternContentDeserializerState::P(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_param<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Param>,
        output: DeserializerOutput<'de, super::Param>,
        fallback: &mut Option<PatternContentDeserializerState>,
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
                None => PatternContentDeserializerState::Param(values, None),
                Some(PatternContentDeserializerState::Param(_, Some(deserializer))) => {
                    PatternContentDeserializerState::Param(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PatternContentDeserializerState::Param(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PatternContentDeserializer::store_param(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PatternContentDeserializer::store_param(&mut values, data)?;
                let data = PatternContentDeserializer::finish_state(
                    reader,
                    PatternContentDeserializerState::Param(values, None),
                )?;
                *self.state = PatternContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PatternContentDeserializerState::Param(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_let_<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Let>,
        output: DeserializerOutput<'de, super::Let>,
        fallback: &mut Option<PatternContentDeserializerState>,
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
                None => PatternContentDeserializerState::Let(values, None),
                Some(PatternContentDeserializerState::Let(_, Some(deserializer))) => {
                    PatternContentDeserializerState::Let(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PatternContentDeserializerState::Let(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PatternContentDeserializer::store_let_(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PatternContentDeserializer::store_let_(&mut values, data)?;
                let data = PatternContentDeserializer::finish_state(
                    reader,
                    PatternContentDeserializerState::Let(values, None),
                )?;
                *self.state = PatternContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PatternContentDeserializerState::Let(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_rule<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Rule>,
        output: DeserializerOutput<'de, super::Rule>,
        fallback: &mut Option<PatternContentDeserializerState>,
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
                None => PatternContentDeserializerState::Rule(values, None),
                Some(PatternContentDeserializerState::Rule(_, Some(deserializer))) => {
                    PatternContentDeserializerState::Rule(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PatternContentDeserializerState::Rule(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PatternContentDeserializer::store_rule(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PatternContentDeserializer::store_rule(&mut values, data)?;
                let data = PatternContentDeserializer::finish_state(
                    reader,
                    PatternContentDeserializerState::Rule(values, None),
                )?;
                *self.state = PatternContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PatternContentDeserializerState::Rule(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::PatternContent> for Box<PatternContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PatternContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(PatternContentDeserializer {
            state: Box::new(PatternContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, PatternContentDeserializerState::Init__) =>
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
    ) -> DeserializerResult<'de, super::PatternContent>
    where
        R: DeserializeReader,
    {
        use PatternContentDeserializerState as S;
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
                (S::Title(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_title(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::P(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_p(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Param(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_param(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Let(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_let_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Rule(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_rule(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            PatternContentDeserializer::finish_state(reader, state)?,
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
                (S::Title(values, None), event) => {
                    let output =
                        <super::Title as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_title(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::P(values, None), event) => {
                    let output = <super::P as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_p(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Param(values, None), event) => {
                    let output =
                        <super::Param as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_param(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Let(values, None), event) => {
                    let output =
                        <super::Let as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_let_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Rule(values, None), event) => {
                    let output =
                        <super::Rule as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_rule(reader, values, output, &mut fallback)? {
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
    fn finish<R>(self, reader: &R) -> Result<super::PatternContent, Error>
    where
        R: DeserializeReader,
    {
        PatternContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct DiagnosticsDeserializer {
    content: Vec<super::DiagnosticsContent>,
    state: Box<DiagnosticsDeserializerState>,
}
#[derive(Debug)]
enum DiagnosticsDeserializerState {
    Init__,
    Next__,
    Content__(<super::DiagnosticsContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl DiagnosticsDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        Ok(Box::new(Self {
            content: Vec::new(),
            state: Box::new(DiagnosticsDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: DiagnosticsDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let DiagnosticsDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::DiagnosticsContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::DiagnosticsContent>,
        fallback: &mut Option<DiagnosticsDeserializerState>,
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
                .unwrap_or(DiagnosticsDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = DiagnosticsDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = DiagnosticsDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(DiagnosticsDeserializerState::Content__(deserializer));
                        *self.state = DiagnosticsDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Diagnostics> for Box<DiagnosticsDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Diagnostics>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, DiagnosticsDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::Diagnostics>
    where
        R: DeserializeReader,
    {
        use DiagnosticsDeserializerState as S;
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
                        <super::DiagnosticsContent as WithDeserializer>::Deserializer::init(
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Diagnostics, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, DiagnosticsDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Diagnostics {
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct DiagnosticsContentDeserializer {
    state: Box<DiagnosticsContentDeserializerState>,
}
#[derive(Debug)]
pub enum DiagnosticsContentDeserializerState {
    Init__,
    Include(
        Option<super::Include>,
        Option<<super::Include as WithDeserializer>::Deserializer>,
    ),
    Diagnostic(
        Option<super::Diagnostic>,
        Option<<super::Diagnostic as WithDeserializer>::Deserializer>,
    ),
    Done__(super::DiagnosticsContent),
    Unknown__,
}
impl DiagnosticsContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<DiagnosticsContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"include")
            ) {
                let output =
                    <super::Include as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_include(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"diagnostic")
            ) {
                let output =
                    <super::Diagnostic as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_diagnostic(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(DiagnosticsContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: DiagnosticsContentDeserializerState,
    ) -> Result<super::DiagnosticsContent, Error>
    where
        R: DeserializeReader,
    {
        use DiagnosticsContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Include(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    DiagnosticsContentDeserializer::store_include(&mut values, value)?;
                }
                Ok(super::DiagnosticsContent::Include(values.ok_or_else(
                    || ErrorKind::MissingElement("include".into()),
                )?))
            }
            S::Diagnostic(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    DiagnosticsContentDeserializer::store_diagnostic(&mut values, value)?;
                }
                Ok(super::DiagnosticsContent::Diagnostic(values.ok_or_else(
                    || ErrorKind::MissingElement("diagnostic".into()),
                )?))
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
    fn store_diagnostic(
        values: &mut Option<super::Diagnostic>,
        value: super::Diagnostic,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"diagnostic",
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
        fallback: &mut Option<DiagnosticsContentDeserializerState>,
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
                None => DiagnosticsContentDeserializerState::Include(values, None),
                Some(DiagnosticsContentDeserializerState::Include(_, Some(deserializer))) => {
                    DiagnosticsContentDeserializerState::Include(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(DiagnosticsContentDeserializerState::Include(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                DiagnosticsContentDeserializer::store_include(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                DiagnosticsContentDeserializer::store_include(&mut values, data)?;
                let data = DiagnosticsContentDeserializer::finish_state(
                    reader,
                    DiagnosticsContentDeserializerState::Include(values, None),
                )?;
                *self.state = DiagnosticsContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    DiagnosticsContentDeserializerState::Include(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_diagnostic<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Diagnostic>,
        output: DeserializerOutput<'de, super::Diagnostic>,
        fallback: &mut Option<DiagnosticsContentDeserializerState>,
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
                None => DiagnosticsContentDeserializerState::Diagnostic(values, None),
                Some(DiagnosticsContentDeserializerState::Diagnostic(_, Some(deserializer))) => {
                    DiagnosticsContentDeserializerState::Diagnostic(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(DiagnosticsContentDeserializerState::Diagnostic(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                DiagnosticsContentDeserializer::store_diagnostic(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                DiagnosticsContentDeserializer::store_diagnostic(&mut values, data)?;
                let data = DiagnosticsContentDeserializer::finish_state(
                    reader,
                    DiagnosticsContentDeserializerState::Diagnostic(values, None),
                )?;
                *self.state = DiagnosticsContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    DiagnosticsContentDeserializerState::Diagnostic(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::DiagnosticsContent> for Box<DiagnosticsContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DiagnosticsContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(DiagnosticsContentDeserializer {
            state: Box::new(DiagnosticsContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, DiagnosticsContentDeserializerState::Init__) =>
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
    ) -> DeserializerResult<'de, super::DiagnosticsContent>
    where
        R: DeserializeReader,
    {
        use DiagnosticsContentDeserializerState as S;
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
                (S::Diagnostic(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_diagnostic(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            DiagnosticsContentDeserializer::finish_state(reader, state)?,
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
                (S::Diagnostic(values, None), event) => {
                    let output =
                        <super::Diagnostic as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_diagnostic(reader, values, output, &mut fallback)? {
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
    fn finish<R>(self, reader: &R) -> Result<super::DiagnosticsContent, Error>
    where
        R: DeserializeReader,
    {
        DiagnosticsContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct PropertiesDeserializer {
    property: Vec<super::Property>,
    state: Box<PropertiesDeserializerState>,
}
#[derive(Debug)]
enum PropertiesDeserializerState {
    Init__,
    Property(Option<<super::Property as WithDeserializer>::Deserializer>),
    Done__,
    Unknown__,
}
impl PropertiesDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            reader.raise_unexpected_attrib_checked(attrib)?;
        }
        Ok(Box::new(Self {
            property: Vec::new(),
            state: Box::new(PropertiesDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: PropertiesDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        use PropertiesDeserializerState as S;
        match state {
            S::Property(Some(deserializer)) => self.store_property(deserializer.finish(reader)?)?,
            _ => (),
        }
        Ok(())
    }
    fn store_property(&mut self, value: super::Property) -> Result<(), Error> {
        self.property.push(value);
        Ok(())
    }
    fn handle_property<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::Property>,
        fallback: &mut Option<PropertiesDeserializerState>,
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
            fallback.get_or_insert(PropertiesDeserializerState::Property(None));
            *self.state = PropertiesDeserializerState::Done__;
            return Ok(ElementHandlerOutput::from_event(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_property(data)?;
                *self.state = PropertiesDeserializerState::Property(None);
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(PropertiesDeserializerState::Property(Some(
                            deserializer,
                        )));
                        *self.state = PropertiesDeserializerState::Property(None);
                    }
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = PropertiesDeserializerState::Property(Some(deserializer));
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Properties> for Box<PropertiesDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Properties>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, PropertiesDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::Properties>
    where
        R: DeserializeReader,
    {
        use PropertiesDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let mut allow_any_element = false;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Property(Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_property(reader, output, &mut fallback)? {
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
                    *self.state = PropertiesDeserializerState::Property(None);
                    event
                }
                (S::Property(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                    if reader.check_start_tag_name(&event, Some(&super::super::NS_SCH), b"property")
                    {
                        let output = <super::Property as WithDeserializer>::Deserializer::init(
                            reader, event,
                        )?;
                        match self.handle_property(reader, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Property(None));
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Properties, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, PropertiesDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Properties {
            property: self.property,
        })
    }
}
#[derive(Debug)]
pub struct DirDeserializer {
    value: Option<super::DirValueType>,
    state: Box<DirDeserializerState>,
}
#[derive(Debug)]
enum DirDeserializerState {
    Init__,
    Unknown__,
}
impl DirDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut value: Option<super::DirValueType> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"value")
            ) {
                reader.read_attrib(&mut value, b"value", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            value: value,
            state: Box::new(DirDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: DirDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        Ok(())
    }
}
impl<'de> Deserializer<'de, super::Dir> for Box<DirDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Dir>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, DirDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Dir>
    where
        R: DeserializeReader,
    {
        if let Event::End(_) = &event {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Data(self.finish(reader)?),
                event: DeserializerEvent::None,
                allow_any: false,
            })
        } else {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: true,
            })
        }
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Dir, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, DirDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Dir { value: self.value })
    }
}
#[derive(Debug)]
pub struct EmphDeserializer {
    state: Box<EmphDeserializerState>,
}
#[derive(Debug)]
enum EmphDeserializerState {
    Init__,
    Unknown__,
}
impl EmphDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            reader.raise_unexpected_attrib_checked(attrib)?;
        }
        Ok(Box::new(Self {
            state: Box::new(EmphDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: EmphDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        Ok(())
    }
}
impl<'de> Deserializer<'de, super::Emph> for Box<EmphDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Emph>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, EmphDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Emph>
    where
        R: DeserializeReader,
    {
        if let Event::End(_) = &event {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Data(self.finish(reader)?),
                event: DeserializerEvent::None,
                allow_any: false,
            })
        } else {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: false,
            })
        }
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Emph, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, EmphDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Emph {})
    }
}
#[derive(Debug)]
pub struct SpanDeserializer {
    class: String,
    state: Box<SpanDeserializerState>,
}
#[derive(Debug)]
enum SpanDeserializerState {
    Init__,
    Unknown__,
}
impl SpanDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut class: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"class")
            ) {
                reader.read_attrib(&mut class, b"class", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            class: class
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("class".into())))?,
            state: Box::new(SpanDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: SpanDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        Ok(())
    }
}
impl<'de> Deserializer<'de, super::Span> for Box<SpanDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Span>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, SpanDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Span>
    where
        R: DeserializeReader,
    {
        if let Event::End(_) = &event {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Data(self.finish(reader)?),
                event: DeserializerEvent::None,
                allow_any: false,
            })
        } else {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: true,
            })
        }
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Span, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, SpanDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Span { class: self.class })
    }
}
#[derive(Debug)]
pub struct ActiveDeserializer {
    pattern: String,
    content: Vec<super::ActiveContent>,
    state: Box<ActiveDeserializerState>,
}
#[derive(Debug)]
enum ActiveDeserializerState {
    Init__,
    Next__,
    Content__(<super::ActiveContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl ActiveDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut pattern: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"pattern")
            ) {
                reader.read_attrib(&mut pattern, b"pattern", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            pattern: pattern
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("pattern".into())))?,
            content: Vec::new(),
            state: Box::new(ActiveDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: ActiveDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let ActiveDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::ActiveContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::ActiveContent>,
        fallback: &mut Option<ActiveDeserializerState>,
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
            *self.state = fallback.take().unwrap_or(ActiveDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = ActiveDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = ActiveDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(ActiveDeserializerState::Content__(deserializer));
                        *self.state = ActiveDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Active> for Box<ActiveDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Active>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, ActiveDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Active>
    where
        R: DeserializeReader,
    {
        use ActiveDeserializerState as S;
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
                    let output = <super::ActiveContent as WithDeserializer>::Deserializer::init(
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Active, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, ActiveDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Active {
            pattern: self.pattern,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct ActiveContentDeserializer {
    state: Box<ActiveContentDeserializerState>,
}
#[derive(Debug)]
pub enum ActiveContentDeserializerState {
    Init__,
    Dir(
        Option<super::Dir>,
        Option<<super::Dir as WithDeserializer>::Deserializer>,
    ),
    Emph(
        Option<super::Emph>,
        Option<<super::Emph as WithDeserializer>::Deserializer>,
    ),
    Span(
        Option<super::Span>,
        Option<<super::Span as WithDeserializer>::Deserializer>,
    ),
    Done__(super::ActiveContent),
    Unknown__,
}
impl ActiveContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<ActiveContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"dir")
            ) {
                let output = <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_dir(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"emph")
            ) {
                let output = <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_emph(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"span")
            ) {
                let output = <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_span(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(ActiveContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: ActiveContentDeserializerState,
    ) -> Result<super::ActiveContent, Error>
    where
        R: DeserializeReader,
    {
        use ActiveContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Dir(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ActiveContentDeserializer::store_dir(&mut values, value)?;
                }
                Ok(super::ActiveContent::Dir(
                    values.ok_or_else(|| ErrorKind::MissingElement("dir".into()))?,
                ))
            }
            S::Emph(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ActiveContentDeserializer::store_emph(&mut values, value)?;
                }
                Ok(super::ActiveContent::Emph(
                    values.ok_or_else(|| ErrorKind::MissingElement("emph".into()))?,
                ))
            }
            S::Span(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ActiveContentDeserializer::store_span(&mut values, value)?;
                }
                Ok(super::ActiveContent::Span(
                    values.ok_or_else(|| ErrorKind::MissingElement("span".into()))?,
                ))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_dir(values: &mut Option<super::Dir>, value: super::Dir) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"dir")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_emph(values: &mut Option<super::Emph>, value: super::Emph) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"emph")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_span(values: &mut Option<super::Span>, value: super::Span) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"span")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_dir<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Dir>,
        output: DeserializerOutput<'de, super::Dir>,
        fallback: &mut Option<ActiveContentDeserializerState>,
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
                None => ActiveContentDeserializerState::Dir(values, None),
                Some(ActiveContentDeserializerState::Dir(_, Some(deserializer))) => {
                    ActiveContentDeserializerState::Dir(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ActiveContentDeserializerState::Dir(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ActiveContentDeserializer::store_dir(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ActiveContentDeserializer::store_dir(&mut values, data)?;
                let data = ActiveContentDeserializer::finish_state(
                    reader,
                    ActiveContentDeserializerState::Dir(values, None),
                )?;
                *self.state = ActiveContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ActiveContentDeserializerState::Dir(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_emph<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Emph>,
        output: DeserializerOutput<'de, super::Emph>,
        fallback: &mut Option<ActiveContentDeserializerState>,
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
                None => ActiveContentDeserializerState::Emph(values, None),
                Some(ActiveContentDeserializerState::Emph(_, Some(deserializer))) => {
                    ActiveContentDeserializerState::Emph(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ActiveContentDeserializerState::Emph(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ActiveContentDeserializer::store_emph(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ActiveContentDeserializer::store_emph(&mut values, data)?;
                let data = ActiveContentDeserializer::finish_state(
                    reader,
                    ActiveContentDeserializerState::Emph(values, None),
                )?;
                *self.state = ActiveContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ActiveContentDeserializerState::Emph(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_span<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Span>,
        output: DeserializerOutput<'de, super::Span>,
        fallback: &mut Option<ActiveContentDeserializerState>,
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
                None => ActiveContentDeserializerState::Span(values, None),
                Some(ActiveContentDeserializerState::Span(_, Some(deserializer))) => {
                    ActiveContentDeserializerState::Span(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ActiveContentDeserializerState::Span(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ActiveContentDeserializer::store_span(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ActiveContentDeserializer::store_span(&mut values, data)?;
                let data = ActiveContentDeserializer::finish_state(
                    reader,
                    ActiveContentDeserializerState::Span(values, None),
                )?;
                *self.state = ActiveContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ActiveContentDeserializerState::Span(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::ActiveContent> for Box<ActiveContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ActiveContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(ActiveContentDeserializer {
            state: Box::new(ActiveContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, ActiveContentDeserializerState::Init__) =>
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
    ) -> DeserializerResult<'de, super::ActiveContent>
    where
        R: DeserializeReader,
    {
        use ActiveContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Dir(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            ActiveContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Dir(values, None), event) => {
                    let output =
                        <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, None), event) => {
                    let output =
                        <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, None), event) => {
                    let output =
                        <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
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
    fn finish<R>(self, reader: &R) -> Result<super::ActiveContent, Error>
    where
        R: DeserializeReader,
    {
        ActiveContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct ParamDeserializer {
    name: String,
    value: String,
    state: Box<ParamDeserializerState>,
}
#[derive(Debug)]
enum ParamDeserializerState {
    Init__,
    Unknown__,
}
impl ParamDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut name: Option<String> = None;
        let mut value: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"name")
            ) {
                reader.read_attrib(&mut name, b"name", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"value")
            ) {
                reader.read_attrib(&mut value, b"value", &attrib.value)?;
            } else {
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
        }
        Ok(Box::new(Self {
            name: name
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("name".into())))?,
            value: value
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("value".into())))?,
            state: Box::new(ParamDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: ParamDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        Ok(())
    }
}
impl<'de> Deserializer<'de, super::Param> for Box<ParamDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Param>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, ParamDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Param>
    where
        R: DeserializeReader,
    {
        if let Event::End(_) = &event {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Data(self.finish(reader)?),
                event: DeserializerEvent::None,
                allow_any: false,
            })
        } else {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: false,
            })
        }
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Param, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, ParamDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Param {
            name: self.name,
            value: self.value,
        })
    }
}
#[derive(Debug)]
pub struct RuleDeserializer {
    flag: Option<String>,
    icon: Option<String>,
    see: Option<String>,
    fpi: Option<String>,
    lang: Option<String>,
    space: Option<super::super::xml::Space>,
    role: Option<String>,
    subject: Option<String>,
    abstract_: Option<super::PatternAbstractType>,
    id: Option<String>,
    context: Option<String>,
    content: Vec<super::RuleContent>,
    state: Box<RuleDeserializerState>,
}
#[derive(Debug)]
enum RuleDeserializerState {
    Init__,
    Next__,
    Content__(<super::RuleContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl RuleDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut flag: Option<String> = None;
        let mut icon: Option<String> = None;
        let mut see: Option<String> = None;
        let mut fpi: Option<String> = None;
        let mut lang: Option<String> = None;
        let mut space: Option<super::super::xml::Space> = None;
        let mut role: Option<String> = None;
        let mut subject: Option<String> = None;
        let mut abstract_: Option<super::PatternAbstractType> = None;
        let mut id: Option<String> = None;
        let mut context: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"flag")
            ) {
                reader.read_attrib(&mut flag, b"flag", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"icon")
            ) {
                reader.read_attrib(&mut icon, b"icon", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"see")
            ) {
                reader.read_attrib(&mut see, b"see", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"fpi")
            ) {
                reader.read_attrib(&mut fpi, b"fpi", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"lang")
            ) {
                reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"space")
            ) {
                reader.read_attrib(&mut space, b"space", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"role")
            ) {
                reader.read_attrib(&mut role, b"role", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"subject")
            ) {
                reader.read_attrib(&mut subject, b"subject", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"abstract")
            ) {
                reader.read_attrib(&mut abstract_, b"abstract", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"context")
            ) {
                reader.read_attrib(&mut context, b"context", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            flag: flag,
            icon: icon,
            see: see,
            fpi: fpi,
            lang: lang,
            space: space,
            role: role,
            subject: subject,
            abstract_: abstract_,
            id: id,
            context: context,
            content: Vec::new(),
            state: Box::new(RuleDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: RuleDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let RuleDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::RuleContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::RuleContent>,
        fallback: &mut Option<RuleDeserializerState>,
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
            *self.state = fallback.take().unwrap_or(RuleDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = RuleDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = RuleDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(RuleDeserializerState::Content__(deserializer));
                        *self.state = RuleDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Rule> for Box<RuleDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Rule>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, RuleDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Rule>
    where
        R: DeserializeReader,
    {
        use RuleDeserializerState as S;
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
                    let output = <super::RuleContent as WithDeserializer>::Deserializer::init(
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Rule, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, RuleDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Rule {
            flag: self.flag,
            icon: self.icon,
            see: self.see,
            fpi: self.fpi,
            lang: self.lang,
            space: self.space,
            role: self.role,
            subject: self.subject,
            abstract_: self.abstract_,
            id: self.id,
            context: self.context,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct RuleContentDeserializer {
    state: Box<RuleContentDeserializerState>,
}
#[derive(Debug)]
pub enum RuleContentDeserializerState {
    Init__,
    Include(
        Option<super::Include>,
        Option<<super::Include as WithDeserializer>::Deserializer>,
    ),
    Let(
        Option<super::Let>,
        Option<<super::Let as WithDeserializer>::Deserializer>,
    ),
    Assert(
        Option<super::Assert>,
        Option<<super::Assert as WithDeserializer>::Deserializer>,
    ),
    Report(
        Option<super::Report>,
        Option<<super::Report as WithDeserializer>::Deserializer>,
    ),
    Extends(
        Option<super::Extends>,
        Option<<super::Extends as WithDeserializer>::Deserializer>,
    ),
    Done__(super::RuleContent),
    Unknown__,
}
impl RuleContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<RuleContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"include")
            ) {
                let output =
                    <super::Include as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_include(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"let")
            ) {
                let output = <super::Let as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_let_(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"assert")
            ) {
                let output =
                    <super::Assert as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_assert(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"report")
            ) {
                let output =
                    <super::Report as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_report(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"extends")
            ) {
                let output =
                    <super::Extends as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_extends(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(RuleContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: RuleContentDeserializerState,
    ) -> Result<super::RuleContent, Error>
    where
        R: DeserializeReader,
    {
        use RuleContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Include(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RuleContentDeserializer::store_include(&mut values, value)?;
                }
                Ok(super::RuleContent::Include(values.ok_or_else(|| {
                    ErrorKind::MissingElement("include".into())
                })?))
            }
            S::Let(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RuleContentDeserializer::store_let_(&mut values, value)?;
                }
                Ok(super::RuleContent::Let(
                    values.ok_or_else(|| ErrorKind::MissingElement("let".into()))?,
                ))
            }
            S::Assert(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RuleContentDeserializer::store_assert(&mut values, value)?;
                }
                Ok(super::RuleContent::Assert(values.ok_or_else(|| {
                    ErrorKind::MissingElement("assert".into())
                })?))
            }
            S::Report(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RuleContentDeserializer::store_report(&mut values, value)?;
                }
                Ok(super::RuleContent::Report(values.ok_or_else(|| {
                    ErrorKind::MissingElement("report".into())
                })?))
            }
            S::Extends(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    RuleContentDeserializer::store_extends(&mut values, value)?;
                }
                Ok(super::RuleContent::Extends(values.ok_or_else(|| {
                    ErrorKind::MissingElement("extends".into())
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
    fn store_let_(values: &mut Option<super::Let>, value: super::Let) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"let")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_assert(values: &mut Option<super::Assert>, value: super::Assert) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"assert",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_report(values: &mut Option<super::Report>, value: super::Report) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"report",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_extends(
        values: &mut Option<super::Extends>,
        value: super::Extends,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"extends",
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
        fallback: &mut Option<RuleContentDeserializerState>,
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
                None => RuleContentDeserializerState::Include(values, None),
                Some(RuleContentDeserializerState::Include(_, Some(deserializer))) => {
                    RuleContentDeserializerState::Include(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RuleContentDeserializerState::Include(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RuleContentDeserializer::store_include(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RuleContentDeserializer::store_include(&mut values, data)?;
                let data = RuleContentDeserializer::finish_state(
                    reader,
                    RuleContentDeserializerState::Include(values, None),
                )?;
                *self.state = RuleContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = RuleContentDeserializerState::Include(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_let_<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Let>,
        output: DeserializerOutput<'de, super::Let>,
        fallback: &mut Option<RuleContentDeserializerState>,
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
                None => RuleContentDeserializerState::Let(values, None),
                Some(RuleContentDeserializerState::Let(_, Some(deserializer))) => {
                    RuleContentDeserializerState::Let(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RuleContentDeserializerState::Let(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RuleContentDeserializer::store_let_(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RuleContentDeserializer::store_let_(&mut values, data)?;
                let data = RuleContentDeserializer::finish_state(
                    reader,
                    RuleContentDeserializerState::Let(values, None),
                )?;
                *self.state = RuleContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = RuleContentDeserializerState::Let(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_assert<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Assert>,
        output: DeserializerOutput<'de, super::Assert>,
        fallback: &mut Option<RuleContentDeserializerState>,
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
                None => RuleContentDeserializerState::Assert(values, None),
                Some(RuleContentDeserializerState::Assert(_, Some(deserializer))) => {
                    RuleContentDeserializerState::Assert(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RuleContentDeserializerState::Assert(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RuleContentDeserializer::store_assert(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RuleContentDeserializer::store_assert(&mut values, data)?;
                let data = RuleContentDeserializer::finish_state(
                    reader,
                    RuleContentDeserializerState::Assert(values, None),
                )?;
                *self.state = RuleContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = RuleContentDeserializerState::Assert(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_report<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Report>,
        output: DeserializerOutput<'de, super::Report>,
        fallback: &mut Option<RuleContentDeserializerState>,
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
                None => RuleContentDeserializerState::Report(values, None),
                Some(RuleContentDeserializerState::Report(_, Some(deserializer))) => {
                    RuleContentDeserializerState::Report(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RuleContentDeserializerState::Report(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RuleContentDeserializer::store_report(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RuleContentDeserializer::store_report(&mut values, data)?;
                let data = RuleContentDeserializer::finish_state(
                    reader,
                    RuleContentDeserializerState::Report(values, None),
                )?;
                *self.state = RuleContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = RuleContentDeserializerState::Report(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_extends<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Extends>,
        output: DeserializerOutput<'de, super::Extends>,
        fallback: &mut Option<RuleContentDeserializerState>,
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
                None => RuleContentDeserializerState::Extends(values, None),
                Some(RuleContentDeserializerState::Extends(_, Some(deserializer))) => {
                    RuleContentDeserializerState::Extends(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(RuleContentDeserializerState::Extends(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                RuleContentDeserializer::store_extends(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                RuleContentDeserializer::store_extends(&mut values, data)?;
                let data = RuleContentDeserializer::finish_state(
                    reader,
                    RuleContentDeserializerState::Extends(values, None),
                )?;
                *self.state = RuleContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = RuleContentDeserializerState::Extends(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::RuleContent> for Box<RuleContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::RuleContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(RuleContentDeserializer {
            state: Box::new(RuleContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, RuleContentDeserializerState::Init__) =>
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
    ) -> DeserializerResult<'de, super::RuleContent>
    where
        R: DeserializeReader,
    {
        use RuleContentDeserializerState as S;
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
                (S::Let(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_let_(reader, values, output, &mut fallback)? {
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
                (S::Report(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_report(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Extends(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_extends(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            RuleContentDeserializer::finish_state(reader, state)?,
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
                (S::Let(values, None), event) => {
                    let output =
                        <super::Let as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_let_(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Assert(values, None), event) => {
                    let output =
                        <super::Assert as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_assert(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Report(values, None), event) => {
                    let output =
                        <super::Report as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_report(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Extends(values, None), event) => {
                    let output =
                        <super::Extends as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_extends(reader, values, output, &mut fallback)? {
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
    fn finish<R>(self, reader: &R) -> Result<super::RuleContent, Error>
    where
        R: DeserializeReader,
    {
        RuleContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct DiagnosticDeserializer {
    id: String,
    icon: Option<String>,
    see: Option<String>,
    fpi: Option<String>,
    lang: Option<String>,
    space: Option<super::super::xml::Space>,
    content: Vec<super::DiagnosticContent>,
    state: Box<DiagnosticDeserializerState>,
}
#[derive(Debug)]
enum DiagnosticDeserializerState {
    Init__,
    Next__,
    Content__(<super::DiagnosticContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl DiagnosticDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut icon: Option<String> = None;
        let mut see: Option<String> = None;
        let mut fpi: Option<String> = None;
        let mut lang: Option<String> = None;
        let mut space: Option<super::super::xml::Space> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"icon")
            ) {
                reader.read_attrib(&mut icon, b"icon", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"see")
            ) {
                reader.read_attrib(&mut see, b"see", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"fpi")
            ) {
                reader.read_attrib(&mut fpi, b"fpi", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"lang")
            ) {
                reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"space")
            ) {
                reader.read_attrib(&mut space, b"space", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id.ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("id".into())))?,
            icon: icon,
            see: see,
            fpi: fpi,
            lang: lang,
            space: space,
            content: Vec::new(),
            state: Box::new(DiagnosticDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(
        &mut self,
        reader: &R,
        state: DiagnosticDeserializerState,
    ) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let DiagnosticDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::DiagnosticContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::DiagnosticContent>,
        fallback: &mut Option<DiagnosticDeserializerState>,
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
                .unwrap_or(DiagnosticDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = DiagnosticDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = DiagnosticDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback
                            .get_or_insert(DiagnosticDeserializerState::Content__(deserializer));
                        *self.state = DiagnosticDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Diagnostic> for Box<DiagnosticDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Diagnostic>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, DiagnosticDeserializer::from_bytes_start)
    }
    fn next<R>(
        mut self,
        reader: &R,
        event: Event<'de>,
    ) -> DeserializerResult<'de, super::Diagnostic>
    where
        R: DeserializeReader,
    {
        use DiagnosticDeserializerState as S;
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
                        <super::DiagnosticContent as WithDeserializer>::Deserializer::init(
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Diagnostic, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, DiagnosticDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Diagnostic {
            id: self.id,
            icon: self.icon,
            see: self.see,
            fpi: self.fpi,
            lang: self.lang,
            space: self.space,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct DiagnosticContentDeserializer {
    state: Box<DiagnosticContentDeserializerState>,
}
#[derive(Debug)]
pub enum DiagnosticContentDeserializerState {
    Init__,
    ValueOf(
        Option<super::ValueOf>,
        Option<<super::ValueOf as WithDeserializer>::Deserializer>,
    ),
    Emph(
        Option<super::Emph>,
        Option<<super::Emph as WithDeserializer>::Deserializer>,
    ),
    Dir(
        Option<super::Dir>,
        Option<<super::Dir as WithDeserializer>::Deserializer>,
    ),
    Span(
        Option<super::Span>,
        Option<<super::Span as WithDeserializer>::Deserializer>,
    ),
    Done__(super::DiagnosticContent),
    Unknown__,
}
impl DiagnosticContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<DiagnosticContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"value-of")
            ) {
                let output =
                    <super::ValueOf as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_value_of(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"emph")
            ) {
                let output = <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_emph(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"dir")
            ) {
                let output = <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_dir(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"span")
            ) {
                let output = <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_span(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(DiagnosticContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: DiagnosticContentDeserializerState,
    ) -> Result<super::DiagnosticContent, Error>
    where
        R: DeserializeReader,
    {
        use DiagnosticContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::ValueOf(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    DiagnosticContentDeserializer::store_value_of(&mut values, value)?;
                }
                Ok(super::DiagnosticContent::ValueOf(values.ok_or_else(
                    || ErrorKind::MissingElement("value-of".into()),
                )?))
            }
            S::Emph(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    DiagnosticContentDeserializer::store_emph(&mut values, value)?;
                }
                Ok(super::DiagnosticContent::Emph(
                    values.ok_or_else(|| ErrorKind::MissingElement("emph".into()))?,
                ))
            }
            S::Dir(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    DiagnosticContentDeserializer::store_dir(&mut values, value)?;
                }
                Ok(super::DiagnosticContent::Dir(
                    values.ok_or_else(|| ErrorKind::MissingElement("dir".into()))?,
                ))
            }
            S::Span(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    DiagnosticContentDeserializer::store_span(&mut values, value)?;
                }
                Ok(super::DiagnosticContent::Span(
                    values.ok_or_else(|| ErrorKind::MissingElement("span".into()))?,
                ))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_value_of(
        values: &mut Option<super::ValueOf>,
        value: super::ValueOf,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"value-of",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_emph(values: &mut Option<super::Emph>, value: super::Emph) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"emph")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_dir(values: &mut Option<super::Dir>, value: super::Dir) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"dir")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_span(values: &mut Option<super::Span>, value: super::Span) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"span")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_value_of<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ValueOf>,
        output: DeserializerOutput<'de, super::ValueOf>,
        fallback: &mut Option<DiagnosticContentDeserializerState>,
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
                None => DiagnosticContentDeserializerState::ValueOf(values, None),
                Some(DiagnosticContentDeserializerState::ValueOf(_, Some(deserializer))) => {
                    DiagnosticContentDeserializerState::ValueOf(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(DiagnosticContentDeserializerState::ValueOf(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                DiagnosticContentDeserializer::store_value_of(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                DiagnosticContentDeserializer::store_value_of(&mut values, data)?;
                let data = DiagnosticContentDeserializer::finish_state(
                    reader,
                    DiagnosticContentDeserializerState::ValueOf(values, None),
                )?;
                *self.state = DiagnosticContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state =
                    DiagnosticContentDeserializerState::ValueOf(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_emph<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Emph>,
        output: DeserializerOutput<'de, super::Emph>,
        fallback: &mut Option<DiagnosticContentDeserializerState>,
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
                None => DiagnosticContentDeserializerState::Emph(values, None),
                Some(DiagnosticContentDeserializerState::Emph(_, Some(deserializer))) => {
                    DiagnosticContentDeserializerState::Emph(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(DiagnosticContentDeserializerState::Emph(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                DiagnosticContentDeserializer::store_emph(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                DiagnosticContentDeserializer::store_emph(&mut values, data)?;
                let data = DiagnosticContentDeserializer::finish_state(
                    reader,
                    DiagnosticContentDeserializerState::Emph(values, None),
                )?;
                *self.state = DiagnosticContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = DiagnosticContentDeserializerState::Emph(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_dir<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Dir>,
        output: DeserializerOutput<'de, super::Dir>,
        fallback: &mut Option<DiagnosticContentDeserializerState>,
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
                None => DiagnosticContentDeserializerState::Dir(values, None),
                Some(DiagnosticContentDeserializerState::Dir(_, Some(deserializer))) => {
                    DiagnosticContentDeserializerState::Dir(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(DiagnosticContentDeserializerState::Dir(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                DiagnosticContentDeserializer::store_dir(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                DiagnosticContentDeserializer::store_dir(&mut values, data)?;
                let data = DiagnosticContentDeserializer::finish_state(
                    reader,
                    DiagnosticContentDeserializerState::Dir(values, None),
                )?;
                *self.state = DiagnosticContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = DiagnosticContentDeserializerState::Dir(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_span<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Span>,
        output: DeserializerOutput<'de, super::Span>,
        fallback: &mut Option<DiagnosticContentDeserializerState>,
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
                None => DiagnosticContentDeserializerState::Span(values, None),
                Some(DiagnosticContentDeserializerState::Span(_, Some(deserializer))) => {
                    DiagnosticContentDeserializerState::Span(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(DiagnosticContentDeserializerState::Span(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                DiagnosticContentDeserializer::store_span(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                DiagnosticContentDeserializer::store_span(&mut values, data)?;
                let data = DiagnosticContentDeserializer::finish_state(
                    reader,
                    DiagnosticContentDeserializerState::Span(values, None),
                )?;
                *self.state = DiagnosticContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = DiagnosticContentDeserializerState::Span(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::DiagnosticContent> for Box<DiagnosticContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DiagnosticContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(DiagnosticContentDeserializer {
            state: Box::new(DiagnosticContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, DiagnosticContentDeserializerState::Init__) =>
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
    ) -> DeserializerResult<'de, super::DiagnosticContent>
    where
        R: DeserializeReader,
    {
        use DiagnosticContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::ValueOf(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_value_of(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Dir(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            DiagnosticContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::ValueOf(values, None), event) => {
                    let output =
                        <super::ValueOf as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_value_of(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, None), event) => {
                    let output =
                        <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Dir(values, None), event) => {
                    let output =
                        <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, None), event) => {
                    let output =
                        <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
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
    fn finish<R>(self, reader: &R) -> Result<super::DiagnosticContent, Error>
    where
        R: DeserializeReader,
    {
        DiagnosticContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct PropertyDeserializer {
    id: String,
    role: Option<String>,
    scheme: Option<String>,
    content: Vec<super::PropertyContent>,
    state: Box<PropertyDeserializerState>,
}
#[derive(Debug)]
enum PropertyDeserializerState {
    Init__,
    Next__,
    Content__(<super::PropertyContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl PropertyDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut id: Option<String> = None;
        let mut role: Option<String> = None;
        let mut scheme: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"role")
            ) {
                reader.read_attrib(&mut role, b"role", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"scheme")
            ) {
                reader.read_attrib(&mut scheme, b"scheme", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            id: id.ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("id".into())))?,
            role: role,
            scheme: scheme,
            content: Vec::new(),
            state: Box::new(PropertyDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: PropertyDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let PropertyDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::PropertyContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::PropertyContent>,
        fallback: &mut Option<PropertyDeserializerState>,
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
            *self.state = fallback.take().unwrap_or(PropertyDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = PropertyDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = PropertyDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(PropertyDeserializerState::Content__(deserializer));
                        *self.state = PropertyDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Property> for Box<PropertyDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Property>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, PropertyDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Property>
    where
        R: DeserializeReader,
    {
        use PropertyDeserializerState as S;
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
                    let output = <super::PropertyContent as WithDeserializer>::Deserializer::init(
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Property, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, PropertyDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Property {
            id: self.id,
            role: self.role,
            scheme: self.scheme,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct PropertyContentDeserializer {
    state: Box<PropertyContentDeserializerState>,
}
#[derive(Debug)]
pub enum PropertyContentDeserializerState {
    Init__,
    Name(
        Option<super::Name>,
        Option<<super::Name as WithDeserializer>::Deserializer>,
    ),
    ValueOf(
        Option<super::ValueOf>,
        Option<<super::ValueOf as WithDeserializer>::Deserializer>,
    ),
    Emph(
        Option<super::Emph>,
        Option<<super::Emph as WithDeserializer>::Deserializer>,
    ),
    Dir(
        Option<super::Dir>,
        Option<<super::Dir as WithDeserializer>::Deserializer>,
    ),
    Span(
        Option<super::Span>,
        Option<<super::Span as WithDeserializer>::Deserializer>,
    ),
    Done__(super::PropertyContent),
    Unknown__,
}
impl PropertyContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<PropertyContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"name")
            ) {
                let output = <super::Name as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_name(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"value-of")
            ) {
                let output =
                    <super::ValueOf as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_value_of(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"emph")
            ) {
                let output = <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_emph(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"dir")
            ) {
                let output = <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_dir(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"span")
            ) {
                let output = <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_span(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(PropertyContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: PropertyContentDeserializerState,
    ) -> Result<super::PropertyContent, Error>
    where
        R: DeserializeReader,
    {
        use PropertyContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Name(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PropertyContentDeserializer::store_name(&mut values, value)?;
                }
                Ok(super::PropertyContent::Name(
                    values.ok_or_else(|| ErrorKind::MissingElement("name".into()))?,
                ))
            }
            S::ValueOf(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PropertyContentDeserializer::store_value_of(&mut values, value)?;
                }
                Ok(super::PropertyContent::ValueOf(values.ok_or_else(
                    || ErrorKind::MissingElement("value-of".into()),
                )?))
            }
            S::Emph(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PropertyContentDeserializer::store_emph(&mut values, value)?;
                }
                Ok(super::PropertyContent::Emph(
                    values.ok_or_else(|| ErrorKind::MissingElement("emph".into()))?,
                ))
            }
            S::Dir(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PropertyContentDeserializer::store_dir(&mut values, value)?;
                }
                Ok(super::PropertyContent::Dir(
                    values.ok_or_else(|| ErrorKind::MissingElement("dir".into()))?,
                ))
            }
            S::Span(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    PropertyContentDeserializer::store_span(&mut values, value)?;
                }
                Ok(super::PropertyContent::Span(
                    values.ok_or_else(|| ErrorKind::MissingElement("span".into()))?,
                ))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_name(values: &mut Option<super::Name>, value: super::Name) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"name")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_value_of(
        values: &mut Option<super::ValueOf>,
        value: super::ValueOf,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"value-of",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_emph(values: &mut Option<super::Emph>, value: super::Emph) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"emph")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_dir(values: &mut Option<super::Dir>, value: super::Dir) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"dir")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_span(values: &mut Option<super::Span>, value: super::Span) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"span")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_name<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Name>,
        output: DeserializerOutput<'de, super::Name>,
        fallback: &mut Option<PropertyContentDeserializerState>,
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
                None => PropertyContentDeserializerState::Name(values, None),
                Some(PropertyContentDeserializerState::Name(_, Some(deserializer))) => {
                    PropertyContentDeserializerState::Name(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PropertyContentDeserializerState::Name(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PropertyContentDeserializer::store_name(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PropertyContentDeserializer::store_name(&mut values, data)?;
                let data = PropertyContentDeserializer::finish_state(
                    reader,
                    PropertyContentDeserializerState::Name(values, None),
                )?;
                *self.state = PropertyContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PropertyContentDeserializerState::Name(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_value_of<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ValueOf>,
        output: DeserializerOutput<'de, super::ValueOf>,
        fallback: &mut Option<PropertyContentDeserializerState>,
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
                None => PropertyContentDeserializerState::ValueOf(values, None),
                Some(PropertyContentDeserializerState::ValueOf(_, Some(deserializer))) => {
                    PropertyContentDeserializerState::ValueOf(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PropertyContentDeserializerState::ValueOf(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PropertyContentDeserializer::store_value_of(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PropertyContentDeserializer::store_value_of(&mut values, data)?;
                let data = PropertyContentDeserializer::finish_state(
                    reader,
                    PropertyContentDeserializerState::ValueOf(values, None),
                )?;
                *self.state = PropertyContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PropertyContentDeserializerState::ValueOf(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_emph<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Emph>,
        output: DeserializerOutput<'de, super::Emph>,
        fallback: &mut Option<PropertyContentDeserializerState>,
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
                None => PropertyContentDeserializerState::Emph(values, None),
                Some(PropertyContentDeserializerState::Emph(_, Some(deserializer))) => {
                    PropertyContentDeserializerState::Emph(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PropertyContentDeserializerState::Emph(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PropertyContentDeserializer::store_emph(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PropertyContentDeserializer::store_emph(&mut values, data)?;
                let data = PropertyContentDeserializer::finish_state(
                    reader,
                    PropertyContentDeserializerState::Emph(values, None),
                )?;
                *self.state = PropertyContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PropertyContentDeserializerState::Emph(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_dir<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Dir>,
        output: DeserializerOutput<'de, super::Dir>,
        fallback: &mut Option<PropertyContentDeserializerState>,
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
                None => PropertyContentDeserializerState::Dir(values, None),
                Some(PropertyContentDeserializerState::Dir(_, Some(deserializer))) => {
                    PropertyContentDeserializerState::Dir(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PropertyContentDeserializerState::Dir(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PropertyContentDeserializer::store_dir(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PropertyContentDeserializer::store_dir(&mut values, data)?;
                let data = PropertyContentDeserializer::finish_state(
                    reader,
                    PropertyContentDeserializerState::Dir(values, None),
                )?;
                *self.state = PropertyContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PropertyContentDeserializerState::Dir(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_span<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Span>,
        output: DeserializerOutput<'de, super::Span>,
        fallback: &mut Option<PropertyContentDeserializerState>,
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
                None => PropertyContentDeserializerState::Span(values, None),
                Some(PropertyContentDeserializerState::Span(_, Some(deserializer))) => {
                    PropertyContentDeserializerState::Span(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(PropertyContentDeserializerState::Span(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                PropertyContentDeserializer::store_span(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                PropertyContentDeserializer::store_span(&mut values, data)?;
                let data = PropertyContentDeserializer::finish_state(
                    reader,
                    PropertyContentDeserializerState::Span(values, None),
                )?;
                *self.state = PropertyContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = PropertyContentDeserializerState::Span(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::PropertyContent> for Box<PropertyContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PropertyContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(PropertyContentDeserializer {
            state: Box::new(PropertyContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, PropertyContentDeserializerState::Init__) =>
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
    ) -> DeserializerResult<'de, super::PropertyContent>
    where
        R: DeserializeReader,
    {
        use PropertyContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Name(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_name(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ValueOf(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_value_of(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Dir(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            PropertyContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Name(values, None), event) => {
                    let output =
                        <super::Name as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_name(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ValueOf(values, None), event) => {
                    let output =
                        <super::ValueOf as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_value_of(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, None), event) => {
                    let output =
                        <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Dir(values, None), event) => {
                    let output =
                        <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, None), event) => {
                    let output =
                        <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
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
    fn finish<R>(self, reader: &R) -> Result<super::PropertyContent, Error>
    where
        R: DeserializeReader,
    {
        PropertyContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct AssertDeserializer {
    test: String,
    flag: Option<String>,
    id: Option<String>,
    diagnostics: Option<super::super::xs::EntitiesType>,
    properties: Option<super::super::xs::EntitiesType>,
    icon: Option<String>,
    see: Option<String>,
    fpi: Option<String>,
    lang: Option<String>,
    space: Option<super::super::xml::Space>,
    role: Option<String>,
    subject: Option<String>,
    content: Vec<super::AssertContent>,
    state: Box<AssertDeserializerState>,
}
#[derive(Debug)]
enum AssertDeserializerState {
    Init__,
    Next__,
    Content__(<super::AssertContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl AssertDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut test: Option<String> = None;
        let mut flag: Option<String> = None;
        let mut id: Option<String> = None;
        let mut diagnostics: Option<super::super::xs::EntitiesType> = None;
        let mut properties: Option<super::super::xs::EntitiesType> = None;
        let mut icon: Option<String> = None;
        let mut see: Option<String> = None;
        let mut fpi: Option<String> = None;
        let mut lang: Option<String> = None;
        let mut space: Option<super::super::xml::Space> = None;
        let mut role: Option<String> = None;
        let mut subject: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"test")
            ) {
                reader.read_attrib(&mut test, b"test", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"flag")
            ) {
                reader.read_attrib(&mut flag, b"flag", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"diagnostics")
            ) {
                reader.read_attrib(&mut diagnostics, b"diagnostics", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"properties")
            ) {
                reader.read_attrib(&mut properties, b"properties", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"icon")
            ) {
                reader.read_attrib(&mut icon, b"icon", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"see")
            ) {
                reader.read_attrib(&mut see, b"see", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"fpi")
            ) {
                reader.read_attrib(&mut fpi, b"fpi", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"lang")
            ) {
                reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"space")
            ) {
                reader.read_attrib(&mut space, b"space", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"role")
            ) {
                reader.read_attrib(&mut role, b"role", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"subject")
            ) {
                reader.read_attrib(&mut subject, b"subject", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            test: test
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("test".into())))?,
            flag: flag,
            id: id,
            diagnostics: diagnostics,
            properties: properties,
            icon: icon,
            see: see,
            fpi: fpi,
            lang: lang,
            space: space,
            role: role,
            subject: subject,
            content: Vec::new(),
            state: Box::new(AssertDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: AssertDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let AssertDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::AssertContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::AssertContent>,
        fallback: &mut Option<AssertDeserializerState>,
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
            *self.state = fallback.take().unwrap_or(AssertDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = AssertDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = AssertDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(AssertDeserializerState::Content__(deserializer));
                        *self.state = AssertDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Assert> for Box<AssertDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Assert>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, AssertDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Assert>
    where
        R: DeserializeReader,
    {
        use AssertDeserializerState as S;
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
                    let output = <super::AssertContent as WithDeserializer>::Deserializer::init(
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Assert, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, AssertDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Assert {
            test: self.test,
            flag: self.flag,
            id: self.id,
            diagnostics: self.diagnostics,
            properties: self.properties,
            icon: self.icon,
            see: self.see,
            fpi: self.fpi,
            lang: self.lang,
            space: self.space,
            role: self.role,
            subject: self.subject,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct AssertContentDeserializer {
    state: Box<AssertContentDeserializerState>,
}
#[derive(Debug)]
pub enum AssertContentDeserializerState {
    Init__,
    Name(
        Option<super::Name>,
        Option<<super::Name as WithDeserializer>::Deserializer>,
    ),
    ValueOf(
        Option<super::ValueOf>,
        Option<<super::ValueOf as WithDeserializer>::Deserializer>,
    ),
    Emph(
        Option<super::Emph>,
        Option<<super::Emph as WithDeserializer>::Deserializer>,
    ),
    Dir(
        Option<super::Dir>,
        Option<<super::Dir as WithDeserializer>::Deserializer>,
    ),
    Span(
        Option<super::Span>,
        Option<<super::Span as WithDeserializer>::Deserializer>,
    ),
    Done__(super::AssertContent),
    Unknown__,
}
impl AssertContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<AssertContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"name")
            ) {
                let output = <super::Name as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_name(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"value-of")
            ) {
                let output =
                    <super::ValueOf as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_value_of(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"emph")
            ) {
                let output = <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_emph(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"dir")
            ) {
                let output = <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_dir(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"span")
            ) {
                let output = <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_span(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(AssertContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: AssertContentDeserializerState,
    ) -> Result<super::AssertContent, Error>
    where
        R: DeserializeReader,
    {
        use AssertContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Name(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AssertContentDeserializer::store_name(&mut values, value)?;
                }
                Ok(super::AssertContent::Name(
                    values.ok_or_else(|| ErrorKind::MissingElement("name".into()))?,
                ))
            }
            S::ValueOf(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AssertContentDeserializer::store_value_of(&mut values, value)?;
                }
                Ok(super::AssertContent::ValueOf(values.ok_or_else(|| {
                    ErrorKind::MissingElement("value-of".into())
                })?))
            }
            S::Emph(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AssertContentDeserializer::store_emph(&mut values, value)?;
                }
                Ok(super::AssertContent::Emph(
                    values.ok_or_else(|| ErrorKind::MissingElement("emph".into()))?,
                ))
            }
            S::Dir(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AssertContentDeserializer::store_dir(&mut values, value)?;
                }
                Ok(super::AssertContent::Dir(
                    values.ok_or_else(|| ErrorKind::MissingElement("dir".into()))?,
                ))
            }
            S::Span(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    AssertContentDeserializer::store_span(&mut values, value)?;
                }
                Ok(super::AssertContent::Span(
                    values.ok_or_else(|| ErrorKind::MissingElement("span".into()))?,
                ))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_name(values: &mut Option<super::Name>, value: super::Name) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"name")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_value_of(
        values: &mut Option<super::ValueOf>,
        value: super::ValueOf,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"value-of",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_emph(values: &mut Option<super::Emph>, value: super::Emph) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"emph")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_dir(values: &mut Option<super::Dir>, value: super::Dir) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"dir")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_span(values: &mut Option<super::Span>, value: super::Span) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"span")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_name<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Name>,
        output: DeserializerOutput<'de, super::Name>,
        fallback: &mut Option<AssertContentDeserializerState>,
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
                None => AssertContentDeserializerState::Name(values, None),
                Some(AssertContentDeserializerState::Name(_, Some(deserializer))) => {
                    AssertContentDeserializerState::Name(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AssertContentDeserializerState::Name(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AssertContentDeserializer::store_name(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AssertContentDeserializer::store_name(&mut values, data)?;
                let data = AssertContentDeserializer::finish_state(
                    reader,
                    AssertContentDeserializerState::Name(values, None),
                )?;
                *self.state = AssertContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = AssertContentDeserializerState::Name(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_value_of<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ValueOf>,
        output: DeserializerOutput<'de, super::ValueOf>,
        fallback: &mut Option<AssertContentDeserializerState>,
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
                None => AssertContentDeserializerState::ValueOf(values, None),
                Some(AssertContentDeserializerState::ValueOf(_, Some(deserializer))) => {
                    AssertContentDeserializerState::ValueOf(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AssertContentDeserializerState::ValueOf(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AssertContentDeserializer::store_value_of(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AssertContentDeserializer::store_value_of(&mut values, data)?;
                let data = AssertContentDeserializer::finish_state(
                    reader,
                    AssertContentDeserializerState::ValueOf(values, None),
                )?;
                *self.state = AssertContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = AssertContentDeserializerState::ValueOf(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_emph<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Emph>,
        output: DeserializerOutput<'de, super::Emph>,
        fallback: &mut Option<AssertContentDeserializerState>,
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
                None => AssertContentDeserializerState::Emph(values, None),
                Some(AssertContentDeserializerState::Emph(_, Some(deserializer))) => {
                    AssertContentDeserializerState::Emph(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AssertContentDeserializerState::Emph(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AssertContentDeserializer::store_emph(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AssertContentDeserializer::store_emph(&mut values, data)?;
                let data = AssertContentDeserializer::finish_state(
                    reader,
                    AssertContentDeserializerState::Emph(values, None),
                )?;
                *self.state = AssertContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = AssertContentDeserializerState::Emph(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_dir<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Dir>,
        output: DeserializerOutput<'de, super::Dir>,
        fallback: &mut Option<AssertContentDeserializerState>,
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
                None => AssertContentDeserializerState::Dir(values, None),
                Some(AssertContentDeserializerState::Dir(_, Some(deserializer))) => {
                    AssertContentDeserializerState::Dir(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AssertContentDeserializerState::Dir(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AssertContentDeserializer::store_dir(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AssertContentDeserializer::store_dir(&mut values, data)?;
                let data = AssertContentDeserializer::finish_state(
                    reader,
                    AssertContentDeserializerState::Dir(values, None),
                )?;
                *self.state = AssertContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = AssertContentDeserializerState::Dir(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_span<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Span>,
        output: DeserializerOutput<'de, super::Span>,
        fallback: &mut Option<AssertContentDeserializerState>,
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
                None => AssertContentDeserializerState::Span(values, None),
                Some(AssertContentDeserializerState::Span(_, Some(deserializer))) => {
                    AssertContentDeserializerState::Span(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(AssertContentDeserializerState::Span(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                AssertContentDeserializer::store_span(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                AssertContentDeserializer::store_span(&mut values, data)?;
                let data = AssertContentDeserializer::finish_state(
                    reader,
                    AssertContentDeserializerState::Span(values, None),
                )?;
                *self.state = AssertContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = AssertContentDeserializerState::Span(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::AssertContent> for Box<AssertContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AssertContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(AssertContentDeserializer {
            state: Box::new(AssertContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, AssertContentDeserializerState::Init__) =>
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
    ) -> DeserializerResult<'de, super::AssertContent>
    where
        R: DeserializeReader,
    {
        use AssertContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Name(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_name(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ValueOf(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_value_of(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Dir(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            AssertContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Name(values, None), event) => {
                    let output =
                        <super::Name as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_name(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ValueOf(values, None), event) => {
                    let output =
                        <super::ValueOf as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_value_of(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, None), event) => {
                    let output =
                        <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Dir(values, None), event) => {
                    let output =
                        <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, None), event) => {
                    let output =
                        <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
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
    fn finish<R>(self, reader: &R) -> Result<super::AssertContent, Error>
    where
        R: DeserializeReader,
    {
        AssertContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct ReportDeserializer {
    test: String,
    flag: Option<String>,
    id: Option<String>,
    diagnostics: Option<super::super::xs::EntitiesType>,
    properties: Option<super::super::xs::EntitiesType>,
    icon: Option<String>,
    see: Option<String>,
    fpi: Option<String>,
    lang: Option<String>,
    space: Option<super::super::xml::Space>,
    role: Option<String>,
    subject: Option<String>,
    content: Vec<super::ReportContent>,
    state: Box<ReportDeserializerState>,
}
#[derive(Debug)]
enum ReportDeserializerState {
    Init__,
    Next__,
    Content__(<super::ReportContent as WithDeserializer>::Deserializer),
    Unknown__,
}
impl ReportDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut test: Option<String> = None;
        let mut flag: Option<String> = None;
        let mut id: Option<String> = None;
        let mut diagnostics: Option<super::super::xs::EntitiesType> = None;
        let mut properties: Option<super::super::xs::EntitiesType> = None;
        let mut icon: Option<String> = None;
        let mut see: Option<String> = None;
        let mut fpi: Option<String> = None;
        let mut lang: Option<String> = None;
        let mut space: Option<super::super::xml::Space> = None;
        let mut role: Option<String> = None;
        let mut subject: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"test")
            ) {
                reader.read_attrib(&mut test, b"test", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"flag")
            ) {
                reader.read_attrib(&mut flag, b"flag", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"id")
            ) {
                reader.read_attrib(&mut id, b"id", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"diagnostics")
            ) {
                reader.read_attrib(&mut diagnostics, b"diagnostics", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"properties")
            ) {
                reader.read_attrib(&mut properties, b"properties", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"icon")
            ) {
                reader.read_attrib(&mut icon, b"icon", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"see")
            ) {
                reader.read_attrib(&mut see, b"see", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"fpi")
            ) {
                reader.read_attrib(&mut fpi, b"fpi", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"lang")
            ) {
                reader.read_attrib(&mut lang, b"lang", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_XML),
                Some(b"space")
            ) {
                reader.read_attrib(&mut space, b"space", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"role")
            ) {
                reader.read_attrib(&mut role, b"role", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"subject")
            ) {
                reader.read_attrib(&mut subject, b"subject", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            test: test
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("test".into())))?,
            flag: flag,
            id: id,
            diagnostics: diagnostics,
            properties: properties,
            icon: icon,
            see: see,
            fpi: fpi,
            lang: lang,
            space: space,
            role: role,
            subject: subject,
            content: Vec::new(),
            state: Box::new(ReportDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: ReportDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        if let ReportDeserializerState::Content__(deserializer) = state {
            self.store_content(deserializer.finish(reader)?)?;
        }
        Ok(())
    }
    fn store_content(&mut self, value: super::ReportContent) -> Result<(), Error> {
        self.content.push(value);
        Ok(())
    }
    fn handle_content<'de, R>(
        &mut self,
        reader: &R,
        output: DeserializerOutput<'de, super::ReportContent>,
        fallback: &mut Option<ReportDeserializerState>,
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
            *self.state = fallback.take().unwrap_or(ReportDeserializerState::Next__);
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        if let Some(fallback) = fallback.take() {
            self.finish_state(reader, fallback)?;
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                self.store_content(data)?;
                *self.state = ReportDeserializerState::Next__;
                ElementHandlerOutput::from_event(event, allow_any)
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                match &ret {
                    ElementHandlerOutput::Break { .. } => {
                        *self.state = ReportDeserializerState::Content__(deserializer);
                    }
                    ElementHandlerOutput::Continue { .. } => {
                        fallback.get_or_insert(ReportDeserializerState::Content__(deserializer));
                        *self.state = ReportDeserializerState::Next__;
                    }
                }
                ret
            }
        })
    }
}
impl<'de> Deserializer<'de, super::Report> for Box<ReportDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Report>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, ReportDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Report>
    where
        R: DeserializeReader,
    {
        use ReportDeserializerState as S;
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
                    let output = <super::ReportContent as WithDeserializer>::Deserializer::init(
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
    fn finish<R>(mut self, reader: &R) -> Result<super::Report, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, ReportDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Report {
            test: self.test,
            flag: self.flag,
            id: self.id,
            diagnostics: self.diagnostics,
            properties: self.properties,
            icon: self.icon,
            see: self.see,
            fpi: self.fpi,
            lang: self.lang,
            space: self.space,
            role: self.role,
            subject: self.subject,
            content: self.content,
        })
    }
}
#[derive(Debug)]
pub struct ReportContentDeserializer {
    state: Box<ReportContentDeserializerState>,
}
#[derive(Debug)]
pub enum ReportContentDeserializerState {
    Init__,
    Name(
        Option<super::Name>,
        Option<<super::Name as WithDeserializer>::Deserializer>,
    ),
    ValueOf(
        Option<super::ValueOf>,
        Option<<super::ValueOf as WithDeserializer>::Deserializer>,
    ),
    Emph(
        Option<super::Emph>,
        Option<<super::Emph as WithDeserializer>::Deserializer>,
    ),
    Dir(
        Option<super::Dir>,
        Option<<super::Dir as WithDeserializer>::Deserializer>,
    ),
    Span(
        Option<super::Span>,
        Option<<super::Span as WithDeserializer>::Deserializer>,
    ),
    Done__(super::ReportContent),
    Unknown__,
}
impl ReportContentDeserializer {
    fn find_suitable<'de, R>(
        &mut self,
        reader: &R,
        event: Event<'de>,
        fallback: &mut Option<ReportContentDeserializerState>,
    ) -> Result<ElementHandlerOutput<'de>, Error>
    where
        R: DeserializeReader,
    {
        if let Event::Start(x) | Event::Empty(x) = &event {
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"name")
            ) {
                let output = <super::Name as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_name(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"value-of")
            ) {
                let output =
                    <super::ValueOf as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_value_of(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"emph")
            ) {
                let output = <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_emph(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"dir")
            ) {
                let output = <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_dir(reader, Default::default(), output, &mut *fallback);
            }
            if matches!(
                reader.resolve_local_name(x.name(), &super::super::NS_SCH),
                Some(b"span")
            ) {
                let output = <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_span(reader, Default::default(), output, &mut *fallback);
            }
        }
        *self.state = fallback
            .take()
            .unwrap_or(ReportContentDeserializerState::Init__);
        Ok(ElementHandlerOutput::return_to_parent(event, true))
    }
    fn finish_state<R>(
        reader: &R,
        state: ReportContentDeserializerState,
    ) -> Result<super::ReportContent, Error>
    where
        R: DeserializeReader,
    {
        use ReportContentDeserializerState as S;
        match state {
            S::Init__ => Err(ErrorKind::MissingContent.into()),
            S::Name(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ReportContentDeserializer::store_name(&mut values, value)?;
                }
                Ok(super::ReportContent::Name(
                    values.ok_or_else(|| ErrorKind::MissingElement("name".into()))?,
                ))
            }
            S::ValueOf(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ReportContentDeserializer::store_value_of(&mut values, value)?;
                }
                Ok(super::ReportContent::ValueOf(values.ok_or_else(|| {
                    ErrorKind::MissingElement("value-of".into())
                })?))
            }
            S::Emph(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ReportContentDeserializer::store_emph(&mut values, value)?;
                }
                Ok(super::ReportContent::Emph(
                    values.ok_or_else(|| ErrorKind::MissingElement("emph".into()))?,
                ))
            }
            S::Dir(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ReportContentDeserializer::store_dir(&mut values, value)?;
                }
                Ok(super::ReportContent::Dir(
                    values.ok_or_else(|| ErrorKind::MissingElement("dir".into()))?,
                ))
            }
            S::Span(mut values, deserializer) => {
                if let Some(deserializer) = deserializer {
                    let value = deserializer.finish(reader)?;
                    ReportContentDeserializer::store_span(&mut values, value)?;
                }
                Ok(super::ReportContent::Span(
                    values.ok_or_else(|| ErrorKind::MissingElement("span".into()))?,
                ))
            }
            S::Done__(data) => Ok(data),
            S::Unknown__ => unreachable!(),
        }
    }
    fn store_name(values: &mut Option<super::Name>, value: super::Name) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"name")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_value_of(
        values: &mut Option<super::ValueOf>,
        value: super::ValueOf,
    ) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                b"value-of",
            )))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_emph(values: &mut Option<super::Emph>, value: super::Emph) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"emph")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_dir(values: &mut Option<super::Dir>, value: super::Dir) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"dir")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn store_span(values: &mut Option<super::Span>, value: super::Span) -> Result<(), Error> {
        if values.is_some() {
            Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"span")))?;
        }
        *values = Some(value);
        Ok(())
    }
    fn handle_name<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Name>,
        output: DeserializerOutput<'de, super::Name>,
        fallback: &mut Option<ReportContentDeserializerState>,
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
                None => ReportContentDeserializerState::Name(values, None),
                Some(ReportContentDeserializerState::Name(_, Some(deserializer))) => {
                    ReportContentDeserializerState::Name(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ReportContentDeserializerState::Name(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ReportContentDeserializer::store_name(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ReportContentDeserializer::store_name(&mut values, data)?;
                let data = ReportContentDeserializer::finish_state(
                    reader,
                    ReportContentDeserializerState::Name(values, None),
                )?;
                *self.state = ReportContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ReportContentDeserializerState::Name(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_value_of<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::ValueOf>,
        output: DeserializerOutput<'de, super::ValueOf>,
        fallback: &mut Option<ReportContentDeserializerState>,
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
                None => ReportContentDeserializerState::ValueOf(values, None),
                Some(ReportContentDeserializerState::ValueOf(_, Some(deserializer))) => {
                    ReportContentDeserializerState::ValueOf(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ReportContentDeserializerState::ValueOf(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ReportContentDeserializer::store_value_of(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ReportContentDeserializer::store_value_of(&mut values, data)?;
                let data = ReportContentDeserializer::finish_state(
                    reader,
                    ReportContentDeserializerState::ValueOf(values, None),
                )?;
                *self.state = ReportContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ReportContentDeserializerState::ValueOf(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_emph<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Emph>,
        output: DeserializerOutput<'de, super::Emph>,
        fallback: &mut Option<ReportContentDeserializerState>,
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
                None => ReportContentDeserializerState::Emph(values, None),
                Some(ReportContentDeserializerState::Emph(_, Some(deserializer))) => {
                    ReportContentDeserializerState::Emph(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ReportContentDeserializerState::Emph(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ReportContentDeserializer::store_emph(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ReportContentDeserializer::store_emph(&mut values, data)?;
                let data = ReportContentDeserializer::finish_state(
                    reader,
                    ReportContentDeserializerState::Emph(values, None),
                )?;
                *self.state = ReportContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ReportContentDeserializerState::Emph(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_dir<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Dir>,
        output: DeserializerOutput<'de, super::Dir>,
        fallback: &mut Option<ReportContentDeserializerState>,
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
                None => ReportContentDeserializerState::Dir(values, None),
                Some(ReportContentDeserializerState::Dir(_, Some(deserializer))) => {
                    ReportContentDeserializerState::Dir(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ReportContentDeserializerState::Dir(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ReportContentDeserializer::store_dir(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ReportContentDeserializer::store_dir(&mut values, data)?;
                let data = ReportContentDeserializer::finish_state(
                    reader,
                    ReportContentDeserializerState::Dir(values, None),
                )?;
                *self.state = ReportContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ReportContentDeserializerState::Dir(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
    fn handle_span<'de, R>(
        &mut self,
        reader: &R,
        mut values: Option<super::Span>,
        output: DeserializerOutput<'de, super::Span>,
        fallback: &mut Option<ReportContentDeserializerState>,
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
                None => ReportContentDeserializerState::Span(values, None),
                Some(ReportContentDeserializerState::Span(_, Some(deserializer))) => {
                    ReportContentDeserializerState::Span(values, Some(deserializer))
                }
                _ => unreachable!(),
            };
            return Ok(ElementHandlerOutput::break_(event, allow_any));
        }
        match fallback.take() {
            None => (),
            Some(ReportContentDeserializerState::Span(_, Some(deserializer))) => {
                let data = deserializer.finish(reader)?;
                ReportContentDeserializer::store_span(&mut values, data)?;
            }
            Some(_) => unreachable!(),
        }
        Ok(match artifact {
            DeserializerArtifact::None => unreachable!(),
            DeserializerArtifact::Data(data) => {
                ReportContentDeserializer::store_span(&mut values, data)?;
                let data = ReportContentDeserializer::finish_state(
                    reader,
                    ReportContentDeserializerState::Span(values, None),
                )?;
                *self.state = ReportContentDeserializerState::Done__(data);
                ElementHandlerOutput::Break { event, allow_any }
            }
            DeserializerArtifact::Deserializer(deserializer) => {
                *self.state = ReportContentDeserializerState::Span(values, Some(deserializer));
                ElementHandlerOutput::from_event_end(event, allow_any)
            }
        })
    }
}
impl<'de> Deserializer<'de, super::ReportContent> for Box<ReportContentDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ReportContent>
    where
        R: DeserializeReader,
    {
        let deserializer = Box::new(ReportContentDeserializer {
            state: Box::new(ReportContentDeserializerState::Init__),
        });
        let mut output = deserializer.next(reader, event)?;
        output.artifact = match output.artifact {
            DeserializerArtifact::Deserializer(x)
                if matches!(&*x.state, ReportContentDeserializerState::Init__) =>
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
    ) -> DeserializerResult<'de, super::ReportContent>
    where
        R: DeserializeReader,
    {
        use ReportContentDeserializerState as S;
        let mut event = event;
        let mut fallback = None;
        let (event, allow_any) = loop {
            let state = replace(&mut *self.state, S::Unknown__);
            event = match (state, event) {
                (S::Name(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_name(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ValueOf(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_value_of(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Dir(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, Some(deserializer)), event) => {
                    let output = deserializer.next(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (state, event @ Event::End(_)) => {
                    return Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(
                            ReportContentDeserializer::finish_state(reader, state)?,
                        ),
                        event: DeserializerEvent::Continue(event),
                        allow_any: false,
                    });
                }
                (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                    ElementHandlerOutput::Break { event, allow_any } => break (event, allow_any),
                    ElementHandlerOutput::Continue { event, .. } => event,
                },
                (S::Name(values, None), event) => {
                    let output =
                        <super::Name as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_name(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::ValueOf(values, None), event) => {
                    let output =
                        <super::ValueOf as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_value_of(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Emph(values, None), event) => {
                    let output =
                        <super::Emph as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_emph(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Dir(values, None), event) => {
                    let output =
                        <super::Dir as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_dir(reader, values, output, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    }
                }
                (S::Span(values, None), event) => {
                    let output =
                        <super::Span as WithDeserializer>::Deserializer::init(reader, event)?;
                    match self.handle_span(reader, values, output, &mut fallback)? {
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
    fn finish<R>(self, reader: &R) -> Result<super::ReportContent, Error>
    where
        R: DeserializeReader,
    {
        ReportContentDeserializer::finish_state(reader, *self.state)
    }
}
#[derive(Debug)]
pub struct ExtendsDeserializer {
    rule: Option<String>,
    href: Option<String>,
    state: Box<ExtendsDeserializerState>,
}
#[derive(Debug)]
enum ExtendsDeserializerState {
    Init__,
    Unknown__,
}
impl ExtendsDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut rule: Option<String> = None;
        let mut href: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"rule")
            ) {
                reader.read_attrib(&mut rule, b"rule", &attrib.value)?;
            } else if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"href")
            ) {
                reader.read_attrib(&mut href, b"href", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            rule: rule,
            href: href,
            state: Box::new(ExtendsDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: ExtendsDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        Ok(())
    }
}
impl<'de> Deserializer<'de, super::Extends> for Box<ExtendsDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Extends>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, ExtendsDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Extends>
    where
        R: DeserializeReader,
    {
        if let Event::End(_) = &event {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Data(self.finish(reader)?),
                event: DeserializerEvent::None,
                allow_any: false,
            })
        } else {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: false,
            })
        }
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Extends, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, ExtendsDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Extends {
            rule: self.rule,
            href: self.href,
        })
    }
}
#[derive(Debug)]
pub struct ValueOfDeserializer {
    select: String,
    state: Box<ValueOfDeserializerState>,
}
#[derive(Debug)]
enum ValueOfDeserializerState {
    Init__,
    Unknown__,
}
impl ValueOfDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut select: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"select")
            ) {
                reader.read_attrib(&mut select, b"select", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            select: select
                .ok_or_else(|| reader.map_error(ErrorKind::MissingAttribute("select".into())))?,
            state: Box::new(ValueOfDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: ValueOfDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        Ok(())
    }
}
impl<'de> Deserializer<'de, super::ValueOf> for Box<ValueOfDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ValueOf>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, ValueOfDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::ValueOf>
    where
        R: DeserializeReader,
    {
        if let Event::End(_) = &event {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Data(self.finish(reader)?),
                event: DeserializerEvent::None,
                allow_any: false,
            })
        } else {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: false,
            })
        }
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::ValueOf, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, ValueOfDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::ValueOf {
            select: self.select,
        })
    }
}
#[derive(Debug)]
pub struct NameDeserializer {
    path: Option<String>,
    state: Box<NameDeserializerState>,
}
#[derive(Debug)]
enum NameDeserializerState {
    Init__,
    Unknown__,
}
impl NameDeserializer {
    fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Box<Self>, Error>
    where
        R: DeserializeReader,
    {
        let mut path: Option<String> = None;
        for attrib in filter_xmlns_attributes(bytes_start) {
            let attrib = attrib?;
            if matches!(
                reader.resolve_local_name(attrib.key, &super::super::NS_SCH),
                Some(b"path")
            ) {
                reader.read_attrib(&mut path, b"path", &attrib.value)?;
            }
        }
        Ok(Box::new(Self {
            path: path,
            state: Box::new(NameDeserializerState::Init__),
        }))
    }
    fn finish_state<R>(&mut self, reader: &R, state: NameDeserializerState) -> Result<(), Error>
    where
        R: DeserializeReader,
    {
        Ok(())
    }
}
impl<'de> Deserializer<'de, super::Name> for Box<NameDeserializer> {
    fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Name>
    where
        R: DeserializeReader,
    {
        reader.init_deserializer_from_start_event(event, NameDeserializer::from_bytes_start)
    }
    fn next<R>(mut self, reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::Name>
    where
        R: DeserializeReader,
    {
        if let Event::End(_) = &event {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Data(self.finish(reader)?),
                event: DeserializerEvent::None,
                allow_any: false,
            })
        } else {
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event: DeserializerEvent::Break(event),
                allow_any: false,
            })
        }
    }
    fn finish<R>(mut self, reader: &R) -> Result<super::Name, Error>
    where
        R: DeserializeReader,
    {
        let state = replace(&mut *self.state, NameDeserializerState::Unknown__);
        self.finish_state(reader, state)?;
        Ok(super::Name { path: self.path })
    }
}
