use std::borrow::Cow;

use crate::models::{
    meta::{
        ElementMeta, ElementMetaVariant, ElementMode, ElementsMeta, GroupMeta, MetaType,
        MetaTypeVariant, MetaTypes,
    },
    schema::{MaxOccurs, MinOccurs},
    TypeIdent,
};

use super::{Error, Optimizer};

impl Optimizer {
    /// This will flatten the nested groups (`xs::all`, `xs::choice` or `xs::sequence`)
    /// of the complex type identified by `ident` to one type instead of rendering
    /// nested structures.
    ///
    /// # Errors
    ///
    /// Returns an error if the passed `ident` could not be found, the referenced
    /// type is not complex type or the complex type has no content.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../../tests/optimizer/complex_flatten.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected0/flatten_complex_types.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../../tests/optimizer/expected1/flatten_complex_types.rs")]
    /// ```
    pub fn flatten_complex_type(mut self, ident: TypeIdent) -> Result<Self, Error> {
        tracing::debug!("flatten_complex_type(ident={ident:?})");

        self.flatten_complex_type_impl(ident)?;

        Ok(self)
    }

    /// This will flatten all complex types.
    ///
    /// For details see [`flatten_complex_type`](Self::flatten_complex_type).
    pub fn flatten_complex_types(mut self) -> Self {
        tracing::debug!("flatten_complex_types");

        let idents = self
            .types
            .items
            .iter()
            .filter_map(|(ident, type_)| {
                if matches!(&type_.variant, MetaTypeVariant::ComplexType(ci) if ci.has_complex_content(&self.types)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            let _ = self.flatten_complex_type_impl(ident);
        }

        self
    }

    fn flatten_complex_type_impl(&mut self, ident: TypeIdent) -> Result<(), Error> {
        tracing::debug!("flatten_complex_type_impl(ident={ident:?})");

        let Some(ty) = self.types.items.get(&ident) else {
            return Err(Error::UnknownType(ident));
        };

        let MetaTypeVariant::ComplexType(ci) = &ty.variant else {
            return Err(Error::ExpectedComplexType(ident));
        };

        let Some(content_ident) = ci.content.clone() else {
            return Err(Error::MissingContentType(ident));
        };

        if let Some((occurs, variant)) = Flatten::new(&self.types, usize::MAX, true)
            .exec(&content_ident, 0)?
            .into_variant()
        {
            let type_ = MetaType::new(variant);

            self.types.items.insert(content_ident, type_);

            if let Some(MetaTypeVariant::ComplexType(ci)) = self.types.get_variant_mut(&ident) {
                ci.min_occurs *= occurs.min;
                ci.max_occurs *= occurs.max;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Flatten<'types> {
    types: &'types MetaTypes,
    max_depth: usize,
    flatten_named_groups: bool,
}

#[derive(Debug)]
struct Builder<'types> {
    mode_old: Mode,
    mode_new: Mode,
    meta_old: &'types GroupMeta,
    meta_new: Option<GroupMeta>,
}

#[derive(Debug, Clone, Copy)]
struct Occurs {
    min: MinOccurs,
    max: MaxOccurs,
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    All,
    Choice,
    Sequence,
    Mixed(Occurs),
}

impl<'types> Flatten<'types> {
    fn new(types: &'types MetaTypes, max_depth: usize, flatten_named_groups: bool) -> Self {
        Self {
            types,
            max_depth,
            flatten_named_groups,
        }
    }

    fn exec(&self, ident: &TypeIdent, depth: usize) -> Result<Builder<'types>, Error> {
        let Some(ty) = self.types.items.get(ident) else {
            return Err(Error::UnknownType(ident.clone()));
        };

        let (mode, gi) = match &ty.variant {
            MetaTypeVariant::All(gi) => (Mode::All, gi),
            MetaTypeVariant::Choice(gi) => (Mode::Choice, gi),
            MetaTypeVariant::Sequence(gi) => (Mode::Sequence, gi),
            _ => return Err(Error::ExpectedChoiceContent(ident.clone())),
        };

        let mut builder = Builder::new(mode, gi);
        if depth >= self.max_depth {
            return Ok(builder);
        }

        for (index, element) in gi.elements.0.iter().enumerate() {
            match &element.variant {
                ElementMetaVariant::Type {
                    mode: ElementMode::Element,
                    ..
                }
                | ElementMetaVariant::Any { .. }
                | ElementMetaVariant::Text => builder.add_element(true, Cow::Borrowed(element))?,
                ElementMetaVariant::Type {
                    type_,
                    mode: ElementMode::Group,
                } => {
                    let flatten = element.ident.name.is_generated() || self.flatten_named_groups;
                    if flatten {
                        let inner = self.exec(type_, depth + 1)?;

                        builder = builder.merge(element, &gi.elements[0..index], inner)?;
                    } else {
                        builder.add_element(true, Cow::Borrowed(element))?;
                    }
                }
            }
        }

        Ok(builder)
    }
}

impl<'types> Builder<'types> {
    fn new(mode: Mode, meta_old: &'types GroupMeta) -> Self {
        Self {
            mode_old: mode,
            mode_new: mode,
            meta_old,
            meta_new: None,
        }
    }

    fn into_mixed(mut self) -> Self {
        let meta_new = self.meta_new.get_or_insert_with(|| self.meta_old.clone());

        self.mode_new = match self.mode_new {
            Mode::All | Mode::Sequence => {
                let mut occurs = Occurs {
                    min: 0,
                    max: MaxOccurs::Bounded(0),
                };

                for element in &mut *meta_new.elements {
                    occurs.min += element.min_occurs;
                    occurs.max += element.max_occurs;

                    element.reset_occurs();
                }

                Mode::Mixed(occurs)
            }
            Mode::Choice => {
                let mut occurs = Occurs {
                    min: 1,
                    max: MaxOccurs::Bounded(1),
                };

                for element in &mut *meta_new.elements {
                    occurs.min = occurs.min.min(element.min_occurs);
                    occurs.max = occurs.max.max(element.max_occurs);

                    element.reset_occurs();
                }

                Mode::Mixed(occurs)
            }
            mode => mode,
        };

        self
    }

    fn into_meta(self) -> (Mode, GroupMeta) {
        let Self {
            mode_old: _,
            mode_new,
            meta_old,
            meta_new,
        } = self;

        let meta = meta_new.unwrap_or_else(|| meta_old.clone());

        (mode_new, meta)
    }

    fn into_variant(self) -> Option<(Occurs, MetaTypeVariant)> {
        let Self {
            mode_old: _,
            mode_new,
            meta_old: _,
            meta_new,
        } = self;

        let meta = meta_new?;

        let variant = match &mode_new {
            Mode::All => MetaTypeVariant::All(meta),
            Mode::Sequence => MetaTypeVariant::Sequence(meta),
            Mode::Choice | Mode::Mixed(_) => MetaTypeVariant::Choice(meta),
        };

        let occurs = match mode_new {
            Mode::All | Mode::Choice | Mode::Sequence => Occurs {
                min: 1,
                max: MaxOccurs::Bounded(1),
            },
            Mode::Mixed(occurs) => occurs,
        };

        Some((occurs, variant))
    }

    fn add_element(
        &mut self,
        update_occurs: bool,
        element: Cow<'_, ElementMeta>,
    ) -> Result<(), Error> {
        let Some(meta_new) = &mut self.meta_new else {
            return Ok(());
        };

        let existing = meta_new
            .elements
            .iter_mut()
            .find(|x| x.ident == element.ident);
        if let Some(existing) = existing {
            match (&existing.variant, &element.variant) {
                (ElementMetaVariant::Text, ElementMetaVariant::Text)
                | (ElementMetaVariant::Any { .. }, ElementMetaVariant::Any { .. }) => (),
                (
                    ElementMetaVariant::Type { type_: a, .. },
                    ElementMetaVariant::Type { type_: b, .. },
                ) if a == b => (),
                (a, b) => {
                    return Err(Error::Custom(format!(
                        "Mismatching element variants: {a:?} != {b:?}"
                    )))
                }
            }

            if update_occurs {
                if let Mode::Mixed(occurs) = &mut self.mode_new {
                    occurs.update(&self.mode_old, element.min_occurs, element.max_occurs);
                } else {
                    existing.min_occurs += element.min_occurs;
                    existing.max_occurs += element.max_occurs;
                }
            }
        } else {
            let mut element = element.into_owned();

            if update_occurs {
                if let Mode::Mixed(occurs) = &mut self.mode_new {
                    occurs.update(&self.mode_old, element.min_occurs, element.max_occurs);
                    element.reset_occurs();
                }
            }

            meta_new.elements.push(element);
        }

        Ok(())
    }

    fn merge(
        mut self,
        el: &ElementMeta,
        prev: &[ElementMeta],
        mut other: Self,
    ) -> Result<Self, Error> {
        self.meta_new.get_or_insert_with(|| GroupMeta {
            is_mixed: self.meta_old.is_mixed,
            elements: ElementsMeta(prev.to_vec()),
        });

        match (self.mode_new, other.mode_new) {
            (Mode::All, Mode::All) => (),
            (Mode::Choice, Mode::Choice) if el.max_occurs <= MaxOccurs::Bounded(1) => (),
            (Mode::Sequence, Mode::Sequence) if el.max_occurs <= MaxOccurs::Bounded(1) => (),
            (_, _) => {
                self = self.into_mixed();
                other = other.into_mixed();
            }
        }

        let (mode, meta) = other.into_meta();
        let meta_new = self.meta_new.as_mut().unwrap();

        meta_new.is_mixed = meta_new.is_mixed || meta.is_mixed;

        let mixed_mode = match (&mut self.mode_new, mode) {
            (Mode::All, Mode::All)
            | (Mode::Choice, Mode::Choice)
            | (Mode::Sequence, Mode::Sequence) => false,
            (Mode::Mixed(a), Mode::Mixed(mut b)) => {
                b.min *= el.min_occurs;
                b.max *= el.max_occurs;

                a.update(&self.mode_old, b.min, b.max);

                true
            }
            (_, _) => unreachable!(),
        };

        for mut element in meta.elements.0 {
            if mixed_mode {
                element.reset_occurs();
            } else {
                element.min_occurs *= el.min_occurs;
                element.max_occurs *= el.max_occurs;
            }

            self.add_element(!mixed_mode, Cow::Owned(element))?;
        }

        Ok(self)
    }
}

impl Occurs {
    fn update(&mut self, mode: &Mode, min: MinOccurs, max: MaxOccurs) {
        match mode {
            Mode::All | Mode::Sequence => {
                self.min += min;
                self.max += max;
            }
            Mode::Choice => {
                self.min = self.min.min(min);
                self.max = self.max.max(max);
            }
            Mode::Mixed(_) => unreachable!(),
        }
    }
}

impl ElementMeta {
    fn reset_occurs(&mut self) {
        self.min_occurs = 1;
        self.max_occurs = MaxOccurs::Bounded(1);
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        meta::{ElementMeta, ElementMode, ElementsMeta, MetaType, MetaTypeVariant, MetaTypes},
        schema::{xs::FormChoiceType, MaxOccurs},
        ElementIdent, TypeIdent,
    };
    use crate::Optimizer;

    macro_rules! make_type {
        ($types:expr, $name:literal, $type:ident $( $rest:tt )*) => {{
            let mut variant = MetaTypeVariant::$type(Default::default());
            let MetaTypeVariant::$type(info) = &mut variant else { unreachable!(); };
            make_type!(__init info $type $( $rest )*);
            let ty = MetaType::new(variant);
            $types.items.insert(TypeIdent::type_($name), ty);
        }};
        (__init $info:ident ComplexType($name:literal) $(,)? ) => {
            $info.content = Some(TypeIdent::type_($name));
        };
        (__init $info:ident $group:ident { $( $element:expr ),* $(,)? } $(,)? ) => {
            $info.elements = ElementsMeta(vec![
                $( $element ),*
            ]);
        };
    }

    macro_rules! max_occurs {
        (Unbounded) => {
            MaxOccurs::Unbounded
        };
        ($value:literal) => {
            MaxOccurs::Bounded($value)
        };
    }

    macro_rules! element_info {
        ($element_name:literal, $type_name:literal, $type:ident { $( $field:ident: $value:tt ),* $(,)? }) => {{
            let type_ident = element_info!(__type_ident $type $type_name);

            #[allow(unused_mut)]
            let mut element = ElementMeta::new(
                ElementIdent::named($element_name),
                type_ident,
                ElementMode::$type,
                FormChoiceType::Unqualified,
            );

            $(
                element_info!(__set_field element.$field $value);
            )*
            element
        }};
        (__type_ident Element $name:literal) => {
            TypeIdent::element($name)
        };
        (__type_ident Group $name:literal) => {
            TypeIdent::type_($name)
        };
        (__set_field $element:ident.min $value:expr) => {
            $element.min_occurs = $value;
        };
        (__set_field $element:ident.max $value:tt) => {
            $element.max_occurs = max_occurs!($value);
        };
    }

    macro_rules! assert_element {
        ($iter:expr, $name:expr, $min:literal, $max:tt) => {{
            let el = $iter.next().unwrap();
            assert_eq!(el.ident.name.to_string(), $name);
            assert_eq!(el.min_occurs, $min);
            assert_eq!(el.max_occurs, max_occurs!($max));
        }};
    }

    macro_rules! assert_type {
        ($types:expr, $main_ident:literal, $content_type:ident($content_ident:literal)) => {{
            let main = $types.get_variant(&TypeIdent::type_($main_ident)).unwrap();
            let content = $types
                .get_variant(&TypeIdent::type_($content_ident))
                .unwrap();

            let MetaTypeVariant::ComplexType(main) = main else {
                panic!("Wrong type");
            };
            let MetaTypeVariant::$content_type(content) = content else {
                panic!("Wrong type");
            };

            (main, content)
        }};
    }

    #[test]
    fn sequence_with_choice_expect_mixed() {
        let mut types = MetaTypes::default();
        make_type!(types, "main", ComplexType("content"));
        make_type!(
            types,
            "content",
            Sequence {
                element_info!("annotation", "annotation", Element { min: 0 }),
                element_info!("content", "inner-choice", Group {}),
                element_info!("extra", "extra", Element {}),
            }
        );
        make_type!(
            types,
            "inner-choice",
            Choice {
                element_info!("restriction", "restriction", Element {}),
                element_info!("list", "list", Element {}),
                element_info!("union", "union", Element {}),
            }
        );

        let types = Optimizer::new(types).flatten_complex_types().finish();

        let (main, content) = assert_type!(types, "main", Choice("content"));

        assert_eq!(main.min_occurs, 2);
        assert_eq!(main.max_occurs, MaxOccurs::Bounded(3));

        let mut it = content.elements.iter();

        assert_element!(it, "annotation", 1, 1);
        assert_element!(it, "restriction", 1, 1);
        assert_element!(it, "list", 1, 1);
        assert_element!(it, "union", 1, 1);
        assert_element!(it, "extra", 1, 1);

        assert!(it.next().is_none());
    }

    #[test]
    fn nested_sequences_expect_sequence() {
        let mut types = MetaTypes::default();
        make_type!(types, "main", ComplexType("content"));
        make_type!(
            types,
            "content",
            Sequence {
                element_info!("seq0", "seq0", Group {}),
                element_info!("seq1", "seq1", Group { min: 0 }),
            }
        );
        make_type!(
            types,
            "seq0",
            Sequence {
                element_info!("a", "a", Element { min: 0 }),
                element_info!("b", "b", Element {}),
                element_info!("c", "c", Element { max: Unbounded }),
            }
        );
        make_type!(
            types,
            "seq1",
            Sequence {
                element_info!("d", "d", Element { min: 0 }),
                element_info!("e", "e", Element {}),
                element_info!("f", "f", Element { max: Unbounded }),
            }
        );

        let types = Optimizer::new(types).flatten_complex_types().finish();

        let (main, content) = assert_type!(types, "main", Sequence("content"));

        assert_eq!(main.min_occurs, 1);
        assert_eq!(main.max_occurs, MaxOccurs::Bounded(1));

        let mut it = content.elements.iter();

        assert_element!(it, "a", 0, 1);
        assert_element!(it, "b", 1, 1);
        assert_element!(it, "c", 1, Unbounded);

        assert_element!(it, "d", 0, 1);
        assert_element!(it, "e", 0, 1);
        assert_element!(it, "f", 0, Unbounded);

        assert!(it.next().is_none());
    }

    #[test]
    fn nested_sequences_expect_mixed() {
        let mut types = MetaTypes::default();
        make_type!(types, "main", ComplexType("content"));
        make_type!(
            types,
            "content",
            Sequence {
                element_info!("seq0", "seq0", Group {}),
                element_info!("seq1", "seq1", Group { min: 0 }),
                element_info!("seq2", "seq2", Group { max: Unbounded }),
            }
        );
        make_type!(
            types,
            "seq0",
            Sequence {
                element_info!("a", "a", Element { min: 0 }),
                element_info!("b", "b", Element {}),
                element_info!("c", "c", Element { max: Unbounded }),
            }
        );
        make_type!(
            types,
            "seq1",
            Sequence {
                element_info!("d", "d", Element { min: 0 }),
                element_info!("e", "e", Element {}),
                element_info!("f", "f", Element { max: Unbounded }),
            }
        );
        make_type!(
            types,
            "seq2",
            Sequence {
                element_info!("g", "g", Element { min: 0 }),
                element_info!("h", "h", Element {}),
                element_info!("i", "i", Element { max: Unbounded }),
            }
        );

        let types = Optimizer::new(types).flatten_complex_types().finish();

        let (main, content) = assert_type!(types, "main", Choice("content"));

        assert_eq!(main.min_occurs, 4);
        assert_eq!(main.max_occurs, MaxOccurs::Unbounded);

        let mut it = content.elements.iter();

        assert_element!(it, "a", 1, 1);
        assert_element!(it, "b", 1, 1);
        assert_element!(it, "c", 1, 1);

        assert_element!(it, "d", 1, 1);
        assert_element!(it, "e", 1, 1);
        assert_element!(it, "f", 1, 1);

        assert_element!(it, "g", 1, 1);
        assert_element!(it, "h", 1, 1);
        assert_element!(it, "i", 1, 1);

        assert!(it.next().is_none());
    }

    #[test]
    fn nested_choices_expect_choice() {
        let mut types = MetaTypes::default();
        make_type!(types, "main", ComplexType("content"));
        make_type!(
            types,
            "content",
            Choice {
                element_info!("ch0", "ch0", Group {}),
                element_info!("ch1", "ch1", Group { min: 0 }),
            }
        );
        make_type!(
            types,
            "ch0",
            Choice {
                element_info!("a", "a", Element { min: 0 }),
                element_info!("b", "b", Element {}),
                element_info!("c", "c", Element { max: Unbounded }),
            }
        );
        make_type!(
            types,
            "ch1",
            Choice {
                element_info!("d", "d", Element { min: 0 }),
                element_info!("e", "e", Element {}),
                element_info!("f", "f", Element { max: Unbounded }),
            }
        );

        let types = Optimizer::new(types).flatten_complex_types().finish();

        let (main, content) = assert_type!(types, "main", Choice("content"));

        assert_eq!(main.min_occurs, 1);
        assert_eq!(main.max_occurs, MaxOccurs::Bounded(1));

        let mut it = content.elements.iter();

        assert_element!(it, "a", 0, 1);
        assert_element!(it, "b", 1, 1);
        assert_element!(it, "c", 1, Unbounded);

        assert_element!(it, "d", 0, 1);
        assert_element!(it, "e", 0, 1);
        assert_element!(it, "f", 0, Unbounded);

        assert!(it.next().is_none());
    }

    #[test]
    fn nested_choices_expect_mixed() {
        let mut types = MetaTypes::default();
        make_type!(types, "main", ComplexType("content"));
        make_type!(
            types,
            "content",
            Choice {
                element_info!("ch0", "ch0", Group {}),
                element_info!("ch1", "ch1", Group { min: 0 }),
                element_info!("ch2", "ch2", Group { max: Unbounded }),
            }
        );
        make_type!(
            types,
            "ch0",
            Choice {
                element_info!("a", "a", Element { min: 0 }),
                element_info!("b", "b", Element {}),
                element_info!("c", "c", Element { max: Unbounded }),
            }
        );
        make_type!(
            types,
            "ch1",
            Choice {
                element_info!("d", "d", Element { min: 0 }),
                element_info!("e", "e", Element {}),
                element_info!("f", "f", Element { max: Unbounded }),
            }
        );
        make_type!(
            types,
            "ch2",
            Choice {
                element_info!("g", "g", Element { min: 0 }),
                element_info!("h", "h", Element {}),
                element_info!("i", "i", Element { max: Unbounded }),
            }
        );

        let types = Optimizer::new(types).flatten_complex_types().finish();

        let (main, content) = assert_type!(types, "main", Choice("content"));

        assert_eq!(main.min_occurs, 0);
        assert_eq!(main.max_occurs, MaxOccurs::Unbounded);

        let mut it = content.elements.iter();

        assert_element!(it, "a", 1, 1);
        assert_element!(it, "b", 1, 1);
        assert_element!(it, "c", 1, 1);

        assert_element!(it, "d", 1, 1);
        assert_element!(it, "e", 1, 1);
        assert_element!(it, "f", 1, 1);

        assert_element!(it, "g", 1, 1);
        assert_element!(it, "h", 1, 1);
        assert_element!(it, "i", 1, 1);

        assert!(it.next().is_none());
    }

    #[test]
    fn group_with_mixed_content_is_referenced_twice_expect_mixed() {
        let mut types = MetaTypes::default();

        make_type!(types, "main", ComplexType("content"));
        make_type!(
            types,
            "content",
            Sequence {
                element_info!("some", "some", Element { }),
                element_info!("group_1", "group", Group { }),
                element_info!("group_2", "group", Group { }),
            }
        );
        make_type!(
            types,
            "group",
            Choice {
                element_info!("bar", "bar", Element { }),
                element_info!("baz", "baz", Element { }),
                element_info!("subgroup", "subgroup", Group { }),
            }
        );
        make_type!(
            types,
            "subgroup",
            Sequence {
                element_info!("fizz", "fizz", Element { }),
                element_info!("buzz", "buzz", Element { }),
            }
        );

        let types = Optimizer::new(types).flatten_complex_types().finish();

        let (main, content) = assert_type!(types, "main", Choice("content"));

        assert_eq!(main.min_occurs, 3);
        assert_eq!(main.max_occurs, MaxOccurs::Bounded(5));

        let mut it = content.elements.iter();

        assert_element!(it, "some", 1, 1);
        assert_element!(it, "bar", 1, 1);
        assert_element!(it, "baz", 1, 1);
        assert_element!(it, "fizz", 1, 1);
        assert_element!(it, "buzz", 1, 1);

        assert!(it.next().is_none());
    }
}
