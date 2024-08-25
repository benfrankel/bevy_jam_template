use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::Command as _;
use bevy::prelude::*;
use tiny_bail::prelude::*;

use super::run_system_cached::RunSystemCached as _;

pub trait SpawnExt {
    // TODO: Workaround for https://github.com/bevyengine/bevy/issues/14231#issuecomment-2216321086.
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands;

    fn spawn_fn<M, T: 'static + Send + IntoSystem<Entity, (), M>>(
        &mut self,
        system: T,
    ) -> EntityCommands;
}

impl SpawnExt for Commands<'_, '_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M, T: 'static + Send + IntoSystem<Entity, (), M>>(
        &mut self,
        system: T,
    ) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add_fn(system);
        e
    }
}

impl SpawnExt for ChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M, T: 'static + Send + IntoSystem<Entity, (), M>>(
        &mut self,
        system: T,
    ) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add_fn(system);
        e
    }
}

pub trait WorldSpawnExt {
    // TODO: Workaround for https://github.com/bevyengine/bevy/issues/14231#issuecomment-2216321086.
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut;

    fn spawn_fn<M, T: 'static + Send + IntoSystem<Entity, (), M>>(
        &mut self,
        system: T,
    ) -> EntityWorldMut;
}

impl WorldSpawnExt for World {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M, T: 'static + Send + IntoSystem<Entity, (), M>>(
        &mut self,
        system: T,
    ) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add_fn(system);
        e
    }
}

impl WorldSpawnExt for WorldChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M, T: 'static + Send + IntoSystem<Entity, (), M>>(
        &mut self,
        system: T,
    ) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add_fn(system);
        e
    }
}

pub trait AddExt {
    // TODO: Workaround for https://github.com/bevyengine/bevy/issues/14278.
    fn add<M: 'static>(&mut self, command: impl EntityCommand<M>) -> &mut Self;
}

impl AddExt for EntityWorldMut<'_> {
    fn add<M: 'static>(&mut self, command: impl EntityCommand<M>) -> &mut Self {
        let id = self.id();
        self.world_scope(|world| {
            command.with_entity(id).apply(world);
        });
        self
    }
}

pub trait AddFnExt {
    fn add_fn<M, T: 'static + Send + IntoSystem<Entity, (), M>>(&mut self, system: T) -> &mut Self;
}

impl AddFnExt for EntityCommands<'_> {
    fn add_fn<M, T: 'static + Send + IntoSystem<Entity, (), M>>(&mut self, system: T) -> &mut Self {
        let id = self.id();
        self.commands()
            .add(move |world: &mut World| r!(world.run_system_cached_with(id, system)));
        self
    }
}

impl AddFnExt for EntityWorldMut<'_> {
    fn add_fn<M, T: 'static + Send + IntoSystem<Entity, (), M>>(&mut self, system: T) -> &mut Self {
        let id = self.id();
        self.world_scope(move |world| r!(world.run_system_cached_with(id, system)));
        self
    }
}

pub trait SpawnSystemExt<M> {
    fn spawn(self) -> impl Fn(Commands);
}

impl<M, T: 'static + Send + Clone + IntoSystem<Entity, (), M>> SpawnSystemExt<M> for T {
    fn spawn(self) -> impl Fn(Commands) {
        move |mut commands: Commands| {
            commands.spawn_fn(self.clone());
        }
    }
}
