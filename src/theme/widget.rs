use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::RunSystemOnce as _;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_mod_picking::prelude::*;

use crate::animation::backup::Backup;
use crate::animation::offset::Offset;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub fn overlay(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
        NodeBundle {
            style: Style::DEFAULT.abs().full_size(),
            z_index: ZIndex::Global(1000),
            ..default()
        },
        Pickable::IGNORE,
    ));
}

pub fn blocking_overlay(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert(NodeBundle {
        style: Style::DEFAULT.abs().full_size(),
        focus_policy: FocusPolicy::Block,
        z_index: ZIndex::Global(1000),
        ..default()
    });
}

pub struct MenuButton {
    text: String,
}

impl MenuButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl EntityCommand for MenuButton {
    fn apply(self, id: Entity, world: &mut World) {
        world.run_system_once_with((id, self), menu_button);
    }
}

fn menu_button(In((id, this)): In<(Entity, MenuButton)>, mut commands: Commands) {
    commands
        .entity(id)
        .insert((
            Name::new(format!("MenuButton(\"{}\")", &this.text)),
            ButtonBundle {
                style: Style {
                    height: Vw(11.0),
                    width: Vw(38.0),
                    ..Style::ROW_CENTER
                },
                border_radius: BorderRadius::MAX,
                ..default()
            },
            ThemeColor::default().set::<BackgroundColor>(),
            InteractionTable {
                normal: ThemeColor::Primary.set::<BackgroundColor>(),
                hovered: ThemeColor::PrimaryHovered.set::<BackgroundColor>(),
                pressed: ThemeColor::PrimaryPressed.set::<BackgroundColor>(),
                disabled: ThemeColor::PrimaryDisabled.set::<BackgroundColor>(),
            },
            Offset::default(),
            Backup::<Transform>::default(),
            InteractionTable {
                hovered: Offset(Vec2::new(0.0, -4.0)),
                pressed: Offset(Vec2::new(0.0, 2.0)),
                ..default()
            },
            Old(Interaction::None),
            InteractionSfx,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("ButtonText"),
                TextBundle::from_sections(parse_rich(&this.text)),
                DynamicFontSize::new(Vw(4.0)).with_step(8.0),
                ThemeColorForText(vec![ThemeColor::PrimaryText]),
            ));
        });
}
