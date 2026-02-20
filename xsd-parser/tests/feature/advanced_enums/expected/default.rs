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
}
impl StringEnumType {
    pub const OFF: &str = "OFF";
    pub const ON: &str = "ON";
    pub const AUTO: &str = "AUTO";
}
#[derive(Debug)]
pub enum QNameEnumType {
    TnsFoo,
    TnsBar,
    TnsBaz,
}
impl QNameEnumType {
    pub const TNS_FOO: &'static QName = &QName::new_const(
        b"tns:Foo",
        Some(3usize),
        Some(Namespace::new_const(b"http://example.com")),
    );
    pub const TNS_BAR: &'static QName = &QName::new_const(
        b"tns:Bar",
        Some(3usize),
        Some(Namespace::new_const(b"http://example.com")),
    );
    pub const TNS_BAZ: &'static QName = &QName::new_const(
        b"tns:Baz",
        Some(3usize),
        Some(Namespace::new_const(b"http://example.com")),
    );
}
