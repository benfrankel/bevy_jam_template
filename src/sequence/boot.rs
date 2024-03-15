use bevy::prelude::*;
use iyes_progress::prelude::*;

use crate::common::config::ConfigHandle;
use crate::sequence::SequenceState::*;
use crate::AppRoot;

pub struct BootStatePlugin;

impl Plugin for BootStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressPlugin::new(Boot).continue_to(SplashScreen))
            .add_systems(OnEnter(Boot), enter_boot)
            .add_systems(OnExit(Boot), exit_boot);

        app.add_systems(
            Update,
            wait_for_config.track_progress().run_if(in_state(Boot)),
        );
    }
}

fn enter_boot(root: Res<AppRoot>, mut window_query: Query<&mut Window>) {
    let Ok(mut window) = window_query.get_mut(root.window) else {
        return;
    };

    window.visible = false;
}

fn exit_boot(root: Res<AppRoot>, mut window_query: Query<&mut Window>) {
    let Ok(mut window) = window_query.get_mut(root.window) else {
        return;
    };

    window.visible = true;
}

fn wait_for_config(ass: Res<AssetServer>, config_handle: Res<ConfigHandle>) -> Progress {
    ass.is_loaded_with_dependencies(&config_handle.0).into()
}
