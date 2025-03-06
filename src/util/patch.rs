pub mod plugin;
pub mod spawn;
pub mod transform;
pub mod trigger;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::plugin::AppExtConfigure as _;
    pub use super::plugin::Configure;
    pub use super::plugin::PluginGroupBuilderExtReplace as _;
    pub use super::spawn::QueueExt as _;
    pub use super::spawn::QueueFnExt as _;
    pub use super::spawn::SpawnExt as _;
    pub use super::spawn::SpawnSystemExt as _;
    pub use super::spawn::WorldSpawnExt as _;
    pub use super::transform::Dir2ExtToQuat as _;
    pub use super::trigger::EntityCommandsExtTrigger as _;
    pub use super::trigger::TriggerExtGetEntity as _;
}
