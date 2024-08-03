//! Custom theming and UI tools.

#![allow(dead_code)]

pub mod color;
pub mod interaction;
pub mod layout;
pub mod text;
pub mod tooltip;
pub mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use bevy::ui::Val::*;

    pub use super::color::ThemeColor;
    pub use super::color::ThemeColorFor;
    pub use super::color::ThemeColorForText;
    pub use super::interaction::InteractionSfx;
    pub use super::interaction::InteractionTable;
    pub use super::interaction::IsDisabled;
    pub use super::layout::StyleExtLayout;
    pub use super::text::parse_rich;
    pub use super::text::parse_rich_custom;
    pub use super::text::DynamicFontSize;
    pub use super::text::BOLD_FONT_HANDLE;
    pub use super::text::FONT_HANDLE;
    pub use super::text::THICK_FONT_HANDLE;
    pub use super::widget;
    pub use super::UiRoot;
}

use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ThemeAssets, UiRoot)>();

    app.add_plugins((
        color::plugin,
        interaction::plugin,
        text::plugin,
        tooltip::plugin,
    ));
}

// TODO: Link these assets dynamically via ThemeConfig, plus volume and pitch values.
#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ThemeAssets {
    // CC0 sounds from freesounds.org:
    #[asset(path = "audio/sfx/251390__deadsillyrabbit__button_hover-mp3.ogg")]
    pub sfx_hover: Handle<AudioSource>,
    #[asset(path = "audio/sfx/253168__suntemple__sfx-ui-button-click.ogg")]
    pub sfx_click: Handle<AudioSource>,
}

impl Configure for ThemeAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
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
