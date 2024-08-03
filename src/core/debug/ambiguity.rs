use bevy::ecs::schedule::LogLevel;
use bevy::prelude::*;

use crate::core::debug::DebugConfig;

// TODO: Ambiguity detection only runs on startup, so this doesn't really work like it should.
pub(super) fn on_load(config: &DebugConfig, world: &mut World) {
    let level = if config.log_ambiguity_detection {
        LogLevel::Warn
    } else {
        LogLevel::Ignore
    };

    for (_, schedule) in world.resource_mut::<Schedules>().iter_mut() {
        let mut settings = schedule.get_build_settings();
        settings.ambiguity_detection = level.clone();
        schedule.set_build_settings(settings);
    }
}
