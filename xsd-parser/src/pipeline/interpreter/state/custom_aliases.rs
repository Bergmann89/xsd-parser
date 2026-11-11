use crate::models::{
    meta::MetaTypeVariant,
    schema::xs::{Annotation, ComplexBaseTypeContent, ElementTypeContent, SimpleBaseTypeContent},
};

use super::{Node, State};

impl State<'_> {
    pub(super) fn prepare_custom_aliases(&mut self) {
        for (ident, ty) in &mut self.types.items {
            if !matches!(&ty.variant, MetaTypeVariant::Custom(custom) if custom.alias) {
                continue;
            }
            let Some(entry) = self.node_cache.get(ident) else {
                continue;
            };
            ty.schema.get_or_insert(ident.schema);
            if !ty.documentation.is_empty() {
                continue;
            }
            // Read only annotations: interpreting the replaced declaration could
            // reintroduce dependencies that the native mapping deliberately removes.
            let annotations: Vec<&Annotation> = match entry.node {
                Node::SimpleType(node) => node
                    .content
                    .iter()
                    .filter_map(|content| match content {
                        SimpleBaseTypeContent::Annotation(annotation) => Some(annotation),
                        _ => None,
                    })
                    .collect(),
                Node::ComplexType(node) => node
                    .content
                    .iter()
                    .filter_map(|content| match content {
                        ComplexBaseTypeContent::Annotation(annotation) => Some(annotation),
                        _ => None,
                    })
                    .collect(),
                Node::Element(node) => node
                    .content
                    .iter()
                    .filter_map(|content| match content {
                        ElementTypeContent::Annotation(annotation) => Some(annotation),
                        _ => None,
                    })
                    .collect(),
                Node::Attribute(node) => node.annotation.iter().collect(),
                Node::Group(_) | Node::AttributeGroup(_) => Vec::new(),
            };
            for annotation in annotations {
                if let Err(error) = annotation.extract_documentation_into(&mut ty.documentation) {
                    tracing::warn!(
                        "Unable to extract documentation for custom alias {ident}: {error}"
                    );
                }
            }
        }
    }
}
