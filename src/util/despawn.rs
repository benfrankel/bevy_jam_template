use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::AppSet;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DespawnSet>()
            .init_resource::<DespawnSet>()
            .add_systems(
                Update,
                (
                    // Flush queued commands first to prevent double despawn
                    apply_deferred,
                    apply_despawn_set,
                )
                    .chain()
                    .in_set(AppSet::Despawn),
            );
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct DespawnSet(HashSet<Entity>);

#[allow(dead_code)]
impl DespawnSet {
    // Only supports recursive despawning, because Commands::despawn breaks the hierarchy
    pub fn recursive(&mut self, entity: Entity) {
        self.0.insert(entity);
    }
}

fn apply_despawn_set(mut commands: Commands, mut despawn: ResMut<DespawnSet>) {
    for entity in despawn.0.drain() {
        // Only despawn entities that still exist
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}
