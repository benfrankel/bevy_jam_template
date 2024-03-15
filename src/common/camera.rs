use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off);

        app.register_type::<CameraRoot>()
            .init_resource::<CameraRoot>();
    }
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
