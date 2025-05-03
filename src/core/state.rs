use pyri_state::prelude::*;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(StatePlugin);
}
