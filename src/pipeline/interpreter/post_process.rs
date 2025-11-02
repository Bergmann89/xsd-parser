use crate::models::{
    meta::{GroupMeta, MetaTypeVariant, MetaTypes},
    schema::{NamespaceInfo, Schemas},
    Ident, Name,
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
                prefix_ident(schemas, &mut meta.elements[j].ident);
            }
        }

        if rename {
            prefix_ident(schemas, &mut meta.elements[i].ident);
        }
    }
}

fn prefix_ident(schemas: &Schemas, ident: &mut Ident) {
    let prefix = ident
        .ns
        .as_ref()
        .and_then(|ns| schemas.get_namespace_info(ns))
        .and_then(NamespaceInfo::name);
    if let Some(prefix) = prefix {
        let name = ident.name.as_str();
        let name = format!("{prefix}_{name}");
        ident.name = Name::new_generated(Name::unify(&name));
    }
}
