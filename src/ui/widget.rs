use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::ui::prelude::*;

pub fn column_left(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style {
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    });
}

pub fn column_mid(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style {
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    });
}

pub fn column_right(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style {
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::End,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    });
}

pub fn column_center(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style {
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    });
}

pub fn row_top(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style {
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Start,
            ..default()
        },
        ..default()
    });
}

pub fn row_mid(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style {
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    });
}

pub fn row_bottom(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style {
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::End,
            ..default()
        },
        ..default()
    });
}

pub fn row_center(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style {
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    });
}

pub fn overlay(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },
        focus_policy: FocusPolicy::Block,
        z_index: ZIndex::Global(1000),
        ..default()
    });
}

pub fn menu_button(text: impl Into<String>) -> impl EntityCommand<World> {
    let text = text.into();
    move |mut entity: EntityWorldMut| {
        entity
            .insert((
                Name::new(format!("{}Button", text.replace(' ', ""))),
                ButtonBundle {
                    style: Style {
                        height: Vw(11.0),
                        width: Vw(38.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                ThemeColor::Invisible.set::<BackgroundColor>(),
                InteractionPalette {
                    normal: ThemeColor::Primary,
                    hovered: ThemeColor::PrimaryHovered,
                    pressed: ThemeColor::PrimaryPressed,
                    disabled: ThemeColor::PrimaryDisabled,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("ButtonText"),
                    TextBundle::from_section(
                        text,
                        TextStyle {
                            font: FONT_HANDLE,
                            ..default()
                        },
                    ),
                    DynamicFontSize::new(Vw(4.0)).with_step(8.0),
                    ThemeColorForText(vec![ThemeColor::PrimaryText]),
                ));
            });
    }
}
