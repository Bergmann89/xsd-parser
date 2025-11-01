mod defaults;
mod namespace_const;
mod quick_xml;
mod serde;
mod types;
mod with_namespace_trait;

use std::fmt::Display;
use std::ops::Bound;
use std::str::FromStr;

use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::config::RendererFlags;
use crate::models::{
    code::IdentPath,
    data::{ConfigValue, ConstrainsData, EnumerationData, SimpleData, UnionData},
};

use super::Context;

pub use self::defaults::DefaultsRenderStep;
pub use self::namespace_const::NamespaceConstantsRenderStep;
pub use self::quick_xml::{QuickXmlDeserializeRenderStep, QuickXmlSerializeRenderStep};
pub use self::serde::{
    SerdeQuickXmlTypesRenderStep, SerdeXmlRsV7TypesRenderStep, SerdeXmlRsV8TypesRenderStep,
};
pub use self::types::TypesRenderStep;
pub use self::with_namespace_trait::WithNamespaceTraitRenderStep;

impl Context<'_, '_> {
    pub(crate) fn render_type_docs(&self) -> Option<TokenStream> {
        self.render_docs(
            RendererFlags::RENDER_TYPE_DOCS,
            &self.data.documentation[..],
        )
    }

    pub(crate) fn render_docs(&self, flags: RendererFlags, docs: &[String]) -> Option<TokenStream> {
        self.check_renderer_flags(flags).then(|| {
            let docs = docs.iter().flat_map(|s| s.split('\n')).map(|s| {
                let s = s.trim_end();

                quote!(#[doc = #s])
            });

            quote!(#( #docs )*)
        })
    }
}

impl EnumerationData<'_> {
    fn render_common_impls(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            constrains,
            type_ident,
            ..
        } = self;

        if let Some(validate_str) = constrains.render_fn_validate_str(ctx) {
            let code = quote! {
                impl #type_ident {
                    #validate_str
                }
            };

            ctx.add_usings(["xsd_parser::quick_xml::ValidateError"]);
            ctx.current_module().append(code);
        }
    }
}

impl UnionData<'_> {
    fn render_common_impls(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            constrains,
            type_ident,
            ..
        } = self;

        if let Some(validate_str) = constrains.render_fn_validate_str(ctx) {
            let code = quote! {
                impl #type_ident {
                    #validate_str
                }
            };

            ctx.add_usings(["xsd_parser::quick_xml::ValidateError"]);
            ctx.current_module().append(code);
        }
    }
}

impl SimpleData<'_> {
    #[allow(clippy::too_many_lines)]
    fn render_common_impls(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            occurs,
            constrains,
            type_ident,
            target_type,
            ..
        } = self;

        let target_type = ctx.resolve_type_for_module(target_type);
        let target_type = occurs.make_type(&target_type, false).unwrap();

        let validate_str = constrains.render_fn_validate_str(ctx);
        let validate_value = constrains.render_fn_validate_value(&target_type);
        let call_validate_value = validate_value
            .as_ref()
            .map(|_| quote! { Self::validate_value(&inner)?; });

        let code = quote! {
            impl #type_ident {
                pub fn new(inner: #target_type) -> Result<Self, ValidateError> {
                    #call_validate_value

                    Ok(Self(inner))
                }

                pub fn into_inner(self) -> #target_type {
                    self.0
                }

                #validate_str
                #validate_value
            }

            impl From<#type_ident> for #target_type {
                fn from(value: #type_ident) -> #target_type {
                    value.0
                }
            }

            impl TryFrom<#target_type> for #type_ident {
                type Error = ValidateError;

                fn try_from(value: #target_type) -> Result<Self, ValidateError> {
                    Self::new(value)
                }
            }

            impl Deref for #type_ident {
                type Target = #target_type;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        };

        ctx.add_usings(["core::ops::Deref", "xsd_parser::quick_xml::ValidateError"]);
        ctx.current_module().append(code);
    }
}

impl ConstrainsData<'_> {
    fn render_fn_validate_str(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        self.meta.need_string_validation().then(|| {
            let validate_pattern = self.render_validate_pattern(ctx);
            let validate_total_digits = self.render_validate_total_digits(ctx);
            let validate_fraction_digits = self.render_validate_fraction_digits(ctx);

            quote! {
                pub fn validate_str(s: &str) -> Result<(), ValidateError> {
                    #validate_pattern
                    #validate_total_digits
                    #validate_fraction_digits

                    Ok(())
                }
            }
        })
    }

    fn render_fn_validate_value(&self, target_type: &TokenStream) -> Option<TokenStream> {
        self.meta.need_value_validation().then(|| {
            let validate_range_start = self.render_validate_range_start();
            let validate_range_end = self.render_validate_range_end();
            let validate_min_length = self.render_validate_min_length();
            let validate_max_length = self.render_validate_max_length();

            quote! {
                pub fn validate_value(value: &#target_type) -> Result<(), ValidateError> {
                    #validate_range_start
                    #validate_range_end
                    #validate_min_length
                    #validate_max_length

                    Ok(())
                }
            }
        })
    }

    fn render_validate_pattern(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        if self.meta.patterns.is_empty() {
            return None;
        }

        ctx.add_usings(["regex::Regex", "std::sync::LazyLock"]);

        let sz = self.meta.patterns.len();
        let patterns = self.meta.patterns.iter().map(|x| {
            let rx = Literal::string(x);

            quote!(Regex::new(#rx).unwrap())
        });

        Some(quote! {
            static PATTERNS: LazyLock<[Regex; #sz]> = LazyLock::new(|| [ #( #patterns )* ]);

            for pattern in PATTERNS.iter() {
                if !pattern.is_match(s) {
                    return Err(ValidateError::Pattern(pattern.as_str()));
                }
            }
        })
    }

    fn render_validate_total_digits(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        self.meta.total_digits.map(|x| {
            ctx.add_usings(["xsd_parser::quick_xml::total_digits"]);

            quote! {
                total_digits(s, #x)?;
            }
        })
    }

    fn render_validate_fraction_digits(&self, ctx: &Context<'_, '_>) -> Option<TokenStream> {
        self.meta.fraction_digits.map(|x| {
            ctx.add_usings(["xsd_parser::quick_xml::fraction_digits"]);

            quote! {
                fraction_digits(s, #x)?;
            }
        })
    }

    fn render_validate_min_length(&self) -> Option<TokenStream> {
        self.meta.min_length.as_ref().and_then(|x| {
            if *x == 0 {
                return None;
            }

            let check = if *x == 1 {
                quote!(value.is_empty())
            } else {
                quote!(value.len() < #x)
            };

            Some(quote! {
                if #check {
                    return Err(ValidateError::MinLength(#x));
                }
            })
        })
    }

    fn render_validate_max_length(&self) -> Option<TokenStream> {
        self.meta.max_length.as_ref().map(|x| {
            let check = if *x == 0 {
                quote!(!value.is_empty())
            } else {
                quote!(value.len() > #x)
            };

            quote! {
                if #check {
                    return Err(ValidateError::MaxLength(#x));
                }
            }
        })
    }

    fn render_validate_range_start(&self) -> Option<TokenStream> {
        match &self.range.start {
            Bound::Unbounded => None,
            Bound::Included(x) => {
                let Bound::Included(val) = &self.meta.range.start else {
                    unreachable!();
                };

                Some(quote! {
                    if *value < #x {
                        return Err(ValidateError::LessThan(#val));
                    }
                })
            }
            Bound::Excluded(x) => {
                let Bound::Excluded(val) = &self.meta.range.start else {
                    unreachable!();
                };

                Some(quote! {
                    if *value <= #x {
                        return Err(ValidateError::LessEqualThan(#val));
                    }
                })
            }
        }
    }

    fn render_validate_range_end(&self) -> Option<TokenStream> {
        match &self.range.end {
            Bound::Unbounded => None,
            Bound::Included(x) => {
                let Bound::Included(val) = &self.meta.range.end else {
                    unreachable!();
                };

                Some(quote! {
                    if *value > #x {
                        return Err(ValidateError::GraterThan(#val));
                    }
                })
            }
            Bound::Excluded(x) => {
                let Bound::Excluded(val) = &self.meta.range.end else {
                    unreachable!();
                };

                Some(quote! {
                    if *value >= #x {
                        return Err(ValidateError::GraterEqualThan(#val));
                    }
                })
            }
        }
    }
}

fn get_derive<I>(ctx: &Context<'_, '_>, extra: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: Display,
{
    let extra = extra
        .into_iter()
        .map(|x| IdentPath::from_str(&x.to_string()).expect("Invalid identifier path"));
    let types = ctx.derive.iter().cloned().chain(extra);

    let types = match &ctx.data.derive {
        ConfigValue::Default => types.collect::<Vec<_>>(),
        ConfigValue::Extend(extra) => types
            .chain(extra.iter().map(|x| IdentPath::from_ident(x.clone())))
            .collect::<Vec<_>>(),
        ConfigValue::Overwrite(types) => types
            .iter()
            .map(|x| IdentPath::from_ident(x.clone()))
            .collect::<Vec<_>>(),
    };

    if types.is_empty() {
        quote! {}
    } else {
        quote! {
            #[derive( #( #types ),* )]
        }
    }
}

fn get_dyn_type_traits<'a, I>(ctx: &'a Context<'_, '_>, extra: I) -> TokenStream
where
    I: IntoIterator<Item = &'a IdentPath>,
{
    format_traits(ctx.dyn_type_traits.iter().chain(extra).map(|ident| {
        ctx.add_usings([quote!(#ident)]);

        let ident = ident.ident();

        quote!(#ident)
    }))
}

fn format_traits<I>(iter: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: ToTokens,
{
    let parts = iter
        .into_iter()
        .enumerate()
        .map(|(i, x)| if i == 0 { quote!(#x) } else { quote!(+ #x) });

    quote! {
        #( #parts )*
    }
}

fn render_trait_impls<'a>(
    type_ident: &'a Ident2,
    trait_idents: &'a [TokenStream],
) -> impl Iterator<Item = TokenStream> + 'a {
    trait_idents.iter().map(move |trait_ident| {
        quote! {
            impl #trait_ident for #type_ident { }
        }
    })
}
