use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let container = app
        .world_mut()
        .spawn((
            Name::new("PrimaryTooltip"),
            Node {
                padding: UiRect::all(Vw(1.0)),
                ..Node::DEFAULT.abs()
            },
            ThemeColor::Popup.set::<BackgroundColor>(),
            BoxShadow(vec![ShadowStyle {
                color: Color::BLACK.with_alpha(0.5),
                x_offset: Val::ZERO,
                y_offset: Val::ZERO,
                spread_radius: Vw(0.5),
                blur_radius: Vw(0.5),
            }]),
            Visibility::Hidden,
            GlobalZIndex(999),
            Pickable::IGNORE,
        ))
        .id();

    let text = app
        .world_mut()
        .spawn((
            Name::new("Text"),
            RichText::default(),
            DynamicFontSize::new(Vw(2.0)).with_step(8.0),
            Node::default(),
            Pickable::IGNORE,
            ChildOf(container),
        ))
        .id();

    app.add_plugins(TooltipPlugin {
        container,
        text,
        ..default()
    });
}
