use bevy::ecs::event::ManualEventReader;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;
use serde::Serialize;

use crate::theme::ThemeConfig;
use crate::window::WindowConfig;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Config>()
            .add_plugins(RonAssetPlugin::<Config>::new(&["config.ron"]))
            .add_systems(Startup, load_config)
            .add_systems(
                PreUpdate,
                apply_config.run_if(on_event::<AssetEvent<Config>>()),
            );
    }
}

#[derive(Resource)]
pub struct ConfigHandle(pub Handle<Config>);

// TODO: DevConfig
#[derive(Asset, Reflect, Serialize, Deserialize)]
#[reflect(from_reflect = false)]
pub struct Config {
    pub window: WindowConfig,
    pub theme: ThemeConfig,
    // TODO: Volume
    // TODO: Mute when out of focus
    // TODO: Keybindings
}

fn load_config(mut commands: Commands, ass: Res<AssetServer>) {
    commands.insert_resource(ConfigHandle(ass.load("main.config.ron")));
}

fn apply_config(world: &mut World, mut reader: Local<ManualEventReader<AssetEvent<Config>>>) {
    if !reader
        .read(world.resource::<Events<AssetEvent<_>>>())
        .any(|event| event.is_loaded_with_dependencies(&world.resource::<ConfigHandle>().0))
    {
        return;
    }

    info!("Applying config");
    world.resource_scope(|world, config: Mut<Assets<Config>>| {
        let config = config.get(&world.resource::<ConfigHandle>().0).unwrap();

        config.window.apply(world);
        config.theme.apply(world);
    });
}
