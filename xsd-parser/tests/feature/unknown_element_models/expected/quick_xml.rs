use core::fmt::Debug;
use xsd_parser_types::{
    misc::{Namespace, NamespacePrefix},
    quick_xml::{
        BoxedSerializer, Error, WithBoxedSerializer, WithDeserializer, WithSerializer,
        XsiTypeSerializer,
    },
    xml::AnyElement,
    AsAny,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_XSI: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema-instance");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com");
pub const PREFIX_XS: NamespacePrefix = NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: NamespacePrefix = NamespacePrefix::new_const(b"xml");
pub const PREFIX_XSI: NamespacePrefix = NamespacePrefix::new_const(b"xsi");
pub const PREFIX_TNS: NamespacePrefix = NamespacePrefix::new_const(b"tns");
pub type Holder = HolderType;
#[derive(Debug)]
pub struct HolderType {
    pub simple: String,
    pub tail: Option<String>,
}
impl WithSerializer for HolderType {
    type Serializer<'x> = quick_xml_serialize::HolderTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::HolderTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::HolderTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Holder"),
            is_root,
        })
    }
}
impl WithDeserializer for HolderType {
    type Deserializer = quick_xml_deserialize::HolderTypeDeserializer;
}
pub type TextHolder = TextHolderType;
#[derive(Debug)]
pub struct TextHolderType {
    pub text: TextType,
    pub tail: Option<String>,
}
impl WithSerializer for TextHolderType {
    type Serializer<'x> = quick_xml_serialize::TextHolderTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TextHolderTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TextHolderTypeSerializerState::Init__),
            name: name.unwrap_or("tns:TextHolder"),
            is_root,
        })
    }
}
impl WithDeserializer for TextHolderType {
    type Deserializer = quick_xml_deserialize::TextHolderTypeDeserializer;
}
#[derive(Debug)]
pub struct TextType {
    pub id: Option<i32>,
    pub content: String,
}
impl WithSerializer for TextType {
    type Serializer<'x> = quick_xml_serialize::TextTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TextTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TextTypeSerializerState::Init__),
            name: name.unwrap_or("tns:TextType"),
            is_root,
        })
    }
}
impl WithDeserializer for TextType {
    type Deserializer = quick_xml_deserialize::TextTypeDeserializer;
}
pub type Mixed = MixedType;
#[derive(Debug)]
pub struct MixedType {
    pub a: String,
    pub b: Option<String>,
}
impl WithSerializer for MixedType {
    type Serializer<'x> = quick_xml_serialize::MixedTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::MixedTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Mixed"),
            is_root,
        })
    }
}
impl WithDeserializer for MixedType {
    type Deserializer = quick_xml_deserialize::MixedTypeDeserializer;
}
pub type AnyRoot = AnyRootType;
#[derive(Debug)]
pub struct AnyRootType {
    pub child: ChildType,
    pub any: Vec<AnyElement>,
}
impl WithSerializer for AnyRootType {
    type Serializer<'x> = quick_xml_serialize::AnyRootTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::AnyRootTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AnyRootTypeSerializerState::Init__),
            name: name.unwrap_or("tns:AnyRoot"),
            is_root,
        })
    }
}
impl WithDeserializer for AnyRootType {
    type Deserializer = quick_xml_deserialize::AnyRootTypeDeserializer;
}
#[derive(Debug)]
pub struct ChildType {
    pub inner: String,
}
impl WithSerializer for ChildType {
    type Serializer<'x> = quick_xml_serialize::ChildTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ChildTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ChildTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ChildType"),
            is_root,
        })
    }
}
impl WithDeserializer for ChildType {
    type Deserializer = quick_xml_deserialize::ChildTypeDeserializer;
}
pub type Zoo = ZooType;
#[derive(Debug)]
pub struct ZooType {
    pub name: String,
    pub animal: Vec<AnimalType>,
}
impl WithSerializer for ZooType {
    type Serializer<'x> = quick_xml_serialize::ZooTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ZooTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ZooTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Zoo"),
            is_root,
        })
    }
}
impl WithDeserializer for ZooType {
    type Deserializer = quick_xml_deserialize::ZooTypeDeserializer;
}
#[derive(Debug)]
pub struct AnimalType(pub Box<dyn AnimalTypeTrait>);
pub trait AnimalTypeTrait: Debug + AsAny + WithBoxedSerializer {}
impl AnimalType {
    pub fn new<T: AnimalTypeTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
impl WithSerializer for AnimalType {
    type Serializer<'x> = BoxedSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        if (*self.0).as_any().is::<DogType>() {
            return Ok(Box::new(XsiTypeSerializer::new(
                self.0.serializer(None, is_root)?,
                name.unwrap_or("tns:animal"),
                is_root,
            )));
        }
        self.0.serializer(None, is_root)
    }
}
impl WithDeserializer for AnimalType {
    type Deserializer = quick_xml_deserialize::AnimalTypeDeserializer;
}
#[derive(Debug)]
pub struct AnimalDyn {
    pub id: i32,
}
impl AnimalTypeTrait for AnimalDyn {}
impl WithSerializer for AnimalDyn {
    type Serializer<'x> = quick_xml_serialize::AnimalDynSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::AnimalDynSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AnimalDynSerializerState::Init__),
            name: name.unwrap_or("tns:animal"),
            is_root,
        })
    }
}
impl WithDeserializer for AnimalDyn {
    type Deserializer = quick_xml_deserialize::AnimalDynDeserializer;
}
#[derive(Debug)]
pub struct DogType {
    pub id: i32,
    pub name: String,
}
impl AnimalTypeTrait for DogType {}
impl WithSerializer for DogType {
    type Serializer<'x> = quick_xml_serialize::DogTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DogTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DogTypeSerializerState::Init__),
            name: name.unwrap_or("tns:dog"),
            is_root,
        })
    }
}
impl WithDeserializer for DogType {
    type Deserializer = quick_xml_deserialize::DogTypeDeserializer;
}
pub type Many = ManyType;
#[derive(Debug)]
pub struct ManyType {
    pub content: Vec<ManyTypeContent>,
}
#[derive(Debug)]
pub enum ManyTypeContent {
    X(String),
    Y(String),
}
impl WithSerializer for ManyType {
    type Serializer<'x> = quick_xml_serialize::ManyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ManyTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ManyTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Many"),
            is_root,
        })
    }
}
impl WithSerializer for ManyTypeContent {
    type Serializer<'x> = quick_xml_serialize::ManyTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::ManyTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ManyTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for ManyType {
    type Deserializer = quick_xml_deserialize::ManyTypeDeserializer;
}
impl WithDeserializer for ManyTypeContent {
    type Deserializer = quick_xml_deserialize::ManyTypeContentDeserializer;
}
pub type GroupPick = GroupPickType;
#[derive(Debug)]
pub enum GroupPickType {
    Pq(GroupPickPqType),
    Y(String),
}
impl WithSerializer for GroupPickType {
    type Serializer<'x> = quick_xml_serialize::GroupPickTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::GroupPickTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::GroupPickTypeSerializerState::Init__),
            name: name.unwrap_or("tns:GroupPick"),
            is_root,
        })
    }
}
impl WithDeserializer for GroupPickType {
    type Deserializer = quick_xml_deserialize::GroupPickTypeDeserializer;
}
#[derive(Debug)]
pub struct GroupPickPqType {
    pub p: String,
    pub q: String,
}
impl WithSerializer for GroupPickPqType {
    type Serializer<'x> = quick_xml_serialize::GroupPickPqTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::GroupPickPqTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::GroupPickPqTypeSerializerState::Init__),
        })
    }
}
impl WithDeserializer for GroupPickPqType {
    type Deserializer = quick_xml_deserialize::GroupPickPqTypeDeserializer;
}
pub type DynPick = DynPickType;
#[derive(Debug)]
pub enum DynPickType {
    Animal(AnimalType),
    Y(String),
}
impl WithSerializer for DynPickType {
    type Serializer<'x> = quick_xml_serialize::DynPickTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DynPickTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DynPickTypeSerializerState::Init__),
            name: name.unwrap_or("tns:DynPick"),
            is_root,
        })
    }
}
impl WithDeserializer for DynPickType {
    type Deserializer = quick_xml_deserialize::DynPickTypeDeserializer;
}
pub type ChoiceGroupPick = ChoiceGroupPickType;
#[derive(Debug)]
pub enum ChoiceGroupPickType {
    Uv(ChoiceGroupPickUvType),
    Y(String),
}
impl WithSerializer for ChoiceGroupPickType {
    type Serializer<'x> = quick_xml_serialize::ChoiceGroupPickTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ChoiceGroupPickTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ChoiceGroupPickTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ChoiceGroupPick"),
            is_root,
        })
    }
}
impl WithDeserializer for ChoiceGroupPickType {
    type Deserializer = quick_xml_deserialize::ChoiceGroupPickTypeDeserializer;
}
#[derive(Debug)]
pub enum ChoiceGroupPickUvType {
    U(String),
    V(String),
}
impl WithSerializer for ChoiceGroupPickUvType {
    type Serializer<'x> = quick_xml_serialize::ChoiceGroupPickUvTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        Ok(quick_xml_serialize::ChoiceGroupPickUvTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ChoiceGroupPickUvTypeSerializerState::Init__),
            is_root,
        })
    }
}
impl WithDeserializer for ChoiceGroupPickUvType {
    type Deserializer = quick_xml_deserialize::ChoiceGroupPickUvTypeDeserializer;
}
pub type SubstPick = SubstPickType;
#[derive(Debug)]
pub enum SubstPickType {
    Shape(Shape),
    Y(String),
}
impl WithSerializer for SubstPickType {
    type Serializer<'x> = quick_xml_serialize::SubstPickTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SubstPickTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SubstPickTypeSerializerState::Init__),
            name: name.unwrap_or("tns:SubstPick"),
            is_root,
        })
    }
}
impl WithDeserializer for SubstPickType {
    type Deserializer = quick_xml_deserialize::SubstPickTypeDeserializer;
}
#[derive(Debug)]
pub struct Shape(pub Box<dyn ShapeTrait>);
pub trait ShapeTrait: Debug + AsAny + WithBoxedSerializer {}
impl Shape {
    pub fn new<T: ShapeTrait + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }
}
impl WithSerializer for Shape {
    type Serializer<'x> = BoxedSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        if (*self.0).as_any().is::<ShapeType>() {
            return self.0.serializer(Some("tns:Circle"), is_root);
        }
        self.0.serializer(None, is_root)
    }
}
impl WithDeserializer for Shape {
    type Deserializer = quick_xml_deserialize::ShapeDeserializer;
}
#[derive(Debug)]
pub struct ShapeType {
    pub id: Option<i32>,
}
impl ShapeTrait for ShapeType {}
impl WithSerializer for ShapeType {
    type Serializer<'x> = quick_xml_serialize::ShapeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ShapeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ShapeTypeSerializerState::Init__),
            name: name.unwrap_or("tns:ShapeType"),
            is_root,
        })
    }
}
impl WithDeserializer for ShapeType {
    type Deserializer = quick_xml_deserialize::ShapeTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use std::borrow::Cow;
    use xsd_parser_types::{
        quick_xml::{
            BytesStart, ContentDeserializer, DeserializeHelper, Deserializer, DeserializerArtifact,
            DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
            ErrorKind, Event, QName, RawByteStr, WithDeserializer,
        },
        xml::AnyElement,
    };
    #[derive(Debug)]
    pub struct HolderTypeDeserializer {
        simple: Option<String>,
        tail: Option<String>,
        state__: Box<HolderTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HolderTypeDeserializerState {
        Init__,
        Simple(Option<<String as WithDeserializer>::Deserializer>),
        Tail(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl HolderTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                simple: None,
                tail: None,
                state__: Box::new(HolderTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: HolderTypeDeserializerState,
        ) -> Result<(), Error> {
            use HolderTypeDeserializerState as S;
            match state {
                S::Simple(Some(deserializer)) => self.store_simple(deserializer.finish(helper)?)?,
                S::Tail(Some(deserializer)) => self.store_tail(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_simple(&mut self, value: String) -> Result<(), Error> {
            if self.simple.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Simple",
                )))?;
            }
            self.simple = Some(value);
            Ok(())
        }
        fn store_tail(&mut self, value: String) -> Result<(), Error> {
            if self.tail.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Tail")))?;
            }
            self.tail = Some(value);
            Ok(())
        }
        fn handle_simple<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<HolderTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HolderTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Simple(None));
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
                    self.store_simple(data)?;
                    *self.state__ = S::Tail(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Simple(Some(deserializer)));
                    *self.state__ = S::Tail(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_tail<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<HolderTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use HolderTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Tail(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tail(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Tail(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::HolderType> for HolderTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HolderType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HolderType> {
            use HolderTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Simple(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_simple(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Tail(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_tail(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Simple(None);
                        event
                    }
                    (S::Simple(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Simple",
                            false,
                        )?;
                        match self.handle_simple(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Tail(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Tail",
                            false,
                        )?;
                        match self.handle_tail(helper, output, &mut fallback)? {
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
                helper.decide_unknown_element::<super::HolderType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::HolderType, Error> {
            let state = replace(&mut *self.state__, HolderTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::HolderType {
                simple: helper.finish_element("Simple", self.simple)?,
                tail: self.tail,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Simple" | b"Tail")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct TextHolderTypeDeserializer {
        text: Option<super::TextType>,
        tail: Option<String>,
        state__: Box<TextHolderTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TextHolderTypeDeserializerState {
        Init__,
        Text(Option<<super::TextType as WithDeserializer>::Deserializer>),
        Tail(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TextHolderTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                text: None,
                tail: None,
                state__: Box::new(TextHolderTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TextHolderTypeDeserializerState,
        ) -> Result<(), Error> {
            use TextHolderTypeDeserializerState as S;
            match state {
                S::Text(Some(deserializer)) => self.store_text(deserializer.finish(helper)?)?,
                S::Tail(Some(deserializer)) => self.store_tail(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_text(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.text.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Text")))?;
            }
            self.text = Some(value);
            Ok(())
        }
        fn store_tail(&mut self, value: String) -> Result<(), Error> {
            if self.tail.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Tail")))?;
            }
            self.tail = Some(value);
            Ok(())
        }
        fn handle_text<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TextHolderTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TextHolderTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Text(None));
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
                    self.store_text(data)?;
                    *self.state__ = S::Tail(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Text(Some(deserializer)));
                    *self.state__ = S::Tail(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_tail<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<TextHolderTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use TextHolderTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Tail(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tail(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Tail(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TextHolderType> for TextHolderTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TextHolderType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TextHolderType> {
            use TextHolderTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Text(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Tail(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_tail(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Text(None);
                        event
                    }
                    (S::Text(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Text",
                            false,
                        )?;
                        match self.handle_text(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Tail(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Tail",
                            false,
                        )?;
                        match self.handle_tail(helper, output, &mut fallback)? {
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
                helper.decide_unknown_element::<super::TextHolderType>(event, allow_any)?;
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
        ) -> Result<super::TextHolderType, Error> {
            let state = replace(
                &mut *self.state__,
                TextHolderTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::TextHolderType {
                text: helper.finish_element("Text", self.text)?,
                tail: self.tail,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Text" | b"Tail")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct TextTypeDeserializer {
        id: Option<i32>,
        content: Option<String>,
        state__: Box<TextTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TextTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TextTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id,
                content: None,
                state__: Box::new(TextTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: TextTypeDeserializerState,
        ) -> Result<(), Error> {
            if let TextTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::TextType> {
            use TextTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TextType> for TextTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TextType> {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TextType> {
            use TextTypeDeserializerState as S;
            match replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output = ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::TextType, Error> {
            let state = replace(&mut *self.state__, TextTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::TextType {
                id: self.id,
                content: helper.finish_content(self.content)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if <String as WithDeserializer>::Deserializer::is_known_start_tag(helper, x) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct MixedTypeDeserializer {
        a: Option<String>,
        b: Option<String>,
        state__: Box<MixedTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedTypeDeserializerState {
        Init__,
        A(Option<<String as WithDeserializer>::Deserializer>),
        B(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MixedTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                a: None,
                b: None,
                state__: Box::new(MixedTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: MixedTypeDeserializerState,
        ) -> Result<(), Error> {
            use MixedTypeDeserializerState as S;
            match state {
                S::A(Some(deserializer)) => self.store_a(deserializer.finish(helper)?)?,
                S::B(Some(deserializer)) => self.store_b(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_a(&mut self, value: String) -> Result<(), Error> {
            if self.a.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"A")))?;
            }
            self.a = Some(value);
            Ok(())
        }
        fn store_b(&mut self, value: String) -> Result<(), Error> {
            if self.b.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"B")))?;
            }
            self.b = Some(value);
            Ok(())
        }
        fn handle_a<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<MixedTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::A(None));
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
                    self.store_a(data)?;
                    *self.state__ = S::B(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::A(Some(deserializer)));
                    *self.state__ = S::B(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_b<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<MixedTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use MixedTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::B(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_b(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::B(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::MixedType> for MixedTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedType> {
            use MixedTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::A(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_a(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::B(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_b(helper, output, &mut fallback)? {
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
                        *self.state__ = S::A(None);
                        event
                    }
                    (S::A(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"A",
                            false,
                        )?;
                        match self.handle_a(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::B(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"B",
                            false,
                        )?;
                        match self.handle_b(helper, output, &mut fallback)? {
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
                    (state, Event::Text(_) | Event::CData(_)) => {
                        *self.state__ = state;
                        break (DeserializerEvent::None, false);
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            let (event, allow_any) =
                helper.decide_unknown_element::<super::MixedType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::MixedType, Error> {
            let state = replace(&mut *self.state__, MixedTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::MixedType {
                a: helper.finish_element("A", self.a)?,
                b: self.b,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"A" | b"B")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct AnyRootTypeDeserializer {
        child: Option<super::ChildType>,
        any: Vec<AnyElement>,
        state__: Box<AnyRootTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AnyRootTypeDeserializerState {
        Init__,
        Child(Option<<super::ChildType as WithDeserializer>::Deserializer>),
        Any(Option<<AnyElement as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl AnyRootTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                child: None,
                any: Vec::new(),
                state__: Box::new(AnyRootTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnyRootTypeDeserializerState,
        ) -> Result<(), Error> {
            use AnyRootTypeDeserializerState as S;
            match state {
                S::Child(Some(deserializer)) => self.store_child(deserializer.finish(helper)?)?,
                S::Any(Some(deserializer)) => self.store_any(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_child(&mut self, value: super::ChildType) -> Result<(), Error> {
            if self.child.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Child",
                )))?;
            }
            self.child = Some(value);
            Ok(())
        }
        fn store_any(&mut self, value: AnyElement) -> Result<(), Error> {
            self.any.push(value);
            Ok(())
        }
        fn handle_child<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ChildType>,
            fallback: &mut Option<AnyRootTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnyRootTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Child(None));
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
                    self.store_child(data)?;
                    *self.state__ = S::Any(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Child(Some(deserializer)));
                    *self.state__ = S::Any(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_any<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, AnyElement>,
            fallback: &mut Option<AnyRootTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use AnyRootTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Any(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_any(data)?;
                    *self.state__ = S::Any(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Any(Some(deserializer)));
                    *self.state__ = S::Any(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AnyRootType> for AnyRootTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyRootType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnyRootType> {
            use AnyRootTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let mut is_any_retry = false;
            let mut any_fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Child(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_child(helper, output, &mut fallback)? {
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
                        let output = deserializer.next(helper, event)?;
                        match self.handle_any(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Child(None);
                        event
                    }
                    (S::Child(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Child",
                            false,
                        )?;
                        match self.handle_child(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Any(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if is_any_retry {
                            let output = <AnyElement as WithDeserializer>::init(helper, event)?;
                            match self.handle_any(helper, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            any_fallback.get_or_insert(S::Any(None));
                            *self.state__ = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        if let Some(state) = any_fallback.take() {
                            is_any_retry = true;
                            *self.state__ = state;
                            event
                        } else {
                            *self.state__ = S::Done__;
                            break (DeserializerEvent::Continue(event), allow_any_element);
                        }
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AnyRootType, Error> {
            let state = replace(&mut *self.state__, AnyRootTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::AnyRootType {
                child: helper.finish_element("Child", self.child)?,
                any: self.any,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Child")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ChildTypeDeserializer {
        inner: Option<String>,
        state__: Box<ChildTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ChildTypeDeserializerState {
        Init__,
        Inner(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ChildTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                inner: None,
                state__: Box::new(ChildTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ChildTypeDeserializerState,
        ) -> Result<(), Error> {
            use ChildTypeDeserializerState as S;
            match state {
                S::Inner(Some(deserializer)) => self.store_inner(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_inner(&mut self, value: String) -> Result<(), Error> {
            if self.inner.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Inner",
                )))?;
            }
            self.inner = Some(value);
            Ok(())
        }
        fn handle_inner<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ChildTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ChildTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Inner(None));
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
                    self.store_inner(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Inner(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ChildType> for ChildTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChildType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChildType> {
            use ChildTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Inner(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_inner(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Inner(None);
                        event
                    }
                    (S::Inner(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Inner",
                            false,
                        )?;
                        match self.handle_inner(helper, output, &mut fallback)? {
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
                helper.decide_unknown_element::<super::ChildType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ChildType, Error> {
            let state = replace(&mut *self.state__, ChildTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ChildType {
                inner: helper.finish_element("Inner", self.inner)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Inner")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ZooTypeDeserializer {
        name: Option<String>,
        animal: Vec<super::AnimalType>,
        state__: Box<ZooTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ZooTypeDeserializerState {
        Init__,
        Name(Option<<String as WithDeserializer>::Deserializer>),
        Animal(Option<<super::AnimalType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ZooTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                name: None,
                animal: Vec::new(),
                state__: Box::new(ZooTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ZooTypeDeserializerState,
        ) -> Result<(), Error> {
            use ZooTypeDeserializerState as S;
            match state {
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(helper)?)?,
                S::Animal(Some(deserializer)) => self.store_animal(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_name(&mut self, value: String) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_animal(&mut self, value: super::AnimalType) -> Result<(), Error> {
            self.animal.push(value);
            Ok(())
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<ZooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ZooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Name(None));
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
                    self.store_name(data)?;
                    *self.state__ = S::Animal(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Name(Some(deserializer)));
                    *self.state__ = S::Animal(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_animal<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::AnimalType>,
            fallback: &mut Option<ZooTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ZooTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Animal(None));
                *self.state__ = S::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_animal(data)?;
                    *self.state__ = S::Animal(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Animal(Some(deserializer)));
                    *self.state__ = S::Animal(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ZooType> for ZooTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ZooType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ZooType> {
            use ZooTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Animal(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_animal(helper, output, &mut fallback)? {
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
                        *self.state__ = S::Name(None);
                        event
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Name",
                            false,
                        )?;
                        match self.handle_name(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Animal(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = <super::AnimalType as WithDeserializer>::init(helper, event)?;
                        match self.handle_animal(helper, output, &mut fallback)? {
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
                helper.decide_unknown_element::<super::ZooType>(event, allow_any)?;
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ZooType, Error> {
            let state = replace(&mut *self.state__, ZooTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ZooType {
                name: helper.finish_element("Name", self.name)?,
                animal: self.animal,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Name" | b"animal")
            ) {
                return true;
            }
            if <super::AnimalType as WithDeserializer>::Deserializer::is_known_start_tag(helper, x)
            {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub enum AnimalTypeDeserializer {
        Animal(<super::AnimalDyn as WithDeserializer>::Deserializer),
        Dog(<super::DogType as WithDeserializer>::Deserializer),
    }
    impl<'de> Deserializer<'de, super::AnimalType> for AnimalTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnimalType> {
            let Some(type_name) = helper.get_dynamic_type_from_tag(&event) else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            };
            let type_name = type_name.into_owned();
            if matches!(
                helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"animal")
            ) {
                let type_name = helper
                    .get_dynamic_type_from_attrib(&event)?
                    .unwrap_or(Cow::Borrowed("animal"))
                    .into_owned();
                if matches!(
                    helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                    Some(b"animal")
                ) {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = <super::AnimalDyn as WithDeserializer>::init(helper, event)?;
                    return Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::AnimalType(Box::new(x)), |x| Self::Animal(x)),
                        event,
                        allow_any,
                    });
                }
                if matches!(
                    helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                    Some(b"dog")
                ) {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = <super::DogType as WithDeserializer>::init(helper, event)?;
                    return Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::AnimalType(Box::new(x)), |x| Self::Dog(x)),
                        event,
                        allow_any,
                    });
                }
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::None,
                event: DeserializerEvent::Break(event),
                allow_any: false,
            })
        }
        fn next(
            self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnimalType> {
            match self {
                Self::Animal(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::AnimalType(Box::new(x)), |x| Self::Animal(x)),
                        event,
                        allow_any,
                    })
                }
                Self::Dog(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact
                            .map(|x| super::AnimalType(Box::new(x)), |x| Self::Dog(x)),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::AnimalType, Error> {
            match self {
                Self::Animal(x) => Ok(super::AnimalType(Box::new(x.finish(helper)?))),
                Self::Dog(x) => Ok(super::AnimalType(Box::new(x.finish(helper)?))),
            }
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            let _ = (helper, x);
            false
        }
    }
    #[derive(Debug)]
    pub struct AnimalDynDeserializer {
        id: i32,
        state__: Box<AnimalDynDeserializerState>,
    }
    #[derive(Debug)]
    enum AnimalDynDeserializerState {
        Init__,
        Unknown__,
    }
    impl AnimalDynDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                state__: Box::new(AnimalDynDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: AnimalDynDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::AnimalDyn> for AnimalDynDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnimalDyn> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AnimalDyn> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                let event = DeserializerEvent::Break(event);
                let allow_any = false;
                let (event, allow_any) =
                    helper.decide_unknown_element::<super::AnimalDyn>(event, allow_any)?;
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::AnimalDyn, Error> {
            let state = replace(&mut *self.state__, AnimalDynDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::AnimalDyn { id: self.id })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            let _ = (helper, x);
            false
        }
    }
    #[derive(Debug)]
    pub struct DogTypeDeserializer {
        id: i32,
        name: String,
        state__: Box<DogTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DogTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl DogTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            let mut name: Option<String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| ErrorKind::MissingAttribute("id".into()))?,
                name: name.ok_or_else(|| ErrorKind::MissingAttribute("name".into()))?,
                state__: Box::new(DogTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: DogTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::DogType> for DogTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DogType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DogType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                let event = DeserializerEvent::Break(event);
                let allow_any = false;
                let (event, allow_any) =
                    helper.decide_unknown_element::<super::DogType>(event, allow_any)?;
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::DogType, Error> {
            let state = replace(&mut *self.state__, DogTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::DogType {
                id: self.id,
                name: self.name,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            let _ = (helper, x);
            false
        }
    }
    #[derive(Debug)]
    pub struct ManyTypeDeserializer {
        content: Vec<super::ManyTypeContent>,
        state__: Box<ManyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ManyTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::ManyTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl ManyTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                content: Vec::new(),
                state__: Box::new(ManyTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ManyTypeDeserializerState,
        ) -> Result<(), Error> {
            if let ManyTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::ManyTypeContent) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, super::ManyTypeContent>,
            fallback: &mut Option<ManyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ManyTypeDeserializerState as S;
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
    impl<'de> Deserializer<'de, super::ManyType> for ManyTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ManyType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ManyType> {
            use ManyTypeDeserializerState as S;
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
                            <super::ManyTypeContent as WithDeserializer>::init(helper, event)?;
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
                helper.decide_unknown_element::<super::ManyType>(event, allow_any)?;
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
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ManyType, Error> {
            let state = replace(&mut *self.state__, ManyTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ManyType {
                content: helper.finish_vec(1usize, None, self.content)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if <super::ManyTypeContent as WithDeserializer>::Deserializer::is_known_start_tag(
                helper, x,
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ManyTypeContentDeserializer {
        state__: Box<ManyTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ManyTypeContentDeserializerState {
        Init__,
        X(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Y(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ManyTypeContent),
        Unknown__,
    }
    impl ManyTypeContentDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"X")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_x(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Y")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_y(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ManyTypeContentDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ManyTypeContentDeserializerState,
        ) -> Result<super::ManyTypeContent, Error> {
            use ManyTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::X(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_x(&mut values, value)?;
                    }
                    Ok(super::ManyTypeContent::X(
                        helper.finish_element("X", values)?,
                    ))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::ManyTypeContent::Y(
                        helper.finish_element("Y", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_x(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"X")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_y(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_x<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ManyTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::X(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_x(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_x(&mut values, data)?;
                    let data = Self::finish_state(helper, S::X(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::X(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_y<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ManyTypeContentDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Y(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_y(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_y(&mut values, data)?;
                    let data = Self::finish_state(helper, S::Y(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Y(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ManyTypeContent> for ManyTypeContentDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ManyTypeContent> {
            let deserializer = Self {
                state__: Box::new(ManyTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ManyTypeContentDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ManyTypeContent> {
            use ManyTypeContentDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::X(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_x(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_y(helper, values, fallback, output)? {
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
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::X(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"X",
                            false,
                        )?;
                        match self.handle_x(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Y",
                            false,
                        )?;
                        match self.handle_y(helper, values, fallback, output)? {
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::ManyTypeContent, Error> {
            Self::finish_state(helper, *self.state__)
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"X" | b"Y")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct GroupPickTypeDeserializer {
        state__: Box<GroupPickTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum GroupPickTypeDeserializerState {
        Init__,
        Pq(
            Option<super::GroupPickPqType>,
            Option<<super::GroupPickPqType as WithDeserializer>::Deserializer>,
            Option<<super::GroupPickPqType as WithDeserializer>::Deserializer>,
        ),
        Y(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::GroupPickType),
        Unknown__,
    }
    impl GroupPickTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Y")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_y(helper, Default::default(), None, output);
                }
                event = {
                    let output = <super::GroupPickPqType as WithDeserializer>::init(helper, event)?;
                    match self.handle_pq(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = GroupPickTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                state__: Box::new(GroupPickTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: GroupPickTypeDeserializerState,
        ) -> Result<super::GroupPickType, Error> {
            use GroupPickTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Pq(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_pq(&mut values, value)?;
                    }
                    Ok(super::GroupPickType::Pq(
                        helper.finish_element("PQ", values)?,
                    ))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::GroupPickType::Y(helper.finish_element("Y", values)?))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_pq(
            values: &mut Option<super::GroupPickPqType>,
            value: super::GroupPickPqType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"PQ")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_y(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_pq<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::GroupPickPqType>,
            fallback: Option<<super::GroupPickPqType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::GroupPickPqType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupPickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Pq(values, None, fallback);
                if !fresh {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_pq(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_pq(&mut values, data)?;
                    *self.state__ = S::Pq(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Pq(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_y<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupPickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Y(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_y(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_y(&mut values, data)?;
                    *self.state__ = S::Y(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Y(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::GroupPickType> for GroupPickTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupPickType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupPickType> {
            use GroupPickTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Pq(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_pq(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_y(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Pq(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output =
                            <super::GroupPickPqType as WithDeserializer>::init(helper, event)?;
                        match self.handle_pq(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Y",
                            false,
                        )?;
                        match self.handle_y(helper, values, fallback, output)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::GroupPickType>(event, allow_any)?;
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::GroupPickType, Error> {
            Self::finish_state(helper, *self.state__)
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Y")
            ) {
                return true;
            }
            if <super::GroupPickPqType as WithDeserializer>::Deserializer::is_known_start_tag(
                helper, x,
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct GroupPickPqTypeDeserializer {
        p: Option<String>,
        q: Option<String>,
        state__: Box<GroupPickPqTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum GroupPickPqTypeDeserializerState {
        Init__,
        P(Option<<String as WithDeserializer>::Deserializer>),
        Q(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl GroupPickPqTypeDeserializer {
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: GroupPickPqTypeDeserializerState,
        ) -> Result<(), Error> {
            use GroupPickPqTypeDeserializerState as S;
            match state {
                S::P(Some(deserializer)) => self.store_p(deserializer.finish(helper)?)?,
                S::Q(Some(deserializer)) => self.store_q(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_p(&mut self, value: String) -> Result<(), Error> {
            if self.p.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"P")))?;
            }
            self.p = Some(value);
            Ok(())
        }
        fn store_q(&mut self, value: String) -> Result<(), Error> {
            if self.q.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Q")))?;
            }
            self.q = Some(value);
            Ok(())
        }
        fn handle_p<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<GroupPickPqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupPickPqTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::P(None));
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
                    self.store_p(data)?;
                    *self.state__ = S::Q(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::P(Some(deserializer)));
                    *self.state__ = S::Q(None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
        fn handle_q<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<GroupPickPqTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use GroupPickPqTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Q(None));
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
                    self.store_q(data)?;
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Q(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::GroupPickPqType> for GroupPickPqTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GroupPickPqType> {
            let deserializer = Self {
                p: None,
                q: None,
                state__: Box::new(GroupPickPqTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, GroupPickPqTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::GroupPickPqType> {
            use GroupPickPqTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::P(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_p(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Q(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_q(helper, output, &mut fallback)? {
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
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(helper)?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::P(None);
                        event
                    }
                    (S::P(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"P",
                            false,
                        )?;
                        match self.handle_p(helper, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Q(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Q",
                            false,
                        )?;
                        match self.handle_q(helper, output, &mut fallback)? {
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
        ) -> Result<super::GroupPickPqType, Error> {
            let state = replace(
                &mut *self.state__,
                GroupPickPqTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::GroupPickPqType {
                p: helper.finish_element("P", self.p)?,
                q: helper.finish_element("Q", self.q)?,
            })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"P" | b"Q")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct DynPickTypeDeserializer {
        state__: Box<DynPickTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum DynPickTypeDeserializerState {
        Init__,
        Animal(
            Option<super::AnimalType>,
            Option<<super::AnimalType as WithDeserializer>::Deserializer>,
            Option<<super::AnimalType as WithDeserializer>::Deserializer>,
        ),
        Y(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::DynPickType),
        Unknown__,
    }
    impl DynPickTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Y")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_y(helper, Default::default(), None, output);
                }
                event = {
                    let output = <super::AnimalType as WithDeserializer>::init(helper, event)?;
                    match self.handle_animal(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = DynPickTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                state__: Box::new(DynPickTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: DynPickTypeDeserializerState,
        ) -> Result<super::DynPickType, Error> {
            use DynPickTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Animal(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_animal(&mut values, value)?;
                    }
                    Ok(super::DynPickType::Animal(
                        helper.finish_element("animal", values)?,
                    ))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::DynPickType::Y(helper.finish_element("Y", values)?))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_animal(
            values: &mut Option<super::AnimalType>,
            value: super::AnimalType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"animal",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_y(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_animal<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::AnimalType>,
            fallback: Option<<super::AnimalType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::AnimalType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DynPickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Animal(values, None, fallback);
                if !fresh {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_animal(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_animal(&mut values, data)?;
                    *self.state__ = S::Animal(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Animal(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_y<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use DynPickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Y(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_y(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_y(&mut values, data)?;
                    *self.state__ = S::Y(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Y(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DynPickType> for DynPickTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DynPickType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DynPickType> {
            use DynPickTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Animal(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_animal(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_y(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Animal(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <super::AnimalType as WithDeserializer>::init(helper, event)?;
                        match self.handle_animal(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Y",
                            false,
                        )?;
                        match self.handle_y(helper, values, fallback, output)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::DynPickType>(event, allow_any)?;
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::DynPickType, Error> {
            Self::finish_state(helper, *self.state__)
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"animal" | b"Y")
            ) {
                return true;
            }
            if <super::AnimalType as WithDeserializer>::Deserializer::is_known_start_tag(helper, x)
            {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ChoiceGroupPickTypeDeserializer {
        state__: Box<ChoiceGroupPickTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ChoiceGroupPickTypeDeserializerState {
        Init__,
        Uv(
            Option<super::ChoiceGroupPickUvType>,
            Option<<super::ChoiceGroupPickUvType as WithDeserializer>::Deserializer>,
            Option<<super::ChoiceGroupPickUvType as WithDeserializer>::Deserializer>,
        ),
        Y(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ChoiceGroupPickType),
        Unknown__,
    }
    impl ChoiceGroupPickTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Y")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_y(helper, Default::default(), None, output);
                }
                event = {
                    let output =
                        <super::ChoiceGroupPickUvType as WithDeserializer>::init(helper, event)?;
                    match self.handle_uv(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = ChoiceGroupPickTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                state__: Box::new(ChoiceGroupPickTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ChoiceGroupPickTypeDeserializerState,
        ) -> Result<super::ChoiceGroupPickType, Error> {
            use ChoiceGroupPickTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Uv(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_uv(&mut values, value)?;
                    }
                    Ok(super::ChoiceGroupPickType::Uv(
                        helper.finish_element("UV", values)?,
                    ))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::ChoiceGroupPickType::Y(
                        helper.finish_element("Y", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_uv(
            values: &mut Option<super::ChoiceGroupPickUvType>,
            value: super::ChoiceGroupPickUvType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"UV")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_y(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_uv<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::ChoiceGroupPickUvType>,
            fallback: Option<<super::ChoiceGroupPickUvType as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::ChoiceGroupPickUvType>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ChoiceGroupPickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Uv(values, None, fallback);
                if !fresh {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_uv(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_uv(&mut values, data)?;
                    *self.state__ = S::Uv(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Uv(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_y<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ChoiceGroupPickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Y(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_y(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_y(&mut values, data)?;
                    *self.state__ = S::Y(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Y(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ChoiceGroupPickType> for ChoiceGroupPickTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChoiceGroupPickType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChoiceGroupPickType> {
            use ChoiceGroupPickTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Uv(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_uv(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_y(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Uv(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <super::ChoiceGroupPickUvType as WithDeserializer>::init(
                            helper, event,
                        )?;
                        match self.handle_uv(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Y",
                            false,
                        )?;
                        match self.handle_y(helper, values, fallback, output)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::ChoiceGroupPickType>(event, allow_any)?;
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
        ) -> Result<super::ChoiceGroupPickType, Error> {
            Self::finish_state(helper, *self.state__)
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Y")
            ) {
                return true;
            }
            if <super::ChoiceGroupPickUvType as WithDeserializer>::Deserializer::is_known_start_tag(
                helper, x,
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ChoiceGroupPickUvTypeDeserializer {
        state__: Box<ChoiceGroupPickUvTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum ChoiceGroupPickUvTypeDeserializerState {
        Init__,
        U(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        V(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::ChoiceGroupPickUvType),
        Unknown__,
    }
    impl ChoiceGroupPickUvTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"U")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_u(helper, Default::default(), None, output);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"V")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_v(helper, Default::default(), None, output);
                }
            }
            *self.state__ = ChoiceGroupPickUvTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: ChoiceGroupPickUvTypeDeserializerState,
        ) -> Result<super::ChoiceGroupPickUvType, Error> {
            use ChoiceGroupPickUvTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::U(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_u(&mut values, value)?;
                    }
                    Ok(super::ChoiceGroupPickUvType::U(
                        helper.finish_element("U", values)?,
                    ))
                }
                S::V(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_v(&mut values, value)?;
                    }
                    Ok(super::ChoiceGroupPickUvType::V(
                        helper.finish_element("V", values)?,
                    ))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_u(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"U")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_v(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"V")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_u<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ChoiceGroupPickUvTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::U(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_u(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_u(&mut values, data)?;
                    let data = Self::finish_state(helper, S::U(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::U(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
        fn handle_v<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use ChoiceGroupPickUvTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::V(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_v(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_v(&mut values, data)?;
                    let data = Self::finish_state(helper, S::V(values, None, None))?;
                    *self.state__ = S::Done__(data);
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::V(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::break_(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::ChoiceGroupPickUvType> for ChoiceGroupPickUvTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ChoiceGroupPickUvType> {
            let deserializer = Self {
                state__: Box::new(ChoiceGroupPickUvTypeDeserializerState::Init__),
            };
            let mut output = deserializer.next(helper, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state__, ChoiceGroupPickUvTypeDeserializerState::Init__) =>
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
        ) -> DeserializerResult<'de, super::ChoiceGroupPickUvType> {
            use ChoiceGroupPickUvTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::U(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_u(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::V(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_v(helper, values, fallback, output)? {
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
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::U(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"U",
                            false,
                        )?;
                        match self.handle_u(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::V(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"V",
                            false,
                        )?;
                        match self.handle_v(helper, values, fallback, output)? {
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
        ) -> Result<super::ChoiceGroupPickUvType, Error> {
            Self::finish_state(helper, *self.state__)
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"U" | b"V")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct SubstPickTypeDeserializer {
        state__: Box<SubstPickTypeDeserializerState>,
    }
    #[derive(Debug)]
    pub enum SubstPickTypeDeserializerState {
        Init__,
        Shape(
            Option<super::Shape>,
            Option<<super::Shape as WithDeserializer>::Deserializer>,
            Option<<super::Shape as WithDeserializer>::Deserializer>,
        ),
        Y(
            Option<String>,
            Option<<String as WithDeserializer>::Deserializer>,
            Option<<String as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SubstPickType),
        Unknown__,
    }
    impl SubstPickTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            let mut event = event;
            let mut allow_any_element = false;
            if let Event::Start(x) | Event::Empty(x) = &event {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_TNS),
                    Some(b"Y")
                ) {
                    let output = <String as WithDeserializer>::init(helper, event)?;
                    return self.handle_y(helper, Default::default(), None, output);
                }
                event = {
                    let output = <super::Shape as WithDeserializer>::init(helper, event)?;
                    match self.handle_shape(helper, Default::default(), None, output)? {
                        ElementHandlerOutput::Continue { event, allow_any } => {
                            allow_any_element = allow_any_element || allow_any;
                            event
                        }
                        output => {
                            return Ok(output);
                        }
                    }
                };
            }
            *self.state__ = SubstPickTypeDeserializerState::Init__;
            Ok(ElementHandlerOutput::return_to_parent(
                event,
                allow_any_element,
            ))
        }
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                state__: Box::new(SubstPickTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            helper: &mut DeserializeHelper,
            state: SubstPickTypeDeserializerState,
        ) -> Result<super::SubstPickType, Error> {
            use SubstPickTypeDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Shape(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_shape(&mut values, value)?;
                    }
                    Ok(super::SubstPickType::Shape(
                        helper.finish_element("Shape", values)?,
                    ))
                }
                S::Y(mut values, None, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(helper)?;
                        Self::store_y(&mut values, value)?;
                    }
                    Ok(super::SubstPickType::Y(helper.finish_element("Y", values)?))
                }
                S::Done__(data) => Ok(data),
                _ => unreachable!(),
            }
        }
        fn store_shape(
            values: &mut Option<super::Shape>,
            value: super::Shape,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Shape",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_y(values: &mut Option<String>, value: String) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Y")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_shape<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<super::Shape>,
            fallback: Option<<super::Shape as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, super::Shape>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SubstPickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Shape(values, None, fallback);
                if !fresh {
                    return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
                }
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_shape(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_shape(&mut values, data)?;
                    *self.state__ = S::Shape(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Shape(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
        fn handle_y<'de>(
            &mut self,
            helper: &mut DeserializeHelper,
            mut values: Option<String>,
            fallback: Option<<String as WithDeserializer>::Deserializer>,
            output: DeserializerOutput<'de, String>,
        ) -> Result<ElementHandlerOutput<'de>, Error> {
            use SubstPickTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let fresh = values.is_none() && fallback.is_none();
                *self.state__ = S::Y(values, None, fallback);
                return Ok(ElementHandlerOutput::return_to_root(event, allow_any));
            }
            if let Some(deserializer) = fallback {
                let data = deserializer.finish(helper)?;
                Self::store_y(&mut values, data)?;
            }
            match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_y(&mut values, data)?;
                    *self.state__ = S::Y(values, None, None);
                    Ok(ElementHandlerOutput::from_event(event, allow_any))
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Y(values, None, Some(deserializer));
                    Ok(ElementHandlerOutput::from_event_end(event, allow_any))
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SubstPickType> for SubstPickTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SubstPickType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SubstPickType> {
            use SubstPickTypeDeserializerState as S;
            let mut event = event;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Shape(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_shape(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_y(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                helper, state,
                            )?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(helper, event)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (
                        S::Shape(values, fallback, None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        let output = <super::Shape as WithDeserializer>::init(helper, event)?;
                        match self.handle_shape(helper, values, fallback, output)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Y(values, fallback, None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Y",
                            false,
                        )?;
                        match self.handle_y(helper, values, fallback, output)? {
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
            let (event, allow_any) =
                helper.decide_unknown_element::<super::SubstPickType>(event, allow_any)?;
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
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::SubstPickType, Error> {
            Self::finish_state(helper, *self.state__)
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Shape" | b"Y")
            ) {
                return true;
            }
            if <super::Shape as WithDeserializer>::Deserializer::is_known_start_tag(helper, x) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub enum ShapeDeserializer {
        Circle(<super::ShapeType as WithDeserializer>::Deserializer),
    }
    impl<'de> Deserializer<'de, super::Shape> for ShapeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Shape> {
            let Some(type_name) = helper.get_dynamic_type_from_tag(&event) else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::None,
                    allow_any: false,
                });
            };
            let type_name = type_name.into_owned();
            if matches!(
                helper.resolve_local_name(QName(&type_name), &super::NS_TNS),
                Some(b"Circle")
            ) {
                let DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = <super::ShapeType as WithDeserializer>::init(helper, event)?;
                return Ok(DeserializerOutput {
                    artifact: artifact.map(|x| super::Shape(Box::new(x)), |x| Self::Circle(x)),
                    event,
                    allow_any,
                });
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::None,
                event: DeserializerEvent::Break(event),
                allow_any: false,
            })
        }
        fn next(
            self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::Shape> {
            match self {
                Self::Circle(x) => {
                    let DeserializerOutput {
                        artifact,
                        event,
                        allow_any,
                    } = x.next(helper, event)?;
                    Ok(DeserializerOutput {
                        artifact: artifact.map(|x| super::Shape(Box::new(x)), |x| Self::Circle(x)),
                        event,
                        allow_any,
                    })
                }
            }
        }
        fn finish(self, helper: &mut DeserializeHelper) -> Result<super::Shape, Error> {
            match self {
                Self::Circle(x) => Ok(super::Shape(Box::new(x.finish(helper)?))),
            }
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            if matches!(
                helper.resolve_local_name(x.name(), &super::NS_TNS),
                Some(b"Circle")
            ) {
                return true;
            }
            false
        }
    }
    #[derive(Debug)]
    pub struct ShapeTypeDeserializer {
        id: Option<i32>,
        state__: Box<ShapeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ShapeTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl ShapeTypeDeserializer {
        fn from_bytes_start(
            helper: &mut DeserializeHelper,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error> {
            let mut id: Option<i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_TNS),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id,
                state__: Box::new(ShapeTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut DeserializeHelper,
            state: ShapeTypeDeserializerState,
        ) -> Result<(), Error> {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::ShapeType> for ShapeTypeDeserializer {
        fn init(
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ShapeType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut DeserializeHelper,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ShapeType> {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(helper)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                let event = DeserializerEvent::Break(event);
                let allow_any = false;
                let (event, allow_any) =
                    helper.decide_unknown_element::<super::ShapeType>(event, allow_any)?;
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
        }
        fn finish(mut self, helper: &mut DeserializeHelper) -> Result<super::ShapeType, Error> {
            let state = replace(&mut *self.state__, ShapeTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::ShapeType { id: self.id })
        }
        fn is_known_start_tag(helper: &DeserializeHelper, x: &BytesStart<'_>) -> bool {
            let _ = (helper, x);
            false
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::{
        quick_xml::{
            BytesEnd, BytesStart, Error, Event, IterSerializer, SerializeHelper, Serializer,
            WithSerializer,
        },
        xml::AnyElement,
    };
    #[derive(Debug)]
    pub struct HolderTypeSerializer<'ser> {
        pub(super) value: &'ser super::HolderType,
        pub(super) state: Box<HolderTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum HolderTypeSerializerState<'ser> {
        Init__,
        Simple(<String as WithSerializer>::Serializer<'ser>),
        Tail(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HolderTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    HolderTypeSerializerState::Init__ => {
                        *self.state =
                            HolderTypeSerializerState::Simple(WithSerializer::serializer(
                                &self.value.simple,
                                Some("tns:Simple"),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    HolderTypeSerializerState::Simple(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = HolderTypeSerializerState::Tail(IterSerializer::new(
                                self.value.tail.as_ref(),
                                Some("tns:Tail"),
                                false,
                            ))
                        }
                    },
                    HolderTypeSerializerState::Tail(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = HolderTypeSerializerState::End__,
                    },
                    HolderTypeSerializerState::End__ => {
                        *self.state = HolderTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    HolderTypeSerializerState::Done__ => return Ok(None),
                    HolderTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for HolderTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = HolderTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TextHolderTypeSerializer<'ser> {
        pub(super) value: &'ser super::TextHolderType,
        pub(super) state: Box<TextHolderTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TextHolderTypeSerializerState<'ser> {
        Init__,
        Text(<super::TextType as WithSerializer>::Serializer<'ser>),
        Tail(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TextHolderTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TextHolderTypeSerializerState::Init__ => {
                        *self.state = TextHolderTypeSerializerState::Text(
                            WithSerializer::serializer(&self.value.text, Some("tns:Text"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TextHolderTypeSerializerState::Text(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TextHolderTypeSerializerState::Tail(IterSerializer::new(
                                self.value.tail.as_ref(),
                                Some("tns:Tail"),
                                false,
                            ))
                        }
                    },
                    TextHolderTypeSerializerState::Tail(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TextHolderTypeSerializerState::End__,
                    },
                    TextHolderTypeSerializerState::End__ => {
                        *self.state = TextHolderTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TextHolderTypeSerializerState::Done__ => return Ok(None),
                    TextHolderTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TextHolderTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TextHolderTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TextTypeSerializer<'ser> {
        pub(super) value: &'ser super::TextType,
        pub(super) state: Box<TextTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TextTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TextTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TextTypeSerializerState::Init__ => {
                        *self.state = TextTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TextTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TextTypeSerializerState::End__,
                    },
                    TextTypeSerializerState::End__ => {
                        *self.state = TextTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TextTypeSerializerState::Done__ => return Ok(None),
                    TextTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for TextTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TextTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct MixedTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedType,
        pub(super) state: Box<MixedTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MixedTypeSerializerState<'ser> {
        Init__,
        A(<String as WithSerializer>::Serializer<'ser>),
        B(IterSerializer<'ser, Option<&'ser String>, String>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedTypeSerializerState::Init__ => {
                        *self.state = MixedTypeSerializerState::A(WithSerializer::serializer(
                            &self.value.a,
                            Some("tns:A"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedTypeSerializerState::A(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = MixedTypeSerializerState::B(IterSerializer::new(
                                self.value.b.as_ref(),
                                Some("tns:B"),
                                false,
                            ))
                        }
                    },
                    MixedTypeSerializerState::B(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = MixedTypeSerializerState::End__,
                    },
                    MixedTypeSerializerState::End__ => {
                        *self.state = MixedTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedTypeSerializerState::Done__ => return Ok(None),
                    MixedTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for MixedTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AnyRootTypeSerializer<'ser> {
        pub(super) value: &'ser super::AnyRootType,
        pub(super) state: Box<AnyRootTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AnyRootTypeSerializerState<'ser> {
        Init__,
        Child(<super::ChildType as WithSerializer>::Serializer<'ser>),
        Any(IterSerializer<'ser, &'ser [AnyElement], AnyElement>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnyRootTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AnyRootTypeSerializerState::Init__ => {
                        *self.state =
                            AnyRootTypeSerializerState::Child(WithSerializer::serializer(
                                &self.value.child,
                                Some("tns:Child"),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    AnyRootTypeSerializerState::Child(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = AnyRootTypeSerializerState::Any(IterSerializer::new(
                                &self.value.any[..],
                                None,
                                false,
                            ))
                        }
                    },
                    AnyRootTypeSerializerState::Any(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AnyRootTypeSerializerState::End__,
                    },
                    AnyRootTypeSerializerState::End__ => {
                        *self.state = AnyRootTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    AnyRootTypeSerializerState::Done__ => return Ok(None),
                    AnyRootTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for AnyRootTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AnyRootTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ChildTypeSerializer<'ser> {
        pub(super) value: &'ser super::ChildType,
        pub(super) state: Box<ChildTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ChildTypeSerializerState<'ser> {
        Init__,
        Inner(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ChildTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ChildTypeSerializerState::Init__ => {
                        *self.state = ChildTypeSerializerState::Inner(WithSerializer::serializer(
                            &self.value.inner,
                            Some("tns:Inner"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ChildTypeSerializerState::Inner(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ChildTypeSerializerState::End__,
                    },
                    ChildTypeSerializerState::End__ => {
                        *self.state = ChildTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ChildTypeSerializerState::Done__ => return Ok(None),
                    ChildTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ChildTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ChildTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ZooTypeSerializer<'ser> {
        pub(super) value: &'ser super::ZooType,
        pub(super) state: Box<ZooTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ZooTypeSerializerState<'ser> {
        Init__,
        Name(<String as WithSerializer>::Serializer<'ser>),
        Animal(IterSerializer<'ser, &'ser [super::AnimalType], super::AnimalType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ZooTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ZooTypeSerializerState::Init__ => {
                        *self.state = ZooTypeSerializerState::Name(WithSerializer::serializer(
                            &self.value.name,
                            Some("tns:Name"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_XSI),
                                &super::NS_XSI,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ZooTypeSerializerState::Name(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ZooTypeSerializerState::Animal(IterSerializer::new(
                                &self.value.animal[..],
                                Some("tns:animal"),
                                false,
                            ))
                        }
                    },
                    ZooTypeSerializerState::Animal(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ZooTypeSerializerState::End__,
                    },
                    ZooTypeSerializerState::End__ => {
                        *self.state = ZooTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ZooTypeSerializerState::Done__ => return Ok(None),
                    ZooTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ZooTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ZooTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AnimalDynSerializer<'ser> {
        pub(super) value: &'ser super::AnimalDyn,
        pub(super) state: Box<AnimalDynSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AnimalDynSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AnimalDynSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AnimalDynSerializerState::Init__ => {
                        *self.state = AnimalDynSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    AnimalDynSerializerState::Done__ => return Ok(None),
                    AnimalDynSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for AnimalDynSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AnimalDynSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DogTypeSerializer<'ser> {
        pub(super) value: &'ser super::DogType,
        pub(super) state: Box<DogTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DogTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DogTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DogTypeSerializerState::Init__ => {
                        *self.state = DogTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    DogTypeSerializerState::Done__ => return Ok(None),
                    DogTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DogTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DogTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ManyTypeSerializer<'ser> {
        pub(super) value: &'ser super::ManyType,
        pub(super) state: Box<ManyTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ManyTypeSerializerState<'ser> {
        Init__,
        Content__(IterSerializer<'ser, &'ser [super::ManyTypeContent], super::ManyTypeContent>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ManyTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ManyTypeSerializerState::Init__ => {
                        *self.state = ManyTypeSerializerState::Content__(IterSerializer::new(
                            &self.value.content[..],
                            None,
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ManyTypeSerializerState::Content__(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ManyTypeSerializerState::End__,
                    },
                    ManyTypeSerializerState::End__ => {
                        *self.state = ManyTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ManyTypeSerializerState::Done__ => return Ok(None),
                    ManyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ManyTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ManyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ManyTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::ManyTypeContent,
        pub(super) state: Box<ManyTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum ManyTypeContentSerializerState<'ser> {
        Init__,
        X(<String as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ManyTypeContentSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ManyTypeContentSerializerState::Init__ => match self.value {
                        super::ManyTypeContent::X(x) => {
                            *self.state = ManyTypeContentSerializerState::X(
                                WithSerializer::serializer(x, Some("tns:X"), false)?,
                            )
                        }
                        super::ManyTypeContent::Y(x) => {
                            *self.state = ManyTypeContentSerializerState::Y(
                                WithSerializer::serializer(x, Some("tns:Y"), false)?,
                            )
                        }
                    },
                    ManyTypeContentSerializerState::X(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ManyTypeContentSerializerState::Done__,
                    },
                    ManyTypeContentSerializerState::Y(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ManyTypeContentSerializerState::Done__,
                    },
                    ManyTypeContentSerializerState::Done__ => return Ok(None),
                    ManyTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ManyTypeContentSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ManyTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct GroupPickTypeSerializer<'ser> {
        pub(super) value: &'ser super::GroupPickType,
        pub(super) state: Box<GroupPickTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum GroupPickTypeSerializerState<'ser> {
        Init__,
        Pq(<super::GroupPickPqType as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> GroupPickTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    GroupPickTypeSerializerState::Init__ => {
                        match self.value {
                            super::GroupPickType::Pq(x) => {
                                *self.state = GroupPickTypeSerializerState::Pq(
                                    WithSerializer::serializer(x, Some("PQ"), self.is_root)?,
                                )
                            }
                            super::GroupPickType::Y(x) => {
                                *self.state = GroupPickTypeSerializerState::Y(
                                    WithSerializer::serializer(x, Some("tns:Y"), self.is_root)?,
                                )
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    GroupPickTypeSerializerState::Pq(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = GroupPickTypeSerializerState::End__,
                    },
                    GroupPickTypeSerializerState::Y(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = GroupPickTypeSerializerState::End__,
                    },
                    GroupPickTypeSerializerState::End__ => {
                        *self.state = GroupPickTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    GroupPickTypeSerializerState::Done__ => return Ok(None),
                    GroupPickTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for GroupPickTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = GroupPickTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct GroupPickPqTypeSerializer<'ser> {
        pub(super) value: &'ser super::GroupPickPqType,
        pub(super) state: Box<GroupPickPqTypeSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum GroupPickPqTypeSerializerState<'ser> {
        Init__,
        P(<String as WithSerializer>::Serializer<'ser>),
        Q(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> GroupPickPqTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    GroupPickPqTypeSerializerState::Init__ => {
                        *self.state = GroupPickPqTypeSerializerState::P(
                            WithSerializer::serializer(&self.value.p, Some("tns:P"), false)?,
                        );
                    }
                    GroupPickPqTypeSerializerState::P(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = GroupPickPqTypeSerializerState::Q(
                                WithSerializer::serializer(&self.value.q, Some("tns:Q"), false)?,
                            )
                        }
                    },
                    GroupPickPqTypeSerializerState::Q(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = GroupPickPqTypeSerializerState::Done__,
                    },
                    GroupPickPqTypeSerializerState::Done__ => return Ok(None),
                    GroupPickPqTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for GroupPickPqTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = GroupPickPqTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DynPickTypeSerializer<'ser> {
        pub(super) value: &'ser super::DynPickType,
        pub(super) state: Box<DynPickTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DynPickTypeSerializerState<'ser> {
        Init__,
        Animal(<super::AnimalType as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DynPickTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DynPickTypeSerializerState::Init__ => {
                        match self.value {
                            super::DynPickType::Animal(x) => {
                                *self.state =
                                    DynPickTypeSerializerState::Animal(WithSerializer::serializer(
                                        x,
                                        Some("tns:animal"),
                                        self.is_root,
                                    )?)
                            }
                            super::DynPickType::Y(x) => {
                                *self.state = DynPickTypeSerializerState::Y(
                                    WithSerializer::serializer(x, Some("tns:Y"), self.is_root)?,
                                )
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_XSI),
                                &super::NS_XSI,
                            );
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DynPickTypeSerializerState::Animal(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DynPickTypeSerializerState::End__,
                    },
                    DynPickTypeSerializerState::Y(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DynPickTypeSerializerState::End__,
                    },
                    DynPickTypeSerializerState::End__ => {
                        *self.state = DynPickTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DynPickTypeSerializerState::Done__ => return Ok(None),
                    DynPickTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for DynPickTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DynPickTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ChoiceGroupPickTypeSerializer<'ser> {
        pub(super) value: &'ser super::ChoiceGroupPickType,
        pub(super) state: Box<ChoiceGroupPickTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ChoiceGroupPickTypeSerializerState<'ser> {
        Init__,
        Uv(<super::ChoiceGroupPickUvType as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ChoiceGroupPickTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ChoiceGroupPickTypeSerializerState::Init__ => {
                        match self.value {
                            super::ChoiceGroupPickType::Uv(x) => {
                                *self.state = ChoiceGroupPickTypeSerializerState::Uv(
                                    WithSerializer::serializer(x, Some("UV"), self.is_root)?,
                                )
                            }
                            super::ChoiceGroupPickType::Y(x) => {
                                *self.state = ChoiceGroupPickTypeSerializerState::Y(
                                    WithSerializer::serializer(x, Some("tns:Y"), self.is_root)?,
                                )
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ChoiceGroupPickTypeSerializerState::Uv(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ChoiceGroupPickTypeSerializerState::End__,
                        }
                    }
                    ChoiceGroupPickTypeSerializerState::Y(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ChoiceGroupPickTypeSerializerState::End__,
                        }
                    }
                    ChoiceGroupPickTypeSerializerState::End__ => {
                        *self.state = ChoiceGroupPickTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ChoiceGroupPickTypeSerializerState::Done__ => return Ok(None),
                    ChoiceGroupPickTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ChoiceGroupPickTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ChoiceGroupPickTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ChoiceGroupPickUvTypeSerializer<'ser> {
        pub(super) value: &'ser super::ChoiceGroupPickUvType,
        pub(super) state: Box<ChoiceGroupPickUvTypeSerializerState<'ser>>,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ChoiceGroupPickUvTypeSerializerState<'ser> {
        Init__,
        U(<String as WithSerializer>::Serializer<'ser>),
        V(<String as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ChoiceGroupPickUvTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ChoiceGroupPickUvTypeSerializerState::Init__ => match self.value {
                        super::ChoiceGroupPickUvType::U(x) => {
                            *self.state = ChoiceGroupPickUvTypeSerializerState::U(
                                WithSerializer::serializer(x, Some("tns:U"), self.is_root)?,
                            )
                        }
                        super::ChoiceGroupPickUvType::V(x) => {
                            *self.state = ChoiceGroupPickUvTypeSerializerState::V(
                                WithSerializer::serializer(x, Some("tns:V"), self.is_root)?,
                            )
                        }
                    },
                    ChoiceGroupPickUvTypeSerializerState::U(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ChoiceGroupPickUvTypeSerializerState::Done__,
                        }
                    }
                    ChoiceGroupPickUvTypeSerializerState::V(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ChoiceGroupPickUvTypeSerializerState::Done__,
                        }
                    }
                    ChoiceGroupPickUvTypeSerializerState::Done__ => return Ok(None),
                    ChoiceGroupPickUvTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ChoiceGroupPickUvTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ChoiceGroupPickUvTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SubstPickTypeSerializer<'ser> {
        pub(super) value: &'ser super::SubstPickType,
        pub(super) state: Box<SubstPickTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SubstPickTypeSerializerState<'ser> {
        Init__,
        Shape(<super::Shape as WithSerializer>::Serializer<'ser>),
        Y(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SubstPickTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SubstPickTypeSerializerState::Init__ => {
                        match self.value {
                            super::SubstPickType::Shape(x) => {
                                *self.state = SubstPickTypeSerializerState::Shape(
                                    WithSerializer::serializer(x, Some("tns:Shape"), self.is_root)?,
                                )
                            }
                            super::SubstPickType::Y(x) => {
                                *self.state = SubstPickTypeSerializerState::Y(
                                    WithSerializer::serializer(x, Some("tns:Y"), self.is_root)?,
                                )
                            }
                        }
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SubstPickTypeSerializerState::Shape(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SubstPickTypeSerializerState::End__,
                    },
                    SubstPickTypeSerializerState::Y(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = SubstPickTypeSerializerState::End__,
                    },
                    SubstPickTypeSerializerState::End__ => {
                        *self.state = SubstPickTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SubstPickTypeSerializerState::Done__ => return Ok(None),
                    SubstPickTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for SubstPickTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SubstPickTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ShapeTypeSerializer<'ser> {
        pub(super) value: &'ser super::ShapeType,
        pub(super) state: Box<ShapeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ShapeTypeSerializerState<'ser> {
        Init__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShapeTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut SerializeHelper,
        ) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ShapeTypeSerializerState::Init__ => {
                        *self.state = ShapeTypeSerializerState::Done__;
                        let mut bytes = BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&super::PREFIX_TNS),
                                &super::NS_TNS,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                        helper.end_ns_scope();
                        return Ok(Some(Event::Empty(bytes)));
                    }
                    ShapeTypeSerializerState::Done__ => return Ok(None),
                    ShapeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Serializer<'ser> for ShapeTypeSerializer<'ser> {
        fn next(&mut self, helper: &mut SerializeHelper) -> Option<Result<Event<'ser>, Error>> {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ShapeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
