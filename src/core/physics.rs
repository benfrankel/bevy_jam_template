use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().with_length_unit(PIXELS_PER_METER));
    app.insert_resource(Gravity::ZERO);
}

const PIXELS_PER_METER: f32 = 16.0;
