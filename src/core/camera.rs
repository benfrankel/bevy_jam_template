use bevy::prelude::*;

use crate::core::UpdateSet;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Msaa::Off);

    app.register_type::<CameraRoot>();
    app.init_resource::<CameraRoot>();

    app.register_type::<AbsoluteScale>();
    app.add_systems(Update, apply_absolute_scale.in_set(UpdateSet::End));
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CameraRoot {
    pub primary: Entity,
}

impl FromWorld for CameraRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            primary: world
                .spawn((
                    Name::new("PrimaryCamera"),
                    Camera2dBundle {
                        projection: OrthographicProjection {
                            near: -1000.0,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .id(),
        }
    }
}

// Camera zoom-independent scale
// (workaround for https://github.com/bevyengine/bevy/issues/1890)
#[derive(Component, Reflect)]
pub struct AbsoluteScale(pub Vec3);

impl Default for AbsoluteScale {
    fn default() -> Self {
        Self(Vec3::ONE)
    }
}

fn apply_absolute_scale(
    camera_root: Res<CameraRoot>,
    camera_query: Query<(&OrthographicProjection, &Camera)>,
    mut scale_query: Query<(&mut Transform, &AbsoluteScale)>,
) {
    let Ok((camera_proj, camera)) = camera_query.get(camera_root.primary) else {
        return;
    };
    let Some(viewport_size) = camera.logical_viewport_size() else {
        return;
    };

    let units_per_pixel = camera_proj.area.width() / viewport_size.x;
    let camera_scale_inverse = Vec2::splat(units_per_pixel).extend(1.0);
    for (mut transform, scale) in &mut scale_query {
        transform.scale = camera_scale_inverse * scale.0;
    }
}
