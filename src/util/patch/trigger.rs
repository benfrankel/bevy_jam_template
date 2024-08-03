use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14233.
pub trait EntityCommandsExtTrigger {
    fn trigger(&mut self, event: impl Event) -> &mut Self;
}

impl EntityCommandsExtTrigger for EntityCommands<'_> {
    fn trigger(&mut self, event: impl Event) -> &mut Self {
        let entity = self.id();
        self.commands().trigger_targets(event, entity);
        self
    }
}

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14236.
pub trait TriggerExtGetEntity {
    fn get_entity(&self) -> Option<Entity>;
}

impl<E, B: Bundle> TriggerExtGetEntity for Trigger<'_, E, B> {
    fn get_entity(&self) -> Option<Entity> {
        Some(self.entity()).filter(|&x| x != Entity::PLACEHOLDER)
    }
}
