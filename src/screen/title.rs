use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use pyri_state::prelude::*;

use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::FadeOut;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Title.on_enter(title.spawn()));

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

fn title(In(id): In<Entity>, mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands
        .entity(id)
        .insert(Node::COLUMN_MID.full_size().named("Title"))
        .set_parent(screen_root.ui)
        .with_children(|children| {
            children.spawn_fn(header);
            children.spawn_fn(buttons);
        });
}

fn header(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
        Name::new("Header"),
        RichText::from_sections(parse_rich("[b]bevy_jam_template")),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
        Node {
            margin: UiRect::vertical(Vw(5.0)),
            ..default()
        },
    ));
}

fn buttons(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert(
            Node {
                margin: UiRect::vertical(VMin(9.0)),
                row_gap: Vw(2.5),
                ..Node::COLUMN_MID.full_width()
            }
            .named("Buttons"),
        )
        .with_children(|children| {
            children.spawn_fn(play_button);
            children.spawn_fn(quit_button);
        });
}

fn play_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .queue(widget::MenuButton::new("Play"))
        .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.spawn_with(FadeOut::to(Screen::Intro));
        });
}

fn quit_button(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).queue(widget::MenuButton::new("Quit"));

    #[cfg(feature = "web")]
    commands.entity(id).insert(IsDisabled(true));

    #[cfg(not(feature = "web"))]
    commands
        .entity(id)
        .observe(|_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<_>| {
            app_exit.send(bevy::app::AppExit::Success);
        });
}
