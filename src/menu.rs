pub mod pause;

use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Menu>();
}

#[derive(State, Copy, Clone, Eq, PartialEq, Debug, Reflect)]
#[state(react, log_flush)]
#[reflect(Resource)]
pub enum Menu {
    Pause,
    Victory,
    Defeat,
}

impl Configure for Menu {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_plugins(pause::plugin);
    }
}
