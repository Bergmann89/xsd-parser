use xsd_parser_types::{misc::Namespace, quick_xml::WithDeserializer};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub type Reference = ReferenceType;
#[derive(Debug)]
pub struct ReferenceType {
    pub transforms: Option<TransformsType>,
    pub digest_method: String,
    pub digest_value: String,
}
impl WithDeserializer for ReferenceType {
    type Deserializer = quick_xml_deserialize::ReferenceTypeDeserializer;
}
#[derive(Debug)]
pub struct TransformsType {
    pub transform: Vec<TransformType>,
}
impl WithDeserializer for TransformsType {
    type Deserializer = quick_xml_deserialize::TransformsTypeDeserializer;
}
#[derive(Debug)]
pub struct TransformType {
    pub algorithm: String,
    pub content: Vec<TransformTypeContent>,
}
#[derive(Debug)]
pub enum TransformTypeContent {
    XPath(String),
}
impl WithDeserializer for TransformType {
    type Deserializer = quick_xml_deserialize::TransformTypeDeserializer;
}
impl WithDeserializer for TransformTypeContent {
    type Deserializer = quick_xml_deserialize::TransformTypeContentDeserializer;
}
pub type Signature = SignatureType;
#[derive(Debug)]
pub struct SignatureType {
    pub signed_info: Vec<SignedInfoType>,
    pub signature_value: String,
}
impl WithDeserializer for SignatureType {
    type Deserializer = quick_xml_deserialize::SignatureTypeDeserializer;
}
#[derive(Debug)]
pub struct SignedInfoType {
    pub canonicalization_method: String,
}
impl WithDeserializer for SignedInfoType {
    type Deserializer = quick_xml_deserialize::SignedInfoTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser_types::quick_xml::{
        BytesStart, DeserializeHelper, Deserializer, DeserializerArtifact, DeserializerEvent,
        DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event,
        RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct ReferenceTypeDeserializer {
        transforms: Option<super::TransformsType>,
        digest_method: Option<String>,
        digest_value: Option<String>,
        state__: Box<ReferenceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ReferenceTypeDeserializerState {
        Init__,
        Transforms(Option<<super::TransformsType as WithDeserializer>::Deserializer>),
        DigestMethod(Option<<String as WithDeserializer>::Deserializer>),
        DigestValue(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ReferenceTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                transforms: None,
                digest_method: None,
                digest_value: None,
                state__: Box::new(ReferenceTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ReferenceTypeDeserializerState,
        ) -> Result<(), Error> {
            use ReferenceTypeDeserializerState as S;
            match state {
                S::Transforms(Some(deserializer)) => {
                    self.store_transforms(deserializer.finish(helper)?)?
                }
                S::DigestMethod(Some(deserializer)) => {
                    self.store_digest_method(deserializer.finish(helper)?)?
                }
                S::DigestValue(Some(deserializer)) => {
                    self.store_digest_value(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_transforms(&mut self, value: super::TransformsType) -> Result<(), Error> {
            if self.transforms.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Transforms",
                )))?;
            }
            self.transforms = Some(value);
            Ok(())
        }
        fn store_digest_method(&mut self, value: String) -> Result<(), Error> {
            if self.digest_method.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DigestMethod",
                )))?;
            }
            self.digest_method = Some(value);
            Ok(())
        }
        fn store_digest_value(&mut self, value: String) -> Result<(), Error> {
            if self.digest_value.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DigestValue",
                )))?;
            }
            self.digest_value = Some(value);
            Ok(())
        }
        fn handle_transforms<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TransformsType>,
            fallback: &mut Option<ReferenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ReferenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Transforms(None));
                *self.state__ = S::DigestMethod(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_transforms(data)?;
                    *self.state__ = S::DigestMethod(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Transforms(Some(deserializer)));
                    *self.state__ = S::DigestMethod(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_digest_method<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ReferenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ReferenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::DigestMethod(None));
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
                    self.store_digest_method(data)?;
                    *self.state__ = S::DigestValue(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::DigestMethod(Some(deserializer)));
                    *self.state__ = S::DigestValue(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_digest_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ReferenceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ReferenceTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::DigestValue(None));
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
                    self.store_digest_value(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::DigestValue(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ReferenceType> for ReferenceTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferenceType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferenceType> {
            use ReferenceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Transforms(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_transforms(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DigestMethod(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_digest_method(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DigestValue(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_digest_value(helper, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Transforms(None);
                        event
                    }
                    (S::Transforms(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Transforms",
                            true,
                        )?;
                        match self.handle_transforms(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DigestMethod(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"DigestMethod",
                            false,
                        )?;
                        match self.handle_digest_method(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DigestValue(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"DigestValue",
                            false,
                        )?;
                        match self.handle_digest_value(helper, output, &mut fallback)? {
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
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            let (event, allow_any) =
                helper.decide_unknown_element::<super::ReferenceType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ReferenceType, Error> {
            let state = replace(
                &mut *self.state__,
                ReferenceTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ReferenceType {
                transforms: self.transforms,
                digest_method: helper.finish_element("DigestMethod", self.digest_method)?,
                digest_value: helper.finish_element("DigestValue", self.digest_value)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"Transforms" | b"DigestMethod" | b"DigestValue")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct TransformsTypeDeserializer {
        transform: Vec<super::TransformType>,
        state__: Box<TransformsTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TransformsTypeDeserializerState {
        Init__,
        Transform(Option<<super::TransformType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TransformsTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                transform: Vec::new(),
                state__: Box::new(TransformsTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TransformsTypeDeserializerState,
        ) -> Result<(), Error> {
            use TransformsTypeDeserializerState as S;
            match state {
                S::Transform(Some(deserializer)) => {
                    self.store_transform(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_transform(&mut self, value: super::TransformType) -> Result<(), Error> {
            self.transform.push(value);
            Ok(())
        }
        fn handle_transform<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TransformType>,
            fallback: &mut Option<TransformsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TransformsTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
                if let Some(fallback) = fallback.take() {
                    self.finish_state(helper, fallback)?;
                }
                if self.transform.len() < 1usize {
                    fallback.get_or_insert(S::Transform(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::Transform(None));
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
                    self.store_transform(data)?;
                    *self.state__ = S::Transform(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Transform(Some(deserializer)));
                    *self.state__ = S::Transform(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TransformsType> for TransformsTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformsType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformsType> {
            use TransformsTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Transform(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_transform(helper, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Transform(None);
                        event
                    }
                    (S::Transform(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Transform",
                            true,
                        )?;
                        match self.handle_transform(helper, output, &mut fallback)? {
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
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            let (event, allow_any) =
                helper.decide_unknown_element::<super::TransformsType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TransformsType, Error> {
            let state = replace(
                &mut *self.state__,
                TransformsTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TransformsType {
                transform: helper.finish_vec(1usize, None, self.transform)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"Transform")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeDeserializer {
        algorithm: String,
        content: Vec<super::TransformTypeContent>,
        state__: Box<TransformTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TransformTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::TransformTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TransformTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut algorithm: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_attrib_local_name(attrib.key, &super::NS_TNS),
                    Some(b"Algorithm")
                ) {
                    helper.read_attrib(&mut algorithm, b"Algorithm", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                algorithm: algorithm
                    .ok_or_else(|| ErrorKind::MissingAttribute("Algorithm".into()))?,
                content: Vec::new(),
                state__: Box::new(TransformTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TransformTypeDeserializerState,
        ) -> Result<(), Error> {
            if let TransformTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::TransformTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TransformTypeContent>,
            fallback: &mut Option<TransformTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TransformTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = fallback.take().unwrap_or(S::Next__);
                return Ok(ElementHandlerOutput::from_event_end(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *fallback = Some(S::Content__(deserializer));
                    *self.state__ = S::Next__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TransformType> for TransformTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformType> {
            use TransformTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::TransformTypeContent as WithDeserializer>::init(helper, event)?;
                        match self.handle_content(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                }
            };
            let (event, allow_any) =
                helper.decide_unknown_element::<super::TransformType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::TransformType, Error> {
            let state = replace(
                &mut *self.state__,
                TransformTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TransformType {
                algorithm: self.algorithm,
                content: helper.finish_vec(0usize, None, self.content)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if <super::TransformTypeContent as WithDeserializer>::Deserializer::is_known_start_tag(
                helper, x,
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct TransformTypeContentDeserializer {
        state__: Box<TransformTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum TransformTypeContentDeserializerState {
        Init__,
        XPath(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::TransformTypeContent),
        Unknown__,
    }
    impl TransformTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                    Some(b"XPath")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_x_path(helper, Default::default(), None, output);
                }
            }
            *self.state__ = TransformTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, true))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: TransformTypeContentDeserializerState,
        ) -> Result<super::TransformTypeContent, Error> {
            use TransformTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::XPath(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x_path(&mut values, value)?;
                    }
                    Ok(super::TransformTypeContent::XPath(
                        helper.finish_element("XPath", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_x_path(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"XPath",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_x_path<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TransformTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::XPath(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_x_path(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x_path(&mut values, data)?;
                    let data = Self::finish_state(helper, S::XPath(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::XPath(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TransformTypeContent> for TransformTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformTypeContent> {
            let deserializer = Self {
                state__: Box::new(TransformTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, TransformTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TransformTypeContent> {
            use TransformTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::XPath(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x_path(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::XPath(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"XPath",
                            false,
                        )?;
                        match self.handle_x_path(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state @ S::Done__(_), event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Continue(event), false);
                    }
                }
            };
            let artifact = if matches!(&*self.state__, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(helper)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish(
            self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::TransformTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"XPath")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct SignatureTypeDeserializer {
        signed_info: Vec<super::SignedInfoType>,
        signature_value: Option<String>,
        state__: Box<SignatureTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SignatureTypeDeserializerState {
        Init__,
        SignedInfo(Option<<super::SignedInfoType as WithDeserializer>::Deserializer>),
        SignatureValue(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SignatureTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                signed_info: Vec::new(),
                signature_value: None,
                state__: Box::new(SignatureTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SignatureTypeDeserializerState,
        ) -> Result<(), Error> {
            use SignatureTypeDeserializerState as S;
            match state {
                S::SignedInfo(Some(deserializer)) => {
                    self.store_signed_info(deserializer.finish(helper)?)?
                }
                S::SignatureValue(Some(deserializer)) => {
                    self.store_signature_value(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_signed_info(&mut self, value: super::SignedInfoType) -> Result<(), Error> {
            self.signed_info.push(value);
            Ok(())
        }
        fn store_signature_value(&mut self, value: String) -> Result<(), Error> {
            if self.signature_value.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SignatureValue",
                )))?;
            }
            self.signature_value = Some(value);
            Ok(())
        }
        fn handle_signed_info<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::SignedInfoType>,
            fallback: &mut Option<SignatureTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignatureTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
                if let Some(fallback) = fallback.take() {
                    self.finish_state(helper, fallback)?;
                }
                if self.signed_info.len() < 1usize {
                    fallback.get_or_insert(S::SignedInfo(None));
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                } else {
                    fallback.get_or_insert(S::SignedInfo(None));
                    *self.state__ = S::SignatureValue(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_signed_info(data)?;
                    *self.state__ = S::SignedInfo(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SignedInfo(Some(deserializer)));
                    *self.state__ = S::SignedInfo(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_signature_value<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<SignatureTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignatureTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::SignatureValue(None));
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
                    self.store_signature_value(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SignatureValue(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SignatureType> for SignatureTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignatureType> {
            use SignatureTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::SignedInfo(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_signed_info(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SignatureValue(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_signature_value(helper, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::SignedInfo(None);
                        event
                    }
                    (S::SignedInfo(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"SignedInfo",
                            false,
                        )?;
                        match self.handle_signed_info(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SignatureValue(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"SignatureValue",
                            false,
                        )?;
                        match self.handle_signature_value(helper, output, &mut fallback)? {
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
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            let (event, allow_any) =
                helper.decide_unknown_element::<super::SignatureType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::SignatureType, Error> {
            let state = replace(
                &mut *self.state__,
                SignatureTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SignatureType {
                signed_info: helper.finish_vec(1usize, None, self.signed_info)?,
                signature_value: helper.finish_element("SignatureValue", self.signature_value)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"SignedInfo" | b"SignatureValue")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct SignedInfoTypeDeserializer {
        canonicalization_method: Option<String>,
        state__: Box<SignedInfoTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SignedInfoTypeDeserializerState {
        Init__,
        CanonicalizationMethod(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SignedInfoTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                canonicalization_method: None,
                state__: Box::new(SignedInfoTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: SignedInfoTypeDeserializerState,
        ) -> Result<(), Error> {
            use SignedInfoTypeDeserializerState as S;
            match state {
                S::CanonicalizationMethod(Some(deserializer)) => {
                    self.store_canonicalization_method(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_canonicalization_method(&mut self, value: String) -> Result<(), Error> {
            if self.canonicalization_method.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CanonicalizationMethod",
                )))?;
            }
            self.canonicalization_method = Some(value);
            Ok(())
        }
        fn handle_canonicalization_method<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<SignedInfoTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SignedInfoTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::CanonicalizationMethod(None));
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
                    self.store_canonicalization_method(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::CanonicalizationMethod(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SignedInfoType> for SignedInfoTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignedInfoType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SignedInfoType> {
            use SignedInfoTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::CanonicalizationMethod(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_canonicalization_method(helper, output, &mut fallback)? {
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
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::CanonicalizationMethod(None);
                        event
                    }
                    (
                        S::CanonicalizationMethod(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"CanonicalizationMethod",
                            false,
                        )?;
                        match self.handle_canonicalization_method(helper, output, &mut fallback)? {
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
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            let (event, allow_any) =
                helper.decide_unknown_element::<super::SignedInfoType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut DeserializeHelper,
        ) -> Result<super::SignedInfoType, Error> {
            let state = replace(
                &mut *self.state__,
                SignedInfoTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::SignedInfoType {
                canonicalization_method: helper
                    .finish_element("CanonicalizationMethod", self.canonicalization_method)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_element_local_name(x.name(), &super::NS_TNS),
                Some(b"CanonicalizationMethod")
            ) {
                return true;
            }
            false
        }
    }
}
