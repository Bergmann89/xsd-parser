use crate::models::meta::{AttributeMeta, ElementMeta};
use crate::models::schema::xs::{AttributeType, ElementType, GroupType};

pub(super) trait Update<T> {
    fn update(&mut self, other: &T);
}

impl<T: Clone> Update<Option<T>> for T {
    #[inline]
    fn update(&mut self, other: &Option<T>) {
        if let Some(value) = other {
            *self = value.clone();
        }
    }
}

impl<T: Clone> Update<Option<T>> for Option<T> {
    #[inline]
    fn update(&mut self, other: &Option<T>) {
        if let Some(value) = other {
            *self = Some(value.clone());
        }
    }
}

impl Update<GroupType> for ElementMeta {
    #[inline]
    fn update(&mut self, other: &GroupType) {
        self.min_occurs = other.min_occurs;
        self.max_occurs = other.max_occurs;
    }
}

impl Update<ElementType> for ElementMeta {
    #[inline]
    fn update(&mut self, other: &ElementType) {
        self.min_occurs = other.min_occurs;
        self.max_occurs = other.max_occurs;
        self.nillable.update(&other.nillable);
    }
}

impl Update<AttributeType> for AttributeMeta {
    #[inline]
    fn update(&mut self, other: &AttributeType) {
        self.use_ = other.use_;
        self.default.update(&other.default);
    }
}
