use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<AudioConfig>>();

    app.add_plugins(AudioPlugin);
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct AudioConfig {
    pub global_volume: f64,
    pub music_volume: f64,
    pub ui_volume: f64,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            global_volume: 0.5,
            music_volume: 0.5,
            ui_volume: 0.5,
        }
    }
}

impl Config for AudioConfig {
    const FILE: &'static str = "audio.ron";

    fn on_load(&mut self, world: &mut World) {
        world.resource::<Audio>().set_volume(self.global_volume);
    }
}
