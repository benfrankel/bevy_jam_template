use bevy::ecs::component::Mutable;
use bevy::reflect::GetTypeRegistration;
use bevy::reflect::Typed;
use bevy::transform::systems::mark_dirty_trees;
use bevy::transform::systems::propagate_parent_transforms;
use bevy::transform::systems::sync_simple_transforms;

use crate::animation::SaveBackupSystems;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Backup<Transform>, Backup<BoxShadow>)>();

    // Restore `GlobalTransform` after restoring `Transform`.
    app.add_systems(
        First,
        (
            mark_dirty_trees,
            sync_simple_transforms,
            propagate_parent_transforms,
        )
            .chain()
            .after(restore_from_backup::<Transform>),
    );
}

/// Saves the pre-animation value of another component to be restored next frame.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Backup<C: Component<Mutability = Mutable> + Clone>(Option<C>);

impl<C: Component<Mutability = Mutable> + Clone + Typed + FromReflect + GetTypeRegistration>
    Configure for Backup<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        // This has to run before `UiSystem::Focus` in `PreUpdate` anyways, so may as well
        // go all the way back to `First`.
        app.add_systems(First, restore_from_backup::<C>);
        app.add_systems(PostUpdate, save_to_backup::<C>.in_set(SaveBackupSystems));
    }
}

fn restore_from_backup<C: Component<Mutability = Mutable> + Clone>(
    mut backup_query: Query<(&mut Backup<C>, &mut C)>,
) {
    for (mut backup, mut target) in &mut backup_query {
        *target = c!(backup.0.take());
    }
}

fn save_to_backup<C: Component<Mutability = Mutable> + Clone>(
    mut backup_query: Query<(&mut Backup<C>, &C)>,
) {
    for (mut backup, target) in &mut backup_query {
        backup.0 = Some(target.clone());
    }
}
