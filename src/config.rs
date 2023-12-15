use bevy::prelude::*;
use ron::from_str;
use serde::Deserialize;
use serde::Serialize;
use tap::TapFallible;

use crate::window::WindowConfig;
use crate::AppRoot;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "web")]
        let config_str = include_str!("../assets/config.ron");
        #[cfg(not(feature = "web"))]
        let config_str = &std::fs::read_to_string("assets/config.ron")
            .tap_err(|e| error!("Reading config: {e}"))
            .unwrap_or_default();
        let config = from_str::<Config>(config_str)
            .tap_err(|e| error!("Deserializing config: {e}"))
            .unwrap_or_default();
        info!("Loaded config");

        app.register_type::<Config>()
            .insert_resource(config)
            .add_systems(PreUpdate, apply_config.run_if(resource_changed::<Config>()));
    }
}

// TODO: DevConfig
#[derive(Resource, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct Config {
    pub window: WindowConfig,
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
            window: default(),
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
        }
    }
}

// TODO: Make this an exclusive system
fn apply_config(
    config: Res<Config>,
    root: Res<AppRoot>,
    mut clear_color: ResMut<ClearColor>,
    mut window_query: Query<&mut Window>,
) {
    info!("Applying config");

    if let Ok(mut window) = window_query.get_mut(root.window) {
        config.window.apply(&mut window);
    }

    clear_color.0 = config.bg_color;
}
