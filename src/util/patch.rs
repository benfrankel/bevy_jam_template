use bevy::app::PluginGroupBuilder;
use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

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

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14261.
pub trait Configure {
    fn configure(app: &mut App);
}

macro_rules! impl_configure {
    ($($T:ident),*)  => {
        impl<$($T: Configure),*> Configure for ($($T,)*) {
            fn configure(app: &mut App) {
                $($T::configure(app);)*
                let _ = app;
            }
        }
    }
}

bevy::utils::all_tuples!(impl_configure, 0, 15, T);

pub trait AppExtConfigure {
    fn configure<T: Configure>(&mut self) -> &mut Self;
}

impl AppExtConfigure for App {
    fn configure<T: Configure>(&mut self) -> &mut Self {
        T::configure(self);
        self
    }
}

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14231#issuecomment-2216321086.
pub trait SpawnWithExt {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands;
}

impl SpawnWithExt for Commands<'_, '_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }
}

impl SpawnWithExt for ChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityCommands {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }
}

pub trait WorldSpawnWithExt {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut;
}

impl WorldSpawnWithExt for World {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }
}

impl WorldSpawnWithExt for WorldChildBuilder<'_> {
    fn spawn_with<M: 'static>(&mut self, command: impl EntityCommand<M>) -> EntityWorldMut {
        let mut e = self.spawn_empty();
        e.add(command);
        e
    }
}

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14262.
pub trait PluginGroupBuilderExtReplace {
    fn replace<Target: Plugin>(self, plugin: impl Plugin) -> Self;
}

impl PluginGroupBuilderExtReplace for PluginGroupBuilder {
    fn replace<Target: Plugin>(self, plugin: impl Plugin) -> Self {
        self.disable::<Target>().add_after::<Target, _>(plugin)
    }
}

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14233.
pub trait EntityCommandsExtTrigger {
    fn trigger(&mut self, event: impl Event) -> &mut Self;
}

impl EntityCommandsExtTrigger for EntityCommands<'_> {
    fn trigger(&mut self, event: impl Event) -> &mut Self {
        let entity = self.id();
        self.commands().trigger_targets(event, entity);
        self
    }
}

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14236.
pub trait TriggerExtGetEntity {
    fn get_entity(&self) -> Option<Entity>;
}

impl<E, B: Bundle> TriggerExtGetEntity for Trigger<'_, E, B> {
    fn get_entity(&self) -> Option<Entity> {
        Some(self.entity()).filter(|&x| x != Entity::PLACEHOLDER)
    }
}

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14525.
pub trait Dir2ExtToQuat {
    fn to_quat(self) -> Quat;
}

impl Dir2ExtToQuat for Dir2 {
    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.to_angle())
    }
}
