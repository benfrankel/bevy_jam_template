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
        self.world_scope(|world| world.commands().add(command.with_entity(id)));
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
