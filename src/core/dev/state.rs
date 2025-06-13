use crate::core::dev::DevConfig;
use crate::menu::Menu;
use crate::menu::MenuTime;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<StateDebugSettings>();

    // Extend loading menu.
    app.add_systems(
        Update,
        (
            Screen::Title.on_update(force_loading_menu.track_progress::<BevyState<Screen>>()),
            Menu::Loading.on_update(extend_loading_menu.track_progress::<BevyState<Screen>>()),
        ),
    );
}

pub(super) fn on_load(config: &DevConfig, world: &mut World) {
    r!(world.get_resource_mut::<StateDebugSettings>()).log_flush = config.log_state_flush;
    if let Some(screen) = config.initial_screen {
        r!(world.get_resource_mut::<NextStateBuffer<Screen>>()).enter(screen);
    }
}

fn force_loading_menu(config: ConfigRef<DevConfig>, menu: CurrentRef<Menu>) -> Progress {
    let config = r!(config.get());
    (config.extend_loading_menu <= 0.0 || menu.is_in(&Menu::Loading)).into()
}

fn extend_loading_menu(config: ConfigRef<DevConfig>, menu_time: Res<MenuTime>) -> Progress {
    let config = r!(config.get());
    (menu_time.0.as_secs_f32() >= config.extend_loading_menu).into()
}
