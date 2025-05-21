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

// SAFETY: This internally relies on the `()` Bundle implementation, which is sound.
unsafe impl<F: 'static + Send + Sync + FnOnce(&mut EntityWorldMut)> Bundle for Patch<F> {
    fn component_ids(components: &mut ComponentsRegistrator, ids: &mut impl FnMut(ComponentId)) {
        <() as Bundle>::component_ids(components, ids)
    }

    fn get_component_ids(components: &Components, ids: &mut impl FnMut(Option<ComponentId>)) {
        <() as Bundle>::get_component_ids(components, ids)
    }

    fn register_required_components(
        components: &mut ComponentsRegistrator,
        required_components: &mut RequiredComponents,
    ) {
        <() as Bundle>::register_required_components(components, required_components)
    }
}

impl<F: 'static + Send + Sync + FnOnce(&mut EntityWorldMut)> DynamicBundle for Patch<F> {
    type Effect = Self;

    fn get_components(self, func: &mut impl FnMut(StorageType, OwningPtr<'_>)) -> Self::Effect {
        <() as DynamicBundle>::get_components((), func);
        self
    }
}

impl<F: 'static + Send + Sync + FnOnce(&mut EntityWorldMut)> BundleEffect for Patch<F> {
    fn apply(self, entity: &mut EntityWorldMut) {
        self.0(entity);
    }
}
