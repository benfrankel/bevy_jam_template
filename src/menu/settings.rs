use bevy_simple_prefs::Prefs;
use bevy_simple_prefs::PrefsPlugin;

use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Settings.on_enter(spawn_settings_menu));

    app.configure::<Settings>();
}

fn spawn_settings_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::body(children![
            widget::header("[b]Settings"),
            widget::button_column(children![widget::button("Back", go_back)]),
        ]));
}

fn go_back(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.pop();
}

#[derive(Prefs, Reflect, Default)]
struct Settings {}

impl Configure for Settings {
    fn configure(app: &mut App) {
        // Create the config folder if necessary.
        #[cfg(not(feature = "native"))]
        let path = default();
        #[cfg(feature = "native")]
        let path = {
            let path = r!(dirs::config_local_dir()).join(env!("CARGO_PKG_NAME"));
            r!(std::fs::create_dir_all(&path).is_ok());
            r!(std::fs::exists(&path));
            path
        };

        // If there were no issues, initialize settings.
        app.add_plugins(PrefsPlugin::<Settings> {
            filename: "settings.ron".to_string(),
            path,
            ..default()
        });
    }
}
