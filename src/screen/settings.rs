use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::fade_out;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Settings.on_enter(spawn_settings_screen));
}

fn spawn_settings_screen(mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands
        .entity(screen_root.ui)
        .with_child(widget::body(children![
            widget::header("[b]Settings"),
            widget::button_column(children![widget::button("Back", go_back)]),
        ]));
}

fn go_back(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn(fade_out(Screen::Title));
}
