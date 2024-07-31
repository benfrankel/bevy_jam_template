use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_mod_picking::prelude::*;

use crate::theme::prelude::*;

pub fn overlay(mut entity: EntityWorldMut) {
    entity.insert((
        NodeBundle {
            style: Style::DEFAULT.abs().full_size(),
            z_index: ZIndex::Global(1000),
            ..default()
        },
        Pickable::IGNORE,
    ));
}

pub fn blocking_overlay(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style::DEFAULT.abs().full_size(),
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
                        ..Style::ROW_CENTER
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
