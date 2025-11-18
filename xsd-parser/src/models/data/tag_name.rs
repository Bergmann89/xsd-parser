use xsd_parser_types::misc::Namespace;

use crate::models::{
    meta::{MetaTypes, ModuleMeta},
    schema::xs::FormChoiceType,
    Ident,
};

/// Represents the name of a XML tag including an optional namespace prefix.
#[derive(Default, Debug)]
pub struct TagName<'types> {
    /// Name of the XML tag.
    pub name: String,

    /// Module the tag belongs to.
    pub module: Option<&'types ModuleMeta>,
}

impl<'types> TagName<'types> {
    /// Creates a new [`TagName`] instance.
    ///
    /// # Arguments:
    /// - `types` is used to resolve the module this tag belongs to
    /// - `ident` contains the name and the namespace of the tag
    /// - `form` is used to decide if a prefix needs to be added to the tag in general.
    #[must_use]
    pub fn new(types: &'types MetaTypes, ident: &Ident, form: FormChoiceType) -> Self {
        let name = ident.name.to_string();
        let module = form
            .eq(&FormChoiceType::Qualified)
            .then_some(())
            .and(ident.ns.as_ref())
            .and_then(|ns| types.modules.get(ns));

        Self { name, module }
    }

    /// Get the resolved name of the the XML tag
    ///
    /// The `with_prefix` parameter tells whether the namespace prefix should be added to tag or not.
    #[must_use]
    pub fn get(&self, with_prefix: bool) -> String {
        let Self { name, module } = self;
        let prefix = with_prefix
            .then_some(*module)
            .flatten()
            .and_then(|module| module.prefix.as_ref());

        if let Some(prefix) = prefix {
            format!("{prefix}:{name}")
        } else {
            name.clone()
        }
    }

    /// Get the resolved name of the the XML tag for the specified default namespace.
    #[must_use]
    pub fn get_for_default_namespace(&self, ns: &Option<Namespace>) -> String {
        let module_ns = self
            .module
            .as_ref()
            .and_then(|module| module.namespace.as_ref());
        let with_prefix =
            !matches!((module_ns, ns), (Some(module_ns), Some(ns)) if module_ns == ns);

        self.get(with_prefix)
    }
}
