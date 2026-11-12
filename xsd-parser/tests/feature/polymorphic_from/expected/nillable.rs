use xsd_parser_types::xml::Nillable;
#[derive(Debug)]
pub enum Head {
    Plain(bool),
    Nil(NilDyn),
}
impl From<bool> for Head {
    fn from(value: bool) -> Self {
        Self::Plain(value)
    }
}
impl From<NilDyn> for Head {
    fn from(value: NilDyn) -> Self {
        Self::Nil(value)
    }
}
pub type NilDyn = Nillable<bool>;
#[derive(Debug)]
pub enum Ambiguous {
    PlainMember(bool),
    NilA(NilADyn),
    NilB(NilBDyn),
}
impl From<bool> for Ambiguous {
    fn from(value: bool) -> Self {
        Self::PlainMember(value)
    }
}
pub type NilADyn = Nillable<bool>;
pub type NilBDyn = Nillable<bool>;
