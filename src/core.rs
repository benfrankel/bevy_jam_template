//! Foundational features and cross-cutting concerns.

pub mod asset;
pub mod audio;
pub mod camera;
#[cfg(feature = "dev")]
pub mod debug;
pub mod physics;
pub mod state;
pub mod window;

use avian2d::prelude::*;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::ui::UiSystem;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(UpdateSet, PostTransformSet, PostColorSet)>();

    // Add Bevy plugins.
    app.add_plugins(
        DefaultPlugins
            .build()
            // TODO: Doing this instead of `.replace` because `window::plugin` requires `AssetPlugin` to load its config.
            .disable::<AssetPlugin>()
            .add_after::<LogPlugin, _>(asset::plugin)
            .add_after::<LogPlugin, _>(state::plugin)
            .replace::<WindowPlugin>(window::plugin)
            .set(ImagePlugin::default_nearest()),
    );

    // Add other plugins.
    app.add_plugins((
        audio::plugin,
        camera::plugin,
        #[cfg(feature = "dev")]
        debug::plugin,
        physics::plugin,
    ));
}

/// Game logic system ordering in the [`Update`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum UpdateSet {
    /// Synchronize start-of-frame values.
    SyncEarly,
    /// Tick timers.
    TickTimers,
    /// Record player and AI input.
    RecordInput,
    /// Step game logic.
    Update,
    /// Handle events emitted this frame.
    HandleEvents,
    /// Spawn and despawn entities.
    SpawnDespawn,
    /// Synchronize end-of-frame values.
    SyncLate,
}

impl Configure for UpdateSet {
    fn configure(app: &mut App) {
        app.configure_sets(
            Update,
            (
                Self::SyncEarly,
                Self::TickTimers,
                Self::Update,
                Self::RecordInput,
                Self::HandleEvents,
                Self::SpawnDespawn,
                Self::SyncLate,
            )
                .chain(),
        );
    }
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
