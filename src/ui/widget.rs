use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

use crate::core::theme::ThemeBackgroundColor;
use crate::core::theme::ThemeColor;
use crate::core::theme::ThemeTextColors;
use crate::ui::prelude::*;

pub fn menu_button(text: impl Into<String>) -> impl EntityCommand<World> {
    let text = text.into();
    move |mut entity: EntityWorldMut| {
        entity
            .insert((
                Name::new(format!("{}Button", text.replace(' ', ""))),
                ButtonBundle {
                    style: Style {
                        height: Vw(8.0),
                        width: Vw(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                ThemeBackgroundColor(ThemeColor::Invisible),
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
                    FontSize::new(Vw(4.0)).with_step(8.0),
                    ThemeTextColors(vec![ThemeColor::PrimaryText]),
                ));
            });
    }
}
