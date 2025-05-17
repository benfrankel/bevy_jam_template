use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Main.on_enter(spawn_main_menu));
}

fn spawn_main_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::body(children![
            widget::header("[b]Pyri New Jam"),
            widget::button_column(children![
                widget::big_button("Play", play_game),
                widget::big_button("Settings", open_settings),
                (
                    widget::big_button("Quit", quit_to_desktop),
                    #[cfg(feature = "web")]
                    InteractionDisabled(true),
                ),
            ]),
        ]));
}

fn play_game(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn(fade_out(Screen::Intro));
}

fn open_settings(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Settings);
}

fn quit_to_desktop(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    if cfg!(not(feature = "web")) {
        app_exit.write(AppExit::Success);
    }
}
