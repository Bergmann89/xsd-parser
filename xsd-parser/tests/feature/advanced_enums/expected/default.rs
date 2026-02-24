use xsd_parser_types::{misc::Namespace, xml::QName};
pub type Foo = FooType;
#[derive(Debug)]
pub struct FooType {
    pub string_enum: StringEnumType,
    pub q_name_enum: QNameEnumType,
}
#[derive(Debug)]
pub enum StringEnumType {
    Off,
    On,
    Auto,
    X123,
}
impl StringEnumType {
    pub const VALUE_OFF: &str = "OFF";
    pub const VALUE_ON: &str = "ON";
    pub const VALUE_AUTO: &str = "AUTO";
    pub const VALUE_X123: &str = "X123";
}
#[derive(Debug)]
pub enum QNameEnumType {
    TnsFoo,
    TnsBar,
    TnsBaz,
}
impl QNameEnumType {
    pub const VALUE_TNS_FOO: &'static QName = &QName::new_const(
        b"tns:Foo",
        Some(3usize),
        Some(Namespace::new_const(b"http://example.com")),
    );
    pub const VALUE_TNS_BAR: &'static QName = &QName::new_const(
        b"tns:Bar",
        Some(3usize),
        Some(Namespace::new_const(b"http://example.com")),
    );
    pub const VALUE_TNS_BAZ: &'static QName = &QName::new_const(
        b"tns:Baz",
        Some(3usize),
        Some(Namespace::new_const(b"http://example.com")),
    );
}
