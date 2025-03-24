use inflector::Inflector;
use quote::format_ident;

use crate::{code::IdentPath, types::Module};

mod impl_;
mod quick_xml;
mod types;

impl Module {
    pub(super) fn make_ns_const(&self) -> IdentPath {
        let ident = format_ident!(
            "NS_{}",
            self.name
                .as_ref()
                .map_or_else(|| String::from("DEFAULT"), ToString::to_string)
                .to_screaming_snake_case()
        );

        IdentPath::from_parts([], ident)
    }
}
