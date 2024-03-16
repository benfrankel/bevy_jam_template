use bevy::prelude::*;
use bevy::ui::Val::*;

use crate::common::theme::ThemeBackgroundColor;
use crate::common::theme::ThemeColor;
use crate::common::PostColorSet;
use crate::sequence::SequenceState;
use crate::util::DespawnSet;

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FadeIn>()
            .add_systems(PostUpdate, apply_fade_in.in_set(PostColorSet::Blend));

        app.register_type::<FadeOut>()
            .add_systems(PostUpdate, apply_fade_out.in_set(PostColorSet::Blend));
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
    next_state: SequenceState,
}

impl FadeOut {
    pub fn new(duration: f32, next_state: SequenceState) -> Self {
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
    mut next_state: ResMut<NextState<SequenceState>>,
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
