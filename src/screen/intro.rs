use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::screen::playing::PlayingAssets;
use crate::screen::FadeIn;
use crate::screen::FadeOut;
use crate::screen::Screen;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Intro.bevy()).load_collection::<PlayingAssets>(),
    );
    app.add_plugins(ProgressPlugin::new(Screen::Intro.bevy()));
    app.add_systems(StateFlush, Screen::Intro.on_edge(exit_intro, enter_intro));
}

fn enter_intro(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(FadeIn::default());
    commands.spawn_fn(intro_screen).set_parent(ui_root.body);
}

fn exit_intro(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}

fn intro_screen(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert(Style::COLUMN_CENTER.full_size().node("IntroScreen"))
        .with_children(|children| {
            children.spawn_fn(header);
            children.spawn_fn(body);
            children.spawn_fn(buttons);
        });
}

fn header(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
        Name::new("Header"),
        TextBundle::from_sections(parse_rich("[b]How to play")).with_style(Style {
            margin: UiRect::bottom(Vw(5.0)),
            ..default()
        }),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
    ));
}

fn body(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert(
            Style {
                row_gap: Vw(1.4),
                ..Style::COLUMN_MID
            }
            .node("Body"),
        )
        .with_children(|children| {
            for (i, text) in ["Be skillful,", "win the game!", "Press P to pause."]
                .into_iter()
                .enumerate()
            {
                children.spawn((
                    Name::new(format!("Span{}", i)),
                    TextBundle::from_sections(parse_rich(text)),
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
            Style {
                margin: UiRect::vertical(VMin(9.0)),
                column_gap: Vw(2.5),
                ..Style::ROW
            }
            .node("Buttons"),
        )
        .with_children(|children| {
            children.spawn_fn(start_button);
        });
}

fn start_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .add(widget::MenuButton::new("Start"))
        .insert(On::<Pointer<Click>>::run(
            |mut commands: Commands, progress: Res<ProgressCounter>| {
                let Progress { done, total } = progress.progress_complete();
                commands.spawn_with(FadeOut::to(if done >= total {
                    Screen::Playing
                } else {
                    Screen::Loading
                }));
            },
        ));
}
