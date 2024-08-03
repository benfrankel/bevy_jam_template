use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::core::PostTransformSet;
use crate::core::UpdateSet;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(TooltipRoot, Tooltip, TooltipHover)>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct TooltipRoot {
    pub container: Entity,
    pub text: Entity,
}

impl Configure for TooltipRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for TooltipRoot {
    fn from_world(world: &mut World) -> Self {
        let container = world
            .spawn((
                Name::new("Tooltip"),
                NodeBundle {
                    style: Style {
                        max_width: Vw(40.0),
                        padding: UiRect::all(Px(8.0)),
                        ..Style::DEFAULT.abs()
                    },
                    visibility: Visibility::Hidden,
                    z_index: ZIndex::Global(999),
                    ..default()
                },
                ThemeColor::Popup.set::<BackgroundColor>(),
                Selection::default(),
            ))
            .id();

        Self {
            container,
            text: world
                .spawn((
                    Name::new("TooltipText"),
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font: FONT_HANDLE,
                            ..default()
                        },
                    )
                    .with_text_justify(JustifyText::Center),
                    // TODO: Adjustable font sizes in ThemeConfig
                    DynamicFontSize::new(Px(16.0)),
                    ThemeColorForText(vec![ThemeColor::BodyText]),
                ))
                .set_parent(container)
                .id(),
        }
    }
}

// TODO: Support a delay before showing tooltip.
// TODO: Clamp within window by default.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Tooltip {
    pub text: String,
    pub self_anchor: Anchor,
    pub tooltip_anchor: Anchor,
    // TODO: Val?
    pub offset: Vec2,
}

impl Configure for Tooltip {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            (
                update_tooltip_selection,
                update_tooltip_visibility,
                update_tooltip_text,
            )
                .in_set(UpdateSet::SyncLate)
                .run_if(on_event::<TooltipHover>()),
        );
        app.add_systems(
            PostUpdate,
            update_tooltip_position
                .in_set(PostTransformSet::Blend)
                .run_if(on_event::<TooltipHover>()),
        );
    }
}

fn update_tooltip_selection(
    mut events: EventReader<TooltipHover>,
    tooltip_root: Res<TooltipRoot>,
    mut tooltip_query: Query<&mut Selection>,
) {
    let event = r!(events.read().last());
    let mut selection = r!(tooltip_query.get_mut(tooltip_root.container));

    selection.0 = event.0.unwrap_or(Entity::PLACEHOLDER);
}

fn update_tooltip_visibility(
    mut events: EventReader<TooltipHover>,
    tooltip_root: Res<TooltipRoot>,
    mut tooltip_query: Query<&mut Visibility>,
) {
    let event = r!(events.read().last());
    let mut visibility = r!(tooltip_query.get_mut(tooltip_root.container));

    *visibility = match event.0 {
        Some(_) => Visibility::Inherited,
        None => Visibility::Hidden,
    };
}

fn update_tooltip_text(
    mut events: EventReader<TooltipHover>,
    selected_query: Query<&Tooltip>,
    tooltip_root: Res<TooltipRoot>,
    mut tooltip_query: Query<&mut Text>,
) {
    let event = r!(events.read().last());
    let entity = rq!(event.0);
    let tooltip = r!(selected_query.get(entity));
    let mut text = r!(tooltip_query.get_mut(tooltip_root.text));

    text.sections[0].value.clone_from(&tooltip.text);
}

fn update_tooltip_position(
    mut events: EventReader<TooltipHover>,
    selected_query: Query<(&Tooltip, &GlobalTransform, &Node)>,
    tooltip_root: Res<TooltipRoot>,
    mut tooltip_query: Query<(&mut Style, &mut Transform, &GlobalTransform, &Node)>,
) {
    let event = r!(events.read().last());
    let entity = rq!(event.0);
    let (tooltip, selected_gt, selected_node) = r!(selected_query.get(entity));
    let (mut style, mut transform, gt, node) = r!(tooltip_query.get_mut(tooltip_root.container));

    // Convert `self_anchor` to a window-space offset.
    let self_rect = selected_node.logical_rect(selected_gt);
    let self_anchor = self_rect.size() * tooltip.self_anchor.as_vec();

    // Convert `tooltip_anchor` to a window-space offset.
    let tooltip_rect = node.logical_rect(gt);
    let tooltip_anchor = tooltip_rect.size() * tooltip.tooltip_anchor.as_vec();

    // Calculate the combined anchor (adjusted by bonus offset).
    let anchor = tooltip_anchor - self_anchor + tooltip.offset;

    // Convert to absolute position.
    let center = self_rect.center() + anchor;
    let top_left = center - tooltip_rect.half_size();
    style.top = Px(top_left.y);
    style.left = Px(top_left.x);

    // This system has to run after `UiSystem::Layout` so that its size is calculated
    // from the updated text. However, that means that `Style` positioning will be
    // delayed by 1 frame. As a workaround, update the `Transform` directly as well.
    transform.translation.x = center.x;
    transform.translation.y = center.y;
}

/// A buffered event sent when an entity with tooltip is hovered.
#[derive(Event)]
struct TooltipHover(Option<Entity>);

impl Configure for TooltipHover {
    fn configure(app: &mut App) {
        app.add_event::<TooltipHover>();
        app.add_systems(Update, detect_tooltip_hover.in_set(UpdateSet::RecordInput));
    }
}

fn detect_tooltip_hover(
    mut events: EventWriter<TooltipHover>,
    tooltip_root: Res<TooltipRoot>,
    tooltip_query: Query<&Selection>,
    interaction_query: Query<
        (Entity, &Interaction),
        (With<Tooltip>, With<GlobalTransform>, With<Node>),
    >,
) {
    let selection = r!(tooltip_query.get(tooltip_root.container));

    // TODO: Sorting by `ZIndex` would be nice.
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::None) {
            continue;
        }

        // Hovering something new: Update tooltip.
        if selection.0 != entity {
            events.send(TooltipHover(Some(entity)));
        }
        return;
    }

    // Not hovering anything: Hide tooltip.
    events.send(TooltipHover(None));
}
