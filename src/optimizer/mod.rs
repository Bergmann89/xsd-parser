//! The `optimizer` module contains the type information [`Optimizer`] and all related types.

use std::collections::{btree_map::Entry, HashMap, HashSet};

use crate::{
    schema::{MaxOccurs, MinOccurs},
    types::{
        Base, ComplexInfo, ElementInfo, ElementMode, EnumerationInfo, GroupInfo, Ident, Name,
        ReferenceInfo, Type, TypeEq, Types, UnionInfo, UnionTypeInfo, VariantInfo, VecHelper,
    },
};

/// The [`Optimizer`] is a structure that can be used to reduce the size and the
/// complexity of a [`Types`] instance.
///
/// The optimizer contains different optimizations that could be applied to a
/// [`Types`] instance. Optimizations are usually used to reduce the size or the
/// complexity of the different types.
#[must_use]
#[derive(Debug)]
pub struct Optimizer {
    types: Types,
    bases: Option<BaseMap>,
    typedefs: Option<TypedefMap>,
}

#[derive(Debug)]
struct BaseMap(HashMap<Ident, Base>);

#[derive(Debug)]
struct TypedefMap(HashMap<Ident, Ident>);

struct FlattenComplexInfo {
    info: GroupInfo,
    count: usize,
    is_choice: bool,
}

struct FlattenUnionInfo {
    count: usize,
    info: UnionInfo,
}

macro_rules! get_bases {
    ($this:expr) => {{
        if $this.bases.is_none() {
            $this.bases = Some(BaseMap::new(&$this.types));
        }

        $this.bases.as_ref().unwrap()
    }};
}

macro_rules! get_typedefs {
    ($this:expr) => {{
        if $this.typedefs.is_none() {
            $this.typedefs = Some(TypedefMap::new(&$this.types));
        }

        $this.typedefs.as_ref().unwrap()
    }};
}

impl Optimizer {
    /// Create a new [`Optimizer`] instance from the passed `types`.
    pub fn new(types: Types) -> Self {
        Self {
            types,
            bases: None,
            typedefs: None,
        }
    }

    /// Finish the optimization and return the resulting [`Types`].
    #[must_use]
    pub fn finish(self) -> Types {
        self.types
    }

    /// This will remove any enum variant that has an empty string as name.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/enum_empty_variant.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/remove_empty_enum_variants.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/remove_empty_enum_variants.rs")]
    /// ```
    pub fn remove_empty_enum_variants(mut self) -> Self {
        tracing::trace!("remove_empty_enum_variants");

        for type_ in self.types.types.values_mut() {
            if let Type::Enumeration(x) = type_ {
                x.variants
                    .retain(|x| !matches!(&x.ident.name, Name::Named(x) if x.is_empty()));
            }
        }

        self
    }

    /// This will remove replace the enum with a type reference to the enums base
    /// type if the enum does not have any variant.
    ///
    /// This optimization is usually used in combination with
    /// [`remove_empty_enum_variants`](Self::remove_empty_enum_variants).
    ///
    /// # Examples
    ///
    /// Consider the following XML schema:
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/enum_empty.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/remove_empty_enums.rs")]
    /// ```
    ///
    /// With this optimization (and the [`remove_empty_enum_variants`](Self::remove_empty_enum_variants))
    /// the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/remove_empty_enums.rs")]
    /// ```
    pub fn remove_empty_enums(mut self) -> Self {
        tracing::trace!("remove_empty_enums");

        for type_ in self.types.types.values_mut() {
            if let Type::Enumeration(x) = type_ {
                if x.variants.is_empty() {
                    if let Some(base) = x.base.as_ident() {
                        self.typedefs = None;
                        *type_ = Type::Reference(ReferenceInfo::new(base.clone()));
                    }
                }
            }
        }

        self
    }

    /// This will remove variants of an union, if the type of the variant resolves
    /// to the same type as an other variant. In other words, the variant will be
    /// removed if the types are identical.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema. `xs:language` and `xs:string` are both
    /// type definitions for [`String`].
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/union_duplicate.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/remove_duplicate_union_variants.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/remove_duplicate_union_variants.rs")]
    /// ```
    pub fn remove_duplicate_union_variants(mut self) -> Self {
        tracing::trace!("remove_duplicate_union_variants");

        let typedefs = get_typedefs!(self);

        for type_ in self.types.types.values_mut() {
            if let Type::Union(x) = type_ {
                let mut i = 0;
                let mut types_ = HashSet::new();

                while i < x.types.len() {
                    let type_ = typedefs.resolve(&x.types[i].type_).clone();
                    if types_.insert(type_) {
                        i += 1;
                    } else {
                        x.types.remove(i);
                    }
                }
            }
        }

        self
    }

    /// This will replace an union with a simple type definition, if the union
    /// has only one variant.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/union_empty.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/remove_empty_unions.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/remove_empty_unions.rs")]
    /// ```
    pub fn remove_empty_unions(mut self) -> Self {
        tracing::trace!("remove_empty_unions");

        for type_ in self.types.types.values_mut() {
            if let Type::Union(x) = type_ {
                if x.types.len() <= 1 {
                    let base = x.types.first().map(|x| &x.type_).or(x.base.as_ident());
                    if let Some(base) = base {
                        self.typedefs = None;
                        *type_ = Type::Reference(ReferenceInfo::new(base.clone()));
                    }
                }
            }
        }

        self
    }

    /// This will use the unrestricted base type instead more the restricted
    /// version when ever possible.
    ///
    /// This is useful if you want to reduce the amount of different type, then
    /// the base type can store the same data than the restricted one.
    /// However this is only useful if you want to deserialize the type only. Using
    /// this feature for serializing types will cause problems because the type
    /// information is dropped during deserialization.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/complex_restricted.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/use_unrestricted_base_type.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/use_unrestricted_base_type.rs")]
    /// ```
    pub fn use_unrestricted_base_type(mut self) -> Self {
        tracing::trace!("use_unrestricted_base_type");

        let bases = get_bases!(self);

        for (ident, type_) in &mut *self.types {
            match type_ {
                Type::ComplexType(_) | Type::Enumeration(_) | Type::Union(_) => {
                    let base = bases.get_unrestricted(ident).clone();
                    if *ident != base {
                        self.typedefs = None;
                        *type_ = Type::Reference(ReferenceInfo::new(base));
                    }
                }
                _ => (),
            }
        }

        self
    }

    /// This will use a enum that contains all known variants of the dynamic
    /// type instead of a dynamic box.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/abstract.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/convert_dynamic_to_choice.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/convert_dynamic_to_choice.rs")]
    /// ```
    pub fn convert_dynamic_to_choice(mut self) -> Self {
        tracing::trace!("convert_dynamic_to_choice");

        let idents = self
            .types
            .iter()
            .filter_map(|(ident, ty)| {
                if matches!(ty, Type::Dynamic(_)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            let content_ident = Ident::new(self.types.make_unnamed()).with_ns(ident.ns);

            let type_ = self.types.get_mut(&ident).unwrap();
            let Type::Dynamic(x) = type_ else {
                crate::unreachable!();
            };

            let mut si = GroupInfo::default();
            for derived in &x.derived_types {
                si.elements.find_or_insert(derived.clone(), |ident| {
                    ElementInfo::new(ident, derived.clone(), ElementMode::Element)
                });
            }

            *type_ = Type::ComplexType(ComplexInfo {
                content: Some(content_ident.clone()),
                is_dynamic: true,
                ..Default::default()
            });

            match self.types.entry(content_ident) {
                Entry::Vacant(e) => {
                    e.insert(Type::Choice(si));
                }
                Entry::Occupied(_) => crate::unreachable!(),
            }
        }

        self
    }

    /// This will flatten the nested groups (`xs::all`, `xs::choice` or `xs::sequence`)
    /// to one type instead of rendering nested structures.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/complex_flatten.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/flatten_element_content.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/flatten_element_content.rs")]
    /// ```
    pub fn flatten_element_content(mut self) -> Self {
        tracing::trace!("flatten_element_content");

        let idents = self
            .types
            .iter()
            .filter_map(|(ident, type_)| {
                if matches!(type_, Type::ComplexType(ci) if ci.has_complex_content(&self.types)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            let type_ = self.types.get(&ident).unwrap();
            let Type::ComplexType(ci) = type_ else {
                crate::unreachable!();
            };
            let Some(content_ident) = ci.content.clone() else {
                continue;
            };

            let mut info = FlattenComplexInfo {
                info: GroupInfo::default(),
                count: 0,
                is_choice: false,
            };

            self.flatten_complex(&content_ident, ci.min_occurs, ci.max_occurs, &mut info);

            if info.count > 1 {
                let type_ = if info.is_choice {
                    Type::Choice(info.info)
                } else {
                    Type::Sequence(info.info)
                };

                self.types.insert(content_ident, type_);
            }
        }

        self
    }

    /// This will flatten nested union types to one single union.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/union_flatten.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/flatten_unions.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/flatten_unions.rs")]
    /// ```
    pub fn flatten_unions(mut self) -> Self {
        tracing::trace!("flatten_unions");

        let idents = self
            .types
            .iter()
            .filter_map(|(ident, type_)| {
                if matches!(type_, Type::Union(_)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            let Some(Type::Union(x)) = self.types.get(&ident) else {
                continue;
            };
            let mut info = FlattenUnionInfo {
                count: 0,
                info: UnionInfo::default(),
            };
            self.flatten_union(&ident, &mut info);
            if info.count > 1 {
                info.info.base = x.base.clone();

                self.types.insert(ident, Type::Union(info.info));
            }
        }

        self
    }

    /// This will flatten nested union and enum types to one single enum type.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/union_flatten.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/merge_enum_unions.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/merge_enum_unions.rs")]
    /// ```
    pub fn merge_enum_unions(mut self) -> Self {
        tracing::trace!("merge_enum_unions");

        let idents = self
            .types
            .iter()
            .filter_map(|(ident, type_)| {
                if matches!(type_, Type::Union(_)) {
                    Some(ident)
                } else {
                    None
                }
            })
            .cloned()
            .collect::<Vec<_>>();

        for ident in idents {
            let Some(Type::Union(_)) = self.types.get(&ident) else {
                continue;
            };
            let mut next = None;
            self.flatten_enum_union(&ident, &mut next);
            if let Some(next) = next {
                self.types.insert(ident, next);
            }
        }

        self
    }

    /// This will resolve all simple type definitions and use the target type
    /// directly.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/complex_flatten.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/resolve_typedefs.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/resolve_typedefs.rs")]
    /// ```
    pub fn resolve_typedefs(mut self) -> Self {
        tracing::trace!("resolve_typedefs");

        let typedefs = get_typedefs!(self);

        macro_rules! resolve_base {
            ($base:expr) => {
                match &mut $base {
                    Base::None => (),
                    Base::Extension(x) => *x = typedefs.resolve(x).clone(),
                    Base::Restriction(x) => *x = typedefs.resolve(x).clone(),
                }
            };
        }

        let mut replaced_references = HashMap::new();

        for type_ in self.types.values_mut() {
            match type_ {
                Type::Reference(x) if x.is_single() => {
                    let new_type = typedefs.resolve(&x.type_).clone();
                    replaced_references
                        .entry(x.type_.clone())
                        .or_insert_with(|| new_type.clone());
                    x.type_ = new_type;
                }
                Type::Union(x) => {
                    resolve_base!(&mut x.base);

                    for x in &mut *x.types {
                        x.type_ = typedefs.resolve(&x.type_).clone();
                    }
                }
                Type::Dynamic(x) => {
                    x.type_ = x.type_.as_ref().map(|x| typedefs.resolve(x)).cloned();

                    for x in &mut x.derived_types {
                        *x = typedefs.resolve(x).clone();
                    }
                }
                Type::Enumeration(x) => {
                    resolve_base!(&mut x.base);

                    for x in &mut *x.variants {
                        if let Some(x) = &mut x.type_ {
                            *x = typedefs.resolve(x).clone();
                        }
                    }
                }
                Type::ComplexType(x) => {
                    resolve_base!(&mut x.base);

                    if let Some(ident) = &mut x.content {
                        *ident = typedefs.resolve(ident).clone();
                    }

                    for attrib in &mut *x.attributes {
                        attrib.type_ = typedefs.resolve(&attrib.type_).clone();
                    }
                }
                Type::All(x) | Type::Choice(x) | Type::Sequence(x) => {
                    for element in &mut *x.elements {
                        element.type_ = typedefs.resolve(&element.type_).clone();
                    }
                }
                _ => (),
            }
        }

        for type_ in self.types.values_mut() {
            let Type::Dynamic(ai) = type_ else {
                continue;
            };

            for derived in &mut ai.derived_types {
                if let Some(new_type) = replaced_references.get(derived) {
                    *derived = new_type.clone();
                }
            }
        }

        self
    }

    /// If two types are completely equal this optimization will generate the
    /// first type complete and just a type definition for the second one.
    ///
    /// # Examples
    ///
    /// Consider the following XML schema.
    /// ```xml
    #[doc = include_str!("../../tests/optimizer/duplicate.xsd")]
    /// ```
    ///
    /// Without this optimization this will result in the following code:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected0/remove_duplicates.rs")]
    /// ```
    ///
    /// With this optimization the following code is generated:
    /// ```rust
    #[doc = include_str!("../../tests/optimizer/expected1/remove_duplicates.rs")]
    /// ```
    pub fn remove_duplicates(mut self) -> Self {
        let idents = self.types.keys().cloned().collect::<Vec<_>>();
        let mut changed = true;

        while changed {
            changed = false;

            for a in &idents {
                for b in &idents {
                    if a == b {
                        continue;
                    }

                    let Some(type_a) = self.types.get(a) else {
                        continue;
                    };
                    let Some(type_b) = self.types.get(b) else {
                        continue;
                    };

                    if matches!(type_a, Type::Reference(_)) || matches!(type_b, Type::Reference(_))
                    {
                        continue;
                    }

                    if type_a.type_eq(type_b, &self.types) {
                        self.typedefs = None;

                        changed = true;

                        self.types
                            .insert(b.clone(), Type::Reference(ReferenceInfo::new(a.clone())));
                    }
                }
            }
        }

        self
    }

    fn flatten_complex(
        &self,
        ident: &Ident,
        min: MinOccurs,
        max: MaxOccurs,
        next: &mut FlattenComplexInfo,
    ) {
        let Some(type_) = self.types.get(ident) else {
            return;
        };

        let mut is_choice = false;
        let si = match type_ {
            Type::Choice(si) => {
                is_choice = true;
                next.is_choice = true;

                si
            }
            Type::All(si) | Type::Sequence(si) => si,
            Type::Reference(ti) if ti.is_single() => {
                self.flatten_complex(
                    &ti.type_,
                    min.min(ti.min_occurs),
                    max.max(ti.max_occurs),
                    next,
                );

                return;
            }
            x => crate::unreachable!("{x:#?}"),
        };

        next.count += 1;

        for x in &*si.elements {
            match x.element_mode {
                ElementMode::Group => {
                    let (min, max) = if is_choice {
                        (min.min(x.min_occurs), max.max(x.max_occurs))
                    } else {
                        (min + x.min_occurs, max + x.max_occurs)
                    };

                    self.flatten_complex(&x.type_, min, max, next);
                }
                ElementMode::Element => {
                    let element = next
                        .info
                        .elements
                        .find_or_insert(x.ident.clone(), |_| x.clone());
                    element.min_occurs = min.min(x.min_occurs);
                    element.max_occurs = max.max(x.max_occurs);
                }
            }
        }

        if let Some(any) = &si.any {
            next.info.any = Some(any.clone());
        }
    }

    fn flatten_union(&self, ident: &Ident, next: &mut FlattenUnionInfo) {
        let Some(type_) = self.types.get(ident) else {
            return;
        };

        match type_ {
            Type::Union(x) => {
                next.count += 1;
                for variant in &*x.types {
                    self.flatten_union(&variant.type_, next);
                }
            }
            Type::Reference(x) if x.is_single() => {
                self.flatten_union(&x.type_, next);
            }
            _ => {
                next.info.types.push(UnionTypeInfo::new(ident.clone()));
            }
        }
    }

    fn flatten_enum_union(&self, ident: &Ident, next: &mut Option<Type>) {
        let Some(type_) = self.types.get(ident) else {
            return;
        };

        match type_ {
            Type::Union(x) => {
                for t in &*x.types {
                    self.flatten_enum_union(&t.type_, next);
                }
            }
            Type::Enumeration(x) => {
                *next = match next.take() {
                    None => Some(Type::Enumeration(EnumerationInfo::default())),
                    Some(Type::Enumeration(ei)) => Some(Type::Enumeration(ei)),
                    Some(Type::Union(ui)) => {
                        let mut ei = EnumerationInfo::default();

                        for t in &*ui.types {
                            ei.variants.find_or_insert(t.type_.clone(), |ident| {
                                VariantInfo::new(ident).with_type(Some(t.type_.clone()))
                            });
                        }

                        Some(Type::Enumeration(ei))
                    }
                    _ => crate::unreachable!(),
                };

                let Some(Type::Enumeration(ei)) = next else {
                    crate::unreachable!();
                };

                for var in &*x.variants {
                    ei.variants.find_or_insert(var.ident.clone(), |ident| {
                        VariantInfo::new(ident).with_type(var.type_.clone())
                    });
                }
            }
            Type::Reference(x) if x.is_single() => {
                self.flatten_enum_union(&x.type_, next);
            }
            _ => {
                if next.is_none() {
                    *next = Some(Type::Union(UnionInfo::default()));
                }

                match next {
                    Some(Type::Union(ui)) => {
                        ui.types.push(UnionTypeInfo::new(ident.clone()));
                    }
                    Some(Type::Enumeration(ei)) => {
                        ei.variants.find_or_insert(ident.clone(), |x| {
                            VariantInfo::new(x).with_type(Some(ident.clone()))
                        });
                    }
                    _ => crate::unreachable!(),
                }
            }
        }
    }
}

impl BaseMap {
    fn new(types: &Types) -> Self {
        let mut ret = HashMap::new();

        for (ident, type_) in &types.types {
            match type_ {
                Type::Enumeration(ei) => {
                    if matches!(
                        ei.base.as_ident().and_then(|base| types.get(base)),
                        Some(Type::Enumeration(_))
                    ) {
                        ret.insert(ident.clone(), ei.base.clone());
                    }
                }
                Type::Union(ei) => {
                    if matches!(
                        ei.base.as_ident().and_then(|base| types.get(base)),
                        Some(Type::Union(_))
                    ) {
                        ret.insert(ident.clone(), ei.base.clone());
                    }
                }
                Type::ComplexType(ci) => {
                    if matches!(
                        ci.base.as_ident().and_then(|base| types.get(base)),
                        Some(Type::ComplexType(_))
                    ) {
                        ret.insert(ident.clone(), ci.base.clone());
                    }
                }
                _ => (),
            }
        }

        Self(ret)
    }

    fn get_unrestricted<'a>(&'a self, ident: &'a Ident) -> &'a Ident {
        match self.0.get(ident) {
            Some(Base::Restriction(base)) => self.get_unrestricted(base),
            _ => ident,
        }
    }
}

impl TypedefMap {
    fn new(types: &Types) -> Self {
        let mut ret = HashMap::new();

        for (ident, type_) in &types.types {
            if let Type::Reference(x) = type_ {
                if x.is_single() {
                    ret.insert(ident.clone(), x.type_.clone());
                }
            }
        }

        Self(ret)
    }

    fn resolve<'a>(&'a self, ident: &'a Ident) -> &'a Ident {
        let x = self.0.get(ident).map_or(ident, |x| self.resolve(x));

        x
    }
}
