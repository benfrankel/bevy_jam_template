#![allow(unused_imports)]
#![allow(dead_code)]

pub use font::parse_rich;
pub use font::parse_rich_custom;
pub use font::FontSize;
pub use font::BOLD_FONT_HANDLE;
pub use font::FONT_HANDLE;
pub use font::THICK_FONT_HANDLE;
pub use interaction::InteractionPalette;
pub use interaction::IsDisabled;
pub use tooltip::Tooltip;
pub use tooltip::TooltipSide;

mod font;
mod interaction;
mod tooltip;

use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_mod_picking::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiRoot>().init_resource::<UiRoot>();

        app.add_plugins((
            font::FontPlugin,
            interaction::InteractionPlugin,
            tooltip::TooltipPlugin,
        ));
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct UiRoot {
    pub body: Entity,
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
