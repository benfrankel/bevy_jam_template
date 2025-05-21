use bevy::ecs::system::IntoObserverSystem;

use crate::animation::backup::Backup;
use crate::animation::offset::NodeOffset;
use crate::core::audio::AudioConfig;
use crate::core::audio::IsMusicAudio;
use crate::core::audio::IsUiAudio;
use crate::prelude::*;

#[tweak_fn]
pub fn overlay(z: i32) -> impl Bundle {
    (
        Name::new("Overlay"),
        Node::DEFAULT.full_size().abs(),
        Pickable::IGNORE,
        GlobalZIndex(z),
    )
}

#[tweak_fn]
pub fn blocking_overlay(z: i32) -> impl Bundle {
    (
        Name::new("BlockingOverlay"),
        Node::DEFAULT.full_size().abs(),
        FocusPolicy::Block,
        GlobalZIndex(z),
    )
}

#[tweak_fn]
pub fn body(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Body"),
        Node {
            display: Display::Block,
            padding: UiRect::all(Vw(3.5)),
            ..Node::DEFAULT.full_size()
        },
        children,
    )
}

#[tweak_fn]
pub fn column_center(children: impl Bundle) -> impl Bundle {
    (
        Name::new("ColumnCenter"),
        Node::COLUMN_CENTER.full_size(),
        children,
    )
}

#[tweak_fn]
pub fn button_column(children: impl Bundle) -> impl Bundle {
    (
        Name::new("ButtonColumn"),
        Node {
            margin: UiRect::vertical(Vw(2.5)),
            row_gap: Vw(2.5),
            ..Node::COLUMN_MID
        },
        children,
    )
}

#[tweak_fn]
pub fn stretch(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Stretch"),
        Node {
            justify_content: JustifyContent::Center,
            flex_grow: 1.0,
            ..Node::ROW
        },
        children,
    )
}

#[tweak_fn]
pub fn header(text: impl AsRef<str>) -> impl Bundle {
    (
        label_helper(Vw(5.0), ThemeColor::BodyText, text),
        Node {
            margin: UiRect::bottom(Vw(5.0)),
            ..default()
        },
    )
}

#[tweak_fn]
pub fn big_label(text: impl AsRef<str>) -> impl Bundle {
    label_helper(Vw(5.0), ThemeColor::BodyText, text)
}

#[tweak_fn]
pub fn label(text: impl AsRef<str>) -> impl Bundle {
    label_helper(Vw(3.5), ThemeColor::BodyText, text)
}

#[tweak_fn]
pub fn paragraph(text: &'static str) -> impl Bundle {
    (
        Name::new("Paragraph"),
        Node {
            margin: UiRect::vertical(Vw(5.0)),
            row_gap: Vw(1.4),
            ..Node::COLUMN_MID
        },
        Children::spawn(SpawnIter(text.lines().map(label))),
    )
}

#[tweak_fn]
fn label_helper(font_size: Val, text_color: ThemeColor, text: impl AsRef<str>) -> impl Bundle {
    let text = text.as_ref();
    (
        Name::new(format!("Label(\"{text}\")")),
        RichText::from_sections(parse_rich(text)).with_justify(JustifyText::Center),
        DynamicFontSize::new(font_size).with_step(8.0),
        ThemeColorForText(vec![text_color]),
    )
}

#[tweak_fn]
pub fn small_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_helper(Vw(3.0), Vw(4.0), Vw(3.0), text, action)
}

#[tweak_fn]
pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_helper(Vw(38.0), Vw(7.0), Vw(3.0), text, action)
}

#[tweak_fn]
pub fn big_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_helper(Vw(38.0), Vw(10.0), Vw(4.0), text, action)
}

#[tweak_fn]
fn button_helper<E, B, M, I>(
    width: Val,
    height: Val,
    font_size: Val,
    text: impl Into<String>,
    action: I,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    (
        Name::new(format!("Button(\"{text}\")")),
        Button,
        Node {
            width,
            height,
            ..Node::ROW_CENTER
        },
        BorderRadius::MAX,
        ThemeColor::default().set::<BackgroundColor>(),
        BoxShadow::from(ShadowStyle {
            color: Color::BLACK.with_alpha(0.5),
            x_offset: Val::ZERO,
            y_offset: Vw(0.7),
            spread_radius: Vw(0.5),
            blur_radius: Vw(0.5),
        }),
        Backup::<BoxShadow>::default(),
        InteractionTheme {
            none: ThemeColor::Primary.set::<BackgroundColor>(),
            hovered: ThemeColor::PrimaryHovered.set::<BackgroundColor>(),
            pressed: ThemeColor::PrimaryPressed.set::<BackgroundColor>(),
            disabled: ThemeColor::PrimaryDisabled.set::<BackgroundColor>(),
        },
        NodeOffset::default(),
        InteractionTheme {
            hovered: NodeOffset::new(Val::ZERO, Vw(-0.5)),
            pressed: NodeOffset::new(Val::ZERO, Vw(0.5)),
            ..default()
        },
        InteractionSfx,
        children![(
            label_helper(font_size, ThemeColor::PrimaryText, text),
            Pickable::IGNORE,
        )],
        Patch(|entity| {
            entity.observe(action);
        }),
    )
}

#[tweak_fn]
pub fn selector<E1, B1, M1, I1, C, E2, B2, M2, I2>(
    left_action: I1,
    label_marker: C,
    right_action: I2,
) -> impl Bundle
where
    E1: Event,
    B1: Bundle,
    I1: Sync + IntoObserverSystem<E1, B1, M1>,
    C: Component,
    E2: Event,
    B2: Bundle,
    I2: Sync + IntoObserverSystem<E2, B2, M2>,
{
    (
        Name::new("Selector"),
        Node {
            width: Vw(35.0),
            ..Node::ROW
        },
        children![
            small_button("<", left_action),
            stretch(children![(label(""), label_marker)]),
            small_button(">", right_action),
        ],
    )
}

#[tweak_fn]
pub fn loading_bar<S: State + Clone + PartialEq + Eq + Hash + Debug>() -> impl Bundle {
    (
        Name::new("LoadingBar"),
        Node {
            width: Percent(60.0),
            height: Vw(4.0),
            margin: UiRect::all(Vw(1.0)),
            padding: UiRect::all(Vw(0.5)),
            border: UiRect::all(Vw(0.5)),
            ..default()
        },
        ThemeColor::BodyText.set::<BorderColor>(),
        children![(
            Name::new("LoadingBarFill"),
            Node::DEFAULT.full_height(),
            ThemeColor::Primary.set::<BackgroundColor>(),
            IsLoadingBarFill::<S>(PhantomData),
        )],
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsLoadingBarFill<S: State + Clone + PartialEq + Eq + Hash + Debug>(
    #[reflect(ignore)] PhantomData<S>,
);

impl<S: State + Clone + PartialEq + Eq + Hash + Debug + TypePath> Configure
    for IsLoadingBarFill<S>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, update_loading_bar_fill::<S>);
    }
}

fn update_loading_bar_fill<S: State + Clone + PartialEq + Eq + Hash + Debug>(
    progress: Res<ProgressTracker<BevyState<S>>>,
    mut fill_query: Query<&mut Node, With<IsLoadingBarFill<S>>>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    for mut node in &mut fill_query {
        node.width = Percent(100.0 * done as f32 / total as f32);
    }
}

#[tweak_fn]
pub fn music_audio(audio_config: &AudioConfig, handle: Handle<AudioSource>) -> impl Bundle {
    (
        Name::new("MusicAudio"),
        AudioPlayer(handle),
        PlaybackSettings::LOOP.with_volume(audio_config.music_volume()),
        IsMusicAudio,
    )
}

#[tweak_fn]
pub fn ui_audio(audio_config: &AudioConfig, handle: Handle<AudioSource>) -> impl Bundle {
    (
        Name::new("UiAudio"),
        AudioPlayer(handle),
        PlaybackSettings::DESPAWN
            .with_volume(audio_config.ui_volume())
            .with_speed(thread_rng().gen_range(0.9..1.5)),
        IsUiAudio,
    )
}
