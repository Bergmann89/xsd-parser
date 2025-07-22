use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::models::{
    meta::{
        AttributeMetaVariant, ElementMetaVariant, ElementMode, MetaType, MetaTypeVariant, MetaTypes,
    },
    Ident,
};

/// Pretty-printer for [`MetaTypes`] content.
///
/// The [`MetaTypesPrinter`] recursively formats all known [`MetaType`] entries
/// in a structured and indented way. It handles all variant types
/// (`BuildIn`, `Custom`, `Reference`, `ComplexType`, etc.) and also detects
/// and prevents cycles by tracking visited type identifiers.
///
/// This type is typically used via its [`Display`] implementation.
#[derive(Debug)]
pub struct MetaTypesPrinter<'a> {
    types: &'a MetaTypes,
}

#[derive(Default)]
struct State {
    level: usize,
    visit: HashSet<Ident>,
}

impl<'a> MetaTypesPrinter<'a> {
    /// Create a new [`MetaTypesPrinter`] from the passed `types`.
    #[must_use]
    pub fn new(types: &'a MetaTypes) -> Self {
        Self { types }
    }

    /// Print the information of all meta types to the passed formatter `f`.
    ///
    /// # Errors
    ///
    /// Forwards the error raised by the formatter.
    pub fn print_all(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut s = State::default();

        for (ident, ty) in &self.types.items {
            self.print_type_impl(f, &mut s, ident, ty)?;
        }

        Ok(())
    }

    /// Print the information of a single meta type identified by `ident` to
    /// the passed formatter `f`.
    ///
    /// # Errors
    ///
    /// Forwards the error raised by the formatter.
    pub fn print_type(&self, ident: &Ident, f: &mut Formatter<'_>) -> FmtResult {
        let mut s = State::default();

        if let Some(ty) = self.types.items.get(ident) {
            self.print_type_impl(f, &mut s, ident, ty)?;
        }

        Ok(())
    }

    fn resolve_complex_type(
        &self,
        f: &mut Formatter<'_>,
        s: &mut State,
        ident: &Ident,
    ) -> FmtResult {
        if let Some(x) = self.types.items.get(ident) {
            self.print_type_impl(f, s, ident, x)
        } else {
            writeln!(f, "NOT FOUND")?;

            Ok(())
        }
    }

    #[allow(clippy::too_many_lines)]
    fn print_type_impl(
        &self,
        f: &mut Formatter<'_>,
        s: &mut State,
        ident: &Ident,
        ty: &MetaType,
    ) -> FmtResult {
        macro_rules! indent {
            ($( $tt:tt )*) => {{
                write!(f, "{0:1$}", "", 4 * s.level)?;
                write!(f, $( $tt )*)?;
            }};
        }
        macro_rules! indentln {
            ($( $tt:tt )*) => {{
                write!(f, "{0:1$}", "", 4 * s.level)?;
                writeln!(f, $( $tt )*)?;
            }};
        }

        if !s.visit.insert(ident.clone()) {
            writeln!(f, "LOOP DETECTED ({})", ident.name)?;

            return Ok(());
        }

        match &ty.variant {
            MetaTypeVariant::BuildIn(x) => {
                writeln!(f, "{}: BuildIn", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("type={x:?}");

                s.level -= 1;
            }
            MetaTypeVariant::Custom(x) => {
                writeln!(f, "{}: Custom", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("type={x:?}");

                s.level -= 1;
            }
            MetaTypeVariant::Union(x) => {
                writeln!(f, "{}: Union", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("base={}", x.base);
                indentln!("types:");

                s.level += 1;

                for ty in &*x.types {
                    indentln!("{}", &ty.type_);
                }

                s.level -= 2;
            }
            MetaTypeVariant::Reference(x) => {
                writeln!(f, "{}: Reference", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("min={}", x.min_occurs);
                indentln!("max={:?}", x.max_occurs);
                indentln!("type={}", x.type_);

                s.level -= 1;
            }
            MetaTypeVariant::Dynamic(x) => {
                writeln!(f, "{}: Dynamic", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("types:");

                s.level += 1;

                for ty in &*x.derived_types {
                    indentln!("{}", ty);
                }

                s.level -= 2;
            }
            MetaTypeVariant::Enumeration(x) => {
                writeln!(f, "{}: Enumeration", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("base={}", x.base);
                indentln!("variants:");

                s.level += 1;

                for var in &*x.variants {
                    indentln!("{}={:?}", var.ident.name, var.use_);
                }

                s.level -= 2;
            }
            MetaTypeVariant::All(x) | MetaTypeVariant::Choice(x) | MetaTypeVariant::Sequence(x) => {
                match &ty.variant {
                    MetaTypeVariant::All(_) => writeln!(f, "{}: All", ident)?,
                    MetaTypeVariant::Choice(_) => writeln!(f, "{}: Choice", ident)?,
                    MetaTypeVariant::Sequence(_) => writeln!(f, "{}: Sequence", ident)?,
                    _ => (),
                }

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("is_mixed={:?}", x.is_mixed);

                for x in &*x.elements {
                    indentln!("element:");

                    s.level += 1;

                    indentln!("name={}", x.ident.name);
                    indentln!("form={:?}", x.form);
                    indentln!("min_occurs={}", x.min_occurs);
                    indentln!("max_occurs={:?}", x.max_occurs);

                    match &x.variant {
                        ElementMetaVariant::Text => {
                            indentln!("variant=Text");
                        }
                        ElementMetaVariant::Type {
                            type_,
                            mode: ElementMode::Element,
                        } => {
                            indentln!("variant=Type({})", type_);
                        }
                        ElementMetaVariant::Type {
                            type_,
                            mode: ElementMode::Group,
                        } => {
                            indent!("variant=");
                            self.resolve_complex_type(f, s, type_)?;
                        }
                        ElementMetaVariant::Any { meta: x } => {
                            indentln!("variant=Any");

                            s.level += 1;

                            if let Some(x) = &x.id {
                                indentln!("id={x:?}");
                            }
                            if let Some(x) = &x.namespace {
                                indentln!("namespace={x:?}");
                            }
                            if let Some(x) = &x.not_q_name {
                                indentln!("not_q_name={x:?}");
                            }
                            if let Some(x) = &x.not_namespace {
                                indentln!("not_namespace={x:?}");
                            }

                            indentln!("process_contents={:?}", x.process_contents);

                            s.level -= 1;
                        }
                    }

                    s.level -= 1;
                }

                s.level -= 1;
            }
            MetaTypeVariant::ComplexType(x) => {
                writeln!(f, "{}: ComplexType", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("base={}", x.base);
                indentln!("min_occurs={}", x.min_occurs);
                indentln!("max_occurs={:?}", x.max_occurs);
                indentln!("is_dynamic={}", x.is_dynamic);
                indentln!("is_mixed={:?}", x.is_mixed);

                for x in &*x.attributes {
                    indentln!("attribute:");

                    s.level += 1;

                    indentln!("name={}", x.ident.name);
                    indentln!("use={:?}", x.use_);
                    indentln!("form={:?}", x.form);
                    indentln!("default={:?}", x.default);

                    match &x.variant {
                        AttributeMetaVariant::Type(type_) => indentln!("type=Type({})", type_),
                        AttributeMetaVariant::Any(x) => {
                            indentln!("type=Any");

                            s.level += 1;

                            if let Some(x) = &x.id {
                                indentln!("id={x:?}");
                            }
                            if let Some(x) = &x.namespace {
                                indentln!("namespace={x:?}");
                            }
                            if let Some(x) = &x.not_q_name {
                                indentln!("not_q_name={x:?}");
                            }
                            if let Some(x) = &x.not_namespace {
                                indentln!("not_namespace={x:?}");
                            }

                            indentln!("process_contents={:?}", x.process_contents);

                            s.level -= 1;
                        }
                    }

                    s.level -= 1;
                }

                if let Some(content) = &x.content {
                    indent!("content=");
                    self.resolve_complex_type(f, s, content)?;
                }

                s.level -= 1;
            }
        }

        s.visit.remove(ident);

        Ok(())
    }
}

impl Display for MetaTypesPrinter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.print_all(f)
    }
}
