use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::fade_out;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading.bevy()).load_collection::<GameplayAssets>(),
    );
    app.add_systems(StateFlush, Screen::Loading.on_enter(spawn_loading_screen));
    app.add_systems(Update, Screen::Loading.on_update(update_loading));
}

#[cfg_attr(feature = "native_dev", hot)]
fn spawn_loading_screen(mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands
        .entity(screen_root.ui)
        .with_child(widget::column_center(children![
            widget::big_label("[b]Loading..."),
            widget::loading_bar::<Screen>(),
        ]));
}

//#[cfg_attr(feature = "native_dev", hot)]
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
