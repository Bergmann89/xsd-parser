mod defaults;
mod namespace_const;
mod quick_xml;
mod serde;
mod types;
mod with_namespace_trait;

use std::fmt::Display;
use std::ops::Bound;

use proc_macro2::{Ident as Ident2, Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::config::RendererFlags;
use crate::models::{
    code::IdentPath,
    data::{ConfigValue, SimpleData},
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

impl SimpleData<'_> {
    #[allow(clippy::too_many_lines)]
    fn render_common_impls(&self, ctx: &mut Context<'_, '_>) {
        let Self {
            type_ident,
            target_type,
            ..
        } = self;

        let target_type = ctx.resolve_type_for_module(target_type);

        /* Min Length */

        let min_length = self.meta.min_length.as_ref().map(|x| {
            quote! {
                if s.len() < #x {
                    return Err(ValidateError::MinLength(#x));
                }
            }
        });

        /* Max Length */

        let max_length = self.meta.max_length.as_ref().map(|x| {
            quote! {
                if s.len() > #x {
                    return Err(ValidateError::MaxLength(#x));
                }
            }
        });

        /* Pattern */

        let pattern = self.meta.pattern.as_ref().map(|x| {
            ctx.add_usings(["regex::Regex", "std::sync::LazyLock"]);

            let rx = Literal::string(x);

            quote! {
                static PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(#rx).unwrap());
                if !PATTERN.is_match(s) {
                    return Err(ValidateError::Pattern(#rx));
                }
            }
        });

        /* Total Digits */

        let total_digits = self.meta.total_digits.map(|x| {
            ctx.add_usings(["xsd_parser::quick_xml::total_digits"]);

            quote! {
                total_digits(s, #x)?;
            }
        });

        /* Fraction Digits */

        let fraction_digits = self.meta.fraction_digits.map(|x| {
            ctx.add_usings(["xsd_parser::quick_xml::fraction_digits"]);

            quote! {
                fraction_digits(s, #x)?;
            }
        });

        /* Range Start */

        let range_start = match &self.range.start {
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
        };

        /* Range End */

        let range_end = match &self.range.end {
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
        };

        /* fn validate_str */

        let validate_str = self.need_string_validation().then(|| {
            quote! {
                pub fn validate_str(s: &str) -> Result<(), ValidateError> {
                    #pattern
                    #min_length
                    #max_length
                    #total_digits
                    #fraction_digits

                    Ok(())
                }
            }
        });

        /* fn validate_value */

        let validate_value = self.need_value_validation().then(|| {
            quote! {
                pub fn validate_value(value: &#target_type) -> Result<(), ValidateError> {
                    #range_start
                    #range_end

                    Ok(())
                }
            }
        });
        let call_validate_value = validate_value
            .is_some()
            .then(|| quote! { Self::validate_value(&inner)?; });

        /* Render Actual Code */

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

fn get_derive<I>(ctx: &Context<'_, '_>, extra: I) -> TokenStream
where
    I: IntoIterator,
    I::Item: Display,
{
    let extra = extra.into_iter().map(|x| format_ident!("{x}"));
    let types = ctx.derive.iter().cloned().chain(extra);

    let types = match &ctx.data.derive {
        ConfigValue::Default => types.collect::<Vec<_>>(),
        ConfigValue::Extend(extra) => types.chain(extra.iter().cloned()).collect::<Vec<_>>(),
        ConfigValue::Overwrite(types) => types.clone(),
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
