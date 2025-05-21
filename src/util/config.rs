use std::any::type_name;

use bevy::diagnostic::FrameCount;
use bevy::ecs::event::EventCursor;
use bevy::ecs::system::SystemParam;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::prelude::*;

pub trait Config: Asset + Serialize + for<'de> Deserialize<'de> {
    const FILE: &'static str;
    const FOLDER: &'static str = "config";

    fn on_load(&self, world: &mut World) {
        let _ = world;
    }

    fn count_progress(&self, asset_server: &AssetServer) -> Progress {
        let _ = asset_server;
        true.into()
    }

    fn progress(config: ConfigRef<Self>, asset_server: Res<AssetServer>) -> Progress {
        config
            .get()
            .map(|x| x.count_progress(&asset_server))
            .unwrap_or(false.into())
    }
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
            apply_config::<C>.run_if(on_event::<AssetEvent<C>>),
        );
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn load_config<C: Config>(world: &mut World) {
    let asset_server = r!(world.get_resource_mut::<AssetServer>());
    let handle = asset_server.load(format!("{}/{}", C::FOLDER, C::FILE));
    world.insert_resource(ConfigHandle::<C>(handle));
}

//#[cfg_attr(feature = "native_dev", hot)]
fn apply_config<C: Config>(world: &mut World, mut cursor: Local<EventCursor<AssetEvent<C>>>) {
    if !cursor
        .read(r!(world.get_resource::<Events<AssetEvent<_>>>()))
        .any(|event| {
            let handle = &r!(world.get_resource::<ConfigHandle<C>>()).0;
            event.is_loaded_with_dependencies(handle) || event.is_modified(handle)
        })
    {
        return;
    }

    info!(
        "[Frame {}] Applying config: {}",
        r!(world.get_resource::<FrameCount>()).0,
        type_name::<C>(),
    );
    world.resource_scope(|world, config: Mut<Assets<C>>| {
        let config_handle = r!(world.get_resource::<ConfigHandle<C>>());
        let config = r!(config.get(&config_handle.0));
        config.on_load(world);
    });
}

#[derive(SystemParam)]
pub struct ConfigRef<'w, C: Config> {
    handle: Option<Res<'w, ConfigHandle<C>>>,
    assets: Res<'w, Assets<C>>,
}

impl<C: Config> ConfigRef<'_, C> {
    pub fn get(&self) -> Option<&C> {
        self.handle.as_ref().and_then(|x| self.assets.get(&x.0))
    }
}

#[derive(SystemParam)]
pub struct ConfigMut<'w, C: Config> {
    handle: Option<Res<'w, ConfigHandle<C>>>,
    assets: ResMut<'w, Assets<C>>,
}

impl<C: Config> ConfigMut<'_, C> {
    pub fn get_mut(&mut self) -> Option<&mut C> {
        self.handle.as_ref().and_then(|x| self.assets.get_mut(&x.0))
    }
}
