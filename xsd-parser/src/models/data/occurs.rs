use crate::models::schema::{MaxOccurs, MinOccurs};

/// Defines the occurrence (how often the field is used) of a field in a specific type.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Occurs {
    /// The field is not used at all.
    #[default]
    None,

    /// The field is used exactly one time.
    Single,

    /// The field is used optional (zero or exactly one time).
    Optional,

    /// The field is used as a dynamic list.
    DynamicList,

    /// The field is used as a list with a fixed size.
    StaticList(usize),
}

impl Occurs {
    /// Create the [`Occurs`] value from the [`MinOccurs`] and [`MaxOccurs`] from
    /// the XSD schema.
    #[must_use]
    pub fn from_occurs(min: MinOccurs, max: MaxOccurs) -> Self {
        match (min, max) {
            (0, MaxOccurs::Bounded(0)) => Self::None,
            (1, MaxOccurs::Bounded(1)) => Self::Single,
            (0, MaxOccurs::Bounded(1)) => Self::Optional,
            (a, MaxOccurs::Bounded(b)) if a == b => Self::StaticList(a),
            (_, _) => Self::DynamicList,
        }
    }

    /// Returns `true` if not `Occurs::None`
    #[must_use]
    pub fn is_some(&self) -> bool {
        *self != Self::None
    }

    /// Returns `true` if [`make_type`](Self::make_type) would generate a static type
    /// (a type without memory indirection), like `T`, `Option<T>` or `[T; SIZE]`.
    #[must_use]
    pub fn is_direct(&self) -> bool {
        matches!(self, Self::Single | Self::Optional | Self::StaticList(_))
    }
}
