use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(GameplayAssets, GameplayAction)>();
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameplayAssets {}

impl Configure for GameplayAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameplayAction {
    TogglePause,
}

impl Configure for GameplayAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::TogglePause, GamepadButton::Start)
                .with(Self::TogglePause, KeyCode::Escape)
                .with(Self::TogglePause, KeyCode::KeyP),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            Screen::Gameplay.on_update(
                Menu::Pause
                    .toggle()
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(
                        action_just_pressed(Self::TogglePause)
                            .and(Menu::is_disabled.or(Menu::Pause.will_update())),
                    ),
            ),
        );
    }
}
