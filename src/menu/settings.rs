use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Settings.on_enter(settings.spawn()));
}

fn settings(In(id): In<Entity>, mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(id)
        .insert((
            Name::new("Settings"),
            Node {
                padding: UiRect::all(Vw(4.5)),
                ..Node::COLUMN_MID.full_size().abs()
            },
            GlobalZIndex(2),
            DespawnOnExit::<Menu>::Recursive,
        ))
        .set_parent(menu_root.ui)
        .with_children(|parent| {
            parent.spawn_fn(header);
            parent.spawn_fn(buttons);
        });
}

const HEADER: &str = "[b]Settings";

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
        .with_children(|parent| {
            parent.spawn_fn(back_button);
        });
}

fn back_button(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .queue(widget::MenuButton::new("Back"))
        .insert(Node {
            height: Vw(9.0),
            width: Vw(38.0),
            ..Node::ROW_CENTER
        })
        .observe(
            |_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>| {
                menu.pop();
            },
        );
}
