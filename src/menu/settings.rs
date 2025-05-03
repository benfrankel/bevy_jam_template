use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Settings.on_enter(spawn_settings_menu));
}

fn spawn_settings_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands.entity(menu_root.ui).with_child(settings());
}

#[tweak_fn]
fn settings() -> impl Bundle {
    (
        Name::new("Settings"),
        Node {
            padding: UiRect::all(Vw(4.0)),
            ..Node::COLUMN_MID.full_size().abs()
        },
        GlobalZIndex(2),
        DespawnOnExitState::<Menu>::Recursive,
        children![header(), buttons()],
    )
}

const HEADER: &str = "[b]Settings";

#[tweak_fn]
fn header() -> impl Bundle {
    (
        Name::new("Header"),
        RichText::from_sections(parse_rich(HEADER)),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
        Node {
            margin: UiRect::bottom(Vw(2.0)),
            ..default()
        },
    )
}

#[tweak_fn]
fn buttons() -> impl Bundle {
    (
        Name::new("Buttons"),
        Node {
            margin: UiRect::top(VMin(4.0)),
            row_gap: Vw(2.5),
            ..Node::COLUMN_CENTER
        },
        children![widget::small_button("Back", go_back)],
    )
}

fn go_back(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.pop();
}
