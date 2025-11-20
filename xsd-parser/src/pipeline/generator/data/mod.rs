mod complex;
mod constrains;
mod dynamic;
mod enumeration;
mod reference;
mod simple;
mod type_;
mod union;

use std::mem::swap;

use crate::models::{
    code::IdentPath,
    data::{BuildInData, CustomData, PathData},
    meta::{BuildInMeta, CustomMeta},
};

use super::Context;

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

impl Context<'_, '_> {
    fn path_data_nillable(&self, is_mixed: bool, absolute: bool, mut path: PathData) -> PathData {
        if !is_mixed {
            path
        } else if absolute {
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

    fn path_data_mixed(&self, is_mixed: bool, absolute: bool, mut path: PathData) -> PathData {
        if !is_mixed {
            path
        } else if absolute {
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

    fn path_data_text(&self, absolute: bool) -> PathData {
        if absolute {
            let target_type = self.text_type.clone();

            PathData::from_path(target_type)
        } else {
            let target_type = self.text_type.ident().clone();
            let target_type = IdentPath::from_ident(target_type);

            PathData::from_path(target_type).with_using(self.text_type.to_string())
        }
    }
}
