use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::fade_out;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Intro.bevy()).load_collection::<GameplayAssets>(),
    );
    app.add_systems(StateFlush, Screen::Intro.on_enter(spawn_intro_screen));
}

#[cfg_attr(feature = "native_dev", hot)]
fn spawn_intro_screen(mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands
        .entity(screen_root.ui)
        .with_child(widget::body(children![
            widget::header("[b]How to play:"),
            widget::paragraph("Be skillful,\nwin the game!\nPress P to pause."),
            widget::button_column(children![widget::big_button("Start", start_game)]),
        ]));
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
