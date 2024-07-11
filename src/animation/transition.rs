use bevy::prelude::*;

use crate::core::PostColorSet;
use crate::screen::Screen;
use crate::util::despawn::DespawnSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(FadeIn, FadeOut)>();
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
    mut despawn: ResMut<DespawnSet>,
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
    next_screen: Screen,
}

impl Configure for FadeOut {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(PostUpdate, apply_fade_out.in_set(PostColorSet::Blend));
    }
}

impl FadeOut {
    pub fn new(duration: f32, next_screen: Screen) -> Self {
        Self {
            duration,
            remaining: duration,
            next_screen,
        }
    }
}

fn apply_fade_out(
    time: Res<Time>,
    mut despawn: ResMut<DespawnSet>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut fade_query: Query<(Entity, &mut FadeOut, &mut BackgroundColor)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut fade, mut color) in &mut fade_query {
        // TODO: Non-linear alpha?
        color
            .0
            .set_alpha(1.0 - (fade.remaining / fade.duration).max(0.0));
        if fade.remaining <= 0.0 {
            next_screen.set(fade.next_screen);
            despawn.recursive(entity);
        }
        fade.remaining -= dt;
    }
}
