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
        IsMasterVolumeSelector,
        IsMusicVolumeSelector,
        IsUiVolumeSelector,
    )>();
}

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
            widget::selector(IsMasterVolumeSelector, master_volume_down, master_volume_up),
            widget::label("Music volume"),
            widget::selector(IsMusicVolumeSelector, music_volume_down, music_volume_up),
            widget::label("UI volume"),
            widget::selector(IsUiVolumeSelector, ui_volume_down, ui_volume_up),
        ],
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsMasterVolumeSelector;

impl Configure for IsMasterVolumeSelector {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            Menu::Settings.on_update(update_master_volume_selector.in_set(UpdateSystems::Update)),
        );
    }
}

fn update_master_volume_selector(
    audio_settings: Res<AudioSettings>,
    selector_query: Query<Entity, With<IsMasterVolumeSelector>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut RichText>,
    mut disabled_query: Query<&mut InteractionDisabled>,
) {
    for entity in &selector_query {
        let children = c!(children_query.get(entity))
            .into_iter()
            .collect::<Vec<_>>();

        let left = **c!(children.first());
        c!(disabled_query.get_mut(left)).0 = audio_settings.master_volume <= f32::EPSILON;

        let mid = **c!(children.get(1));
        let mid_children = c!(children_query.get(mid));
        let label = *c!(mid_children.first());
        c!(text_query.get_mut(label)).sections =
            parse_rich(format!("{:.0}%", audio_settings.master_volume * 100.0));

        let right = **c!(children.get(2));
        c!(disabled_query.get_mut(right)).0 = audio_settings.master_volume >= 1.0 - f32::EPSILON;
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
struct IsMusicVolumeSelector;

impl Configure for IsMusicVolumeSelector {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            Menu::Settings.on_update(update_music_volume_selector.in_set(UpdateSystems::Update)),
        );
    }
}

fn update_music_volume_selector(
    audio_settings: Res<AudioSettings>,
    selector_query: Query<Entity, With<IsMusicVolumeSelector>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut RichText>,
    mut disabled_query: Query<&mut InteractionDisabled>,
) {
    for entity in &selector_query {
        let children = c!(children_query.get(entity))
            .into_iter()
            .collect::<Vec<_>>();

        let left = **c!(children.first());
        c!(disabled_query.get_mut(left)).0 = audio_settings.music_volume <= f32::EPSILON;

        let mid = **c!(children.get(1));
        let mid_children = c!(children_query.get(mid));
        let label = *c!(mid_children.first());
        c!(text_query.get_mut(label)).sections =
            parse_rich(format!("{:.0}%", audio_settings.music_volume * 100.0));

        let right = **c!(children.get(2));
        c!(disabled_query.get_mut(right)).0 = audio_settings.music_volume >= 1.0 - f32::EPSILON;
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
struct IsUiVolumeSelector;

impl Configure for IsUiVolumeSelector {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            Menu::Settings.on_update(update_ui_volume_selector.in_set(UpdateSystems::Update)),
        );
    }
}

fn update_ui_volume_selector(
    audio_settings: Res<AudioSettings>,
    selector_query: Query<Entity, With<IsUiVolumeSelector>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut RichText>,
    mut disabled_query: Query<&mut InteractionDisabled>,
) {
    for entity in &selector_query {
        let children = c!(children_query.get(entity))
            .into_iter()
            .collect::<Vec<_>>();

        let left = **c!(children.first());
        c!(disabled_query.get_mut(left)).0 = audio_settings.ui_volume <= f32::EPSILON;

        let mid = **c!(children.get(1));
        let mid_children = c!(children_query.get(mid));
        let label = *c!(mid_children.first());
        c!(text_query.get_mut(label)).sections =
            parse_rich(format!("{:.0}%", audio_settings.ui_volume * 100.0));

        let right = **c!(children.get(2));
        c!(disabled_query.get_mut(right)).0 = audio_settings.ui_volume >= 1.0 - f32::EPSILON;
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
        app.add_plugins(PrefsPlugin::<Settings> {
            #[cfg(feature = "native")]
            path: {
                let path = r!(dirs::config_local_dir()).join(env!("CARGO_PKG_NAME"));
                // Create parent directories if necessary.
                r!(std::fs::create_dir_all(&path).is_ok());
                r!(std::fs::exists(&path));
                path.join("settings.ron")
            },
            ..default()
        });
    }
}
