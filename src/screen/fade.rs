use crate::animation::PostColorSystems;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(FadeIn, FadeOut)>();
}

pub const FADE_IN_SECS: f32 = 0.5;
const FADE_OUT_SECS: f32 = 0.2;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct FadeIn {
    duration: f32,
    remaining: f32,
}

impl Configure for FadeIn {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(PostUpdate, apply_fade_in.in_set(PostColorSystems::Blend));
        app.add_systems(StateFlush, Screen::ANY.on_enter(spawn_fade_in));
    }
}

impl FadeIn {
    fn new(duration: f32) -> Self {
        Self {
            duration,
            remaining: duration,
        }
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn apply_fade_in(
    time: Res<Time>,
    mut late: LateCommands,
    mut fade_query: Query<(Entity, &mut FadeIn, &mut BackgroundColor)>,
) {
    let dt = time.delta_secs();
    for (entity, mut fade, mut color) in &mut fade_query {
        // TODO: Non-linear alpha?
        color.0.set_alpha((fade.remaining / fade.duration).max(0.0));
        if fade.remaining <= 0.0 {
            late.commands().entity(entity).despawn();
        }
        fade.remaining -= dt;
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn spawn_fade_in(mut commands: Commands) {
    commands.spawn(fade_in());
}

/// A screen transition animation for entering the current [`Screen`].
pub fn fade_in() -> impl Bundle {
    (
        widget::overlay(1000),
        FadeIn::new(FADE_IN_SECS),
        ThemeColor::Body.set::<BackgroundColor>(),
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct FadeOut {
    duration: f32,
    remaining: f32,
    to_screen: Screen,
}

impl Configure for FadeOut {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(PostUpdate, apply_fade_out.in_set(PostColorSystems::Blend));
    }
}

impl FadeOut {
    fn new(duration: f32, to_screen: Screen) -> Self {
        Self {
            duration,
            remaining: duration,
            to_screen,
        }
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn apply_fade_out(
    time: Res<Time>,
    mut late: LateCommands,
    mut screen: NextMut<Screen>,
    mut fade_query: Query<(Entity, &mut FadeOut, &mut BackgroundColor)>,
) {
    let dt = time.delta_secs();
    for (entity, mut fade, mut color) in &mut fade_query {
        // TODO: Non-linear alpha?
        color
            .0
            .set_alpha(1.0 - (fade.remaining / fade.duration).max(0.0));
        if fade.remaining <= 0.0 {
            screen.trigger().enter(fade.to_screen);
            late.commands().entity(entity).despawn();
        }
        fade.remaining -= dt;
    }
}

/// A screen transition animation for exiting the current [`Screen`].
pub fn fade_out(to_screen: Screen) -> impl Bundle {
    (
        widget::blocking_overlay(1000),
        FadeOut::new(FADE_OUT_SECS, to_screen),
        ThemeColor::Body.set::<BackgroundColor>(),
    )
}
