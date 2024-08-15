pub mod pause;
pub mod settings;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(MenuRoot, Menu, MenuAction)>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct MenuRoot {
    pub ui: Entity,
}

impl Configure for MenuRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for MenuRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            ui: world
                .spawn((
                    Style::DEFAULT.full_size().node("Menu"),
                    Pickable::IGNORE,
                    DespawnOnDisable::<Menu>::Descendants,
                ))
                .id(),
        }
    }
}

#[derive(State, Copy, Clone, Eq, PartialEq, Debug, Reflect)]
#[state(before(Pause), next(NextStateStack<Self>), react, log_flush)]
#[reflect(Resource)]
pub enum Menu {
    Pause,
    Settings,
}

impl Configure for Menu {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(
            StateFlush,
            (
                Menu::ANY.on_disable(Pause::disable),
                Menu::ANY.on_enable((Pause::enable_default, menu_overlay.spawn())),
            ),
        );
        app.add_plugins((pause::plugin, settings::plugin));
    }
}

fn menu_overlay(In(id): In<Entity>, mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(id)
        .add_fn(widget::blocking_overlay)
        .insert((
            Name::new("MenuOverlay"),
            ZIndex::Global(1),
            ThemeColor::Overlay.set::<BackgroundColor>(),
        ))
        .set_parent(menu_root.ui);
}

#[derive(Actionlike, Copy, Clone, Eq, PartialEq, Hash, Reflect, Debug)]
pub enum MenuAction {
    Back,
}

impl Configure for MenuAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::Back, GamepadButtonType::South)
                .with(Self::Back, KeyCode::Escape),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            Menu::pop
                .in_set(UpdateSet::RecordInput)
                .run_if(action_just_pressed(Self::Back).and_then(Menu::is_enabled)),
        );
    }
}
