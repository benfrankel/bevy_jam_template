pub use crate::animation::transition::FadeIn;
pub use crate::animation::transition::FadeOut;

mod transition;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(transition::plugin);
}
