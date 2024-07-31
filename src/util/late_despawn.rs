use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<LateDespawn>();
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct LateDespawn(HashSet<Entity>);

impl Configure for LateDespawn {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(Update, apply_late_despawn.in_set(UpdateSet::Despawn));
    }
}

impl LateDespawn {
    // Only supports recursive despawning, because `Commands::despawn` breaks the hierarchy.
    pub fn recursive(&mut self, entity: Entity) {
        self.0.insert(entity);
    }
}

fn apply_late_despawn(mut commands: Commands, mut despawn: ResMut<LateDespawn>) {
    for entity in despawn.0.drain() {
        // Only despawn entities that still exist.
        c!(commands.get_entity(entity)).despawn_recursive();
    }
}
