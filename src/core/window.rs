use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;
use bevy::window::WindowPlugin as BevyWindowPlugin;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::config::ConfigHandle;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyWindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            ..default()
        }),
        exit_condition: ExitCondition::OnPrimaryClosed,
        ..default()
    });

    app.configure::<(WindowRoot, WindowState)>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct WindowRoot {
    pub primary: Entity,
}

impl Configure for WindowRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for WindowRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            primary: world
                .query_filtered::<Entity, With<PrimaryWindow>>()
                .single(world),
        }
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
        let Some(mut window) = world.get_mut::<Window>(world.resource::<WindowRoot>().primary)
        else {
            return;
        };

        window.title.clone_from(&self.title);
        window.mode = self.window_mode;
        window.present_mode = self.present_mode;
    }
}

#[derive(State, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[state(bevy_state, log_flush)]
pub enum WindowState {
    #[default]
    Booting,
    Ready,
}

impl Configure for WindowState {
    fn configure(app: &mut App) {
        app.init_state::<WindowState>();
        app.add_plugins(
            ProgressPlugin::new(WindowState::Booting.bevy()).continue_to(WindowState::Ready.bevy()),
        );
        app.add_systems(
            StateFlush,
            WindowState::Booting.on_edge(show_window, hide_window),
        );
        app.add_systems(
            Update,
            WindowState::Booting.on_update(wait_for_config.track_progress()),
        );
    }
}

fn hide_window(window_root: Res<WindowRoot>, mut window_query: Query<&mut Window>) {
    let Ok(mut window) = window_query.get_mut(window_root.primary) else {
        return;
    };

    window.visible = false;
}

fn show_window(window_root: Res<WindowRoot>, mut window_query: Query<&mut Window>) {
    let Ok(mut window) = window_query.get_mut(window_root.primary) else {
        return;
    };

    window.visible = true;
}

// TODO: Load window config separately from other configs.
fn wait_for_config(asset_server: Res<AssetServer>, config_handle: Res<ConfigHandle>) -> Progress {
    asset_server
        .is_loaded_with_dependencies(&config_handle.0)
        .into()
}
