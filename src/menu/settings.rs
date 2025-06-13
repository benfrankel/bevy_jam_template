use bevy_simple_prefs::Prefs;
use bevy_simple_prefs::PrefsPlugin;

use crate::core::audio::AudioSettings;
use crate::menu::Menu;
use crate::menu::MenuRootUi;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Menu::Settings.on_enter(spawn_settings_menu));

    app.configure::<(
        Settings,
        MasterVolumeSelector,
        MusicVolumeSelector,
        SfxVolumeSelector,
        UiVolumeSelector,
    )>();
}

fn spawn_settings_menu(mut commands: Commands, menu_root_ui: Single<Entity, With<MenuRootUi>>) {
    commands
        .entity(*menu_root_ui)
        .with_child(widget::root(children![widget::full_popup(children![
            widget::center(children![
                widget::header(children![widget::h1("[b]Settings")]),
                grid(),
                widget::footer(children![widget::row_of_buttons(children![
                    widget::wide_button("Back", go_back),
                ])]),
            ]),
        ])]));
}

fn go_back(_: Trigger<Pointer<Click>>, mut menu: ResMut<NextStateStack<Menu>>) {
    menu.pop();
}

fn grid() -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
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
            widget::selector(MasterVolumeSelector, master_volume_down, master_volume_up),
            widget::label("Music volume"),
            widget::selector(MusicVolumeSelector, music_volume_down, music_volume_up),
            widget::label("SFX volume"),
            widget::selector(SfxVolumeSelector, sfx_volume_down, sfx_volume_up),
            widget::label("UI volume"),
            widget::selector(UiVolumeSelector, ui_volume_down, ui_volume_up),
        ],
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct MasterVolumeSelector;

impl Configure for MasterVolumeSelector {
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
    selector_query: Query<Entity, With<MasterVolumeSelector>>,
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
struct MusicVolumeSelector;

impl Configure for MusicVolumeSelector {
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
    selector_query: Query<Entity, With<MusicVolumeSelector>>,
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
struct SfxVolumeSelector;

impl Configure for SfxVolumeSelector {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            Menu::Settings.on_update(update_sfx_volume_selector.in_set(UpdateSystems::Update)),
        );
    }
}

fn update_sfx_volume_selector(
    audio_settings: Res<AudioSettings>,
    selector_query: Query<Entity, With<SfxVolumeSelector>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut RichText>,
    mut disabled_query: Query<&mut InteractionDisabled>,
) {
    for entity in &selector_query {
        let children = c!(children_query.get(entity))
            .into_iter()
            .collect::<Vec<_>>();

        let left = **c!(children.first());
        c!(disabled_query.get_mut(left)).0 = audio_settings.sfx_volume <= f32::EPSILON;

        let mid = **c!(children.get(1));
        let mid_children = c!(children_query.get(mid));
        let label = *c!(mid_children.first());
        c!(text_query.get_mut(label)).sections =
            parse_rich(format!("{:.0}%", audio_settings.sfx_volume * 100.0));

        let right = **c!(children.get(2));
        c!(disabled_query.get_mut(right)).0 = audio_settings.sfx_volume >= 1.0 - f32::EPSILON;
    }
}

fn sfx_volume_down(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.sfx_volume = (audio_settings.sfx_volume - 0.1).max(0.0);
}

fn sfx_volume_up(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.sfx_volume = (audio_settings.sfx_volume + 0.1).min(1.0);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct UiVolumeSelector;

impl Configure for UiVolumeSelector {
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
    selector_query: Query<Entity, With<UiVolumeSelector>>,
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
