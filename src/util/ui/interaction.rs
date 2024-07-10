use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::core::theme::ThemeBackgroundColor;
use crate::core::theme::ThemeColor;
use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DefaultPickingPlugins);

    app.configure::<(IsDisabled, InteractionPalette)>();
}

#[derive(Component, Reflect)]
pub struct IsDisabled(pub bool);

impl Configure for IsDisabled {
    fn configure(app: &mut App) {
        app.register_type::<IsDisabled>();
    }
}

// TODO: Text colors
/// The theme color to use for each Interaction state
/// Requires Interaction and ThemeColor components to function
#[derive(Component, Reflect)]
pub struct InteractionPalette {
    pub normal: ThemeColor,
    pub hovered: ThemeColor,
    pub pressed: ThemeColor,
    pub disabled: ThemeColor,
}

impl Configure for InteractionPalette {
    fn configure(app: &mut App) {
        app.register_type::<InteractionPalette>();
        app.add_systems(Update, apply_interaction_palette.in_set(UpdateSet::End));
    }
}

fn apply_interaction_palette(
    mut interaction_query: Query<
        (
            Option<&IsDisabled>,
            &Interaction,
            &InteractionPalette,
            &mut ThemeBackgroundColor,
        ),
        Or<(Changed<Interaction>, Changed<IsDisabled>)>,
    >,
) {
    for (is_disabled, interaction, palette, mut color) in &mut interaction_query {
        color.0 = if matches!(is_disabled, Some(IsDisabled(true))) {
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
