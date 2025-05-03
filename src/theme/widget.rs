use bevy::ecs::system::IntoObserverSystem;

use crate::animation::offset::Offset;
use crate::prelude::*;

pub fn overlay(z: i32) -> impl Bundle {
    (
        Node::DEFAULT.abs().full_size(),
        Pickable::IGNORE,
        GlobalZIndex(z),
    )
}

pub fn blocking_overlay(z: i32) -> impl Bundle {
    (
        Node::DEFAULT.abs().full_size(),
        FocusPolicy::Block,
        GlobalZIndex(z),
    )
}

fn button<E, B, M, I>(
    width: Val,
    height: Val,
    font_size: Val,
    text: impl Into<String>,
    action: I,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    (
        Name::new(format!("Button(\"{}\")", &text)),
        Button,
        Node {
            width,
            height,
            ..Node::ROW_CENTER
        },
        BorderRadius::MAX,
        ThemeColor::default().set::<BackgroundColor>(),
        InteractionTable {
            none: ThemeColor::Primary.set::<BackgroundColor>(),
            hovered: ThemeColor::PrimaryHovered.set::<BackgroundColor>(),
            pressed: ThemeColor::PrimaryPressed.set::<BackgroundColor>(),
            disabled: ThemeColor::PrimaryDisabled.set::<BackgroundColor>(),
        },
        Offset::default(),
        InteractionTable {
            hovered: Offset(Vec2::new(0.0, -4.0)),
            pressed: Offset(Vec2::new(0.0, 2.0)),
            ..default()
        },
        InteractionSfx,
        Children::spawn((
            Spawn((
                Name::new("ButtonText"),
                RichText::from_sections(parse_rich(&text)),
                DynamicFontSize::new(font_size).with_step(8.0),
                ThemeColorForText(vec![ThemeColor::PrimaryText]),
            )),
            SpawnObserver::new(action),
        )),
    )
}

pub fn big_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button(Vw(38.0), Vw(11.0), Vw(4.0), text, action)
}

pub fn small_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button(Vw(38.0), Vw(7.0), Vw(3.0), text, action)
}
