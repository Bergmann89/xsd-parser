use crate::traits::{NameBuilder, NameBuilderExt as _};

use super::StackEntry;

pub(super) trait NameBuilderExt {
    fn auto_extend(self, stack: &[StackEntry<'_, '_>]) -> Self;
}

impl<X> NameBuilderExt for X
where
    X: NameBuilder,
{
    fn auto_extend(self, stack: &[StackEntry<'_, '_>]) -> Self {
        for x in stack.iter().rev() {
            if let StackEntry::Type { ident, .. } = x {
                return self.extend(false, Some(ident.name.as_str()));
            }
        }

        self
    }
}
