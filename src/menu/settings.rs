use bevy_simple_prefs::Prefs;
use bevy_simple_prefs::PrefsPlugin;

use crate::core::audio::AudioSettings;
use crate::menu::Menu;
use crate::menu::MenuRoot;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Settings.on_enter(spawn_settings_menu));

    app.configure::<(
        Settings,
        IsMasterVolumeLabel,
        IsMusicVolumeLabel,
        IsUiVolumeLabel,
    )>();
}

#[cfg_attr(feature = "native_dev", hot)]
fn spawn_settings_menu(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .entity(menu_root.ui)
        .with_child(widget::body(children![
            widget::header("[b]Settings"),
            grid(),
            widget::row_of_buttons(children![widget::wide_button("Back", go_back)]),
        ]));
}

fn go_back(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.pop();
}

fn grid() -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            margin: UiRect::vertical(Vw(5.0)),
            row_gap: Vw(1.4),
            column_gap: Vw(6.0),
            grid_template_columns: vec![
                RepeatedGridTrack::flex(1, 1.0),
                RepeatedGridTrack::flex(1, 1.2),
            ],
            ..default()
        },
        GridAlignment::columns([JustifySelf::End, JustifySelf::Start]),
        children![
            widget::label("Master volume"),
            widget::selector(master_volume_down, IsMasterVolumeLabel, master_volume_up),
            widget::label("Music volume"),
            widget::selector(music_volume_down, IsMusicVolumeLabel, music_volume_up),
            widget::label("UI volume"),
            widget::selector(ui_volume_down, IsUiVolumeLabel, ui_volume_up),
        ],
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsMasterVolumeLabel;

impl Configure for IsMasterVolumeLabel {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            Menu::Settings.on_update(update_master_volume_label.in_set(UpdateSystems::Update)),
        );
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn update_master_volume_label(
    audio_settings: Res<AudioSettings>,
    mut text_query: Query<&mut RichText, With<IsMasterVolumeLabel>>,
) {
    for mut text in &mut text_query {
        text.sections = parse_rich(format!("{:.0}%", audio_settings.master_volume * 100.0));
    }
}

fn master_volume_down(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.master_volume = (audio_settings.master_volume - 0.1).max(0.0);
}

fn master_volume_up(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.master_volume = (audio_settings.master_volume + 0.1).min(1.0);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsMusicVolumeLabel;

impl Configure for IsMusicVolumeLabel {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            Menu::Settings.on_update(update_music_volume_label.in_set(UpdateSystems::Update)),
        );
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn update_music_volume_label(
    audio_settings: Res<AudioSettings>,
    mut text_query: Query<&mut RichText, With<IsMusicVolumeLabel>>,
) {
    for mut text in &mut text_query {
        text.sections = parse_rich(format!("{:.0}%", audio_settings.music_volume * 100.0));
    }
}

fn music_volume_down(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.music_volume = (audio_settings.music_volume - 0.1).max(0.0);
}

fn music_volume_up(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.music_volume = (audio_settings.music_volume + 0.1).min(1.0);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsUiVolumeLabel;

impl Configure for IsUiVolumeLabel {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            Menu::Settings.on_update(update_ui_volume_label.in_set(UpdateSystems::Update)),
        );
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn update_ui_volume_label(
    audio_settings: Res<AudioSettings>,
    mut text_query: Query<&mut RichText, With<IsUiVolumeLabel>>,
) {
    for mut text in &mut text_query {
        text.sections = parse_rich(format!("{:.0}%", audio_settings.ui_volume * 100.0));
    }
}

fn ui_volume_down(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.ui_volume = (audio_settings.ui_volume - 0.1).max(0.0);
}

fn ui_volume_up(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.ui_volume = (audio_settings.ui_volume + 0.1).min(1.0);
}

#[derive(Prefs, Reflect, Default)]
struct Settings {
    pub audio_settings: AudioSettings,
}

impl Configure for Settings {
    fn configure(app: &mut App) {
        // Create the config folder if necessary.
        #[cfg(feature = "native")]
        let path = {
            let path = r!(dirs::config_local_dir()).join(env!("CARGO_PKG_NAME"));
            r!(std::fs::create_dir_all(&path).is_ok());
            r!(std::fs::exists(&path));
            path
        };

        // If there were no issues, initialize settings.
        app.add_plugins(PrefsPlugin::<Settings> {
            filename: "settings.ron".to_string(),
            #[cfg(feature = "native")]
            path,
            ..default()
        });
    }
}
