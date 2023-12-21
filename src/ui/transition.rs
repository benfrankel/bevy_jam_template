use bevy::prelude::*;
use bevy::ui::Val::*;

use crate::state::AppState;
use crate::theme::ThemeColor;
use crate::util::DespawnSet;
use crate::AppSet;

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FadeIn>()
            .add_systems(PostUpdate, apply_fade_in.in_set(AppSet::Animate));

        app.register_type::<FadeOut>()
            .add_systems(PostUpdate, apply_fade_out.in_set(AppSet::Animate));
    }
}

#[derive(Component, Reflect)]
struct FadeIn {
    duration: f32,
    remaining: f32,
}

impl FadeIn {
    fn new(duration: f32) -> Self {
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
        fade.remaining -= dt;
        if fade.remaining > 0.0 {
            // TODO: Non-linear alpha?
            color.0.set_a(fade.remaining / fade.duration);
        } else {
            despawn.recursive(entity);
        }
    }
}

pub fn fade_in(commands: &mut Commands, duration: f32) -> Entity {
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
                z_index: ZIndex::Global(i32::MAX),
                ..default()
            },
            ThemeColor::Body,
            FadeIn::new(duration),
        ))
        .id()
}

#[derive(Component, Reflect)]
struct FadeOut {
    duration: f32,
    remaining: f32,
    next_state: AppState,
}

impl FadeOut {
    fn new(duration: f32, next_state: AppState) -> Self {
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
        fade.remaining -= dt;
        if fade.remaining > 0.0 {
            // TODO: Non-linear alpha?
            color.0.set_a(1.0 - fade.remaining / fade.duration);
        } else {
            next_state.set(fade.next_state);
            despawn.recursive(entity);
        }
    }
}

pub fn fade_out(commands: &mut Commands, duration: f32, next_state: AppState) -> Entity {
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
                z_index: ZIndex::Global(i32::MAX),
                ..default()
            },
            ThemeColor::Body,
            FadeOut::new(duration, next_state),
        ))
        .id()
}
