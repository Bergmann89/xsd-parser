use crate::traits::{NameBuilder, NameBuilderExt as _};

use super::state::{StackEntry, State};

pub(super) trait NameBuilderExt {
    fn auto_extend(self, stop_at_group_ref: bool, replace: bool, state: &mut State<'_>) -> Self;
}

impl<X> NameBuilderExt for X
where
    X: NameBuilder,
{
    fn auto_extend(self, stop_at_group_ref: bool, replace: bool, state: &mut State<'_>) -> Self {
        for x in state.type_stack().iter().rev() {
            match x {
                StackEntry::Type(x, _) => return self.extend(replace, Some(x.name.as_str())),
                StackEntry::GroupRef(_, _) | StackEntry::AttributeGroupRef if stop_at_group_ref => {
                    break
                }
                _ => (),
            }
        }

        self
    }
}
