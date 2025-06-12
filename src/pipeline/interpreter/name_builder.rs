use crate::models::meta::NameBuilder;

use super::state::State;

impl NameBuilder {
    pub(super) fn auto_extend(
        self,
        stop_at_group: bool,
        replace: bool,
        state: &mut State<'_>,
    ) -> Self {
        for x in state.type_stack.iter().rev() {
            match x {
                Some(x) => return self.extend(replace, Some(x.name.as_str())),
                None if stop_at_group => break,
                _ => (),
            }
        }

        self
    }
}
