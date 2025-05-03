use pyri_tooltip::prelude::*;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TooltipPlugin::default());
}
