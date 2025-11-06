use std::borrow::Cow;
use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{
        DeserializeBytes, DeserializeReader, Error, ErrorKind, RawByteStr, SerializeBytes,
        WithDeserializer, WithSerializer,
    },
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_UNNAMED_2: Namespace =
    Namespace::new_const(b"http://www.sitemaps.org/schemas/sitemap/0.9");
pub type Urlset = UrlsetType;
#[derive(Debug)]
pub struct UrlsetType {
    pub url: Vec<TUrlType>,
}
impl WithSerializer for UrlsetType {
    type Serializer<'x> = quick_xml_serialize::UrlsetTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::UrlsetTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::UrlsetTypeSerializerState::Init__),
            name: name.unwrap_or("urlset"),
            is_root,
        })
    }
}
impl WithDeserializer for UrlsetType {
    type Deserializer = quick_xml_deserialize::UrlsetTypeDeserializer;
}
#[derive(Debug)]
pub struct TUrlType {
    pub loc: String,
    pub lastmod: Option<String>,
    pub changefreq: Option<TChangeFreqType>,
    pub priority: Option<f64>,
}
impl WithSerializer for TUrlType {
    type Serializer<'x> = quick_xml_serialize::TUrlTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TUrlTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TUrlTypeSerializerState::Init__),
            name: name.unwrap_or("tUrl"),
            is_root,
        })
    }
}
impl WithDeserializer for TUrlType {
    type Deserializer = quick_xml_deserialize::TUrlTypeDeserializer;
}
#[derive(Debug)]
pub enum TChangeFreqType {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}
impl SerializeBytes for TChangeFreqType {
    fn serialize_bytes(&self) -> Result<Option<Cow<'_, str>>, Error> {
        match self {
            Self::Always => Ok(Some(Cow::Borrowed("always"))),
            Self::Hourly => Ok(Some(Cow::Borrowed("hourly"))),
            Self::Daily => Ok(Some(Cow::Borrowed("daily"))),
            Self::Weekly => Ok(Some(Cow::Borrowed("weekly"))),
            Self::Monthly => Ok(Some(Cow::Borrowed("monthly"))),
            Self::Yearly => Ok(Some(Cow::Borrowed("yearly"))),
            Self::Never => Ok(Some(Cow::Borrowed("never"))),
        }
    }
}
impl DeserializeBytes for TChangeFreqType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"always" => Ok(Self::Always),
            b"hourly" => Ok(Self::Hourly),
            b"daily" => Ok(Self::Daily),
            b"weekly" => Ok(Self::Weekly),
            b"monthly" => Ok(Self::Monthly),
            b"yearly" => Ok(Self::Yearly),
            b"never" => Ok(Self::Never),
            x => Err(reader.map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct UrlsetTypeDeserializer {
        url: Vec<super::TUrlType>,
        state__: Box<UrlsetTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum UrlsetTypeDeserializerState {
        Init__,
        Url(Option<<super::TUrlType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl UrlsetTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                url: Vec::new(),
                state__: Box::new(UrlsetTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: UrlsetTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use UrlsetTypeDeserializerState as S;
            match state {
                S::Url(Some(deserializer)) => self.store_url(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_url(&mut self, value: super::TUrlType) -> Result<(), Error> {
            self.url.push(value);
            Ok(())
        }
        fn handle_url<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TUrlType>,
            fallback: &mut Option<UrlsetTypeDeserializerState>,
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
                if self.url.len() < 1usize {
                    *self.state__ = UrlsetTypeDeserializerState::Url(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(UrlsetTypeDeserializerState::Url(None));
                    *self.state__ = UrlsetTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_url(data)?;
                    *self.state__ = UrlsetTypeDeserializerState::Url(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(UrlsetTypeDeserializerState::Url(Some(
                                deserializer,
                            )));
                            if self.url.len().saturating_add(1) < 1usize {
                                *self.state__ = UrlsetTypeDeserializerState::Url(None);
                            } else {
                                *self.state__ = UrlsetTypeDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = UrlsetTypeDeserializerState::Url(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::UrlsetType> for UrlsetTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::UrlsetType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UrlsetType>
        where
            R: DeserializeReader,
        {
            use UrlsetTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Url(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_url(reader, output, &mut fallback)? {
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
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = UrlsetTypeDeserializerState::Url(None);
                        event
                    }
                    (S::Url(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"url",
                            true,
                        )?;
                        match self.handle_url(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), true);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::UrlsetType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state__, UrlsetTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::UrlsetType { url: self.url })
        }
    }
    #[derive(Debug)]
    pub struct TUrlTypeDeserializer {
        loc: Option<String>,
        lastmod: Option<String>,
        changefreq: Option<super::TChangeFreqType>,
        priority: Option<f64>,
        state__: Box<TUrlTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TUrlTypeDeserializerState {
        Init__,
        Loc(Option<<String as WithDeserializer>::Deserializer>),
        Lastmod(Option<<String as WithDeserializer>::Deserializer>),
        Changefreq(Option<<super::TChangeFreqType as WithDeserializer>::Deserializer>),
        Priority(Option<<f64 as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TUrlTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                loc: None,
                lastmod: None,
                changefreq: None,
                priority: None,
                state__: Box::new(TUrlTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TUrlTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TUrlTypeDeserializerState as S;
            match state {
                S::Loc(Some(deserializer)) => self.store_loc(deserializer.finish(reader)?)?,
                S::Lastmod(Some(deserializer)) => {
                    self.store_lastmod(deserializer.finish(reader)?)?
                }
                S::Changefreq(Some(deserializer)) => {
                    self.store_changefreq(deserializer.finish(reader)?)?
                }
                S::Priority(Some(deserializer)) => {
                    self.store_priority(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_loc(&mut self, value: String) -> Result<(), Error> {
            if self.loc.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"loc")))?;
            }
            self.loc = Some(value);
            Ok(())
        }
        fn store_lastmod(&mut self, value: String) -> Result<(), Error> {
            if self.lastmod.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"lastmod",
                )))?;
            }
            self.lastmod = Some(value);
            Ok(())
        }
        fn store_changefreq(&mut self, value: super::TChangeFreqType) -> Result<(), Error> {
            if self.changefreq.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"changefreq",
                )))?;
            }
            self.changefreq = Some(value);
            Ok(())
        }
        fn store_priority(&mut self, value: f64) -> Result<(), Error> {
            if self.priority.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"priority",
                )))?;
            }
            self.priority = Some(value);
            Ok(())
        }
        fn handle_loc<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<TUrlTypeDeserializerState>,
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
                if self.loc.is_some() {
                    fallback.get_or_insert(TUrlTypeDeserializerState::Loc(None));
                    *self.state__ = TUrlTypeDeserializerState::Lastmod(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = TUrlTypeDeserializerState::Loc(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_loc(data)?;
                    *self.state__ = TUrlTypeDeserializerState::Lastmod(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback
                                .get_or_insert(TUrlTypeDeserializerState::Loc(Some(deserializer)));
                            *self.state__ = TUrlTypeDeserializerState::Lastmod(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = TUrlTypeDeserializerState::Loc(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_lastmod<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<TUrlTypeDeserializerState>,
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
                fallback.get_or_insert(TUrlTypeDeserializerState::Lastmod(None));
                *self.state__ = TUrlTypeDeserializerState::Changefreq(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_lastmod(data)?;
                    *self.state__ = TUrlTypeDeserializerState::Changefreq(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TUrlTypeDeserializerState::Lastmod(Some(
                                deserializer,
                            )));
                            *self.state__ = TUrlTypeDeserializerState::Changefreq(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = TUrlTypeDeserializerState::Lastmod(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_changefreq<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TChangeFreqType>,
            fallback: &mut Option<TUrlTypeDeserializerState>,
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
                fallback.get_or_insert(TUrlTypeDeserializerState::Changefreq(None));
                *self.state__ = TUrlTypeDeserializerState::Priority(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_changefreq(data)?;
                    *self.state__ = TUrlTypeDeserializerState::Priority(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TUrlTypeDeserializerState::Changefreq(Some(
                                deserializer,
                            )));
                            *self.state__ = TUrlTypeDeserializerState::Priority(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                TUrlTypeDeserializerState::Changefreq(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_priority<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, f64>,
            fallback: &mut Option<TUrlTypeDeserializerState>,
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
                fallback.get_or_insert(TUrlTypeDeserializerState::Priority(None));
                *self.state__ = TUrlTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_priority(data)?;
                    *self.state__ = TUrlTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TUrlTypeDeserializerState::Priority(Some(
                                deserializer,
                            )));
                            *self.state__ = TUrlTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = TUrlTypeDeserializerState::Priority(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TUrlType> for TUrlTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TUrlType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TUrlType>
        where
            R: DeserializeReader,
        {
            use TUrlTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Loc(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_loc(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Lastmod(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_lastmod(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Changefreq(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_changefreq(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Priority(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_priority(reader, output, &mut fallback)? {
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
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = TUrlTypeDeserializerState::Loc(None);
                        event
                    }
                    (S::Loc(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"loc",
                            true,
                        )?;
                        match self.handle_loc(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Lastmod(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"lastmod",
                            true,
                        )?;
                        match self.handle_lastmod(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Changefreq(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"changefreq",
                            true,
                        )?;
                        match self.handle_changefreq(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Priority(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"priority",
                            true,
                        )?;
                        match self.handle_priority(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), true);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TUrlType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state__, TUrlTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::TUrlType {
                loc: self
                    .loc
                    .ok_or_else(|| ErrorKind::MissingElement("loc".into()))?,
                lastmod: self.lastmod,
                changefreq: self.changefreq,
                priority: self.priority,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct UrlsetTypeSerializer<'ser> {
        pub(super) value: &'ser super::UrlsetType,
        pub(super) state: Box<UrlsetTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum UrlsetTypeSerializerState<'ser> {
        Init__,
        Url(IterSerializer<'ser, &'ser [super::TUrlType], super::TUrlType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> UrlsetTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    UrlsetTypeSerializerState::Init__ => {
                        *self.state = UrlsetTypeSerializerState::Url(IterSerializer::new(
                            &self.value.url[..],
                            Some("url"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_UNNAMED_2[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    UrlsetTypeSerializerState::Url(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UrlsetTypeSerializerState::End__,
                    },
                    UrlsetTypeSerializerState::End__ => {
                        *self.state = UrlsetTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    UrlsetTypeSerializerState::Done__ => return Ok(None),
                    UrlsetTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for UrlsetTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = UrlsetTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TUrlTypeSerializer<'ser> {
        pub(super) value: &'ser super::TUrlType,
        pub(super) state: Box<TUrlTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TUrlTypeSerializerState<'ser> {
        Init__,
        Loc(<String as WithSerializer>::Serializer<'ser>),
        Lastmod(IterSerializer<'ser, Option<&'ser String>, String>),
        Changefreq(
            IterSerializer<'ser, Option<&'ser super::TChangeFreqType>, super::TChangeFreqType>,
        ),
        Priority(IterSerializer<'ser, Option<&'ser f64>, f64>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TUrlTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TUrlTypeSerializerState::Init__ => {
                        *self.state = TUrlTypeSerializerState::Loc(WithSerializer::serializer(
                            &self.value.loc,
                            Some("loc"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns"[..], &super::NS_UNNAMED_2[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TUrlTypeSerializerState::Loc(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TUrlTypeSerializerState::Lastmod(IterSerializer::new(
                                self.value.lastmod.as_ref(),
                                Some("lastmod"),
                                false,
                            ))
                        }
                    },
                    TUrlTypeSerializerState::Lastmod(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TUrlTypeSerializerState::Changefreq(IterSerializer::new(
                                self.value.changefreq.as_ref(),
                                Some("changefreq"),
                                false,
                            ))
                        }
                    },
                    TUrlTypeSerializerState::Changefreq(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TUrlTypeSerializerState::Priority(IterSerializer::new(
                                self.value.priority.as_ref(),
                                Some("priority"),
                                false,
                            ))
                        }
                    },
                    TUrlTypeSerializerState::Priority(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TUrlTypeSerializerState::End__,
                    },
                    TUrlTypeSerializerState::End__ => {
                        *self.state = TUrlTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TUrlTypeSerializerState::Done__ => return Ok(None),
                    TUrlTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TUrlTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TUrlTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
