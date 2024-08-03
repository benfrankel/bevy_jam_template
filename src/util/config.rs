use std::any::type_name;

use bevy::core::FrameCount;
use bevy::ecs::event::ManualEventReader;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;
use serde::Serialize;

use crate::util::prelude::*;

pub trait Config: Asset + Serialize + for<'de> Deserialize<'de> {
    const FILE: &'static str;
    const FOLDER: &'static str = "config";

    fn on_load(&self, world: &mut World);
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ConfigHandle<C: Config>(pub Handle<C>);

impl<C: Config> Configure for ConfigHandle<C> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_plugins(RonAssetPlugin::<C>::new(&[C::FILE]));
        app.add_systems(Startup, load_config::<C>);
        app.add_systems(
            PreUpdate,
            apply_config::<C>.run_if(on_event::<AssetEvent<C>>()),
        );
    }
}

fn load_config<C: Config>(world: &mut World) {
    let handle = world
        .resource_mut::<AssetServer>()
        .load(format!("{}/{}", C::FOLDER, C::FILE));
    world.insert_resource(ConfigHandle::<C>(handle));
}

fn apply_config<C: Config>(world: &mut World, mut reader: Local<ManualEventReader<AssetEvent<C>>>) {
    if !reader
        .read(world.resource::<Events<AssetEvent<_>>>())
        .any(|event| event.is_loaded_with_dependencies(&world.resource::<ConfigHandle<C>>().0))
    {
        return;
    }

    info!(
        "[Frame {}] Applying config: {}",
        world.resource::<FrameCount>().0,
        type_name::<C>()
    );
    world.resource_scope(|world, config: Mut<Assets<C>>| {
        config
            .get(&world.resource::<ConfigHandle<C>>().0)
            .unwrap()
            .on_load(world);
    });
}
