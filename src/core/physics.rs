use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..RapierConfiguration::new(1.0)
    });
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PIXELS_PER_METER,
    ));
}

const PIXELS_PER_METER: f32 = 16.0;
