use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::animation::PostColorSet;
use crate::screen::Screen;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(FadeIn, FadeOut)>();
}

pub const FADE_IN_SECS: f32 = 0.5;
const FADE_OUT_SECS: f32 = 0.2;

#[derive(Component, Reflect)]
pub struct FadeIn {
    duration: f32,
    remaining: f32,
}

impl Configure for FadeIn {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(PostUpdate, apply_fade_in.in_set(PostColorSet::Blend));
        app.add_systems(StateFlush, Screen::ANY.on_enter(spawn_screen_fade_in));
    }
}

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
            late.commands().entity(entity).despawn_recursive();
        }
        fade.remaining -= dt;
    }
}

fn spawn_screen_fade_in(mut commands: Commands) {
    commands.spawn_with(FadeIn::default());
}

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

impl EntityCommand for FadeIn {
    fn apply(self, id: Entity, world: &mut World) {
        r!(world.run_system_cached_with(fade_in, (id, self)));
    }
}

fn fade_in(In((id, this)): In<(Entity, FadeIn)>, mut commands: Commands) {
    commands.entity(id).queue_fn(widget::overlay).insert((
        Name::new("FadeIn"),
        ThemeColor::Body.set::<BackgroundColor>(),
        this,
    ));
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
            late.commands().entity(entity).despawn_recursive();
        }
        fade.remaining -= dt;
    }
}

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

impl EntityCommand for FadeOut {
    fn apply(self, id: Entity, world: &mut World) {
        r!(world.run_system_cached_with(fade_out, (id, self)));
    }
}

fn fade_out(In((id, this)): In<(Entity, FadeOut)>, mut commands: Commands) {
    commands
        .entity(id)
        .queue_fn(widget::blocking_overlay)
        .insert((
            Name::new("FadeOut"),
            ThemeColor::Body.set::<BackgroundColor>(),
            this,
        ));
}
