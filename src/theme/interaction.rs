use bevy::ecs::component::Mutable;
use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use bevy::reflect::Typed;
use bevy_kira_audio::prelude::*;
use rand::prelude::*;

use crate::animation::offset::Offset;
use crate::core::UpdateSystems;
use crate::core::audio::AudioConfig;
use crate::theme::ThemeAssets;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        InteractionDisabled,
        Previous<Interaction>,
        InteractionTable<ThemeColorFor<BackgroundColor>>,
        InteractionTable<Offset>,
        InteractionSfx,
    )>();
}

#[derive(Component, Reflect)]
pub struct InteractionDisabled(pub bool);

impl Configure for InteractionDisabled {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

// TODO: Text labels are usually child entities, so this is annoying to implement for text colors.
// TODO: Could solve this with a `ParentInteractionTable` component that checks the parent's interaction state.
/// Different values of a component to set for each [`Interaction`] state.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct InteractionTable<C: Component<Mutability = Mutable>> {
    pub none: C,
    pub hovered: C,
    pub pressed: C,
    pub disabled: C,
}

impl<C: Component<Mutability = Mutable> + Clone + Typed + FromReflect + GetTypeRegistration>
    Configure for InteractionTable<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_interaction_table::<C>.in_set(UpdateSystems::RecordInput),
        );
    }
}

fn apply_interaction_table<C: Component<Mutability = Mutable> + Clone>(
    mut interaction_query: Query<
        (
            Option<&InteractionDisabled>,
            &Interaction,
            &InteractionTable<C>,
            &mut C,
        ),
        Or<(Changed<Interaction>, Changed<InteractionDisabled>)>,
    >,
) {
    for (is_disabled, interaction, table, mut target) in &mut interaction_query {
        // Clone the component from the current `Interaction` state.
        *target = if matches!(is_disabled, Some(InteractionDisabled(true))) {
            &table.disabled
        } else {
            match interaction {
                Interaction::None => &table.none,
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
        app.add_systems(
            Update,
            play_interaction_sfx.in_set(UpdateSystems::RecordInput),
        );
    }
}

fn play_interaction_sfx(
    audio_config: ConfigRef<AudioConfig>,
    assets: Res<ThemeAssets>,
    audio: Res<Audio>,
    interaction_query: Query<
        (
            Option<&InteractionDisabled>,
            &Previous<Interaction>,
            &Interaction,
        ),
        (
            With<InteractionSfx>,
            Or<(Changed<Interaction>, Changed<InteractionDisabled>)>,
        ),
    >,
) {
    let audio_config = r!(audio_config.get());

    for (is_disabled, previous, current) in &interaction_query {
        if matches!(is_disabled, Some(InteractionDisabled(true))) {
            continue;
        }

        match (previous.0, current) {
            (Interaction::None, Interaction::Hovered) => {
                audio
                    .play(assets.sfx_hover.clone())
                    .with_volume(2.0 * audio_config.ui_volume)
                    .with_playback_rate(thread_rng().gen_range(0.7..1.6));
            },
            // TODO: This plays a sound on mouse down, not on click.
            (_, Interaction::Pressed) => {
                audio
                    .play(assets.sfx_click.clone())
                    .with_volume(4.0 * audio_config.ui_volume)
                    .with_playback_rate(thread_rng().gen_range(0.7..1.6));
            },
            _ => (),
        }
    }
}
