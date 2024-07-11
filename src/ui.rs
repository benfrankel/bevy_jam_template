//! Custom UI tools.

#![allow(dead_code, unused_imports)]

pub mod font;
pub mod interaction;
pub mod tooltip;

pub mod prelude {
    pub use bevy::ui::Val::*;

    pub use super::font::FontSize;
    pub use super::font::BOLD_FONT_HANDLE;
    pub use super::font::FONT_HANDLE;
    pub use super::font::THICK_FONT_HANDLE;
    pub use super::interaction::InteractionPalette;
    pub use super::UiRoot;
}

use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_mod_picking::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<UiRoot>();

    app.add_plugins((font::plugin, interaction::plugin, tooltip::plugin));
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct UiRoot {
    pub body: Entity,
}

impl Configure for UiRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for UiRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            body: world
                .spawn((
                    Name::new("Ui"),
                    NodeBundle {
                        style: Style {
                            width: Percent(100.0),
                            height: Percent(100.0),
                            ..default()
                        },
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .id(),
        }
    }
}
