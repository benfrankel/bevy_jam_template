use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::FadeOut;
use crate::screen::playing::PlayingAssets;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Intro.bevy()).load_collection::<PlayingAssets>(),
    );
    app.add_systems(StateFlush, Screen::Intro.on_enter(intro.spawn()));
}

fn intro(In(id): In<Entity>, mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands
        .entity(id)
        .insert(Node::COLUMN_CENTER.full_size().named("Intro"))
        .set_parent(screen_root.ui)
        .with_children(|children| {
            children.spawn_fn(header);
            children.spawn_fn(body);
            children.spawn_fn(buttons);
        });
}

fn header(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
        Name::new("Header"),
        RichText::from_sections(parse_rich("[b]How to play")),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
        Node {
            margin: UiRect::bottom(Vw(5.0)),
            ..default()
        },
    ));
}

fn body(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert(
            Node {
                row_gap: Vw(1.4),
                ..Node::COLUMN_MID
            }
            .named("Body"),
        )
        .with_children(|children| {
            for (i, text) in ["Be skillful,", "win the game!", "Press P to pause."]
                .into_iter()
                .enumerate()
            {
                children.spawn((
                    Name::new(format!("Span{}", i)),
                    RichText::from_sections(parse_rich(text)),
                    DynamicFontSize::new(Vw(3.5)).with_step(8.0),
                    ThemeColorForText(vec![ThemeColor::BodyText]),
                ));
            }
        });
}

fn buttons(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert(
            Node {
                margin: UiRect::vertical(VMin(9.0)),
                column_gap: Vw(2.5),
                ..Node::ROW
            }
            .named("Buttons"),
        )
        .with_children(|children| {
            children.spawn_fn(start_button);
        });
}

fn start_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .queue(widget::MenuButton::new("Start"))
        .observe(
            |_: Trigger<Pointer<Click>>,
             mut commands: Commands,
             progress: Res<ProgressTracker<BevyState<Screen>>>| {
                let Progress { done, total } = progress.get_global_combined_progress();
                commands.spawn_with(FadeOut::to(if done >= total {
                    Screen::Playing
                } else {
                    Screen::Loading
                }));
            },
        );
}
