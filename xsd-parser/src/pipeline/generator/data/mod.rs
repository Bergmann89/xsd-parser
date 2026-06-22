mod complex;
mod constrains;
mod custom;
mod dynamic;
mod enumeration;
mod reference;
mod simple;
mod type_;
mod union;

use std::borrow::Cow;
use std::mem::swap;

use crate::config::GeneratorFlags;
use crate::models::{
    code::IdentPath,
    data::{BuildInData, PathData},
    meta::BuildInMeta,
};

use super::Context;

impl<'types> BuildInData<'types> {
    fn new(meta: &'types BuildInMeta) -> Self {
        Self {
            meta: Cow::Borrowed(meta),
        }
    }
}

impl Context<'_, '_> {
    fn path_data_nillable(&self, is_mixed: bool, mut path: PathData) -> PathData {
        if !is_mixed {
            path
        } else if self.check_generator_flags(GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS) {
            let mut tmp = self.nillable_type.clone();

            swap(&mut path.path, &mut tmp);

            path.with_generic(tmp)
        } else {
            let mut tmp = IdentPath::from_ident(self.nillable_type.ident().clone());

            swap(&mut path.path, &mut tmp);

            path.with_generic(tmp)
                .with_using(self.nillable_type.to_string())
        }
    }

    fn path_data_mixed(&self, is_mixed: bool, mut path: PathData) -> PathData {
        if !is_mixed {
            path
        } else if self.check_generator_flags(GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS) {
            let mut tmp = self.mixed_type.clone();

            swap(&mut path.path, &mut tmp);

            path.with_generic(tmp)
        } else {
            let mut tmp = IdentPath::from_ident(self.mixed_type.ident().clone());

            swap(&mut path.path, &mut tmp);

            path.with_generic(tmp)
                .with_using(self.mixed_type.to_string())
        }
    }

    fn path_data_text(&self) -> PathData {
        if self.check_generator_flags(GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS) {
            let target_type = self.text_type.clone();

            PathData::from_path(target_type)
        } else {
            let target_type = self.text_type.ident().clone();
            let target_type = IdentPath::from_ident(target_type);

            PathData::from_path(target_type).with_using(self.text_type.to_string())
        }
    }
}
