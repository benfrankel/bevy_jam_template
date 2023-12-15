use bevy::prelude::*;
use ron::from_str;
use serde::Deserialize;
use serde::Serialize;
use tap::TapFallible;

use crate::window::WindowConfig;

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
            .unwrap();
        info!("Loaded config");

        app.insert_resource(config)
            .add_systems(PreUpdate, apply_config.run_if(resource_changed::<Config>()));
    }
}

// TODO: DevConfig
#[derive(Resource, Serialize, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    // TODO: Color palette
    pub fg_color: Color,
    pub bg_color: Color,
    // TODO: Volume
    // TODO: Mute when out of focus
    // TODO: Keybindings
}

fn apply_config(world: &mut World) {
    info!("Applying config");

    world.resource_scope(|world, config: Mut<Config>| {
        config.window.apply(world);

        world.resource_mut::<ClearColor>().0 = config.bg_color;
    });
}
