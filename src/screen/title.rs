use crate::core::audio::music_sample;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Title.bevy()).load_collection::<GameplayAssets>(),
    );
    app.add_systems(
        StateFlush,
        Screen::Title.on_enter((Menu::Main.enter(), spawn_title_screen)),
    );

    app.configure::<TitleAssets>();
}

fn spawn_title_screen(mut commands: Commands, assets: Res<TitleAssets>) {
    commands.spawn((
        music_sample(assets.music.clone()),
        DespawnOnExitState::<Screen>::Recursive,
    ));
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TitleAssets {
    #[asset(path = "audio/music/240376__edtijo__happy-8bit-pixel-adenture.ogg")]
    music: Handle<Sample>,
}

impl Configure for TitleAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}
