use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy::window::PrimaryWindow;
use serde::Deserialize;
use serde::Serialize;

use crate::config::Config;
use crate::theme::ThemeColor;
use crate::ui::FontSize;
use crate::ui::FONT_HANDLE;
use crate::AppRoot;
use crate::AppSet;

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tooltip>()
            .add_systems(Startup, spawn_tooltip)
            .add_systems(Update, show_tooltip_on_hover.in_set(AppSet::Update));
    }
}

fn spawn_tooltip(mut commands: Commands, mut root: ResMut<AppRoot>) {
    root.tooltip = commands
        .spawn((
            Name::new("Tooltip"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    max_width: Vw(40.0),
                    padding: UiRect::all(Px(8.0)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(1000),
                ..default()
            },
            ThemeColor::Popup,
        ))
        .id();

    root.tooltip_text = commands
        .spawn((
            Name::new("TooltipText"),
            TextBundle::from_section(
                "",
                TextStyle {
                    font: FONT_HANDLE,
                    ..default()
                },
            ),
            // TODO: Adjustable font sizes in ThemeConfig
            FontSize::new(Px(16.0)),
            ThemeColor::BodyText,
        ))
        .set_parent(root.tooltip)
        .id();
}

#[derive(Reflect)]
pub enum TooltipSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component, Reflect)]
pub struct Tooltip {
    pub text: String,
    pub side: TooltipSide,
    // TODO: Val
    pub offset: Vec2,
}

fn show_tooltip_on_hover(
    root: Res<AppRoot>,
    window_query: Query<&Window>,
    mut tooltip_query: Query<(&mut Visibility, &mut Style)>,
    mut tooltip_text_query: Query<&mut Text>,
    interaction_query: Query<(&Interaction, &Tooltip, &GlobalTransform, &Node)>,
) {
    let Ok(window) = window_query.get(root.window) else {
        return;
    };
    let Ok((mut tooltip_visibility, mut tooltip_style)) = tooltip_query.get_mut(root.tooltip)
    else {
        return;
    };
    let Ok(mut tooltip_text) = tooltip_text_query.get_mut(root.tooltip_text) else {
        return;
    };

    for (interaction, tooltip, gt, node) in &interaction_query {
        if matches!(interaction, Interaction::None) {
            *tooltip_visibility = Visibility::Hidden;
            continue;
        }

        let rect = node.logical_rect(gt);

        let width = window.width();
        let height = window.height();
        let (left, right, top, bottom) = (
            rect.min.x + tooltip.offset.x,
            rect.max.x + tooltip.offset.x,
            rect.min.y + tooltip.offset.y,
            rect.max.y + tooltip.offset.y,
        );

        *tooltip_visibility = Visibility::Inherited;
        tooltip_text.sections[0].value = tooltip.text.clone();
        (
            tooltip_style.left,
            tooltip_style.right,
            tooltip_style.top,
            tooltip_style.bottom,
        ) = match tooltip.side {
            TooltipSide::Left => (Auto, Px(width - left), Auto, Px(height - bottom)),
            TooltipSide::Right => (Px(right), Auto, Auto, Px(height - bottom)),
            TooltipSide::Top => (Px(left), Auto, Auto, Px(height - top)),
            TooltipSide::Bottom => (Px(left), Auto, Px(bottom), Auto),
        };
        return;
    }
}
