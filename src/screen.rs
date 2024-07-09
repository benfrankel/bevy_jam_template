mod boot;
mod end;
mod loading;
mod playing;
mod splash;
mod title;

use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy::ui::Val::*;
use strum::EnumIter;

use crate::core::theme::ThemeBackgroundColor;
use crate::core::theme::ThemeColor;
use crate::util::animation::FadeIn;
use crate::util::animation::FadeOut;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.add_plugins((
        boot::plugin,
        splash::plugin,
        title::plugin,
        loading::plugin,
        playing::plugin,
        end::plugin,
    ));
}

#[derive(States, Reflect, Default, Copy, Clone, Eq, PartialEq, Hash, Debug, EnumIter)]
pub enum Screen {
    #[default]
    Boot,
    Splash,
    Title,
    Loading,
    Playing,
    // TODO: Workaround for https://github.com/bevyengine/bevy/issues/9130
    PlayingRestart,
    End,
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

fn fade_out(commands: &mut Commands, to_screen: Screen) -> Entity {
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
            FadeOut::new(FADE_OUT_SECS, to_screen),
        ))
        .id()
}
