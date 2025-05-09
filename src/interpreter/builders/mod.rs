mod element;
pub(crate) use element::ElementBuilder;
mod simple_type;
pub(crate) use simple_type::SimpleTypeBuilder;
mod complex_type;
pub(crate) use complex_type::ComplexTypeBuilder;
mod attribute_type;
pub(crate) use attribute_type::AttributeTypeBuilder;

use std::borrow::Cow;

use crate::schema::xs::{Any, AnyAttribute, AttributeType, ElementType, GroupType};
use crate::types::{AnyAttributeInfo, AnyInfo, AttributeInfo, ElementInfo};

/* Update */

pub(crate) trait Update<T> {
    fn update(&mut self, other: &T);
}

impl<T: Clone> Update<Option<T>> for T {
    fn update(&mut self, other: &Option<T>) {
        if let Some(value) = other {
            *self = value.clone();
        }
    }
}

impl<T: Clone> Update<Option<T>> for Option<T> {
    fn update(&mut self, other: &Option<T>) {
        if let Some(value) = other {
            *self = Some(value.clone());
        }
    }
}

impl Update<GroupType> for ElementInfo {
    fn update(&mut self, other: &GroupType) {
        self.min_occurs = other.min_occurs;
        self.max_occurs = other.max_occurs;
    }
}

impl Update<Any> for AnyInfo {
    fn update(&mut self, other: &Any) {
        self.min_occurs = Some(other.min_occurs);
        self.max_occurs = Some(other.max_occurs);
        self.process_contents = Some(other.process_contents.clone());

        self.namespace.update(&other.namespace);
        self.not_q_name.update(&other.not_q_name);
        self.not_namespace.update(&other.not_namespace);
    }
}

impl Update<AnyAttribute> for AnyAttributeInfo {
    fn update(&mut self, other: &AnyAttribute) {
        self.process_contents = Some(other.process_contents.clone());

        self.namespace.update(&other.namespace);
        self.not_q_name.update(&other.not_q_name);
        self.not_namespace.update(&other.not_namespace);
    }
}

impl Update<ElementType> for ElementInfo {
    fn update(&mut self, other: &ElementType) {
        self.min_occurs = other.min_occurs;
        self.max_occurs = other.max_occurs;
    }
}

impl Update<AttributeType> for AttributeInfo {
    fn update(&mut self, other: &AttributeType) {
        self.use_ = other.use_.clone();
        self.default.update(&other.default);
    }
}

/* CreateOrUpdate */

pub(crate) trait CreateOrUpdate<T> {
    fn create_or_update(&mut self, other: &T);
}

impl<T, X> CreateOrUpdate<T> for Option<X>
where
    X: Update<T> + Default,
{
    fn create_or_update(&mut self, other: &T) {
        if let Some(x) = self {
            x.update(other);
        } else {
            let mut x = X::default();
            x.update(other);
            *self = Some(x);
        }
    }
}

/* Patch */

pub(crate) trait Patch<T>: Clone {
    fn patch(&self, other: &T) -> Cow<'_, Self>;
}

impl Patch<GroupType> for GroupType {
    fn patch(&self, other: &GroupType) -> Cow<'_, Self> {
        let mut ret = self.clone();

        ret.min_occurs = other.min_occurs;
        ret.max_occurs = other.max_occurs;

        Cow::Owned(ret)
    }
}
