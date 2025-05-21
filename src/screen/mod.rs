pub mod fade;
mod gameplay;
mod intro;
mod loading;
mod splash;
mod title;

use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;

use crate::core::camera::CameraRoot;
use crate::core::window::WindowReady;
use crate::menu::Menu;
use crate::prelude::*;
use crate::theme::widget::IsLoadingBarFill;

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
                    Name::new("ScreenUi"),
                    Node::DEFAULT.full_size(),
                    Pickable::IGNORE,
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
    Gameplay,
}

impl Configure for Screen {
    fn configure(app: &mut App) {
        app.add_state::<Self>();
        app.add_plugins(ProgressPlugin::<BevyState<Self>>::new());
        app.configure::<IsLoadingBarFill<Self>>();
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
            gameplay::plugin,
        ));
    }
}

fn reset_screen_camera(camera_root: Res<CameraRoot>, mut camera_query: Query<&mut Transform>) {
    let mut transform = r!(camera_query.get_mut(camera_root.primary));
    *transform = default();
}

/// The total time elapsed in the current screen.
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

#[cfg_attr(feature = "native_dev", hot)]
fn reset_screen_time(mut screen_time: ResMut<ScreenTime>) {
    *screen_time = default();
}

#[cfg_attr(feature = "native_dev", hot)]
fn tick_screen_time(time: Res<Time>, mut screen_time: ResMut<ScreenTime>) {
    screen_time.0 += time.delta();
}

//#[cfg_attr(feature = "native_dev", hot)]
fn wait_in_screen(duration: f32) -> ScheduleConfigs<ScheduleSystem> {
    (move |screen_time: Res<ScreenTime>| -> Progress {
        (screen_time.0.as_secs_f32() >= duration).into()
    })
    .track_progress::<BevyState<Screen>>()
}
