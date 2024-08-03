use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::screen::fade::FadeOut;
use crate::screen::playing::PlayingMenu;
use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        PlayingMenu::Pause.on_edge(Pause::disable, (Pause::enable_default, open_pause_menu)),
    );
}

fn open_pause_menu(mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands.spawn_fn(pause_overlay).set_parent(screen_root.ui);
    commands.spawn_fn(pause_menu).set_parent(screen_root.ui);
}

fn pause_overlay(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .add_fn(widget::blocking_overlay)
        .insert((
            Name::new("PauseOverlay"),
            ZIndex::Global(1),
            ThemeColor::Overlay.set::<BackgroundColor>(),
            DespawnOnExit::<PlayingMenu>::Recursive,
        ));
}

fn pause_menu(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert((
            Name::new("PauseMenu"),
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Vw(4.5)),
                    ..Style::COLUMN_MID.full_size().abs()
                },
                z_index: ZIndex::Global(2),
                ..default()
            },
            DespawnOnExit::<PlayingMenu>::Recursive,
        ))
        .with_children(|children| {
            children.spawn_fn(header);
            children.spawn_fn(buttons);
        });
}

const HEADER: &str = "[b]Game paused";

fn header(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
        Name::new("Header"),
        TextBundle::from_sections(parse_rich(HEADER)).with_style(Style {
            margin: UiRect::bottom(Vw(2.5)),
            ..default()
        }),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
    ));
}

fn buttons(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert(
            Style {
                margin: UiRect::top(VMin(6.0)),
                row_gap: Vw(2.5),
                ..Style::COLUMN_CENTER
            }
            .node("Buttons"),
        )
        .with_children(|children| {
            children.spawn_fn(continue_button);
            children.spawn_fn(restart_button);
            children.spawn_fn(quit_to_title_button);
        });
}

fn continue_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .add(widget::MenuButton::new("Continue"))
        .insert((
            On::<Pointer<Click>>::run(PlayingMenu::disable),
            Style {
                height: Vw(9.0),
                width: Vw(38.0),
                ..Style::ROW_CENTER
            },
        ));
}

fn restart_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .add(widget::MenuButton::new("Restart"))
        .insert((
            On::<Pointer<Click>>::run(|mut commands: Commands| {
                commands.spawn_with(FadeOut::to(Screen::Playing));
            }),
            Style {
                height: Vw(9.0),
                width: Vw(38.0),
                ..Style::ROW_CENTER
            },
        ));
}

fn quit_to_title_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .add(widget::MenuButton::new("Quit to title"))
        .insert((
            On::<Pointer<Click>>::run(|mut commands: Commands| {
                commands.spawn_with(FadeOut::to(Screen::Title));
            }),
            Style {
                height: Vw(9.0),
                width: Vw(38.0),
                ..Style::ROW_CENTER
            },
        ));
}
