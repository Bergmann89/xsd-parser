use xsd_parser_types::misc::Namespace;

use crate::models::{
    meta::{CustomMeta, MetaTypeVariant},
    schema::{NamespaceId, SchemaId},
    IdentType, Name, TypeIdent,
};

use super::Optimizer;

impl Optimizer {
    /// Replaces the `xs:anyType` with [`AnyElement`](xsd_parser_types::xml::AnyElement).
    ///
    /// See [`Optimizer::replace_xs_any_type`] for details.
    pub fn replace_xs_any_type_with_any_element(self) -> Self {
        self.replace_xs_any_type("::xsd_parser_types::xml::AnyElement")
    }

    /// Normally the `xs:anyType` is a complex type accepting any number of
    /// attributes, elements and intermixed text. Dealing with this complex
    /// type is sometimes unneeded and can be represented by a simpler custom
    /// defined type.
    ///
    /// This needs to be done in the optimizer (after the interpreter) because
    /// the interpreter may derive other types from the original `xs:anyType`
    /// complex type, to it can't be simplified before the interpreter has been
    /// executed.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../../tests/optimizer/any_type.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected0/replace_xs_any_type_with_any_element.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected1/replace_xs_any_type_with_any_element.rs")]
    /// ```
    pub fn replace_xs_any_type<X>(mut self, any_type: X) -> Self
    where
        X: Into<String>,
    {
        let ns = self
            .types
            .modules
            .values()
            .find_map(|meta| {
                matches!(&meta.namespace, Some(ns) if *ns == Namespace::XS)
                    .then_some(meta.namespace_id)
            })
            .unwrap_or(NamespaceId::ANONYMOUS);
        let ident = TypeIdent {
            ns,
            schema: SchemaId::UNKNOWN,
            name: Name::ANY_TYPE,
            type_: IdentType::Type,
        };

        if let Some(ty) = self.types.items.get_mut(&ident) {
            ty.variant = MetaTypeVariant::Custom(
                CustomMeta::new("AnyElement")
                    .include_from(any_type)
                    .with_allow_any(true),
            );
        }

        self
    }
}
