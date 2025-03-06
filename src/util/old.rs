use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use bevy::reflect::Typed;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Old<Interaction>>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Old<C: Component>(pub C);

impl<C: Component + Clone + Typed + FromReflect + GetTypeRegistration> Configure for Old<C> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(First, update_old::<C>);
    }
}

fn update_old<C: Component + Clone>(mut old_query: Query<(&mut Old<C>, &C)>) {
    for (mut old, target) in &mut old_query {
        old.0 = target.clone();
    }
}
