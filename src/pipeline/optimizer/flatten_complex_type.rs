use crate::models::{
    meta::{ElementMeta, ElementMetaVariant, ElementMode, GroupMeta, MetaType, MetaTypeVariant},
    schema::{MaxOccurs, MinOccurs},
    Ident,
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
    pub fn flatten_complex_type(mut self, ident: Ident) -> Result<Self, Error> {
        tracing::debug!("flatten_complex_type(ident={ident:?})");

        let Some(ty) = self.types.items.get(&ident) else {
            return Err(Error::UnknownType(ident));
        };

        let MetaTypeVariant::ComplexType(ci) = &ty.variant else {
            return Err(Error::ExpectedComplexType(ident));
        };

        let Some(content_ident) = ci.content.clone() else {
            return Err(Error::MissingContentType(ident));
        };

        let mut ctx = Context::default();

        self.flatten_complex_type_impl(&content_ident, 1, MaxOccurs::Bounded(1), &mut ctx);

        if ctx.count > 1 {
            let variant = match ctx.mode {
                Mode::Unknown => unreachable!(),
                Mode::Sequence => MetaTypeVariant::Sequence(ctx.meta),
                Mode::Mixed | Mode::Choice => MetaTypeVariant::Choice(ctx.meta),
            };
            let type_ = MetaType::new(variant);

            self.types.items.insert(content_ident, type_);

            if let Some(MetaTypeVariant::ComplexType(ci)) = self.types.get_variant_mut(&ident) {
                ci.min_occurs *= ctx.occurs.min;
                ci.max_occurs *= ctx.occurs.max;
            }
        }

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
            self = self.flatten_complex_type(ident).unwrap();
        }

        self
    }

    fn flatten_complex_type_impl(
        &self,
        ident: &Ident,
        min: MinOccurs,
        max: MaxOccurs,
        ctx: &mut Context,
    ) {
        let Some(type_) = self.types.items.get(ident) else {
            return;
        };

        let si = match &type_.variant {
            MetaTypeVariant::Choice(si) => {
                ctx.set_mode(Mode::Choice);

                si
            }
            MetaTypeVariant::All(si) | MetaTypeVariant::Sequence(si) => {
                if max > MaxOccurs::Bounded(1) {
                    ctx.set_mode(Mode::Choice);
                } else {
                    ctx.set_mode(Mode::Sequence);
                }

                si
            }
            MetaTypeVariant::Reference(ti) if ti.is_single() => {
                self.flatten_complex_type_impl(
                    &ti.type_,
                    min * ti.min_occurs,
                    max * ti.max_occurs,
                    ctx,
                );

                return;
            }
            x => crate::unreachable!("{x:#?}"),
        };

        ctx.count += 1;

        for x in &*si.elements {
            match x.element_mode {
                ElementMode::Element => {
                    let mut element = x.clone();

                    element.min_occurs *= min;
                    element.max_occurs *= max;

                    ctx.add_element(element);
                }
                ElementMode::Group => {
                    if let ElementMetaVariant::Type(type_) = &x.variant {
                        self.flatten_complex_type_impl(
                            type_,
                            min * x.min_occurs,
                            max * x.max_occurs,
                            ctx,
                        );
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct Context {
    meta: GroupMeta,
    mode: Mode,
    count: usize,
    occurs: ContextOccurs,
}

#[derive(Debug)]
struct ContextOccurs {
    min: usize,
    max: MaxOccurs,
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
enum Mode {
    #[default]
    Unknown,
    Mixed,
    Choice,
    Sequence,
}

impl Context {
    fn set_mode(&mut self, mode: Mode) {
        let new_mode = match (self.mode, mode) {
            (Mode::Unknown, mode) => mode,
            (Mode::Choice, Mode::Choice) => Mode::Choice,
            (Mode::Sequence, Mode::Sequence) => Mode::Sequence,
            (_, _) => Mode::Mixed,
        };

        self.mode = match (self.mode, new_mode) {
            (Mode::Unknown, mode) => mode,
            (_, Mode::Mixed) => {
                self.occurs.min = 0;
                self.occurs.max = MaxOccurs::Bounded(0);

                for element in &mut *self.meta.elements {
                    self.occurs.update(element);
                }

                Mode::Mixed
            }
            (_, mode) => mode,
        };
    }

    fn add_element(&mut self, mut element: ElementMeta) {
        let existing = self
            .meta
            .elements
            .iter_mut()
            .find(|x| x.ident == element.ident);

        if let Some(existing) = existing {
            if self.mode == Mode::Mixed {
                self.occurs.update(&mut element);
            } else {
                existing.min_occurs += element.min_occurs;
                existing.max_occurs += element.max_occurs;
            }

            self.set_mode(Mode::Choice);
        } else {
            if self.mode == Mode::Mixed {
                self.occurs.update(&mut element);
            }

            self.meta.elements.push(element);
        }
    }
}

impl ContextOccurs {
    fn update(&mut self, element: &mut ElementMeta) {
        self.min += element.min_occurs;
        self.max += element.max_occurs;

        element.min_occurs = 1;
        element.max_occurs = MaxOccurs::Bounded(1);
    }
}

impl Default for ContextOccurs {
    fn default() -> Self {
        Self {
            min: 1,
            max: MaxOccurs::Bounded(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        meta::{ElementMeta, ElementMode, ElementsMeta, MetaType, MetaTypeVariant, MetaTypes},
        schema::MaxOccurs,
        Ident,
    };
    use crate::Optimizer;

    macro_rules! make_type {
        ($types:expr, $name:literal, $type:ident $( $rest:tt )*) => {{
            let mut variant = MetaTypeVariant::$type(Default::default());
            let MetaTypeVariant::$type(info) = &mut variant else { unreachable!(); };
            make_type!(__init info $type $( $rest )*);
            let ty = MetaType::new(variant);
            $types.items.insert(Ident::type_($name), ty);
        }};
        (__init $info:ident ComplexType($name:literal) $(,)? ) => {
            $info.content = Some(Ident::type_($name));
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
            let mut element = ElementMeta::new(Ident::name($element_name), type_ident, ElementMode::$type);
            $(
                element_info!(__set_field element.$field $value);
            )*
            element
        }};
        (__type_ident Element $name:literal) => {
            Ident::element($name)
        };
        (__type_ident Group $name:literal) => {
            Ident::type_($name)
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
            let main = $types.get_variant(&Ident::type_($main_ident)).unwrap();
            let content = $types.get_variant(&Ident::type_($content_ident)).unwrap();

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
    fn sequence_with_choice_expect_choice() {
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

        assert_eq!(main.min_occurs, 4);
        assert_eq!(main.max_occurs, MaxOccurs::Bounded(5));

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
    fn nested_sequences_expect_choice() {
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

        assert_eq!(main.min_occurs, 1);
        assert_eq!(main.max_occurs, MaxOccurs::Bounded(1));

        let mut it = content.elements.iter();

        assert_element!(it, "a", 0, 1);
        assert_element!(it, "b", 1, 1);
        assert_element!(it, "c", 1, Unbounded);

        assert_element!(it, "d", 0, 1);
        assert_element!(it, "e", 0, 1);
        assert_element!(it, "f", 0, Unbounded);

        assert_element!(it, "g", 0, Unbounded);
        assert_element!(it, "h", 1, Unbounded);
        assert_element!(it, "i", 1, Unbounded);

        assert!(it.next().is_none());
    }
}
