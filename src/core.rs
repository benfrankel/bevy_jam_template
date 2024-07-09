//! Foundational features and cross-cutting concerns

pub mod audio;
pub mod camera;
pub mod config;
//#[cfg(feature = "dev")]
//pub mod debug;
pub mod physics;
pub mod theme;
pub mod window;

use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::ui::UiSystem;
use bevy::window::WindowPlugin as BevyWindowPlugin;
use bevy_rapier2d::plugin::PhysicsSet;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Game logic system ordering
        app.configure_sets(
            Update,
            (
                UpdateSet::HandleActions,
                UpdateSet::HandleActionsFlush,
                UpdateSet::Start,
                UpdateSet::Update,
                UpdateSet::React,
                UpdateSet::RecordIntents,
                UpdateSet::ApplyIntents,
                UpdateSet::HandleEvents,
                UpdateSet::QueueDespawn,
                UpdateSet::ApplyDeferred,
                UpdateSet::UpdateUi,
                UpdateSet::End,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                apply_deferred.in_set(UpdateSet::HandleActionsFlush),
                apply_deferred.in_set(UpdateSet::ApplyDeferred),
            ),
        );

        // Post-processing system ordering
        app.configure_sets(
            PostUpdate,
            (
                (
                    (UiSystem::Layout, PhysicsSet::Writeback),
                    PostTransformSet::Save,
                    PostTransformSet::Blend,
                    PostTransformSet::ApplyFacing,
                    TransformSystem::TransformPropagate,
                    PostTransformSet::Finish,
                    // GlobalTransform may be slightly out of sync with Transform at this point...
                )
                    .chain(),
                (PostColorSet::Save, PostColorSet::Blend).chain(),
            ),
        );

        // TODO: Workaround for https://github.com/bevyengine/bevy/issues/10157
        #[cfg(feature = "web")]
        app.insert_resource(bevy::asset::AssetMetaCheck::Never);

        // Bevy plugins
        app.add_plugins(
            DefaultPlugins
                .build()
                .disable::<BevyWindowPlugin>()
                .add_after::<BevyWindowPlugin, _>(window::WindowPlugin)
                .set(ImagePlugin::default_nearest()),
        );

        // Other plugins
        app.add_plugins((
            audio::AudioPlugin,
            camera::CameraPlugin,
            config::ConfigPlugin,
            theme::ThemePlugin,
            physics::PhysicsPlugin,
        ));

        // Debugging tools for dev builds
        /*#[cfg(feature = "dev")]
        app.add_plugins(debug::DebugPlugin {
            log_diagnostics: false,
            log_ambiguity_detection: false,
            //editor: false,
            ..default()
        });*/
    }
}

/// (Update) Game logic system ordering
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum UpdateSet {
    /// Handle actions pressed this frame
    HandleActions,
    /// Apply deferred effects from HandleActions
    HandleActionsFlush,
    /// Initialize start-of-frame values and tick timers
    Start,
    /// Step game logic
    Update,
    /// Run the trigger-effect system
    React,
    /// Record player and AI intents
    RecordIntents,
    /// Apply player and AI intents
    ApplyIntents,
    /// Handle events emitted this frame
    HandleEvents,
    /// Queue despawn commands from DespawnSet
    QueueDespawn,
    /// Apply spawn / despawn and other commands
    ApplyDeferred,
    /// Update UI
    UpdateUi,
    /// Synchronize end-of-frame values
    End,
}

/// (PostUpdate) Transform post-processing system ordering
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostTransformSet {
    /// Save the base transform as a backup
    Save,
    /// Blend via transform multplication (add translation, add rotation, multiply scale)
    Blend,
    /// Apply facing (may multiply translation.x by -1)
    ApplyFacing,
    /// Apply finishing touches to GlobalTransform, like rounding to the nearest pixel
    Finish,
}

/// (PostUpdate) Color post-processing system ordering
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostColorSet {
    /// Save the base color as a backup
    Save,
    /// Blend via color multiplication (multiply RGBA)
    Blend,
}
