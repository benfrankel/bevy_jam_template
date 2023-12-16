use bevy::prelude::*;

use crate::theme::ThemeColor;
use crate::AppSet;

pub struct InteractionPalettePlugin;

impl Plugin for InteractionPalettePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Disabled>()
            .register_type::<InteractionPalette>()
            .add_systems(Update, apply_interaction_palette.in_set(AppSet::End));
    }
}

#[derive(Component, Reflect)]
pub struct Disabled(pub bool);

// TODO: Text colors
/// The theme color to use in each Interaction state
/// Requires Interaction and ThemeColor components to function
#[derive(Component, Reflect)]
pub struct InteractionPalette {
    pub normal: ThemeColor,
    pub hovered: ThemeColor,
    pub pressed: ThemeColor,
    pub disabled: ThemeColor,
}

fn apply_interaction_palette(
    mut interaction_query: Query<
        (
            &Interaction,
            &InteractionPalette,
            Option<&Disabled>,
            &mut ThemeColor,
        ),
        Or<(Changed<Interaction>, Changed<Disabled>)>,
    >,
) {
    for (interaction, palette, disabled, mut color) in &mut interaction_query {
        *color = if matches!(disabled, Some(Disabled(true))) {
            palette.disabled
        } else {
            match interaction {
                Interaction::None => palette.normal,
                Interaction::Hovered => palette.hovered,
                Interaction::Pressed => palette.pressed,
            }
        }
    }
}
