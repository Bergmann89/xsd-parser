use xsd_parser::schema::Namespace;
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub type Shiporder = ShiporderType;
#[derive(Debug, Clone)]
pub struct ShiporderType {
    pub orderid: String,
    pub orderperson: String,
    pub shipto: ShiporderShiptoType,
    pub item: Vec<ShiporderItemType>,
}
impl xsd_parser::quick_xml::WithSerializer for ShiporderType {
    type Serializer<'x> = quick_xml_serialize::ShiporderTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
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
impl xsd_parser::quick_xml::WithSerializer for ShiporderShiptoType {
    type Serializer<'x> = quick_xml_serialize::ShiporderShiptoTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
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
impl xsd_parser::quick_xml::WithSerializer for ShiporderItemType {
    type Serializer<'x> = quick_xml_serialize::ShiporderItemTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, xsd_parser::quick_xml::Error> {
        Ok(quick_xml_serialize::ShiporderItemTypeSerializer {
            value: self,
            state: quick_xml_serialize::ShiporderItemTypeSerializerState::Init__,
            name: name.unwrap_or("ShiporderItem"),
            is_root,
        })
    }
}
pub mod quick_xml_serialize {
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
        Orderperson(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Shipto(
            <super::ShiporderShiptoType as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
        ),
        Item(
            xsd_parser::quick_xml::IterSerializer<
                'ser,
                Vec<super::ShiporderItemType>,
                super::ShiporderItemType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    ShiporderTypeSerializerState::Init__ => {
                        self.state = ShiporderTypeSerializerState::Orderperson(
                            xsd_parser::quick_xml::WithSerializer::serializer(
                                &self.value.orderperson,
                                Some("orderperson"),
                                false,
                            )?,
                        );
                        let mut bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        xsd_parser::quick_xml::write_attrib(
                            &mut bytes,
                            "orderid",
                            &self.value.orderid,
                        )?;
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    ShiporderTypeSerializerState::Orderperson(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderTypeSerializerState::Shipto(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    &self.value.shipto,
                                    Some("shipto"),
                                    false,
                                )?,
                            )
                        }
                    },
                    ShiporderTypeSerializerState::Shipto(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderTypeSerializerState::Item(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.item,
                                    Some("item"),
                                    false,
                                ),
                            )
                        }
                    },
                    ShiporderTypeSerializerState::Item(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = ShiporderTypeSerializerState::End__,
                    },
                    ShiporderTypeSerializerState::End__ => {
                        self.state = ShiporderTypeSerializerState::Done__;
                        return Ok(Some(xsd_parser::quick_xml::Event::End(
                            xsd_parser::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    ShiporderTypeSerializerState::Done__ => return Ok(None),
                    ShiporderTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for ShiporderTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
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
        Name(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Address(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        City(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Country(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderShiptoTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    ShiporderShiptoTypeSerializerState::Init__ => {
                        self.state = ShiporderShiptoTypeSerializerState::Name(
                            xsd_parser::quick_xml::WithSerializer::serializer(
                                &self.value.name,
                                Some("name"),
                                false,
                            )?,
                        );
                        let bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    ShiporderShiptoTypeSerializerState::Name(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderShiptoTypeSerializerState::Address(
                                xsd_parser::quick_xml::WithSerializer::serializer(
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
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    &self.value.city,
                                    Some("city"),
                                    false,
                                )?,
                            )
                        }
                    },
                    ShiporderShiptoTypeSerializerState::City(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderShiptoTypeSerializerState::Country(
                                xsd_parser::quick_xml::WithSerializer::serializer(
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
                        return Ok(Some(xsd_parser::quick_xml::Event::End(
                            xsd_parser::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    ShiporderShiptoTypeSerializerState::Done__ => return Ok(None),
                    ShiporderShiptoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for ShiporderShiptoTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
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
        Title(<String as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Note(xsd_parser::quick_xml::IterSerializer<'ser, Option<String>, String>),
        Quantity(<usize as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        Price(<f64 as xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ShiporderItemTypeSerializer<'ser> {
        fn next_event(
            &mut self,
        ) -> Result<Option<xsd_parser::quick_xml::Event<'ser>>, xsd_parser::quick_xml::Error>
        {
            loop {
                match &mut self.state {
                    ShiporderItemTypeSerializerState::Init__ => {
                        self.state = ShiporderItemTypeSerializerState::Title(
                            xsd_parser::quick_xml::WithSerializer::serializer(
                                &self.value.title,
                                Some("title"),
                                false,
                            )?,
                        );
                        let bytes = xsd_parser::quick_xml::BytesStart::new(self.name);
                        return Ok(Some(xsd_parser::quick_xml::Event::Start(bytes)));
                    }
                    ShiporderItemTypeSerializerState::Title(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderItemTypeSerializerState::Note(
                                xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.note,
                                    Some("note"),
                                    false,
                                ),
                            )
                        }
                    },
                    ShiporderItemTypeSerializerState::Note(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            self.state = ShiporderItemTypeSerializerState::Quantity(
                                xsd_parser::quick_xml::WithSerializer::serializer(
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
                            self.state = ShiporderItemTypeSerializerState::Price(
                                xsd_parser::quick_xml::WithSerializer::serializer(
                                    &self.value.price,
                                    Some("price"),
                                    false,
                                )?,
                            )
                        }
                    },
                    ShiporderItemTypeSerializerState::Price(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => self.state = ShiporderItemTypeSerializerState::End__,
                    },
                    ShiporderItemTypeSerializerState::End__ => {
                        self.state = ShiporderItemTypeSerializerState::Done__;
                        return Ok(Some(xsd_parser::quick_xml::Event::End(
                            xsd_parser::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    ShiporderItemTypeSerializerState::Done__ => return Ok(None),
                    ShiporderItemTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> core::iter::Iterator for ShiporderItemTypeSerializer<'ser> {
        type Item = Result<xsd_parser::quick_xml::Event<'ser>, xsd_parser::quick_xml::Error>;
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
