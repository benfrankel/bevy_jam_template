// Disable console on windows for release builds
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(bevy_jam_template::plugin).run();
}
