use bevy::ecs::entity::Entities;
use bevy::ecs::system::SystemBuffer;
use bevy::ecs::system::SystemParam;
use bevy::ecs::world::CommandQueue;
use bevy::prelude::*;

use crate::core::UpdateSystems;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<LateCommandBuffer>();
}

/// Like [`Commands`], but applied during [`UpdateSystems::ApplyCommands`] instead of
/// at the next sync point.
///
/// Example usage: `late.commands().entity(entity).despawn_recursive()`.
#[derive(SystemParam)]
pub struct LateCommands<'w, 's> {
    queue: Deferred<'s, LateCommandQueue>,
    entities: &'w Entities,
}

impl LateCommands<'_, '_> {
    pub fn commands(&mut self) -> Commands {
        Commands::new_from_entities(&mut self.queue.0, self.entities)
    }
}

#[derive(Default)]
struct LateCommandQueue(CommandQueue);

impl SystemBuffer for LateCommandQueue {
    fn apply(&mut self, _system_meta: &bevy::ecs::system::SystemMeta, world: &mut World) {
        world
            .resource_mut::<LateCommandBuffer>()
            .0
            .append(&mut self.0);
    }
}

#[derive(Resource, Default)]
struct LateCommandBuffer(CommandQueue);

impl Configure for LateCommandBuffer {
    fn configure(app: &mut App) {
        app.init_resource::<Self>();
        app.add_systems(
            Update,
            apply_late_commands.in_set(UpdateSystems::ApplyCommands),
        );
    }
}

fn apply_late_commands(mut commands: Commands, mut late_commands: ResMut<LateCommandBuffer>) {
    commands.append(&mut late_commands.0);
}
