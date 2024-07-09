use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use iyes_progress::prelude::*;

use crate::screen::Screen;

pub fn wait(duration: f32) -> SystemConfigs {
    (move |time: Res<Time>,
           next_state: Res<NextState<Screen>>,
           mut start: Local<f32>|
          -> Progress {
        let elapsed = time.elapsed_seconds();
        if *start == 0.0 {
            *start = elapsed;
        }
        let done = elapsed - *start >= duration;

        // Reset timer on any upcoming state change
        // NOTE: What if next_state == Some(current_state)? Or next_state changes
        //   again this frame after this system runs?
        if matches!(*next_state, NextState::Pending(_)) {
            *start = 0.0;
        }

        done.into()
    })
    .track_progress()
}
