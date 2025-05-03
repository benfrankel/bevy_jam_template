//! Self-contained, re-usable utilities that are not specific to this game.

#![allow(dead_code)]

pub mod config;
pub mod late_commands;
pub mod patch;
pub mod previous;
pub mod selection;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::config::Config;
    pub use super::config::ConfigHandle;
    pub use super::config::ConfigRef;
    pub use super::late_commands::LateCommands;
    pub use super::patch::prelude::*;
    pub use super::previous::Previous;
    pub use super::selection::Selection;
}

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((late_commands::plugin, selection::plugin));
}
