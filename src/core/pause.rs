use pyri_state::prelude::*;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Pause>();
}

#[derive(State, Copy, Clone, Default, Eq, PartialEq, Debug, Reflect)]
#[state(log_flush)]
#[reflect(Resource)]
pub struct Pause;

impl Configure for Pause {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
    }
}
