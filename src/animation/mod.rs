pub mod backup;
pub mod offset;

use bevy::ui::UiSystem;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(SaveBackupSystems, PostTransformSystems, PostColorSystems)>();

    app.add_plugins((backup::plugin, offset::plugin));
}

#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
struct SaveBackupSystems;

impl Configure for SaveBackupSystems {
    fn configure(app: &mut App) {
        app.configure_sets(
            PostUpdate,
            ((UiSystem::Layout, PhysicsSet::Sync), Self).chain(),
        );
    }
}

/// [`Transform`] post-processing steps for the [`PostUpdate`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostTransformSystems {
    /// Blend via transform multplication (add translation, add rotation, multiply scale).
    Blend,
    /// Apply facing (may multiply translation.x by -1).
    ApplyFacing,
    /// Apply finishing touches to [`GlobalTransform`], like rounding to the nearest pixel.
    Finish,
}

impl Configure for PostTransformSystems {
    fn configure(app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                SaveBackupSystems,
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

/// [`Color`] post-processing steps for the [`PostUpdate`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostColorSystems {
    /// Blend via color multiplication (multiply RGBA).
    Blend,
}

impl Configure for PostColorSystems {
    fn configure(app: &mut App) {
        app.configure_sets(PostUpdate, (SaveBackupSystems, Self::Blend).chain());
    }
}
