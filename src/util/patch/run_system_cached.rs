use std::marker::PhantomData;

use bevy::ecs::system::RegisteredSystemError;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

// TODO: Workaround for https://github.com/bevyengine/bevy/pull/14920
/// An extension trait that makes it easy to initialize a system once and run it multiple times.
///
/// Similar to [`RunSystemOnce`](bevy::ecs::system::RunSystemOnce), but saves
/// the [`SystemId`] in a resource.
pub trait RunSystemCached: Sized {
    fn register_system_cached<T, In, Out, Marker>(self, system: T) -> SystemId<In, Out>
    where
        T: 'static + IntoSystem<In, Out, Marker>,
        In: 'static,
        Out: 'static;

    fn run_system_cached<T, Out, Marker>(
        self,
        system: T,
    ) -> Result<Out, RegisteredSystemError<(), Out>>
    where
        T: 'static + IntoSystem<(), Out, Marker>,
        Out: 'static,
    {
        self.run_system_cached_with((), system)
    }

    fn run_system_cached_with<T, In, Out, Marker>(
        self,
        input: In,
        system: T,
    ) -> Result<Out, RegisteredSystemError<In, Out>>
    where
        T: 'static + IntoSystem<In, Out, Marker>,
        In: 'static,
        Out: 'static;
}

impl RunSystemCached for &mut World {
    fn register_system_cached<T, In, Out, Marker>(self, system: T) -> SystemId<In, Out>
    where
        T: 'static + IntoSystem<In, Out, Marker>,
        In: 'static,
        Out: 'static,
    {
        match self.get_resource::<CachedSystemId<T, In, Out>>() {
            Some(id) => id.0,
            None => {
                let id = self.register_system(system);
                self.insert_resource(CachedSystemId::<T, In, Out>(id, PhantomData));
                id
            },
        }
    }

    fn run_system_cached_with<T, In, Out, Marker>(
        self,
        input: In,
        system: T,
    ) -> Result<Out, RegisteredSystemError<In, Out>>
    where
        T: 'static + IntoSystem<In, Out, Marker>,
        In: 'static,
        Out: 'static,
    {
        let system_id = self.register_system_cached(system);
        self.run_system_with_input(system_id, input)
    }
}

/// A resource that stores a [`SystemId`] by its function type.
#[derive(Resource)]
struct CachedSystemId<T, In = (), Out = ()>(SystemId<In, Out>, PhantomData<fn() -> T>);
