use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::AppRoot;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, mut root: ResMut<AppRoot>) {
    root.camera = commands
        .spawn((
            Name::new("MainCamera"),
            Camera2dBundle {
                projection: OrthographicProjection {
                    near: -1000.0,
                    scaling_mode: ScalingMode::AutoMax {
                        max_width: 1920.0,
                        max_height: 1080.0,
                    },
                    ..default()
                },
                ..default()
            },
        ))
        .id();
}
