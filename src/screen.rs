mod boot;
mod end;
mod loading;
mod playing;
mod splash;
mod title;

use bevy::prelude::*;
use strum::EnumIter;

use crate::core::theme::ThemeBackgroundColor;
use crate::core::theme::ThemeColor;
use crate::ui::prelude::*;
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
        .spawn_empty()
        .add(ui_overlay)
        .insert((
            Name::new("ScreenFadeIn"),
            ThemeBackgroundColor(ThemeColor::Body),
            FadeIn::new(FADE_IN_SECS),
        ))
        .id()
}

const FADE_OUT_SECS: f32 = 0.3;

fn fade_out(commands: &mut Commands, to_screen: Screen) -> Entity {
    commands
        .spawn_empty()
        .add(ui_overlay)
        .insert((
            Name::new("ScreenFadeOut"),
            ThemeBackgroundColor(ThemeColor::Body),
            FadeOut::new(FADE_OUT_SECS, to_screen),
        ))
        .id()
}
