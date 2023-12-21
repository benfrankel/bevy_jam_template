mod end_screen;
mod game;
mod loading_screen;
mod splash_screen;
mod title_screen;

use bevy::prelude::*;
use strum::EnumIter;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>().add_plugins((
            splash_screen::SplashScreenStatePlugin,
            title_screen::TitleScreenStatePlugin,
            loading_screen::LoadingScreenStatePlugin,
            game::GameStatePlugin,
            end_screen::EndScreenStatePlugin,
        ));
    }
}

const FADE_IN_DURATION: f32 = 0.3;
const FADE_OUT_DURATION: f32 = 0.3;

#[derive(States, Reflect, Default, Copy, Clone, Eq, PartialEq, Hash, Debug, EnumIter)]
pub enum AppState {
    #[default]
    SplashScreen,
    TitleScreen,
    LoadingScreen,
    Game,
    EndScreen,
}
