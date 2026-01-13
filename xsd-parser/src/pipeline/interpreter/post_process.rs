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
        let mut rename = false;

        for j in i + 1..len {
            if meta.elements[i].ident.name.as_str() == meta.elements[j].ident.name.as_str() {
                rename = true;
                set_display_name(schemas, &mut meta.elements[j]);
            }
        }

        if rename {
            set_display_name(schemas, &mut meta.elements[i]);
        }
    }
}

fn set_display_name(schemas: &Schemas, el: &mut ElementMeta) {
    let prefix = schemas
        .get_namespace_info(&el.ident.ns)
        .and_then(NamespaceInfo::name);
    if let Some(prefix) = prefix {
        let name = el.ident.name.as_str();
        let name = format!("{prefix}_{name}");

        el.display_name = Some(name);
    }
}
