use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::screen::Screen;
use crate::screen::fade::FadeOut;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Pause.on_enter(pause.spawn()));
}

fn pause(In(id): In<Entity>, mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(id)
        .insert((
            Name::new("Pause"),
            Node {
                padding: UiRect::all(Vw(4.5)),
                ..Node::COLUMN_MID.full_size().abs()
            },
            GlobalZIndex(2),
            DespawnOnExit::<Menu>::Recursive,
        ))
        .set_parent(menu_root.ui)
        .with_children(|children| {
            children.spawn_fn(header);
            children.spawn_fn(buttons);
        });
}

const HEADER: &str = "[b]Game paused";

fn header(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
        Name::new("Header"),
        RichText::from_sections(parse_rich(HEADER)),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
        Node {
            margin: UiRect::bottom(Vw(2.5)),
            ..default()
        },
    ));
}

fn buttons(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert(
            Node {
                margin: UiRect::top(VMin(6.0)),
                row_gap: Vw(2.5),
                ..Node::COLUMN_CENTER
            }
            .named("Buttons"),
        )
        .with_children(|children| {
            children.spawn_fn(settings_button);
            children.spawn_fn(continue_button);
            children.spawn_fn(restart_button);
            children.spawn_fn(quit_to_title_button);
        });
}

fn settings_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .queue(widget::MenuButton::new("Settings"))
        .insert(Node {
            height: Vw(9.0),
            width: Vw(38.0),
            ..Node::ROW_CENTER
        })
        .observe(
            |_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>| {
                menu.push(Menu::Settings);
            },
        );
}

fn continue_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .queue(widget::MenuButton::new("Continue"))
        .insert(Node {
            height: Vw(9.0),
            width: Vw(38.0),
            ..Node::ROW_CENTER
        })
        .observe(|_: Trigger<Pointer<Click>>, mut menu: NextMut<Menu>| menu.disable());
}

fn restart_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .queue(widget::MenuButton::new("Restart"))
        .insert(Node {
            height: Vw(9.0),
            width: Vw(38.0),
            ..Node::ROW_CENTER
        })
        .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.spawn_with(FadeOut::to(Screen::Playing));
        });
}

fn quit_to_title_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .queue(widget::MenuButton::new("Quit to title"))
        .insert(Node {
            height: Vw(9.0),
            width: Vw(38.0),
            ..Node::ROW_CENTER
        })
        .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.spawn_with(FadeOut::to(Screen::Title));
        });
}
