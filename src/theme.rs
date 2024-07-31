//! Custom theming and UI tools.

#![allow(dead_code, unused_imports)]

pub mod color;
pub mod interaction;
pub mod text;
pub mod tooltip;
pub mod widget;

pub mod prelude {
    pub use bevy::ui::Val::*;

    pub use super::color::ThemeColor;
    pub use super::color::ThemeColorFor;
    pub use super::color::ThemeColorForText;
    pub use super::interaction::*;
    pub use super::text::*;
    pub use super::widget;
    pub use super::UiRoot;
}

use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_mod_picking::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<UiRoot>();

    app.add_plugins((
        color::plugin,
        interaction::plugin,
        text::plugin,
        tooltip::plugin,
    ));
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
