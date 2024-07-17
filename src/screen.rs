mod end;
mod loading;
mod playing;
mod splash;
mod title;

use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::animation::FadeIn;
use crate::animation::FadeOut;
use crate::core::window::WindowState;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub fn plugin(app: &mut App) {
    app.configure::<Screen>();

    app.add_plugins((
        splash::plugin,
        title::plugin,
        loading::plugin,
        playing::plugin,
        end::plugin,
    ));
}

#[derive(State, Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect, Default)]
#[state(after(WindowState), bevy_state, log_flush)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Playing,
    End,
}

impl Configure for Screen {
    fn configure(app: &mut App) {
        app.add_state::<Self>();
        app.add_systems(
            StateFlush,
            WindowState::Ready.on_enter(Screen::enable_default),
        );
    }
}

const FADE_IN_SECS: f32 = 0.5;

fn fade_in(mut entity: EntityWorldMut) {
    entity.add(widget::ui_overlay).insert((
        Name::new("ScreenFadeIn"),
        ThemeBackgroundColor(ThemeColor::Body),
        FadeIn::new(FADE_IN_SECS),
    ));
}

const FADE_OUT_SECS: f32 = 0.2;

fn fade_out(to_screen: Screen) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity.add(widget::ui_overlay).insert((
            Name::new("ScreenFadeOut"),
            ThemeBackgroundColor(ThemeColor::Body),
            FadeOut::new(FADE_OUT_SECS, to_screen),
        ));
    }
}
