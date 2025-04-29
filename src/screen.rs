pub mod fade;
mod intro;
mod loading;
mod playing;
mod splash;
mod title;

use std::time::Duration;

use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::camera::CameraRoot;
use crate::core::pause::Pause;
use crate::core::window::WindowReady;
use crate::menu::Menu;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub fn plugin(app: &mut App) {
    app.configure::<(ScreenRoot, Screen, ScreenTime)>();

    app.add_plugins(fade::plugin);
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ScreenRoot {
    pub ui: Entity,
}

impl Configure for ScreenRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for ScreenRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            ui: world
                .spawn((
                    Name::new("Screen"),
                    Node::DEFAULT.full_size(),
                    FocusPolicy::Pass,
                    DespawnOnExitState::<Screen>::Descendants,
                ))
                .id(),
        }
    }
}

#[derive(
    State, Copy, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect, Serialize, Deserialize,
)]
#[state(after(WindowReady), before(Menu, Pause), react, bevy_state, log_flush)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Intro,
    Loading,
    Playing,
}

impl Configure for Screen {
    fn configure(app: &mut App) {
        app.add_state::<Self>();
        app.add_plugins(ProgressPlugin::<BevyState<Self>>::new());
        app.add_systems(
            StateFlush,
            (
                WindowReady.on_enter(Screen::enable_default),
                Screen::ANY.on_exit((Pause::disable, Menu::disable, reset_screen_camera)),
            ),
        );
        app.add_plugins((
            splash::plugin,
            title::plugin,
            intro::plugin,
            loading::plugin,
            playing::plugin,
        ));
    }
}

fn reset_screen_camera(camera_root: Res<CameraRoot>, mut camera_query: Query<&mut Transform>) {
    let mut transform = r!(camera_query.get_mut(camera_root.primary));
    *transform = default();
}

// TODO: Screen timer. Then update `wait` to use it.
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ScreenTime(pub Duration);

impl Configure for ScreenTime {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(StateFlush, Screen::ANY.on_exit(reset_screen_time));
        app.add_systems(Update, tick_screen_time.run_if(Screen::is_enabled));
    }
}

fn reset_screen_time(mut screen_time: ResMut<ScreenTime>) {
    *screen_time = default();
}

fn tick_screen_time(time: Res<Time>, mut screen_time: ResMut<ScreenTime>) {
    screen_time.0 += time.delta();
}

fn wait_in_screen(duration: f32) -> ScheduleConfigs<ScheduleSystem> {
    (move |screen_time: Res<ScreenTime>| -> Progress {
        (screen_time.0.as_secs_f32() >= duration).into()
    })
    .track_progress::<BevyState<Screen>>()
}
