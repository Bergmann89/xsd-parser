use crate::models::meta::NameBuilder;

use super::state::{StackEntry, State};

impl NameBuilder {
    pub(super) fn auto_extend(
        self,
        stop_at_group_ref: bool,
        replace: bool,
        state: &mut State<'_>,
    ) -> Self {
        for x in state.type_stack.iter().rev() {
            match x {
                StackEntry::Type(x, _) => return self.extend(replace, Some(x.name.as_str())),
                StackEntry::GroupRef(_) | StackEntry::AttributeGroupRef if stop_at_group_ref => {
                    break
                }
                _ => (),
            }
        }

        self
    }
}
