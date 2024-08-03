use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use bevy::transform::systems::propagate_transforms;
use bevy::transform::systems::sync_simple_transforms;

use crate::animation::SaveBackupSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Backup<Transform>>();

    // Restore `GlobalTransform` after restoring `Transform`.
    app.add_systems(
        First,
        (sync_simple_transforms, propagate_transforms)
            .chain()
            .after(restore_backup::<Transform>),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Backup<C: Component + Clone>(Option<C>);

impl<C: Component + Clone + Reflect + FromReflect + TypePath + GetTypeRegistration> Configure
    for Backup<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        // This has to run before `UiSystem::Focus` in `PreUpdate` anyways, so may as well
        // go all the way back to `First`.
        app.add_systems(First, restore_backup::<C>);
        app.add_systems(PostUpdate, save_backup::<C>.in_set(SaveBackupSet));
    }
}

fn restore_backup<C: Component + Clone>(mut backup_query: Query<(&mut Backup<C>, &mut C)>) {
    for (mut backup, mut target) in &mut backup_query {
        *target = c!(backup.0.take());
    }
}

fn save_backup<C: Component + Clone>(mut backup_query: Query<(&mut Backup<C>, &C)>) {
    for (mut backup, target) in &mut backup_query {
        backup.0 = Some(target.clone());
    }
}
