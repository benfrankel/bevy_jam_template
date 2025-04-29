use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::screen::Screen;
use crate::screen::fade::fade_out;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Pause.on_enter(spawn_pause_menu));
}

fn spawn_pause_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands.entity(menu_root.ui).with_child(pause());
}

fn pause() -> impl Bundle {
    (
        Name::new("Pause"),
        Node {
            padding: UiRect::all(Vw(4.5)),
            ..Node::COLUMN_MID.full_size().abs()
        },
        GlobalZIndex(2),
        DespawnOnExitState::<Menu>::Recursive,
        children![header(), buttons()],
    )
}

const HEADER: &str = "[b]Game paused";

fn header() -> impl Bundle {
    (
        Name::new("Header"),
        RichText::from_sections(parse_rich(HEADER)),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
        Node {
            margin: UiRect::bottom(Vw(2.5)),
            ..default()
        },
    )
}

fn buttons() -> impl Bundle {
    (
        Name::new("Buttons"),
        Node {
            margin: UiRect::top(VMin(6.0)),
            row_gap: Vw(2.5),
            ..Node::COLUMN_CENTER
        },
        children![
            widget::small_button("Settings", open_settings),
            widget::small_button("Continue", disable_menu),
            widget::small_button("Restart", restart_game),
            widget::small_button("Quit to title", quit_to_title),
        ],
    )
}

fn open_settings(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.push(Menu::Settings);
}

fn disable_menu(_: Trigger<Pointer<Click>>, mut menu: NextMut<Menu>) {
    menu.disable();
}

fn restart_game(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn(fade_out(Screen::Playing));
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.spawn(fade_out(Screen::Title));
}
