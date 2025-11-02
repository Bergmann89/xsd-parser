/// Trait that adds namespace information to a type.
pub trait WithNamespace {
    /// The default namespace prefix for this type.
    fn prefix() -> Option<&'static str>;

    /// The namespace for this type.
    fn namespace() -> Option<&'static str>;
}
