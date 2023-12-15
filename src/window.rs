use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;
use bevy::window::WindowPlugin as BevyWindowPlugin;
use serde::Deserialize;
use serde::Serialize;

use crate::config::Config;
use crate::AppRoot;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        let mut window = Window {
            title: WINDOW_TITLE.to_string(),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            ..default()
        };
        app.world.resource::<Config>().window.apply(&mut window);
        app.add_plugins(BevyWindowPlugin {
            primary_window: Some(window),
            exit_condition: ExitCondition::OnPrimaryClosed,
            ..default()
        });

        app.add_systems(Startup, register_window);
    }
}

// TODO: Add this to WindowConfig
const WINDOW_TITLE: &str = "bevy_jam_template";

#[derive(Reflect, Serialize, Deserialize)]
pub struct WindowConfig {
    pub window_mode: WindowMode,
    pub present_mode: PresentMode,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            window_mode: WindowMode::Windowed,
            present_mode: PresentMode::AutoVsync,
        }
    }
}

impl WindowConfig {
    pub fn apply(&self, window: &mut Window) {
        window.mode = self.window_mode;
        window.present_mode = self.present_mode;
    }
}

fn register_window(mut root: ResMut<AppRoot>, window_query: Query<Entity, With<PrimaryWindow>>) {
    root.window = window_query.single();
}
