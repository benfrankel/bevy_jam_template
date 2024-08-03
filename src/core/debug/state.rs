use bevy::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::core::debug::DebugConfig;
use crate::core::window::WindowReady;
use crate::screen::Screen;
use crate::screen::ScreenTime;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<StateDebugSettings>();

    // Skip to a custom initial screen.
    app.add_systems(StateFlush, WindowReady.on_enter(enter_initial_screen));

    // Extend loading screen.
    app.add_systems(
        Update,
        (
            Screen::Intro.on_update(do_not_skip_loading_screen.track_progress()),
            Screen::Loading.on_update(extend_loading_screen.track_progress()),
        ),
    );
}

pub(super) fn on_load(config: &DebugConfig, world: &mut World) {
    world.resource_mut::<StateDebugSettings>().log_flush = config.log_state_flush;
}

fn enter_initial_screen(
    config_handle: Res<ConfigHandle<DebugConfig>>,
    config: Res<Assets<DebugConfig>>,

    mut screen: NextMut<Screen>,
) {
    let config = r!(config.get(&config_handle.0));
    screen.enter(rq!(config.initial_screen));
}

fn do_not_skip_loading_screen(
    config_handle: Res<ConfigHandle<DebugConfig>>,
    config: Res<Assets<DebugConfig>>,
) -> Progress {
    let config = r!(config.get(&config_handle.0));
    (config.extend_loading_screen <= 0.0).into()
}

fn extend_loading_screen(
    config_handle: Res<ConfigHandle<DebugConfig>>,
    config: Res<Assets<DebugConfig>>,
    screen_time: Res<ScreenTime>,
) -> Progress {
    let config = r!(config.get(&config_handle.0));
    (screen_time.0.as_secs_f32() >= config.extend_loading_screen).into()
}
