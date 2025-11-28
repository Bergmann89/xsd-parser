use xsd_parser_types::misc::Namespace;

use crate::models::{
    meta::{MetaTypes, ModuleMeta},
    schema::xs::FormChoiceType,
    Ident,
};

/// Represents the name of a XML tag including an optional namespace prefix.
#[derive(Debug)]
pub struct TagName<'types> {
    /// Name of the XML tag.
    pub name: String,

    /// Indicates if the tag name should be rendered with the namespace prefix or not.
    pub form: FormChoiceType,

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
        let module = ident.ns.as_ref().and_then(|ns| types.modules.get(ns));

        Self { name, form, module }
    }

    /// Get the resolved name of the the XML tag
    ///
    /// The `with_prefix` parameter tells whether the namespace prefix should be added to tag or not.
    #[must_use]
    pub fn get(&self, use_prefix: bool) -> String {
        let Self { name, form, module } = self;
        let prefix = (use_prefix && *form == FormChoiceType::Qualified)
            .then_some(*module)
            .flatten()
            .and_then(|module| module.prefix());

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

impl Default for TagName<'_> {
    fn default() -> Self {
        Self {
            name: String::new(),
            form: FormChoiceType::Unqualified,
            module: None,
        }
    }
}
