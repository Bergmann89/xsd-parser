use crate::models::{
    meta::{ElementMeta, GroupMeta, MetaTypeVariant, MetaTypes},
    schema::{NamespaceInfo, Schemas},
};

pub(super) fn fix_element_naming_conflicts(schemas: &Schemas, types: &mut MetaTypes) {
    for type_ in types.items.values_mut() {
        if let MetaTypeVariant::All(gm)
        | MetaTypeVariant::Choice(gm)
        | MetaTypeVariant::Sequence(gm) = &mut type_.variant
        {
            fix_group_naming_conflicts(schemas, gm);
        }
    }
}

fn fix_group_naming_conflicts(schemas: &Schemas, meta: &mut GroupMeta) {
    let len = meta.elements.len();
    for i in 0..len {
        let mut index = 2;
        let mut ns_renamed = false;

        for j in i + 1..len {
            let ident_i = &meta.elements[i].ident;
            let ident_j = &meta.elements[j].ident;
            if ident_i.name.as_str() == ident_j.name.as_str() {
                if ident_i.ns == ident_j.ns {
                    set_index_display_name(&mut meta.elements[j], &mut index);
                } else {
                    ns_renamed = true;
                    set_ns_display_name(&mut meta.elements[j], schemas);
                }
            }
        }

        if ns_renamed {
            set_ns_display_name(&mut meta.elements[i], schemas);
        }
    }
}

fn set_ns_display_name(el: &mut ElementMeta, schemas: &Schemas) {
    let prefix = schemas
        .get_namespace_info(&el.ident.ns)
        .and_then(NamespaceInfo::name);
    if let Some(prefix) = prefix {
        let name = el.ident.name.as_str();
        let name = format!("{prefix}_{name}");

        el.display_name = Some(name);
    }
}

fn set_index_display_name(el: &mut ElementMeta, index: &mut usize) {
    let name = el.ident.name.as_str();
    let name = format!("{name}_{index}");

    el.display_name = Some(name);

    *index += 1;
}
