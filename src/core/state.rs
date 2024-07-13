use bevy::prelude::*;
use pyri_state::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(StatePlugin);
}
