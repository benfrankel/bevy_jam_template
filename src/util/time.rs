use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use iyes_progress::prelude::*;

// TODO: This won't work on screen re-entry with pyri_state.
//       Rewrite this abstraction.
pub fn wait(duration: f32) -> SystemConfigs {
    (move |time: Res<Time>, mut start: Local<f32>| -> Progress {
        let elapsed = time.elapsed_seconds();
        if *start == 0.0 {
            *start = elapsed;
        }
        let done = elapsed - *start >= duration;

        done.into()
    })
    .track_progress()
}
