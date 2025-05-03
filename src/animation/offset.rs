use bevy::prelude::*;

use crate::animation::PostTransformSystems;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Offset>();
}

#[derive(Component, Reflect, Copy, Clone, Default)]
#[reflect(Component)]
pub struct Offset(pub Vec2);

impl Configure for Offset {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(PostUpdate, apply_offset.in_set(PostTransformSystems::Blend));
    }
}

fn apply_offset(mut offset_query: Query<(&Offset, &mut Transform)>) {
    for (offset, mut transform) in &mut offset_query {
        transform.translation += offset.0.extend(0.0);
    }
}
