use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14261.
pub trait Configure {
    fn configure(app: &mut App);
}

macro_rules! impl_configure {
    ($($T:ident),*)  => {
        impl<$($T: Configure),*> Configure for ($($T,)*) {
            fn configure(app: &mut App) {
                $($T::configure(app);)*
                let _ = app;
            }
        }
    }
}

bevy::utils::all_tuples!(impl_configure, 0, 15, T);

pub trait AppExtConfigure {
    fn configure<T: Configure>(&mut self) -> &mut Self;
}

impl AppExtConfigure for App {
    fn configure<T: Configure>(&mut self) -> &mut Self {
        T::configure(self);
        self
    }
}

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14262.
pub trait PluginGroupBuilderExtReplace {
    fn replace<Target: Plugin>(self, plugin: impl Plugin) -> Self;
}

impl PluginGroupBuilderExtReplace for PluginGroupBuilder {
    fn replace<Target: Plugin>(self, plugin: impl Plugin) -> Self {
        self.disable::<Target>().add_after::<Target>(plugin)
    }
}
