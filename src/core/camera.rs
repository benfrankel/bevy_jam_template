use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(CameraRoot, SmoothFollow, AbsoluteScale)>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CameraRoot {
    pub primary: Entity,
}

impl Configure for CameraRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for CameraRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            primary: world
                .spawn((
                    Name::new("PrimaryCamera"),
                    Camera2d,
                    Projection::Orthographic(OrthographicProjection {
                        near: -1000.0,
                        ..OrthographicProjection::default_2d()
                    }),
                    Msaa::Off,
                    SmoothFollow {
                        target: Entity::PLACEHOLDER,
                        rate: Vec2::splat(100.0),
                    },
                    IsDefaultUiCamera,
                ))
                .id(),
        }
    }
}

/// Follow a target entity smoothly.
///
/// This component should only be used on root entities.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct SmoothFollow {
    target: Entity,
    rate: Vec2,
}

impl Configure for SmoothFollow {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_smooth_follow.run_if(Pause::is_disabled));
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn apply_smooth_follow(
    time: Res<Time>,
    mut follow_query: Query<(&mut Transform, &SmoothFollow)>,
    target_query: Query<&GlobalTransform, Without<SmoothFollow>>,
) {
    let dt = time.delta_secs();
    for (mut transform, follow) in &mut follow_query {
        let target_pos = cq!(target_query.get(follow.target)).translation().xy();
        let mut pos = transform.translation.xy();
        pos += (target_pos - pos) * (follow.rate * dt).clamp(Vec2::ZERO, Vec2::ONE);
        transform.translation = pos.extend(transform.translation.z);
    }
}

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/1890>.
/// Camera zoom-independent scale.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AbsoluteScale(pub Vec3);

impl Configure for AbsoluteScale {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_absolute_scale.in_set(UpdateSystems::SyncLate));
    }
}

impl Default for AbsoluteScale {
    fn default() -> Self {
        Self(Vec3::ONE)
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn apply_absolute_scale(
    camera_root: Res<CameraRoot>,
    camera_query: Query<(&Projection, &Camera)>,
    mut scale_query: Query<(&mut Transform, &AbsoluteScale)>,
) {
    let (projection, camera) = r!(camera_query.get(camera_root.primary));
    let projection = r!(match projection {
        Projection::Orthographic(x) => Some(x),
        _ => None,
    });
    let viewport_size = r!(camera.logical_viewport_size());
    let units_per_pixel = projection.area.width() / viewport_size.x;
    let camera_scale_inverse = Vec2::splat(units_per_pixel).extend(1.0);

    for (mut transform, scale) in &mut scale_query {
        transform.scale = camera_scale_inverse * scale.0;
    }
}
