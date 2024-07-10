use bevy::prelude::*;
use iyes_progress::prelude::*;

use crate::core::config::ConfigHandle;
use crate::core::window::WindowRoot;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ProgressPlugin::new(Screen::Boot).continue_to(Screen::Splash));
    app.add_systems(OnEnter(Screen::Boot), enter_boot);
    app.add_systems(OnExit(Screen::Boot), exit_boot);

    app.add_systems(
        Update,
        wait_for_config
            .track_progress()
            .run_if(in_state(Screen::Boot)),
    );
}

fn enter_boot(window_root: Res<WindowRoot>, mut window_query: Query<&mut Window>) {
    let Ok(mut window) = window_query.get_mut(window_root.primary) else {
        return;
    };

    window.visible = false;
}

fn exit_boot(window_root: Res<WindowRoot>, mut window_query: Query<&mut Window>) {
    let Ok(mut window) = window_query.get_mut(window_root.primary) else {
        return;
    };

    window.visible = true;
}

fn wait_for_config(asset_server: Res<AssetServer>, config_handle: Res<ConfigHandle>) -> Progress {
    asset_server
        .is_loaded_with_dependencies(&config_handle.0)
        .into()
}
