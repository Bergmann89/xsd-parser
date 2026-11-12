use std::{borrow::Cow, collections::HashSet, slice};

use proc_macro2::{Ident as Ident2, TokenStream, TokenTree};
use quote::{quote, ToTokens};

use crate::{
    config::TypedefMode,
    models::{
        data::{
            ComplexDataAttribute, ComplexDataContent, ComplexDataElement, DataTypeVariant,
            EnumerationDataVariant, Occurs, PathData, ReferenceData, SimpleData, UnionTypeVariant,
        },
        meta::{AttributeMetaVariant, ElementMetaVariant},
        TypeIdent,
    },
    pipeline::renderer::Context,
};

use super::{large_arrays, skip_empty};

#[derive(Debug, Clone, Copy)]
#[allow(clippy::struct_excessive_bools)] // Independent renderer policies can be combined.
pub(super) struct SerdeOptions {
    pub(super) large_arrays: bool,
    pub(super) arrays_as_vec: bool,
    pub(super) skip_none: bool,
    pub(super) skip_empty_vecs: bool,
}

impl SerdeOptions {
    fn occurs(self, occurs: Occurs) -> Occurs {
        if self.arrays_as_vec {
            occurs.array_to_vec()
        } else {
            occurs
        }
    }
}

#[derive(Debug, Default)]
#[allow(clippy::struct_excessive_bools)] // Mirrors independent Serde field attributes.
pub(super) struct FieldOverrides {
    pub(super) serialize: bool,
    pub(super) deserialize: bool,
    pub(super) has_default: bool,
    pub(super) has_custom_default: bool,
    pub(super) skip: bool,
}

impl FieldOverrides {
    fn new(attributes: &[TokenStream]) -> Self {
        let mut result = Self::default();
        for attribute in attributes {
            let mut tokens = attribute.clone().into_iter();
            let Some(TokenTree::Ident(name)) = tokens.next() else {
                continue;
            };
            if name == "serde_as" {
                result.serialize = true;
                result.deserialize = true;
                continue;
            }
            if name != "serde" {
                continue;
            }
            let Some(TokenTree::Group(arguments)) = tokens.next() else {
                continue;
            };
            let mut arguments = arguments.stream().into_iter().peekable();
            while let Some(token) = arguments.next() {
                if let TokenTree::Ident(name) = token {
                    match name.to_string().as_str() {
                        "with" => {
                            result.serialize = true;
                            result.deserialize = true;
                        }
                        "serialize_with" => result.serialize = true,
                        "deserialize_with" => result.deserialize = true,
                        "default" => {
                            result.has_default = true;
                            result.has_custom_default |= matches!(
                                arguments.peek(),
                                Some(TokenTree::Punct(p)) if p.as_char() == '='
                            );
                        }
                        "skip" | "skip_serializing" | "skip_serializing_if" => result.skip = true,
                        _ => (),
                    }
                }
            }
        }
        result
    }
}

/// Collects the generated arguments of one field into a single Serde attribute.
pub(super) struct SerdeAttribute {
    overrides: FieldOverrides,
    has_schema_default: bool,
    with: Option<TokenStream>,
    default: Option<TokenStream>,
    skip_serializing_if: Option<TokenStream>,
    rename: Option<TokenStream>,
}

impl SerdeAttribute {
    pub(super) fn new(extra_attributes: &[TokenStream]) -> Self {
        Self {
            overrides: FieldOverrides::new(extra_attributes),
            has_schema_default: false,
            with: None,
            default: None,
            skip_serializing_if: None,
            rename: None,
        }
    }

    pub(super) fn with_default(mut self, default: bool) -> Self {
        if default && !self.overrides.has_default && self.default.is_none() {
            self.default = Some(quote!(default));
        }
        self
    }

    pub(super) fn with_default_function(mut self, path: Option<String>) -> Self {
        if let Some(path) = path.filter(|_| !self.overrides.has_default) {
            self.default = Some(quote!(default = #path));
            self.has_schema_default = true;
        }
        self
    }

    pub(super) fn with_rename(mut self, name: impl AsRef<str>) -> Self {
        let name = name.as_ref();
        self.rename = Some(quote!(rename = #name));
        self
    }

    pub(super) fn with_field<'a>(
        mut self,
        ctx: &'a Context<'_, '_>,
        options: SerdeOptions,
        field: SerdeField<'a>,
    ) -> Self {
        if !options.large_arrays && !options.skip_none && !options.skip_empty_vecs {
            return self;
        }
        let fields = resolve_aliases(ctx, options, &field);
        if let Some(adapter) = large_arrays::arguments(ctx, options, &fields, &self.overrides) {
            self.with = Some(adapter.argument);
            self = self.with_default(adapter.needs_default);
        }
        if self.has_schema_default {
            return self;
        }
        if let Some(skip) = skip_empty::arguments(
            ctx,
            options,
            &fields,
            &self.overrides,
            self.default.is_some() || self.overrides.has_default,
        ) {
            let predicate = skip.predicate;
            self.skip_serializing_if = Some(quote!(skip_serializing_if = #predicate));
            self = self.with_default(skip.needs_default);
        }
        self
    }
}

impl ToTokens for SerdeAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let arguments: Vec<_> = [
            &self.with,
            &self.default,
            &self.skip_serializing_if,
            &self.rename,
        ]
        .into_iter()
        .flatten()
        .collect();
        if !arguments.is_empty() {
            tokens.extend(quote!(#[serde(#(#arguments),*)]));
        }
    }
}

/// The Rust container shape of a named field or a tuple payload.
#[derive(Clone, Copy)]
pub(super) struct SerdeField<'a> {
    pub(super) occurs: Occurs,
    pub(super) boxed: bool,
    pub(super) target: Option<&'a TypeIdent>,
    pub(super) named: bool,
}

impl SerdeField<'_> {
    pub(super) fn as_payload(mut self) -> Self {
        self.named = false;
        self
    }
}

/// Resolve the typedef chain once for both array adapters and omission rules.
fn resolve_aliases<'a, 'b>(
    ctx: &'a Context<'_, '_>,
    options: SerdeOptions,
    field: &'b SerdeField<'a>,
) -> Cow<'b, [SerdeField<'a>]> {
    let resolve = |target: Option<&TypeIdent>| {
        let item = ctx.meta.types.items.get(target?)?;
        match &item.variant {
            DataTypeVariant::Reference(reference) if reference.mode == TypedefMode::Typedef => {
                Some(reference)
            }
            _ => None,
        }
    };
    let mut target = field.target;
    let Some(mut reference) = resolve(target) else {
        return Cow::Borrowed(slice::from_ref(field));
    };
    let mut fields = vec![*field];
    let mut visited = HashSet::new();
    while let Some(ident) = target {
        if !visited.insert(ident) {
            break;
        }
        let field = reference.serde_field(options);
        target = field.target;
        fields.push(field);
        let Some(next) = resolve(target) else {
            break;
        };
        reference = next;
    }
    Cow::Owned(fields)
}

impl PathData {
    fn serde_target_ident<'a>(&self, ident: Option<&'a TypeIdent>) -> Option<&'a TypeIdent> {
        // Generic XML wrappers serialize their own shape, not the schema's inner value.
        ident.filter(|_| self.generics.is_empty())
    }
}

impl Occurs {
    pub(super) fn array_to_vec(self) -> Self {
        match self {
            Self::StaticList(_) => Self::DynamicList,
            x => x,
        }
    }
}

impl SimpleData<'_> {
    pub(super) fn serde_field(&self) -> SerdeField<'_> {
        SerdeField {
            occurs: self.occurs,
            boxed: false,
            target: self.target_type.serde_target_ident(Some(&self.meta.base)),
            named: false,
        }
    }
}

impl ReferenceData<'_> {
    pub(super) fn serde_field(&self, options: SerdeOptions) -> SerdeField<'_> {
        SerdeField {
            occurs: options.occurs(self.occurs),
            boxed: false,
            target: self.target_type.serde_target_ident(Some(&self.meta.type_)),
            named: false,
        }
    }
}

impl UnionTypeVariant<'_> {
    pub(super) fn serde_field(&self) -> SerdeField<'_> {
        SerdeField {
            occurs: Occurs::Single,
            boxed: false,
            target: self.target_type.serde_target_ident(Some(&self.meta.type_)),
            named: false,
        }
    }
}

impl EnumerationDataVariant<'_> {
    pub(super) fn serde_field(&self) -> SerdeField<'_> {
        SerdeField {
            occurs: Occurs::Single,
            boxed: false,
            target: self
                .target_type
                .as_ref()
                .and_then(|target| target.serde_target_ident(self.meta.type_.as_ref())),
            named: false,
        }
    }
}

impl ComplexDataElement<'_> {
    pub(super) fn serde_field(&self, options: SerdeOptions) -> SerdeField<'_> {
        let target = match &self.meta().variant {
            ElementMetaVariant::Type { type_, .. } => Some(type_),
            _ => None,
        };
        SerdeField {
            occurs: options.occurs(self.occurs),
            boxed: self.need_indirection,
            target: self.target_type.serde_target_ident(target),
            named: true,
        }
    }
}

impl ComplexDataContent<'_> {
    pub(super) fn serde_field(&self, options: SerdeOptions) -> SerdeField<'_> {
        SerdeField {
            occurs: options.occurs(self.occurs),
            boxed: false,
            target: self.target_type.serde_target_ident(self.simple_type),
            named: true,
        }
    }
}

impl ComplexDataAttribute<'_> {
    pub(super) fn serde_default_path(&self, type_ident: &Ident2) -> Option<String> {
        self.default_value
            .is_some()
            .then(|| format!("{type_ident}::default_{}", self.ident))
    }

    pub(super) fn serde_field(&self) -> SerdeField<'_> {
        let target = match &self.meta.variant {
            AttributeMetaVariant::Type(type_) => Some(type_),
            _ => None,
        };
        SerdeField {
            occurs: if self.is_option {
                Occurs::Optional
            } else {
                Occurs::Single
            },
            boxed: false,
            target: self.target_type.serde_target_ident(target),
            named: true,
        }
    }
}
