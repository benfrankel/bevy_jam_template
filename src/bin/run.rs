// Disable console on windows for release builds
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_jam_template::AppPlugin;

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
