use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Intro.on_enter(spawn_intro_menu));
}

fn spawn_intro_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::body(children![
            widget::header("[b]How to play"),
            widget::paragraph("Be skillful,\nwin the game!\nPress P to pause."),
            widget::row_of_buttons(children![
                widget::button("Back", go_back),
                widget::button("Start", start_game)
            ]),
        ]));
}

fn go_back(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.pop();
}

fn start_game(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    progress: Res<ProgressTracker<BevyState<Screen>>>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    commands.spawn(fade_out(if done >= total {
        Screen::Gameplay
    } else {
        Screen::Loading
    }));
}
