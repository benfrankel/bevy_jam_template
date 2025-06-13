use bevy::ecs::system::IntoObserverSystem;
use bevy::text::FontSmoothing;
use bevy::text::LineHeight;

use crate::animation::backup::Backup;
use crate::animation::offset::NodeOffset;
use crate::prelude::*;

pub fn overlay(z: i32) -> impl Bundle {
    (
        Name::new("Overlay"),
        Node::DEFAULT.full_size().abs(),
        Pickable::IGNORE,
        GlobalZIndex(z),
    )
}

pub fn blocking_overlay(z: i32) -> impl Bundle {
    (
        Name::new("BlockingOverlay"),
        Node::DEFAULT.full_size().abs(),
        FocusPolicy::Block,
        GlobalZIndex(z),
    )
}

pub fn spacer(space: Val) -> impl Bundle {
    (Name::new("Spacer"), Node::DEFAULT.size(space, space))
}

pub fn root(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Root"),
        Node {
            padding: UiRect::all(Vw(4.0)),
            ..Node::COLUMN.full_size()
        },
        children,
    )
}

pub fn header(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Header"),
        Node {
            margin: UiRect::bottom(Vw(4.0)),
            ..Node::ROW.center().full_width()
        },
        children,
    )
}

pub fn body(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Body"),
        Node::COLUMN.center_left().full_width().grow(),
        children,
    )
}

pub fn footer(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Footer"),
        Node {
            margin: UiRect::top(Vw(4.0)),
            ..Node::ROW.center().full_width()
        },
        children,
    )
}

pub fn full_popup(children: impl Bundle) -> impl Bundle {
    popup(Percent(100.0), Percent(100.0), children)
}

pub fn popup(width: Val, height: Val, children: impl Bundle) -> impl Bundle {
    (
        Name::new("Popup"),
        Node {
            padding: UiRect::all(Vw(3.0)),
            border: UiRect::all(Px(1.0)),
            ..Node::COLUMN.size(width, height)
        },
        ThemeColor::Popup.set::<BackgroundColor>(),
        BorderRadius::all(Vw(3.0)),
        ThemeColor::PopupBorder.set::<BorderColor>(),
        BoxShadow::from(ShadowStyle {
            color: Color::BLACK.with_alpha(0.5),
            x_offset: Val::ZERO,
            y_offset: Val::ZERO,
            spread_radius: Val::ZERO,
            blur_radius: Val::Vw(4.0),
        }),
        FocusPolicy::Block,
        children,
    )
}

pub fn stretch(children: impl Bundle) -> impl Bundle {
    (Name::new("Stretch"), Node::ROW.center().grow(), children)
}

pub fn center(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Center"),
        Node::COLUMN.center().full_size(),
        children,
    )
}

pub fn column_of_buttons(children: impl Bundle) -> impl Bundle {
    (
        Name::new("ColumnOfButtons"),
        Node {
            row_gap: Vw(2.5),
            ..Node::COLUMN.center()
        },
        children,
    )
}

pub fn row_of_buttons(children: impl Bundle) -> impl Bundle {
    (
        Name::new("RowOfButtons"),
        Node {
            column_gap: Vw(2.5),
            ..Node::ROW.center()
        },
        children,
    )
}

pub fn h1(text: impl AsRef<str>) -> impl Bundle {
    label_base(
        Vw(5.0),
        1.2,
        JustifyText::Center,
        ThemeColor::BodyText,
        text,
    )
}

pub fn big_label(text: impl AsRef<str>) -> impl Bundle {
    label_base(Vw(5.0), 1.2, JustifyText::Left, ThemeColor::BodyText, text)
}

pub fn label(text: impl AsRef<str>) -> impl Bundle {
    label_base(Vw(3.5), 1.2, JustifyText::Left, ThemeColor::BodyText, text)
}

pub fn label_base(
    font_size: Val,
    line_height: f32,
    justify: JustifyText,
    text_color: ThemeColor,
    text: impl AsRef<str>,
) -> impl Bundle {
    let text = text.as_ref();
    let rich_text = RichText::from_sections(parse_rich(text))
        .with_justify(justify)
        .with_font_smoothing(FontSmoothing::None)
        .with_line_height(LineHeight::RelativeToFont(line_height));
    let text_colors =
        std::iter::repeat_n(text_color, rich_text.sections.len().max(1)).collect::<Vec<_>>();

    (
        Name::new(format!("Label(\"{text}\")")),
        rich_text,
        DynamicFontSize::new(font_size).with_step(8.0),
        ThemeColorForText(text_colors),
    )
}

pub fn small_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_base(Vw(3.0), Vw(4.0), Vw(3.0), text, action)
}

pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_base(Vw(30.0), Vw(7.0), Vw(3.0), text, action)
}

pub fn wide_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_base(Vw(38.0), Vw(7.0), Vw(3.0), text, action)
}

pub fn big_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    button_base(Vw(38.0), Vw(10.0), Vw(4.0), text, action)
}

pub fn button_base<E, B, M, I>(
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
        Node::ROW.center().size(width, height),
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
            label_base(
                font_size,
                1.2,
                JustifyText::Center,
                ThemeColor::PrimaryText,
                text
            ),
            Pickable::IGNORE,
        )],
        Patch(|entity| {
            entity.observe(action);
        }),
    )
}

pub fn selector<E1, B1, M1, I1, C, E2, B2, M2, I2>(
    marker: C,
    left_action: I1,
    right_action: I2,
) -> impl Bundle
where
    C: Component,
    E1: Event,
    B1: Bundle,
    I1: Sync + IntoObserverSystem<E1, B1, M1>,
    E2: Event,
    B2: Bundle,
    I2: Sync + IntoObserverSystem<E2, B2, M2>,
{
    (
        Name::new("Selector"),
        Node::ROW.width(Vw(35.0)),
        marker,
        children![
            (small_button("<", left_action), InteractionDisabled(false)),
            stretch(children![label("")]),
            (small_button(">", right_action), InteractionDisabled(false)),
        ],
    )
}

pub fn loading_bar<S: State + Clone + PartialEq + Eq + Hash + Debug>() -> impl Bundle {
    (
        Name::new("LoadingBar"),
        Node {
            padding: UiRect::all(Vw(0.5)),
            border: UiRect::all(Vw(0.5)),
            ..Node::DEFAULT.size(Percent(60.0), Vw(4.0))
        },
        ThemeColor::BodyText.set::<BorderColor>(),
        children![(
            Name::new("LoadingBarFill"),
            LoadingBarFill::<S>(PhantomData),
            Node::DEFAULT.full_height(),
            ThemeColor::Primary.set::<BackgroundColor>(),
        )],
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LoadingBarFill<S: State + Clone + PartialEq + Eq + Hash + Debug>(
    #[reflect(ignore)] PhantomData<S>,
);

impl<S: State + Clone + PartialEq + Eq + Hash + Debug + TypePath> Configure for LoadingBarFill<S> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            update_loading_bar_fill::<S>.in_set(UpdateSystems::SyncLate),
        );
    }
}

fn update_loading_bar_fill<S: State + Clone + PartialEq + Eq + Hash + Debug>(
    progress: Res<ProgressTracker<BevyState<S>>>,
    mut fill_query: Query<&mut Node, With<LoadingBarFill<S>>>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();

    for mut node in &mut fill_query {
        node.width = Percent(100.0 * done as f32 / total as f32);
    }
}
