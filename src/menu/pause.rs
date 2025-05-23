use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Pause.on_enter(spawn_pause_menu));
}

#[cfg_attr(feature = "native_dev", hot)]
fn spawn_pause_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::body(children![
            widget::header("[b]Game paused"),
            widget::column_of_buttons(children![
                widget::wide_button("Settings", open_settings),
                widget::wide_button("Continue", disable_menu),
                widget::wide_button("Restart", restart_game),
                widget::wide_button("Quit to title", quit_to_title),
            ])
        ]));
}

fn open_settings(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Settings);
}

fn disable_menu(_: Trigger<Pointer<Click>>, mut menu: NextMut<Menu>) {
    menu.disable();
}

fn restart_game(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn(fade_out(Screen::Gameplay));
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn(fade_out(Screen::Title));
}
