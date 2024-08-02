use bevy::prelude::*;
use bevy::window::ExitCondition;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;
use bevy::window::WindowPlugin as BevyWindowPlugin;
use pyri_state::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BevyWindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            visible: false,
            ..default()
        }),
        exit_condition: ExitCondition::OnPrimaryClosed,
        ..default()
    });

    app.configure::<(WindowRoot, ConfigHandle<WindowConfig>, WindowReady)>();
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

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub window_mode: WindowMode,
    pub present_mode: PresentMode,
}

impl Config for WindowConfig {
    const FILE: &'static str = "window.ron";

    fn apply(&self, world: &mut World) {
        world
            .resource_mut::<NextStateBuffer<_>>()
            .enable(WindowReady);

        let mut window = r!(world.get_mut::<Window>(world.resource::<WindowRoot>().primary));
        window.title.clone_from(&self.title);
        window.mode = self.window_mode;
        window.present_mode = self.present_mode;
    }
}

#[derive(State, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[state(log_flush)]
pub struct WindowReady;

impl Configure for WindowReady {
    fn configure(app: &mut App) {
        app.add_state::<WindowReady>();
        app.add_systems(StateFlush, WindowReady.on_enter(show_window));
    }
}

fn show_window(window_root: Res<WindowRoot>, mut window_query: Query<&mut Window>) {
    r!(window_query.get_mut(window_root.primary)).visible = true;
}
