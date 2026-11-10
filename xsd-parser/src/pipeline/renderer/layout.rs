use quote::{format_ident, quote};

use crate::config::GeneratorFlags;
use crate::models::{code::Module, data::DataTypeVariant};

use super::MetaData;

pub(super) fn finish(meta: &MetaData<'_>, root: &mut Module) {
    if !meta.check_generator_flags(GeneratorFlags::USE_TYPE_MODULES) {
        return;
    }
    for data in meta.types.items.values() {
        if matches!(
            data.variant,
            DataTypeVariant::BuildIn(_) | DataTypeVariant::Custom(_)
        ) {
            continue;
        }
        let mut path = data.module_path.clone();
        let Some(name) = path.0.pop() else { continue };
        let Some(parent) = path.0.iter().try_fold(&mut *root, |module, segment| {
            module.modules.get_mut(&segment.to_string())
        }) else {
            continue;
        };
        let Some(module) = parent.modules.get_mut(&name.to_string()) else {
            continue;
        };
        // Keep private XML helpers in the file containing their owning type.
        for helper in ["quick_xml_serialize", "quick_xml_deserialize"] {
            if let Some(child) = module.modules.remove(helper) {
                let helper = format_ident!("{helper}");
                module.append(quote! { mod #helper { #child } });
            }
        }
        if module.exports.is_empty() {
            continue;
        }
        let exports = module.exports.values();
        let code = quote! { pub use self::#name::{ #( #exports ),* }; };
        parent.append(code);
    }
}
