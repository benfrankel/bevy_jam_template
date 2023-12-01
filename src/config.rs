use std::fs::read_to_string;

use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;
use ron::from_str;
use serde::Deserialize;
use serde::Serialize;
use tap::TapFallible;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config = from_str::<Config>(
            &read_to_string(CONFIG_PATH)
                .tap_err(|e| error!("Reading config: {e}"))
                .unwrap_or_default(),
        )
        .tap_err(|e| error!("Deserializing config: {e}"))
        .unwrap_or_default();
        info!("Loaded config");

        app.register_type::<Config>()
            .add_plugins(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: config.present_mode,
                    mode: config.window_mode,
                    title: WINDOW_TITLE.to_string(),
                    ..default()
                }),
                exit_condition: ExitCondition::OnPrimaryClosed,
                ..default()
            })
            .insert_resource(config)
            .add_systems(Update, apply_config.run_if(resource_changed::<Config>()));
    }
}

const WINDOW_TITLE: &str = "bevy_jam_template";
const CONFIG_PATH: &str = "assets/config.ron";

// TODO: DevConfig
#[derive(Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct Config {
    pub window_mode: WindowMode,
    pub present_mode: PresentMode,
    // TODO: Color palette
    pub fg_color: Color,
    pub bg_color: Color,
    // TODO: Volume
    // TODO: Mute when out of focus
    // TODO: Keybindings
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_mode: WindowMode::BorderlessFullscreen,
            present_mode: PresentMode::AutoVsync,
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
        }
    }
}

fn apply_config(
    config: Res<Config>,
    mut clear_color: ResMut<ClearColor>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    info!("Applying config");

    if let Ok(mut window) = window_query.get_single_mut() {
        window.mode = config.window_mode;
        window.present_mode = config.present_mode;
    }

    clear_color.0 = config.bg_color;
}
