use bevy::prelude::*;
use bevy::ui::Val::*;

use crate::state::AppState;
use crate::theme::ThemeBackgroundColor;
use crate::theme::ThemeColor;
use crate::util::animation::AnimationSet;
use crate::util::DespawnSet;
use crate::AppSet;

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FadeIn>()
            .add_systems(PostUpdate, apply_fade_in.in_set(AnimationSet::Update));

        app.register_type::<FadeOut>()
            .add_systems(PostUpdate, apply_fade_out.in_set(AnimationSet::Update));
    }
}

#[derive(Component, Reflect)]
pub struct FadeIn {
    duration: f32,
    remaining: f32,
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
        color.0.set_a((fade.remaining / fade.duration).max(0.0));
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
    next_state: AppState,
}

impl FadeOut {
    pub fn new(duration: f32, next_state: AppState) -> Self {
        Self {
            duration,
            remaining: duration,
            next_state,
        }
    }
}

fn apply_fade_out(
    time: Res<Time>,
    mut despawn: ResMut<DespawnSet>,
    mut next_state: ResMut<NextState<AppState>>,
    mut fade_query: Query<(Entity, &mut FadeOut, &mut BackgroundColor)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut fade, mut color) in &mut fade_query {
        // TODO: Non-linear alpha?
        color
            .0
            .set_a(1.0 - (fade.remaining / fade.duration).max(0.0));
        if fade.remaining <= 0.0 {
            next_state.set(fade.next_state);
            despawn.recursive(entity);
        }
        fade.remaining -= dt;
    }
}
