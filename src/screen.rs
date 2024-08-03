mod intro;
mod loading;
mod playing;
mod splash;
mod title;

use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::core::window::WindowReady;
use crate::core::PostColorSet;
use crate::theme::prelude::*;
use crate::util::late_despawn::LateDespawn;
use crate::util::prelude::*;

pub fn plugin(app: &mut App) {
    app.configure::<(Screen, FadeIn, FadeOut)>();

    app.add_plugins((
        splash::plugin,
        title::plugin,
        intro::plugin,
        loading::plugin,
        playing::plugin,
    ));
}

#[derive(State, Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect, Default)]
#[state(after(WindowReady), entity_scope, bevy_state, log_flush)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Intro,
    Loading,
    Playing,
}

impl Configure for Screen {
    fn configure(app: &mut App) {
        app.add_state::<Self>();
        app.add_systems(StateFlush, WindowReady.on_enter(Screen::enable_default));
    }
}

#[derive(Component, Reflect)]
pub struct FadeIn {
    duration: f32,
    remaining: f32,
}

impl Configure for FadeIn {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(PostUpdate, apply_fade_in.in_set(PostColorSet::Blend));
    }
}

impl EntityCommand for FadeIn {
    fn apply(self, id: Entity, world: &mut World) {
        world.run_system_once_with((id, self), fade_in);
    }
}

fn fade_in(In((id, this)): In<(Entity, FadeIn)>, mut commands: Commands) {
    commands.entity(id).add_fn(widget::overlay).insert((
        Name::new("FadeIn"),
        ThemeColor::Body.set::<BackgroundColor>(),
        this,
    ));
}

const FADE_IN_SECS: f32 = 0.5;

impl Default for FadeIn {
    fn default() -> Self {
        Self::new(FADE_IN_SECS)
    }
}

impl FadeIn {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            remaining: duration,
        }
    }
}

fn apply_fade_in(
    time: Res<Time>,
    mut despawn: ResMut<LateDespawn>,
    mut fade_query: Query<(Entity, &mut FadeIn, &mut BackgroundColor)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut fade, mut color) in &mut fade_query {
        // TODO: Non-linear alpha?
        color.0.set_alpha((fade.remaining / fade.duration).max(0.0));
        if fade.remaining <= 0.0 {
            despawn.recursive(entity);
        }
        fade.remaining -= dt;
    }
}

#[derive(Component, Reflect)]
pub struct FadeOut {
    duration: f32,
    remaining: f32,
    to_screen: Screen,
}

impl Configure for FadeOut {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(PostUpdate, apply_fade_out.in_set(PostColorSet::Blend));
    }
}

impl EntityCommand for FadeOut {
    fn apply(self, id: Entity, world: &mut World) {
        world.run_system_once_with((id, self), fade_out);
    }
}

fn fade_out(In((id, this)): In<(Entity, FadeOut)>, mut commands: Commands) {
    commands
        .entity(id)
        .add_fn(widget::blocking_overlay)
        .insert((
            Name::new("FadeOut"),
            ThemeColor::Body.set::<BackgroundColor>(),
            this,
        ));
}

const FADE_OUT_SECS: f32 = 0.2;

impl FadeOut {
    pub fn to(screen: Screen) -> Self {
        Self::new(FADE_OUT_SECS, screen)
    }

    pub fn new(duration: f32, to_screen: Screen) -> Self {
        Self {
            duration,
            remaining: duration,
            to_screen,
        }
    }
}

fn apply_fade_out(
    time: Res<Time>,
    mut despawn: ResMut<LateDespawn>,
    mut screen: NextMut<Screen>,
    mut fade_query: Query<(Entity, &mut FadeOut, &mut BackgroundColor)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut fade, mut color) in &mut fade_query {
        // TODO: Non-linear alpha?
        color
            .0
            .set_alpha(1.0 - (fade.remaining / fade.duration).max(0.0));
        if fade.remaining <= 0.0 {
            screen.enter(fade.to_screen);
            despawn.recursive(entity);
        }
        fade.remaining -= dt;
    }
}
