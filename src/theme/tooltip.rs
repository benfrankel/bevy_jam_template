use bevy::prelude::*;
use pyri_tooltip::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TooltipPlugin::default());
}
