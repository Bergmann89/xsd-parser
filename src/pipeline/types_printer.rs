use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::models::{
    types::{AttributeType, ElementMode, ElementType, Type, TypeVariant, Types},
    Ident,
};

pub(crate) struct TypesPrinter<'a> {
    types: &'a Types,
}

#[derive(Default)]
struct State {
    level: usize,
    visit: HashSet<Ident>,
}

impl<'a> TypesPrinter<'a> {
    pub(crate) fn new(types: &'a Types) -> Self {
        Self { types }
    }

    fn print_all(&self, f: &mut Formatter<'_>, s: &mut State) -> FmtResult {
        for (ident, ty) in &**self.types {
            self.print_type(f, s, ident, ty)?;
        }

        Ok(())
    }

    fn resolve_complex_type(
        &self,
        f: &mut Formatter<'_>,
        s: &mut State,
        ident: &Ident,
    ) -> FmtResult {
        if let Some(x) = self.types.get(ident) {
            self.print_type(f, s, ident, x)
        } else {
            writeln!(f, "NOT FOUND")?;

            Ok(())
        }
    }

    #[allow(clippy::too_many_lines)]
    fn print_type(
        &self,
        f: &mut Formatter<'_>,
        s: &mut State,
        ident: &Ident,
        ty: &Type,
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
            TypeVariant::BuildIn(x) => {
                writeln!(f, "{}: BuildIn", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("type={x:?}");

                s.level -= 1;
            }
            TypeVariant::Custom(x) => {
                writeln!(f, "{}: Custom", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("type={x:?}");

                s.level -= 1;
            }
            TypeVariant::Union(x) => {
                writeln!(f, "{}: Union", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("base={}", x.base);
                indentln!("types");

                s.level += 1;

                for ty in &*x.types {
                    indentln!("{}", &ty.type_);
                }

                s.level -= 2;
            }
            TypeVariant::Reference(x) => {
                writeln!(f, "{}: Reference", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("min={}", x.min_occurs);
                indentln!("max={:?}", x.max_occurs);
                indentln!("type={}", x.type_);

                s.level -= 1;
            }
            TypeVariant::Dynamic(x) => {
                writeln!(f, "{}: Dynamic", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("types");

                s.level += 1;

                for ty in &*x.derived_types {
                    indentln!("{}", ty);
                }

                s.level -= 2;
            }
            TypeVariant::Enumeration(x) => {
                writeln!(f, "{}: Enumeration", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("base={}", x.base);
                indentln!("variants");

                s.level += 1;

                for var in &*x.variants {
                    indentln!("{}={:?}", var.ident.name, var.use_);
                }

                s.level -= 2;
            }
            TypeVariant::All(x) | TypeVariant::Choice(x) | TypeVariant::Sequence(x) => {
                match &ty.variant {
                    TypeVariant::All(_) => writeln!(f, "{}: All", ident)?,
                    TypeVariant::Choice(_) => writeln!(f, "{}: Choice", ident)?,
                    TypeVariant::Sequence(_) => writeln!(f, "{}: Sequence", ident)?,
                    _ => (),
                }

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);

                for x in &*x.elements {
                    indentln!("element");

                    s.level += 1;

                    indentln!("name={}", x.ident.name);
                    indentln!("min_occurs={}", x.min_occurs);
                    indentln!("max_occurs={:?}", x.max_occurs);
                    indentln!("element_type={:?}", x.element_mode);

                    match (x.element_mode, &x.type_) {
                        (ElementMode::Element, ElementType::Type(type_)) => {
                            indentln!("type=Type({})", type_);
                        }
                        (ElementMode::Element, ElementType::Any(x)) => {
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
                        (ElementMode::Group, ElementType::Type(type_)) => {
                            indent!("type=");
                            self.resolve_complex_type(f, s, type_)?;
                        }
                        (ElementMode::Group, ElementType::Any(_)) => (),
                    }

                    s.level -= 1;
                }

                s.level -= 1;
            }
            TypeVariant::ComplexType(x) => {
                writeln!(f, "{}: ComplexType", ident)?;

                s.level += 1;

                indentln!("display_name={:?}", &ty.display_name);
                indentln!("base={}", x.base);
                indentln!("min_occurs={}", x.min_occurs);
                indentln!("max_occurs={:?}", x.max_occurs);
                indentln!("is_dynamic={}", x.is_dynamic);

                for x in &*x.attributes {
                    indentln!("attribute");

                    s.level += 1;

                    indentln!("name={}", x.ident.name);
                    indentln!("use={:?}", x.use_);
                    indentln!("default={:?}", x.default);

                    match &x.type_ {
                        AttributeType::Type(type_) => indentln!("type=Type({})", type_),
                        AttributeType::Any(x) => {
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
                    indentln!("content");

                    s.level += 1;

                    indent!("type=");
                    self.resolve_complex_type(f, s, content)?;

                    s.level -= 1;
                }

                s.level -= 1;
            }
        }

        s.visit.remove(ident);

        Ok(())
    }
}

impl Display for TypesPrinter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut s = State::default();

        self.print_all(f, &mut s)
    }
}
