use crate::models::{
    data::{
        BuildInData, ComplexData, ConfigValue, CustomData, DataType, DynamicData, EnumerationData,
        ReferenceData, UnionData,
    },
    meta::MetaType,
};

use super::super::{Context, Error};

impl<'types> DataType<'types> {
    pub(crate) fn new(
        meta: &'types MetaType,
        ctx: &mut Context<'_, 'types>,
    ) -> Result<Self, Error> {
        use crate::models::data::DataTypeVariant as D;
        use crate::models::meta::MetaTypeVariant as M;

        let derive = ConfigValue::Default;
        let variant = match &meta.variant {
            M::BuildIn(x) => D::BuildIn(BuildInData::new(x)),
            M::Custom(x) => D::Custom(CustomData::new(x)),
            M::Union(x) => D::Union(UnionData::new(x, ctx)?),
            M::Dynamic(x) => D::Dynamic(DynamicData::new(x, ctx)?),
            M::Reference(x) => D::Reference(ReferenceData::new(x, ctx)?),
            M::Enumeration(x) => D::Enumeration(EnumerationData::new(x, ctx)?),
            M::All(x) => D::Complex(ComplexData::new_all(x, ctx)?),
            M::Choice(x) => D::Complex(ComplexData::new_choice(x, ctx)?),
            M::Sequence(x) => D::Complex(ComplexData::new_sequence(x, ctx)?),
            M::ComplexType(x) => D::Complex(ComplexData::new_complex(x, ctx)?),
        };

        Ok(Self {
            meta,
            derive,
            variant,
        })
    }
}
