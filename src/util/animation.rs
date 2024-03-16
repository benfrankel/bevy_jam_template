pub use crate::util::animation::transition::FadeIn;
pub use crate::util::animation::transition::FadeOut;

mod transition;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::ui::UiSystem;
use bevy_rapier2d::plugin::PhysicsSet;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(transition::TransitionPlugin);
    }
}
