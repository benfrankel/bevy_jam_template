use bevy::ecs::component::Mutable;
use bevy::reflect::GetTypeRegistration;
use bevy::reflect::Typed;

use crate::animation::offset::NodeOffset;
use crate::core::audio::AudioConfig;
use crate::prelude::*;
use crate::theme::ThemeAssets;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        Previous<Interaction>,
        InteractionDisabled,
        InteractionTheme<ThemeColorFor<BackgroundColor>>,
        InteractionTheme<NodeOffset>,
        TargetInteractionTheme<ThemeColorForText>,
        TargetInteractionTheme<NodeOffset>,
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

/// Values to set a component to by interaction state.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Interaction, Previous<Interaction>)]
pub struct InteractionTheme<C: Component<Mutability = Mutable> + Clone> {
    pub none: C,
    pub hovered: C,
    pub pressed: C,
    pub disabled: C,
}

impl<C: Component<Mutability = Mutable> + Clone + Typed + FromReflect + GetTypeRegistration>
    Configure for InteractionTheme<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_interaction_theme::<C>.in_set(UpdateSystems::RecordInput),
        );
    }
}

fn apply_interaction_theme<C: Component<Mutability = Mutable> + Clone>(
    mut interaction_query: Query<
        (
            Option<&InteractionDisabled>,
            &Previous<Interaction>,
            &Interaction,
            &InteractionTheme<C>,
            &mut C,
        ),
        Or<(
            Changed<InteractionDisabled>,
            Changed<Previous<Interaction>>,
            Changed<Interaction>,
        )>,
    >,
) {
    for (is_disabled, previous, current, table, mut value) in &mut interaction_query {
        // Add 1 frame of delay when going from pressed -> hovered.
        cq!(!matches!(
            (previous.0, current),
            (Interaction::Pressed, Interaction::Hovered),
        ));

        // Clone the field corresponding to the current interaction state.
        *value = if matches!(is_disabled, Some(InteractionDisabled(true))) {
            &table.disabled
        } else {
            match current {
                Interaction::None => &table.none,
                Interaction::Hovered => &table.hovered,
                Interaction::Pressed => &table.pressed,
            }
        }
        .clone();
    }
}

/// Values to set a component to by a target entity's interaction state.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TargetInteractionTheme<C: Component<Mutability = Mutable> + Clone> {
    pub target: Entity,
    pub none: C,
    pub hovered: C,
    pub pressed: C,
    pub disabled: C,
}

impl<C: Component<Mutability = Mutable> + Clone + Typed + FromReflect + GetTypeRegistration>
    Configure for TargetInteractionTheme<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_target_interaction_theme::<C>.in_set(UpdateSystems::RecordInput),
        );
    }
}

fn apply_target_interaction_theme<C: Component<Mutability = Mutable> + Clone>(
    mut table_query: Query<(&TargetInteractionTheme<C>, &mut C)>,
    interaction_query: Query<
        (
            Option<&InteractionDisabled>,
            &Previous<Interaction>,
            &Interaction,
        ),
        Or<(
            Changed<InteractionDisabled>,
            Changed<Previous<Interaction>>,
            Changed<Interaction>,
        )>,
    >,
) {
    for (table, mut value) in &mut table_query {
        let (is_disabled, previous, current) = cq!(interaction_query.get(table.target));
        // Add 1 frame of delay when going from pressed -> hovered.
        cq!(!matches!(
            (previous.0, current),
            (Interaction::Pressed, Interaction::Hovered),
        ));

        // Clone the field corresponding to the current interaction state.
        *value = if matches!(is_disabled, Some(InteractionDisabled(true))) {
            &table.disabled
        } else {
            match current {
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
#[require(Previous<Interaction>)]
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
                    .with_playback_rate(thread_rng().gen_range(0.9..1.5));
            },
            // TODO: This plays a sound on mouse down, not on click.
            (_, Interaction::Pressed) => {
                audio
                    .play(assets.sfx_click.clone())
                    .with_volume(4.0 * audio_config.ui_volume)
                    .with_playback_rate(thread_rng().gen_range(0.9..1.5));
            },
            _ => (),
        }
    }
}
