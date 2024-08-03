pub mod backup;
pub mod offset;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::ui::UiSystem;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(PostTransformSet, PostColorSet)>();

    app.add_plugins((backup::plugin, offset::plugin));
}

/// [`Transform`] post-processing system ordering in the [`PostUpdate`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostTransformSet {
    /// Save the base transform as a backup.
    Save,
    /// Blend via transform multplication (add translation, add rotation, multiply scale).
    Blend,
    /// Apply facing (may multiply translation.x by -1).
    ApplyFacing,
    /// Apply finishing touches to GlobalTransform, like rounding to the nearest pixel.
    Finish,
}

impl Configure for PostTransformSet {
    fn configure(app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                (UiSystem::Layout, PhysicsSet::Sync),
                Self::Save,
                Self::Blend,
                Self::ApplyFacing,
                TransformSystem::TransformPropagate,
                Self::Finish,
                // GlobalTransform may be slightly out of sync with Transform at this point...
            )
                .chain(),
        );
    }
}

/// [`Color`] post-processing system ordering in the [`PostUpdate`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostColorSet {
    /// Save the base color as a backup.
    Save,
    /// Blend via color multiplication (multiply RGBA).
    Blend,
}

impl Configure for PostColorSet {
    fn configure(app: &mut App) {
        app.configure_sets(PostUpdate, (Self::Save, Self::Blend).chain());
    }
}
