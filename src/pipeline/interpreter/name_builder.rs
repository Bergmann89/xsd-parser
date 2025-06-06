use crate::models::types::NameBuilder;

use super::state::State;

impl NameBuilder {
    pub(super) fn auto_extend(
        self,
        stop_at_group: bool,
        replace: bool,
        state: &mut State<'_>,
    ) -> Self {
        self.extend(replace, state.last_named_type(stop_at_group))
    }
}
