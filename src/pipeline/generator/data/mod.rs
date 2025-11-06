mod complex;
mod constrains;
mod dynamic;
mod enumeration;
mod reference;
mod simple;
mod type_;
mod union;

use std::mem::swap;

use quote::format_ident;

use crate::models::{
    code::IdentPath,
    data::{BuildInData, CustomData, PathData},
    meta::{BuildInMeta, CustomMeta},
};

impl<'types> BuildInData<'types> {
    fn new(meta: &'types BuildInMeta) -> Self {
        Self { meta }
    }
}

impl<'types> CustomData<'types> {
    fn new(meta: &'types CustomMeta) -> Self {
        Self { meta }
    }
}

impl PathData {
    fn from_path_data_nillable(is_mixed: bool, mut path: PathData) -> PathData {
        if is_mixed {
            let mut tmp = IdentPath::from_ident(format_ident!("Nillable"));

            swap(&mut path.path, &mut tmp);

            path.with_generic(tmp)
                .with_using("xsd_parser::xml::Nillable")
        } else {
            path
        }
    }

    fn from_path_data_mixed(is_mixed: bool, mut path: PathData) -> PathData {
        if is_mixed {
            let mut tmp = IdentPath::from_ident(format_ident!("Mixed"));

            swap(&mut path.path, &mut tmp);

            path.with_generic(tmp).with_using("xsd_parser::xml::Mixed")
        } else {
            path
        }
    }

    fn text() -> Self {
        let target_type = format_ident!("Text");
        let target_type = IdentPath::from_ident(target_type);

        Self::from_path(target_type).with_using("xsd_parser::xml::Text")
    }
}
