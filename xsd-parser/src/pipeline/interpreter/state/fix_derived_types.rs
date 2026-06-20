use crate::{
    config::MetaType,
    models::meta::{BuildInMeta, MetaTypeVariant},
};

use super::State;

impl State<'_> {
    /// Fixes the derived types by resolving their base types. This is necessary
    /// because the derived types may reference other types that have not been
    /// fully resolved during the initial parsing phase. For example the variant
    /// of an element may change from `Referece` to `Dynamic` after a type references
    /// it as substitution group.
    pub(super) fn fix_derived_types(&mut self) {
        for ident in self.types.items.keys().cloned().collect::<Vec<_>>() {
            let placeholder = MetaType::new(MetaTypeVariant::BuildIn(BuildInMeta::String));
            let mut ty = self.types.items.insert(ident.clone(), placeholder).unwrap();

            let MetaTypeVariant::Dynamic(meta) = &mut ty.variant else {
                self.types.items.insert(ident.clone(), ty);

                continue;
            };

            for (key, derived) in &mut meta.derived_types {
                loop {
                    let Some((ident, derived_ty)) = self.types.get_resolved(&derived.type_) else {
                        break;
                    };
                    if key.type_ != ident.type_ {
                        derived.type_ = ident.clone();
                        break;
                    }
                    let MetaTypeVariant::Dynamic(derived_meta) = &derived_ty.variant else {
                        break;
                    };
                    let Some(base) = &derived_meta.type_ else {
                        break;
                    };

                    derived.type_ = base.clone();
                }
            }

            self.types.items.insert(ident, ty);
        }
    }
}
