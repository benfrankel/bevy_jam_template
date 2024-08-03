use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::prelude::*;
use rand::prelude::*;

use crate::animation::offset::Offset;
use crate::core::UpdateSet;
use crate::theme::prelude::*;
use crate::theme::ThemeAssets;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DefaultPickingPlugins);

    app.configure::<(
        IsDisabled,
        InteractionTable<ThemeColorFor<BackgroundColor>>,
        InteractionTable<TextureAtlas>,
        InteractionTable<Offset>,
        InteractionSfx,
    )>();
}

#[derive(Component, Reflect)]
pub struct IsDisabled(pub bool);

impl Configure for IsDisabled {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

// TODO: Text labels are usually child entities, so this is annoying to implement for text colors.
/// Different values of a component to set for each [`Interaction`] state.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct InteractionTable<C: Component> {
    pub normal: C,
    pub hovered: C,
    pub pressed: C,
    pub disabled: C,
}

impl<C: Component + Clone + Reflect + FromReflect + TypePath + GetTypeRegistration> Configure
    for InteractionTable<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_interaction_table::<C>.in_set(UpdateSet::RecordInput),
        );
    }
}

fn apply_interaction_table<C: Component + Clone>(
    mut interaction_query: Query<
        (
            Option<&IsDisabled>,
            &Interaction,
            &InteractionTable<C>,
            &mut C,
        ),
        Or<(Changed<Interaction>, Changed<IsDisabled>)>,
    >,
) {
    for (is_disabled, interaction, table, mut target) in &mut interaction_query {
        // Clone the component from the current `Interaction` state.
        *target = if matches!(is_disabled, Some(IsDisabled(true))) {
            &table.disabled
        } else {
            match interaction {
                Interaction::None => &table.normal,
                Interaction::Hovered => &table.hovered,
                Interaction::Pressed => &table.pressed,
            }
        }
        .clone();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InteractionSfx;

impl Configure for InteractionSfx {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, play_interaction_sfx.in_set(UpdateSet::RecordInput));
    }
}

fn play_interaction_sfx(
    assets: Res<ThemeAssets>,
    audio: Res<Audio>,
    interaction_query: Query<
        (Option<&IsDisabled>, &Interaction),
        (
            With<InteractionSfx>,
            Or<(Changed<Interaction>, Changed<IsDisabled>)>,
        ),
    >,
) {
    for (is_disabled, interaction) in &interaction_query {
        if matches!(is_disabled, Some(IsDisabled(true))) {
            continue;
        }

        // TODO: Track the previous Interaction to avoid playing the hover SFX on Pressed -> Hovered.
        match interaction {
            Interaction::Hovered => {
                audio
                    .play(assets.sfx_hover.clone())
                    .with_volume(0.6)
                    .with_playback_rate(thread_rng().gen_range(0.7..1.6));
            },
            Interaction::Pressed => {
                audio
                    .play(assets.sfx_click.clone())
                    .with_volume(1.4)
                    .with_playback_rate(thread_rng().gen_range(0.7..1.6));
            },
            _ => (),
        }
    }
}
