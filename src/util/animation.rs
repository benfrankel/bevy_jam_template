mod transition;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::ui::UiSystem;
use bevy_rapier2d::plugin::PhysicsSet;

pub use crate::util::animation::transition::FadeIn;
pub use crate::util::animation::transition::FadeOut;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                (UiSystem::Layout, PhysicsSet::Writeback),
                AnimationSet::Start,
                AnimationSet::Update,
                AnimationSet::End,
                TransformSystem::TransformPropagate,
            )
                .chain(),
        );

        app.add_plugins(transition::TransitionPlugin);
    }
}

#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
enum AnimationSet {
    /// (PostUpdate) Initialize pre-animation values
    Start,
    /// (PostUpdate) Update animations
    Update,
    /// (PostUpdate) Finalize animation values
    End,
}
