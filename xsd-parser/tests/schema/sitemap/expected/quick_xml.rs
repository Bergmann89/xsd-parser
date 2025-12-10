use std::borrow::Cow;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        DeserializeBytes, DeserializeHelper, Error, ErrorKind, RawByteStr, SerializeBytes,
        SerializeHelper, WithDeserializer, WithSerializer,
    },
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_UNNAMED_2: Namespace =
    Namespace::new_const(b"http://www.sitemaps.org/schemas/sitemap/0.9");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
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
    fn serialize_bytes(&self, helper: &mut SerializeHelper) -> Result<Option<Cow<'_, str>>, Error> {
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
    fn deserialize_bytes(helper: &mut DeserializeHelper, bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            b"always" => Ok(Self::Always),
            b"hourly" => Ok(Self::Hourly),
            b"daily" => Ok(Self::Daily),
            b"weekly" => Ok(Self::Weekly),
            b"monthly" => Ok(Self::Monthly),
            b"yearly" => Ok(Self::Yearly),
            b"never" => Ok(Self::Never),
            x => Err(Error::from(ErrorKind::UnknownOrInvalidValue(
                RawByteStr::from_slice(x),
            ))),
        }
    }
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
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
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                url: Vec::new(),
                state__: Box::new(UrlsetTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: UrlsetTypeDeserializerState,
        ) -> Result<(), Error> {
            use UrlsetTypeDeserializerState as S;
            match state {
                S::Url(Some(deserializer)) => self.store_url(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_url(&mut self, value: super::TUrlType) -> Result<(), Error> {
            self.url.push(value);
            Ok(())
        }
        fn handle_url<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TUrlType>,
            fallback: &mut Option<UrlsetTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use UrlsetTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else if self.url.len() < 1usize {
                    fallback.get_or_insert(S::Url(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::Url(None));
                    *self.state__ = S::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_url(data)?;
                    *self.state__ = S::Url(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Url(Some(deserializer)));
                    *self.state__ = S::Url(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::UrlsetType> for UrlsetTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UrlsetType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UrlsetType> {
            use UrlsetTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Url(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_url(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Url(None);
                        event
                    }
                    (S::Url(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"url",
                            true,
                        )?;
                        match self.handle_url(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), true);
                    }
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::UrlsetType, Error> {
            let state = replace(&mut *self.state__, UrlsetTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::UrlsetType {
                url: helper.finish_vec(1usize, None, self.url)?,
            })
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
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                loc: None,
                lastmod: None,
                changefreq: None,
                priority: None,
                state__: Box::new(TUrlTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TUrlTypeDeserializerState,
        ) -> Result<(), Error> {
            use TUrlTypeDeserializerState as S;
            match state {
                S::Loc(Some(deserializer)) => self.store_loc(deserializer.finish(helper)?)?,
                S::Lastmod(Some(deserializer)) => {
                    self.store_lastmod(deserializer.finish(helper)?)?
                }
                S::Changefreq(Some(deserializer)) => {
                    self.store_changefreq(deserializer.finish(helper)?)?
                }
                S::Priority(Some(deserializer)) => {
                    self.store_priority(deserializer.finish(helper)?)?
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
        fn handle_loc<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<TUrlTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TUrlTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Loc(None));
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_loc(data)?;
                    *self.state__ = S::Lastmod(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Loc(Some(deserializer)));
                    *self.state__ = S::Lastmod(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_lastmod<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<TUrlTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TUrlTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Lastmod(None));
                *self.state__ = S::Changefreq(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_lastmod(data)?;
                    *self.state__ = S::Changefreq(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Lastmod(Some(deserializer)));
                    *self.state__ = S::Changefreq(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_changefreq<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TChangeFreqType>,
            fallback: &mut Option<TUrlTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TUrlTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Changefreq(None));
                *self.state__ = S::Priority(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_changefreq(data)?;
                    *self.state__ = S::Priority(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Changefreq(Some(deserializer)));
                    *self.state__ = S::Priority(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_priority<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, f64>,
            fallback: &mut Option<TUrlTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TUrlTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Priority(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_priority(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Priority(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TUrlType> for TUrlTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TUrlType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TUrlType> {
            use TUrlTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Loc(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_loc(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_lastmod(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_changefreq(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_priority(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        allow_any_element = true;
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Loc(None);
                        event
                    }
                    (S::Loc(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"loc",
                            true,
                        )?;
                        match self.handle_loc(helper, output, &mut fallback)? {
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
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"lastmod",
                            true,
                        )?;
                        match self.handle_lastmod(helper, output, &mut fallback)? {
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
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"changefreq",
                            true,
                        )?;
                        match self.handle_changefreq(helper, output, &mut fallback)? {
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
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"priority",
                            true,
                        )?;
                        match self.handle_priority(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Done__;
                        break (DeserializerEvent::Continue(event), true);
                    }
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::TUrlType, Error> {
            let state = replace(&mut *self.state__, TUrlTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::TUrlType {
                loc: helper.finish_element("loc", self.loc)?,
                lastmod: self.lastmod,
                changefreq: self.changefreq,
                priority: self.priority,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::{
        BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
        WithSerializer,
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
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    UrlsetTypeSerializerState::Init__ => {
                        *self.state = UrlsetTypeSerializerState::Url(IterSerializer::new(
                            &self.value.url[..],
                            Some("url"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    UrlsetTypeSerializerState::Url(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = UrlsetTypeSerializerState::End__,
                    },
                    UrlsetTypeSerializerState::End__ => {
                        *self.state = UrlsetTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    UrlsetTypeSerializerState::Done__ => return Ok(None),
                    UrlsetTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for UrlsetTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TUrlTypeSerializerState::Init__ => {
                        *self.state = TUrlTypeSerializerState::Loc(WithSerializer::serializer(
                            &self.value.loc,
                            Some("loc"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TUrlTypeSerializerState::Loc(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TUrlTypeSerializerState::Lastmod(IterSerializer::new(
                                self.value.lastmod.as_ref(),
                                Some("lastmod"),
                                false,
                            ))
                        }
                    },
                    TUrlTypeSerializerState::Lastmod(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TUrlTypeSerializerState::Changefreq(IterSerializer::new(
                                self.value.changefreq.as_ref(),
                                Some("changefreq"),
                                false,
                            ))
                        }
                    },
                    TUrlTypeSerializerState::Changefreq(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TUrlTypeSerializerState::Priority(IterSerializer::new(
                                self.value.priority.as_ref(),
                                Some("priority"),
                                false,
                            ))
                        }
                    },
                    TUrlTypeSerializerState::Priority(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TUrlTypeSerializerState::End__,
                    },
                    TUrlTypeSerializerState::End__ => {
                        *self.state = TUrlTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TUrlTypeSerializerState::Done__ => return Ok(None),
                    TUrlTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TUrlTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
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
