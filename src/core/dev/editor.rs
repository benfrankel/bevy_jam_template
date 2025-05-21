use bevy_editor_pls::EditorPlugin;
use bevy_editor_pls::EditorWindowPlacement;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<IsEditorWindow>();

    let window = app
        .world_mut()
        .spawn((
            Name::new("EditorWindow"),
            Window {
                title: "bevy_editor_pls".to_string(),
                focused: false,
                visible: false,
                ..default()
            },
            IsEditorWindow,
        ))
        .id();

    app.add_plugins(EditorPlugin {
        window: EditorWindowPlacement::Window(window),
    });
}

const TOGGLE_KEY: KeyCode = KeyCode::F3;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsEditorWindow;

impl Configure for IsEditorWindow {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            toggle_editor_window.run_if(input_just_pressed(TOGGLE_KEY)),
        );
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn toggle_editor_window(mut window_query: Query<&mut Window, With<IsEditorWindow>>) {
    for mut window in &mut window_query {
        window.visible ^= true;
    }
}
