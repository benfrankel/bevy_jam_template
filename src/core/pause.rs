use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Pause, PausableSystems)>();
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

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

impl Configure for PausableSystems {
    fn configure(app: &mut App) {
        app.configure_sets(Update, PausableSystems.run_if(Pause::is_disabled));
    }
}
