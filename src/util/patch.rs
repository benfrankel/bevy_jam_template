pub mod plugin;
pub mod spawn;
pub mod transform;
pub mod trigger;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::plugin::AppExtConfigure as _;
    pub use super::plugin::Configure;
    pub use super::plugin::PluginGroupBuilderExtReplace as _;
    pub use super::spawn::AddFnExt as _;
    pub use super::spawn::CommandsExtRunSystemOnce as _;
    pub use super::spawn::EntityWorldMutExtAdd as _;
    pub use super::spawn::EntityWorldMutExtRunSystemOnce as _;
    pub use super::spawn::SpawnWithExt as _;
    pub use super::spawn::WorldSpawnWithExt as _;
    pub use super::transform::Dir2ExtToQuat as _;
    pub use super::trigger::EntityCommandsExtTrigger as _;
    pub use super::trigger::TriggerExtGetEntity as _;
}
