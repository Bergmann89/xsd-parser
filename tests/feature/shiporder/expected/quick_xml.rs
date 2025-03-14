pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
use xsd_parser::{
    quick_xml::{Error, WithSerializer},
    schema::Namespace,
};
pub type Shiporder = ShiporderType;
#[derive(Debug, Clone)]
pub struct ShiporderType {
    pub orderid: String,
    pub orderperson: String,
    pub shipto: ShiporderShiptoType,
    pub item: Vec<ShiporderItemType>,
}
impl WithSerializer for ShiporderType {
    type Serializer<'x> = quick_xml_serialize::ShiporderTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ShiporderTypeSerializer {
            value: self,
            state: quick_xml_serialize::ShiporderTypeSerializerState::Init__,
            name: name.unwrap_or("shiporder"),
            is_root,
        })
    }
}
#[derive(Debug, Clone)]
pub struct ShiporderShiptoType {
    pub name: String,
    pub address: String,
    pub city: String,
    pub country: String,
}
impl WithSerializer for ShiporderShiptoType {
    type Serializer<'x> = quick_xml_serialize::ShiporderShiptoTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ShiporderShiptoTypeSerializer {
            value: self,
            state: quick_xml_serialize::ShiporderShiptoTypeSerializerState::Init__,
            name: name.unwrap_or("ShiporderShipto"),
            is_root,
        })
    }
}
#[derive(Debug, Clone)]
pub struct ShiporderItemType {
    pub title: String,
    pub note: Option<String>,
    pub quantity: usize,
    pub price: f64,
}
impl WithSerializer for ShiporderItemType {
    type Serializer<'x> = quick_xml_serialize::ShiporderItemTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ShiporderItemTypeSerializer {
            value: self,
            state: quick_xml_serialize::ShiporderItemTypeSerializerState::Init__,
            name: name.unwrap_or("ShiporderItem"),
            is_root,
        })
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{
        write_attrib, BytesEnd, BytesStart, Error, Event, IterSerializer, WithSerializer,
    };
    #[derive(Debug)]
    pub struct ShiporderTypeSerializer<'ser> {
        pub(super) value: &'ser super::ShiporderType,
        pub(super) state: ShiporderTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ShiporderTypeSerializerState<'ser> {
        Init__,
        Orderperson(<String as WithSerializer>::Serializer<'ser>),
        Shipto(<super::ShiporderShiptoType as WithSerializer>::Serializer<'ser>),
        Item(IterSerializer<'ser, Vec<super::ShiporderItemType>, super::ShiporderItemType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    ShiporderTypeSerializerState::Init__ => {
                        self.state =
                            ShiporderTypeSerializerState::Orderperson(WithSerializer::serializer(
                                &self.value.orderperson,
                                Some("orderperson"),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        write_attrib(&mut bytes, "orderid", &self.value.orderid)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ShiporderTypeSerializerState::Orderperson(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state =
                                ShiporderTypeSerializerState::Shipto(WithSerializer::serializer(
                                    &self.value.shipto,
                                    Some("shipto"),
                                    false,
                                )?)
                        }
                    },
                    ShiporderTypeSerializerState::Shipto(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                self.state = ShiporderTypeSerializerState::Item(
                                    IterSerializer::new(&self.value.item, Some("item"), false),
                                )
                            }
                        }
                    }
                    ShiporderTypeSerializerState::Item(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = ShiporderTypeSerializerState::End__,
                    },
                    ShiporderTypeSerializerState::End__ => {
                        self.state = ShiporderTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderTypeSerializerState::Done__ => return Ok(None),
                    ShiporderTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ShiporderTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = ShiporderTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ShiporderShiptoTypeSerializer<'ser> {
        pub(super) value: &'ser super::ShiporderShiptoType,
        pub(super) state: ShiporderShiptoTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ShiporderShiptoTypeSerializerState<'ser> {
        Init__,
        Name(<String as WithSerializer>::Serializer<'ser>),
        Address(<String as WithSerializer>::Serializer<'ser>),
        City(<String as WithSerializer>::Serializer<'ser>),
        Country(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderShiptoTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    ShiporderShiptoTypeSerializerState::Init__ => {
                        self.state = ShiporderShiptoTypeSerializerState::Name(
                            WithSerializer::serializer(&self.value.name, Some("name"), false)?,
                        );
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ShiporderShiptoTypeSerializerState::Name(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderShiptoTypeSerializerState::Address(
                                WithSerializer::serializer(
                                    &self.value.address,
                                    Some("address"),
                                    false,
                                )?,
                            )
                        }
                    },
                    ShiporderShiptoTypeSerializerState::Address(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderShiptoTypeSerializerState::City(
                                WithSerializer::serializer(&self.value.city, Some("city"), false)?,
                            )
                        }
                    },
                    ShiporderShiptoTypeSerializerState::City(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderShiptoTypeSerializerState::Country(
                                WithSerializer::serializer(
                                    &self.value.country,
                                    Some("country"),
                                    false,
                                )?,
                            )
                        }
                    },
                    ShiporderShiptoTypeSerializerState::Country(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = ShiporderShiptoTypeSerializerState::End__,
                    },
                    ShiporderShiptoTypeSerializerState::End__ => {
                        self.state = ShiporderShiptoTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderShiptoTypeSerializerState::Done__ => return Ok(None),
                    ShiporderShiptoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ShiporderShiptoTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = ShiporderShiptoTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ShiporderItemTypeSerializer<'ser> {
        pub(super) value: &'ser super::ShiporderItemType,
        pub(super) state: ShiporderItemTypeSerializerState<'ser>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ShiporderItemTypeSerializerState<'ser> {
        Init__,
        Title(<String as WithSerializer>::Serializer<'ser>),
        Note(IterSerializer<'ser, Option<String>, String>),
        Quantity(<usize as WithSerializer>::Serializer<'ser>),
        Price(<f64 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderItemTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut self.state {
                    ShiporderItemTypeSerializerState::Init__ => {
                        self.state = ShiporderItemTypeSerializerState::Title(
                            WithSerializer::serializer(&self.value.title, Some("title"), false)?,
                        );
                        let bytes = BytesStart::new(self.name);
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ShiporderItemTypeSerializerState::Title(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderItemTypeSerializerState::Note(
                                IterSerializer::new(&self.value.note, Some("note"), false),
                            )
                        }
                    },
                    ShiporderItemTypeSerializerState::Note(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderItemTypeSerializerState::Quantity(
                                WithSerializer::serializer(
                                    &self.value.quantity,
                                    Some("quantity"),
                                    false,
                                )?,
                            )
                        }
                    },
                    ShiporderItemTypeSerializerState::Quantity(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state =
                                ShiporderItemTypeSerializerState::Price(WithSerializer::serializer(
                                    &self.value.price,
                                    Some("price"),
                                    false,
                                )?)
                        }
                    },
                    ShiporderItemTypeSerializerState::Price(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = ShiporderItemTypeSerializerState::End__,
                    },
                    ShiporderItemTypeSerializerState::End__ => {
                        self.state = ShiporderItemTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ShiporderItemTypeSerializerState::Done__ => return Ok(None),
                    ShiporderItemTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ShiporderItemTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    self.state = ShiporderItemTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
