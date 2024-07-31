use std::borrow::Cow;

use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy::ui::Val::*;

pub trait StyleExtLayout {
    const ROW: Self;
    const ROW_TOP: Self;
    const ROW_MID: Self;
    const ROW_BOTTOM: Self;
    const ROW_CENTER: Self;

    const COLUMN: Self;
    const COLUMN_LEFT: Self;
    const COLUMN_MID: Self;
    const COLUMN_RIGHT: Self;
    const COLUMN_CENTER: Self;

    fn abs(self) -> Self;

    fn full_width(self) -> Self;

    fn full_height(self) -> Self;

    fn full_size(self) -> Self;

    fn node(self, name: impl Into<Cow<'static, str>>) -> (Name, NodeBundle);
}

impl StyleExtLayout for Style {
    const ROW: Self = Self::DEFAULT;

    const ROW_TOP: Self = {
        let mut style = Style::ROW;
        style.align_items = AlignItems::Start;
        style
    };

    const ROW_MID: Self = {
        let mut style = Style::ROW;
        style.align_items = AlignItems::Center;
        style
    };

    const ROW_BOTTOM: Self = {
        let mut style = Style::ROW;
        style.align_items = AlignItems::End;
        style
    };

    const ROW_CENTER: Self = {
        let mut style = Style::ROW;
        style.align_items = AlignItems::Center;
        style.justify_content = JustifyContent::Center;
        style
    };

    const COLUMN: Self = {
        let mut style = Self::DEFAULT;
        style.flex_direction = FlexDirection::Column;
        style
    };

    const COLUMN_LEFT: Self = {
        let mut style = Style::COLUMN;
        style.align_items = AlignItems::Start;
        style
    };

    const COLUMN_MID: Self = {
        let mut style = Style::COLUMN;
        style.align_items = AlignItems::Center;
        style
    };

    const COLUMN_RIGHT: Self = {
        let mut style = Style::COLUMN;
        style.align_items = AlignItems::End;
        style
    };

    const COLUMN_CENTER: Self = {
        let mut style = Style::COLUMN;
        style.align_items = AlignItems::Center;
        style.justify_content = JustifyContent::Center;
        style
    };

    fn abs(mut self) -> Self {
        self.position_type = PositionType::Absolute;
        self
    }

    fn full_width(mut self) -> Self {
        self.width = Percent(100.0);
        self
    }

    fn full_height(mut self) -> Self {
        self.height = Percent(100.0);
        self
    }

    fn full_size(self) -> Self {
        self.full_width().full_height()
    }

    fn node(self, name: impl Into<Cow<'static, str>>) -> (Name, NodeBundle) {
        (
            Name::new(name),
            NodeBundle {
                style: self,
                ..default()
            },
        )
    }
}
