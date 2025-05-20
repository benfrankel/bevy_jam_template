use crate::prelude::*;

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/14236>.
pub trait TriggerExtGetTarget {
    fn get_target(&self) -> Option<Entity>;
}

impl<E, B: Bundle> TriggerExtGetTarget for Trigger<'_, E, B> {
    fn get_target(&self) -> Option<Entity> {
        Some(self.target()).filter(|&x| x != Entity::PLACEHOLDER)
    }
}
