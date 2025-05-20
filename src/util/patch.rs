use bevy::ecs::bundle::BundleEffect;
use bevy::ecs::bundle::DynamicBundle;
use bevy::ecs::component::ComponentId;
use bevy::ecs::component::Components;
use bevy::ecs::component::ComponentsRegistrator;
use bevy::ecs::component::RequiredComponents;
use bevy::ecs::component::StorageType;
use bevy::ptr::OwningPtr;

use crate::prelude::*;

/// A [`BundleEffect`] that applies an arbitrary function to its entity.
pub struct Patch<F: FnOnce(&mut EntityWorldMut)>(pub F);

unsafe impl<F: 'static + Send + Sync + FnOnce(&mut EntityWorldMut)> Bundle for Patch<F> {
    fn component_ids(_: &mut ComponentsRegistrator, _: &mut impl FnMut(ComponentId)) {}
    fn get_component_ids(_: &Components, _: &mut impl FnMut(Option<ComponentId>)) {}
    fn register_required_components(_: &mut ComponentsRegistrator, _: &mut RequiredComponents) {}
}

impl<F: 'static + Send + Sync + FnOnce(&mut EntityWorldMut)> DynamicBundle for Patch<F> {
    type Effect = Self;

    fn get_components(self, _: &mut impl FnMut(StorageType, OwningPtr<'_>)) -> Self::Effect {
        self
    }
}

impl<F: 'static + Send + Sync + FnOnce(&mut EntityWorldMut)> BundleEffect for Patch<F> {
    fn apply(self, entity: &mut EntityWorldMut) {
        self.0(entity);
    }
}
