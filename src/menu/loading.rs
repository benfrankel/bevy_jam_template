use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::fade::fade_out;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Loading.on_enter(spawn_loading_menu));
    app.add_systems(Update, Menu::Loading.on_update(update_loading));
}

fn spawn_loading_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::root(children![widget::center(children![
            widget::big_label("[b]Loading..."),
            widget::spacer(Vw(1.0)),
            widget::loading_bar::<Screen>(),
        ])]));
}

fn update_loading(
    mut commands: Commands,
    progress: Res<ProgressTracker<BevyState<Screen>>>,
    frame: Res<FrameCount>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    // Continue to the next screen when ready.
    if done == total {
        commands.spawn(fade_out(Screen::Gameplay));
    }

    info!("[Frame {}] Loading: {done} / {total}", frame.0);
}
