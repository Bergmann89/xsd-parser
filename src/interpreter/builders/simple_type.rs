use std::ops::{Deref, DerefMut};
use std::str::from_utf8;

use tracing::instrument;

use crate::schema::xs::{Facet, FacetType, List, Restriction, SimpleBaseType, Union, Use};
use crate::schema::MaxOccurs;
use crate::types::{
    Base, Ident, IdentType, Name, ReferenceInfo, SimpleType, SimpleTypeVariant, Type,
    UnionTypeInfo, VariantInfo, VecHelper,
};

use super::super::{Error, SchemaInterpreter};

#[derive(Debug)]
pub(crate) struct SimpleTypeBuilder<'a, 'schema, 'state> {
    /// Type variant that is constructed by the builder
    pub variant: Option<SimpleTypeVariant>,

    /// `true` if `type_` is fixed and can not be changed anymore
    pub fixed: bool,

    pub owner: &'a mut SchemaInterpreter<'schema, 'state>,
}

/* any type */

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(dead_code)]
enum UpdateMode {
    Extension,
    Restriction,
}

/// Initialize the type of a `$builder` to any type `$variant`.
macro_rules! init_any {
    ($builder:expr, $variant:ident, $value:expr, $fixed:expr) => {{
        $builder.variant = Some(SimpleTypeVariant::$variant($value));
        $builder.fixed = $fixed;

        let SimpleTypeVariant::$variant(ret) = $builder.variant.as_mut().unwrap() else {
            crate::unreachable!();
        };

        ret
    }};
}

/// Get the type `$variant` of a `$builder` or set the type variant if unset.
macro_rules! get_or_init_any {
    ($builder:expr, $variant:ident) => {
        get_or_init_any!($builder, $variant, Default::default())
    };
    ($builder:expr, $variant:ident, $default:expr) => {
        match &mut $builder.variant {
            None => init_any!($builder, $variant, $default, true),
            Some(SimpleTypeVariant::$variant(ret)) => ret,
            _ if !$builder.fixed => init_any!($builder, $variant, $default, true),
            Some(e) => crate::unreachable!("Type is expected to be a {:?}", e),
        }
    };
}

/* TypeBuilder */

impl<'a, 'schema, 'state> SimpleTypeBuilder<'a, 'schema, 'state> {
    pub(crate) fn new(owner: &'a mut SchemaInterpreter<'schema, 'state>) -> Self {
        Self {
            variant: None,
            fixed: false,
            owner,
        }
    }

    pub(crate) fn finish(self) -> Result<Type, Error> {
        self.variant
            .map(SimpleType::new)
            .map(Type::SimpleType)
            .ok_or(Error::NoType)
    }

    #[instrument(err, level = "trace", skip(self))]
    pub(crate) fn apply_simple_type(&mut self, ty: &SimpleBaseType) -> Result<(), Error> {
        use crate::schema::xs::SimpleBaseTypeContent as C;

        for c in &ty.content {
            match c {
                C::Annotation(_) => (),
                C::Restriction(x) => self.apply_simple_type_restriction(x)?,
                C::Union(x) => self.apply_union(x)?,
                C::List(x) => self.apply_list(x)?,
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_simple_type_restriction(&mut self, ty: &Restriction) -> Result<(), Error> {
        use crate::schema::xs::RestrictionContent as C;

        let base = ty
            .base
            .as_ref()
            .map(|base| {
                let base = self.parse_qname(base)?;

                self.copy_base_type(&base, UpdateMode::Restriction)?;

                Ok(base)
            })
            .transpose()?;

        for c in &ty.content {
            match c {
                C::Annotation(_) => (),
                C::SimpleType(x) => self.apply_simple_type(x)?,
                C::Facet(x) => self.apply_facet(x)?,
            }
        }

        if let Some(base) = base {
            match &mut self.variant {
                Some(SimpleTypeVariant::Reference(_)) => (),
                Some(SimpleTypeVariant::Union(e)) => e.base = Base::Extension(base),
                Some(SimpleTypeVariant::Enumeration(e)) => e.base = Base::Extension(base),
                e => unreachable!("Unexpected type: {e:#?}"),
            }
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub fn apply_facet(&mut self, ty: &Facet) -> Result<(), Error> {
        match ty {
            Facet::Enumeration(x) => self.apply_enumeration(x)?,
            x => tracing::warn!("Unknown facet: {x:#?}"),
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub fn apply_enumeration(&mut self, ty: &FacetType) -> Result<(), Error> {
        let name = Name::from(ty.value.trim().to_owned());
        let ident = Ident::new(name)
            .with_ns(self.state.current_ns())
            .with_type(IdentType::Enumeration);

        let ei = get_or_init_any!(self, Enumeration);
        let var = ei.variants.find_or_insert(ident, VariantInfo::new);
        var.use_ = Use::Required;

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    pub fn apply_union(&mut self, ty: &Union) -> Result<(), Error> {
        let ui = get_or_init_any!(self, Union);

        if let Some(types) = &ty.member_types {
            for type_ in &types.0 {
                let type_ = self.owner.parse_qname(type_)?;
                ui.types.push(UnionTypeInfo::new(type_));
            }
        }

        let ns = self.owner.state.current_ns();

        for x in &ty.simple_type {
            let name = self
                .owner
                .state
                .name_builder()
                .or(&x.name)
                .auto_extend2(false, true, self.owner.state)
                .finish();
            let type_ = self.owner.create_simple_type(ns, Some(name), None, x)?;
            ui.types.push(UnionTypeInfo::new(type_));
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip(self))]
    fn apply_list(&mut self, ty: &List) -> Result<(), Error> {
        let mut type_ = None;

        if let Some(s) = &ty.item_type {
            type_ = Some(self.owner.parse_qname(s)?);
        }

        if let Some(x) = &ty.simple_type {
            let last_named_type = self.state.last_named_type(false).map(ToOwned::to_owned);
            let name = self
                .state
                .name_builder()
                .or(&x.name)
                .or_else(|| from_utf8(ty.item_type.as_ref()?.local_name()).ok())
                .or_else(|| {
                    let s = last_named_type?;
                    let s = s.as_str();
                    let s = s.strip_suffix("Type").unwrap_or(s);
                    let s = format!("{s}Item");

                    Some(Name::from(s))
                })
                .finish();
            let ns = self.owner.state.current_ns();
            type_ = Some(self.owner.create_simple_type(ns, Some(name), None, x)?);
        }

        if let Some(type_) = type_ {
            let ti = get_or_init_any!(self, Reference, ReferenceInfo::new(type_.clone()));
            ti.type_ = type_;
            ti.min_occurs = 0;
            ti.max_occurs = MaxOccurs::Unbounded;

            Ok(())
        } else {
            Ok(())
        }
    }

    fn copy_base_type(&mut self, base: &Ident, _mode: UpdateMode) -> Result<(), Error> {
        let base = {
            self.fixed = false;

            self.owner.get_simple_type_variant(base)?
        };

        tracing::debug!("{base:#?}");

        let mut base = base.clone();

        if let SimpleTypeVariant::Enumeration(ei) = &mut base {
            ei.variants.clear();
        }

        self.variant = Some(base);

        tracing::debug!("{:#?}", self.variant);

        Ok(())
    }
}

impl<'schema, 'state> Deref for SimpleTypeBuilder<'_, 'schema, 'state> {
    type Target = SchemaInterpreter<'schema, 'state>;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}

impl DerefMut for SimpleTypeBuilder<'_, '_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.owner
    }
}
