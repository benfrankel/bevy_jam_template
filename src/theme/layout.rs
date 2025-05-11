use crate::prelude::*;

pub trait NodeExtLayout {
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

    fn width(self, percent: f32) -> Self;

    fn full_width(self) -> Self;

    fn height(self, percent: f32) -> Self;

    fn full_height(self) -> Self;

    fn full_size(self) -> Self;

    fn centered(self) -> Self;
}

impl NodeExtLayout for Node {
    const ROW: Self = Self::DEFAULT;

    const ROW_TOP: Self = {
        let mut x = Self::ROW;
        x.align_items = AlignItems::Start;
        x
    };

    const ROW_MID: Self = {
        let mut x = Self::ROW;
        x.align_items = AlignItems::Center;
        x
    };

    const ROW_BOTTOM: Self = {
        let mut x = Self::ROW;
        x.align_items = AlignItems::End;
        x
    };

    const ROW_CENTER: Self = {
        let mut x = Self::ROW;
        x.align_items = AlignItems::Center;
        x.justify_content = JustifyContent::Center;
        x
    };

    const COLUMN: Self = {
        let mut x = Self::DEFAULT;
        x.flex_direction = FlexDirection::Column;
        x
    };

    const COLUMN_LEFT: Self = {
        let mut x = Self::COLUMN;
        x.align_items = AlignItems::Start;
        x
    };

    const COLUMN_MID: Self = {
        let mut x = Self::COLUMN;
        x.align_items = AlignItems::Center;
        x
    };

    const COLUMN_RIGHT: Self = {
        let mut x = Self::COLUMN;
        x.align_items = AlignItems::End;
        x
    };

    const COLUMN_CENTER: Self = {
        let mut x = Self::COLUMN;
        x.align_items = AlignItems::Center;
        x.justify_content = JustifyContent::Center;
        x
    };

    fn abs(mut self) -> Self {
        self.position_type = PositionType::Absolute;
        self
    }

    fn width(mut self, percent: f32) -> Self {
        self.width = Percent(percent);
        self
    }

    fn full_width(self) -> Self {
        self.width(100.0)
    }

    fn height(mut self, percent: f32) -> Self {
        self.height = Percent(percent);
        self
    }

    fn full_height(self) -> Self {
        self.height(100.0)
    }

    fn full_size(self) -> Self {
        self.full_width().full_height()
    }

    fn centered(mut self) -> Self {
        self.align_self = AlignSelf::Center;
        self.justify_self = JustifySelf::Center;
        self
    }
}
