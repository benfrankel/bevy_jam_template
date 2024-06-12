mod boot;
mod end_screen;
mod game;
mod loading_screen;
mod splash_screen;
mod title_screen;

use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::ui::Val::*;
use strum::EnumIter;

use crate::core::theme::ThemeBackgroundColor;
use crate::core::theme::ThemeColor;
use crate::util::animation::FadeIn;
use crate::util::animation::FadeOut;

pub struct SequencePlugin;

impl Plugin for SequencePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SequenceState>().add_plugins((
            boot::BootStatePlugin,
            splash_screen::SplashScreenStatePlugin,
            title_screen::TitleScreenStatePlugin,
            loading_screen::LoadingScreenStatePlugin,
            game::GameStatePlugin,
            end_screen::EndScreenStatePlugin,
        ));
    }
}

#[derive(States, Reflect, Default, Copy, Clone, Eq, PartialEq, Hash, Debug, EnumIter)]
pub enum SequenceState {
    #[default]
    Boot,
    SplashScreen,
    TitleScreen,
    LoadingScreen,
    // TODO: Workaround for https://github.com/bevyengine/bevy/issues/9130
    RestartGame,
    Game,
    EndScreen,
}

const FADE_IN_SECS: f32 = 0.3;

fn fade_in(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("ScreenFadeIn"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Percent(100.0),
                    height: Percent(100.0),
                    ..default()
                },
                focus_policy: FocusPolicy::Block,
                z_index: ZIndex::Global(1000),
                ..default()
            },
            ThemeBackgroundColor(ThemeColor::Body),
            FadeIn::new(FADE_IN_SECS),
        ))
        .id()
}

const FADE_OUT_SECS: f32 = 0.3;

fn fade_out(commands: &mut Commands, next_state: SequenceState) -> Entity {
    commands
        .spawn((
            Name::new("ScreenFadeOut"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Percent(100.0),
                    height: Percent(100.0),
                    ..default()
                },
                focus_policy: FocusPolicy::Block,
                z_index: ZIndex::Global(1000),
                ..default()
            },
            ThemeBackgroundColor(ThemeColor::Body),
            FadeOut::new(FADE_OUT_SECS, next_state),
        ))
        .id()
}
