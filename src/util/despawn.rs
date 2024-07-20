use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<DespawnSet>();
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct DespawnSet(HashSet<Entity>);

impl Configure for DespawnSet {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(Update, apply_despawn_set.in_set(UpdateSet::SpawnDespawn));
    }
}

impl DespawnSet {
    // Only supports recursive despawning, because `Commands::despawn` breaks the hierarchy.
    pub fn recursive(&mut self, entity: Entity) {
        self.0.insert(entity);
    }
}

fn apply_despawn_set(mut commands: Commands, mut despawn: ResMut<DespawnSet>) {
    for entity in despawn.0.drain() {
        // Only despawn entities that still exist.
        c!(commands.get_entity(entity)).despawn_recursive();
    }
}
