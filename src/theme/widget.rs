use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_spawn_observer::SpawnObserver;

use crate::animation::backup::Backup;
use crate::animation::offset::Offset;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub fn overlay(z: i32) -> impl Bundle {
    (
        Node::DEFAULT.abs().full_size(),
        FocusPolicy::Pass,
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

fn button<E, B, M, I>(width: Val, height: Val, text: impl Into<String>, action: I) -> impl Bundle
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
        Backup::<Transform>::default(),
        InteractionTable {
            hovered: Offset(Vec2::new(0.0, -4.0)),
            pressed: Offset(Vec2::new(0.0, 2.0)),
            ..default()
        },
        Old(Interaction::None),
        InteractionSfx,
        Children::spawn((
            Spawn((
                Name::new("ButtonText"),
                RichText::from_sections(parse_rich(&text)),
                DynamicFontSize::new(Vw(4.0)).with_step(8.0),
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
    button(Vw(38.0), Vw(11.0), text, action)
}

pub fn small_button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button(Vw(38.0), Vw(9.0), text, action)
}
