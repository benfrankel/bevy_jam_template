use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;
use bevy::window::WindowPlugin as BevyWindowPlugin;
use serde::Deserialize;
use serde::Serialize;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyWindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            ..default()
        }),
        exit_condition: ExitCondition::OnPrimaryClosed,
        ..default()
    });

    app.register_type::<WindowRoot>();
    app.init_resource::<WindowRoot>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct WindowRoot {
    pub primary: Entity,
}

impl FromWorld for WindowRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            primary: world
                .query_filtered::<Entity, With<PrimaryWindow>>()
                .single(world),
        }
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub window_mode: WindowMode,
    pub present_mode: PresentMode,
}

impl WindowConfig {
    pub fn apply(&self, world: &mut World) {
        let Some(mut window) = world.get_mut::<Window>(world.resource::<WindowRoot>().primary)
        else {
            return;
        };

        window.title.clone_from(&self.title);
        window.mode = self.window_mode;
        window.present_mode = self.present_mode;
    }
}
