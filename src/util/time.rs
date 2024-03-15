use bevy::prelude::*;
use iyes_progress::prelude::*;

use crate::sequence::SequenceState;

pub fn wait(
    duration: f32,
) -> impl FnMut(Res<Time>, Res<NextState<SequenceState>>, Local<f32>) -> Progress {
    move |time: Res<Time>,
          next_state: Res<NextState<SequenceState>>,
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
        if next_state.0.is_some() {
            *start = 0.0;
        }

        done.into()
    }
}
