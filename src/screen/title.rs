use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::prelude::*;
use pyri_state::prelude::*;

use crate::screen::fade::FadeOut;
use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Title.on_enter(spawn_title_screen));

    app.configure::<TitleScreenAssets>();
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TitleScreenAssets {}

impl Configure for TitleScreenAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

fn spawn_title_screen(mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands.spawn_fn(title_screen).set_parent(screen_root.ui);
}

fn title_screen(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert(Style::COLUMN_MID.full_size().node("TitleScreen"))
        .with_children(|children| {
            children.spawn_fn(header);
            children.spawn_fn(buttons);
        });
}

fn header(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
        Name::new("Header"),
        TextBundle::from_sections(parse_rich("[b]bevy_jam_template")).with_style(Style {
            margin: UiRect::vertical(Vw(5.0)),
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
                margin: UiRect::vertical(VMin(9.0)),
                row_gap: Vw(2.5),
                ..Style::COLUMN_MID.full_width()
            }
            .node("Buttons"),
        )
        .with_children(|children| {
            children.spawn_fn(play_button);
            children.spawn_fn(quit_button);
        });
}

fn play_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .add(widget::MenuButton::new("Play"))
        .insert(On::<Pointer<Click>>::run(|mut commands: Commands| {
            commands.spawn_with(FadeOut::to(Screen::Intro));
        }));
}

fn quit_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .add(widget::MenuButton::new("Quit"))
        .insert((
            #[cfg(feature = "web")]
            IsDisabled(true),
            #[cfg(not(feature = "web"))]
            On::<Pointer<Click>>::run(|mut app_exit: EventWriter<_>| {
                app_exit.send(bevy::app::AppExit::Success);
            }),
        ));
}
