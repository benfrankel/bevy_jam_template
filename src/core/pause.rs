use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Pause>();
}

#[derive(State, Reflect, Copy, Clone, Default, Eq, PartialEq, Debug)]
#[state(log_flush)]
#[reflect(Resource)]
pub struct Pause;

impl Configure for Pause {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
    }
}
