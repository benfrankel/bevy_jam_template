use bevy::ecs::spawn::SpawnIter;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::fade_out;
use crate::screen::gameplay::GameplayAssets;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Intro.bevy()).load_collection::<GameplayAssets>(),
    );
    app.add_systems(StateFlush, Screen::Intro.on_enter(spawn_intro_screen));
}

fn spawn_intro_screen(mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands.entity(screen_root.ui).with_child(intro());
}

fn intro() -> impl Bundle {
    (
        Name::new("Intro"),
        Node::COLUMN_CENTER.full_size(),
        children![header(), body(), buttons()],
    )
}

fn header() -> impl Bundle {
    (
        Name::new("Header"),
        RichText::from_sections(parse_rich("[b]How to play")),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
        Node {
            margin: UiRect::bottom(Vw(5.0)),
            ..default()
        },
    )
}

fn body() -> impl Bundle {
    (
        Name::new("Body"),
        Node {
            row_gap: Vw(1.4),
            ..Node::COLUMN_MID
        },
        Children::spawn(SpawnIter(
            ["Be skillful,", "win the game!", "Press P to pause."]
                .into_iter()
                .enumerate()
                .map(|(i, text)| {
                    (
                        Name::new(format!("Span{}", i)),
                        RichText::from_sections(parse_rich(text)),
                        DynamicFontSize::new(Vw(3.5)).with_step(8.0),
                        ThemeColorForText(vec![ThemeColor::BodyText]),
                    )
                }),
        )),
    )
}

fn buttons() -> impl Bundle {
    (
        Name::new("Buttons"),
        Node {
            margin: UiRect::vertical(VMin(9.0)),
            column_gap: Vw(2.5),
            ..Node::ROW
        },
        children![widget::big_button("Start", start_game)],
    )
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
