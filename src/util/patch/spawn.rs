use bevy::prelude::*;
use tiny_bail::prelude::*;

pub trait SpawnExt {
    // TODO: Workaround for https://github.com/bevyengine/bevy/issues/14231#issuecomment-2216321086.
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands;

    fn spawn_fn<M, T: 'static + Send + IntoSystem<In<Entity>, (), M>>(
        &mut self,
        system: T,
    ) -> EntityCommands;
}

impl SpawnExt for Commands<'_, '_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.queue(command);
        e
    }

    fn spawn_fn<M, T: 'static + Send + IntoSystem<In<Entity>, (), M>>(
        &mut self,
        system: T,
    ) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.queue_fn(system);
        e
    }
}

impl SpawnExt for ChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.queue(command);
        e
    }

    fn spawn_fn<M, T: 'static + Send + IntoSystem<In<Entity>, (), M>>(
        &mut self,
        system: T,
    ) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.queue_fn(system);
        e
    }
}

pub trait WorldSpawnExt {
    // TODO: Workaround for https://github.com/bevyengine/bevy/issues/14231#issuecomment-2216321086.
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut;

    fn spawn_fn<M, T: 'static + Send + IntoSystem<In<Entity>, (), M>>(
        &mut self,
        system: T,
    ) -> EntityWorldMut;
}

impl WorldSpawnExt for World {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.queue(command);
        e
    }

    fn spawn_fn<M, T: 'static + Send + IntoSystem<In<Entity>, (), M>>(
        &mut self,
        system: T,
    ) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.queue_fn(system);
        e
    }
}

impl WorldSpawnExt for WorldChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.queue(command);
        e
    }

    fn spawn_fn<M, T: 'static + Send + IntoSystem<In<Entity>, (), M>>(
        &mut self,
        system: T,
    ) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.queue_fn(system);
        e
    }
}

pub trait QueueExt {
    // TODO: Workaround for https://github.com/bevyengine/bevy/issues/14278.
    fn queue<M: 'static>(&mut self, command: impl EntityCommand<M>) -> &mut Self;
}

impl QueueExt for EntityWorldMut<'_> {
    fn queue<M: 'static>(&mut self, command: impl EntityCommand<M>) -> &mut Self {
        let id = self.id();
        self.world_scope(|world| {
            command.with_entity(id).apply(world);
        });
        self
    }
}

pub trait QueueFnExt {
    fn queue_fn<M, T: 'static + Send + IntoSystem<In<Entity>, (), M>>(
        &mut self,
        system: T,
    ) -> &mut Self;
}

impl QueueFnExt for EntityCommands<'_> {
    fn queue_fn<M, T: 'static + Send + IntoSystem<In<Entity>, (), M>>(
        &mut self,
        system: T,
    ) -> &mut Self {
        let id = self.id();
        self.commands()
            .queue(move |world: &mut World| r!(world.run_system_cached_with(system, id)));
        self
    }
}

impl QueueFnExt for EntityWorldMut<'_> {
    fn queue_fn<M, T: 'static + Send + IntoSystem<In<Entity>, (), M>>(
        &mut self,
        system: T,
    ) -> &mut Self {
        let id = self.id();
        self.world_scope(move |world| r!(world.run_system_cached_with(system, id)));
        self
    }
}

pub trait SpawnSystemExt<M> {
    fn spawn(self) -> impl Fn(Commands);
}

impl<M, T: 'static + Send + Clone + IntoSystem<In<Entity>, (), M>> SpawnSystemExt<M> for T {
    fn spawn(self) -> impl Fn(Commands) {
        move |mut commands: Commands| {
            commands.spawn_fn(self.clone());
        }
    }
}
