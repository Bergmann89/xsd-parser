pub mod sch;
pub mod xml;
pub mod xs;
use crate::models::schema::Namespace;
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_SCH: Namespace = Namespace::new_const(b"http://purl.oclc.org/dsdl/schematron");
