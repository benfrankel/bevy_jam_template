use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().with_length_unit(PIXELS_PER_METER));
    app.insert_resource(Gravity::ZERO);
    app.insert_resource(DefaultFriction(Friction::ZERO));

    app.add_systems(StateFlush, Pause.on_edge(unpause_physics, pause_physics));
}

const PIXELS_PER_METER: f32 = 16.0;

fn unpause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.unpause();
}

fn pause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.pause();
}
