use bevy::window::PrimaryWindow;

use crate::animation::PostTransformSystems;
use crate::animation::backup::Backup;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Offset, NodeOffset)>();
}

#[derive(Component, Reflect, Copy, Clone, Default)]
#[reflect(Component)]
#[require(Backup<Transform>)]
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

#[derive(Component, Reflect, Copy, Clone, Default)]
#[reflect(Component)]
#[require(Backup<Transform>)]
pub struct NodeOffset {
    pub x: Val,
    pub y: Val,
}

impl Configure for NodeOffset {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            PostUpdate,
            apply_node_offset.in_set(PostTransformSystems::Blend),
        );
    }
}

impl NodeOffset {
    pub fn new(x: Val, y: Val) -> Self {
        Self { x, y }
    }
}

fn apply_node_offset(
    primary_window: Single<&Window, With<PrimaryWindow>>,
    mut node_offset_query: Query<(
        &NodeOffset,
        &ComputedNode,
        &mut Transform,
        Option<&mut BoxShadow>,
        Has<Backup<BoxShadow>>,
    )>,
) {
    let viewport_size = primary_window.resolution.size();

    for (offset, computed_node, mut transform, box_shadow, has_backup_box_shadow) in
        &mut node_offset_query
    {
        let x = match offset.x {
            Val::Auto => 0.0,
            x => c!(x.resolve(computed_node.size().x, viewport_size)),
        };
        let y = match offset.y {
            Val::Auto => 0.0,
            y => c!(y.resolve(computed_node.size().x, viewport_size)),
        };
        transform.translation += vec3(x, y, 0.0);

        let mut box_shadow = cq!(box_shadow);
        c!(has_backup_box_shadow);
        for shadow in &mut box_shadow.0 {
            if let Ok(x) = shadow
                .x_offset
                .add(Px(-x), computed_node.size().x, viewport_size)
            {
                shadow.x_offset = x;
            }
            if let Ok(y) = shadow
                .y_offset
                .add(Px(-y), computed_node.size().x, viewport_size)
            {
                shadow.y_offset = y;
            }
        }
    }
}
