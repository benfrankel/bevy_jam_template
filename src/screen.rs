mod boot;
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
use crate::ui::prelude::*;
use crate::util::prelude::*;

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

#[derive(State, Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect, Default)]
#[state(bevy_state, log_flush)]
pub enum Screen {
    #[default]
    Boot,
    Splash,
    Title,
    Loading,
    Playing,
    End,
}

const FADE_IN_SECS: f32 = 0.3;

fn fade_in(mut entity: EntityWorldMut) {
    entity.add(widget::ui_overlay).insert((
        Name::new("ScreenFadeIn"),
        ThemeBackgroundColor(ThemeColor::Body),
        FadeIn::new(FADE_IN_SECS),
    ));
}

const FADE_OUT_SECS: f32 = 0.3;

fn fade_out(to_screen: Screen) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity.add(widget::ui_overlay).insert((
            Name::new("ScreenFadeOut"),
            ThemeBackgroundColor(ThemeColor::Body),
            FadeOut::new(FADE_OUT_SECS, to_screen),
        ));
    }
}
