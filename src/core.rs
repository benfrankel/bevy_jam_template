//! Foundational features and cross-cutting concerns

pub mod asset;
pub mod audio;
pub mod camera;
pub mod config;
#[cfg(feature = "dev")]
pub mod debug;
pub mod physics;
pub mod state;
pub mod theme;
pub mod window;

use avian2d::prelude::*;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::ui::UiSystem;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(UpdateSet, PostTransformSet, PostColorSet)>();

    // Bevy plugins
    app.add_plugins(
        DefaultPlugins
            .build()
            .replace::<AssetPlugin>(asset::plugin)
            .add_after::<LogPlugin, _>(state::plugin)
            .replace::<WindowPlugin>(window::plugin)
            .set(ImagePlugin::default_nearest()),
    );

    // Other plugins
    app.add_plugins((
        audio::plugin,
        camera::plugin,
        config::plugin,
        #[cfg(feature = "dev")]
        debug::plugin,
        theme::plugin,
        physics::plugin,
    ));
}

/// Game logic system ordering in the [`Update`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum UpdateSet {
    /// Handle actions pressed this frame.
    HandleActions,
    /// Initialize start-of-frame values and tick timers.
    Start,
    /// Step game logic.
    Update,
    /// Run the trigger-effect system.
    React,
    /// Record player and AI intents.
    RecordIntents,
    /// Apply player and AI intents.
    ApplyIntents,
    /// Handle events emitted this frame.
    HandleEvents,
    /// Queue despawn commands from DespawnSet.
    QueueDespawn,
    /// Update UI.
    UpdateUi,
    /// Synchronize end-of-frame values.
    End,
}

impl Configure for UpdateSet {
    fn configure(app: &mut App) {
        app.configure_sets(
            Update,
            (
                Self::HandleActions,
                Self::Start,
                Self::Update,
                Self::React,
                Self::RecordIntents,
                Self::ApplyIntents,
                Self::HandleEvents,
                Self::QueueDespawn,
                Self::UpdateUi,
                Self::End,
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
