use bevy::window::ExitCondition;
use bevy::window::PresentMode;
use bevy::window::WindowResolution;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(WindowPlugin {
        primary_window: Some(Window {
            name: Some("bevy_app".to_string()),
            title: "Pyri New Jam".to_string(),
            present_mode: PresentMode::AutoVsync,
            resolution: WindowResolution::new(960.0, 540.0),
            fit_canvas_to_parent: true,
            ..default()
        }),
        exit_condition: ExitCondition::OnPrimaryClosed,
        ..default()
    });
}
