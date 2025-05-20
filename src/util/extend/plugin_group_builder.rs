use bevy::app::PluginGroupBuilder;

use crate::prelude::*;

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/14262>.
pub trait PluginGroupBuilderExtReplace {
    fn replace<Target: Plugin>(self, plugin: impl Plugin) -> Self;
}

impl PluginGroupBuilderExtReplace for PluginGroupBuilder {
    fn replace<Target: Plugin>(self, plugin: impl Plugin) -> Self {
        self.disable::<Target>().add_after::<Target>(plugin)
    }
}
