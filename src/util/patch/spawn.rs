use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::EntityCommands;
use bevy::ecs::system::RunSystemOnce as _;
use bevy::prelude::*;

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14231#issuecomment-2216321086.
pub trait SpawnWithExt {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands;

    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityCommands;
}

impl SpawnWithExt for Commands<'_, '_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityCommands {
        let mut e = self.spawn_empty();
        let id = e.id();
        e.commands().run_system_once_with(id, system);
        e
    }
}

impl SpawnWithExt for ChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityCommands {
        let mut e = self.spawn_empty();
        let id = e.id();
        e.commands().run_system_once_with(id, system);
        e
    }
}

pub trait WorldSpawnWithExt {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut;

    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityWorldMut;
}

impl WorldSpawnWithExt for World {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        let id = e.id();
        e.run_system_once_with(id, system);
        e
    }
}

impl WorldSpawnWithExt for WorldChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }

    fn spawn_fn<M>(
        &mut self,
        system: impl IntoSystem<Entity, (), M> + Send + 'static,
    ) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        let id = e.id();
        e.run_system_once_with(id, system);
        e
    }
}

pub trait CommandsExtRunSystemOnce {
    fn run_system_once<T: IntoSystem<(), Out, Marker> + Send + 'static, Out, Marker>(
        &mut self,
        system: T,
    ) {
        self.run_system_once_with((), system);
    }

    fn run_system_once_with<
        T: IntoSystem<In, Out, Marker> + Send + 'static,
        In: Send + 'static,
        Out,
        Marker,
    >(
        &mut self,
        input: In,
        system: T,
    );
}

impl CommandsExtRunSystemOnce for Commands<'_, '_> {
    fn run_system_once_with<
        T: IntoSystem<In, Out, Marker> + Send + 'static,
        In: Send + 'static,
        Out,
        Marker,
    >(
        &mut self,
        input: In,
        system: T,
    ) {
        self.add(|world: &mut World| {
            world.run_system_once_with(input, system);
        });
    }
}

pub trait EntityWorldMutExtRunSystemOnce {
    fn run_system_once<T: IntoSystem<(), Out, Marker> + Send + 'static, Out, Marker>(
        &mut self,
        system: T,
    ) {
        self.run_system_once_with((), system);
    }

    fn run_system_once_with<
        T: IntoSystem<In, Out, Marker> + Send + 'static,
        In: Send + 'static,
        Out,
        Marker,
    >(
        &mut self,
        input: In,
        system: T,
    );
}

impl EntityWorldMutExtRunSystemOnce for EntityWorldMut<'_> {
    fn run_system_once_with<
        T: IntoSystem<In, Out, Marker> + Send + 'static,
        In: Send + 'static,
        Out,
        Marker,
    >(
        &mut self,
        input: In,
        system: T,
    ) {
        self.world_scope(|world| {
            world.run_system_once_with(input, system);
        });
    }
}

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14278.
pub trait EntityWorldMutExtAdd {
    fn add<M: 'static>(&mut self, command: impl EntityCommand<M>) -> &mut Self;
}

impl EntityWorldMutExtAdd for EntityWorldMut<'_> {
    fn add<M: 'static>(&mut self, command: impl EntityCommand<M>) -> &mut Self {
        let id = self.id();
        self.world_scope(|world| {
            world.commands().add(command.with_entity(id));
            world.flush_commands();
        });
        self
    }
}

pub trait AddFnExt {
    fn add_fn<M>(&mut self, system: impl IntoSystem<Entity, (), M> + Send + 'static) -> &mut Self;
}

impl AddFnExt for EntityCommands<'_> {
    fn add_fn<M>(&mut self, system: impl IntoSystem<Entity, (), M> + Send + 'static) -> &mut Self {
        let id = self.id();
        self.commands().run_system_once_with(id, system);
        self
    }
}

impl AddFnExt for EntityWorldMut<'_> {
    fn add_fn<M>(&mut self, system: impl IntoSystem<Entity, (), M> + Send + 'static) -> &mut Self {
        self.run_system_once_with(self.id(), system);
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
