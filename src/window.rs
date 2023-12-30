use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;
use bevy::window::WindowPlugin as BevyWindowPlugin;
use serde::Deserialize;
use serde::Serialize;

use crate::AppRoot;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyWindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".to_string()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed,
            ..default()
        })
        .add_systems(Startup, register_window);
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
        let window = world.resource::<AppRoot>().window;
        if let Some(mut window) = world.entity_mut(window).get_mut::<Window>() {
            window.title = self.title.clone();
            window.mode = self.window_mode;
            window.present_mode = self.present_mode;
        };
    }
}

fn register_window(mut root: ResMut<AppRoot>, window_query: Query<Entity, With<PrimaryWindow>>) {
    root.window = window_query.single();
}
