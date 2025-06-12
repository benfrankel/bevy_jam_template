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

    fn reverse(self) -> Self;

    fn width(self, width: Val) -> Self;
    fn height(self, height: Val) -> Self;
    fn size(self, width: Val, height: Val) -> Self;
    fn full_width(self) -> Self;
    fn full_height(self) -> Self;
    fn full_size(self) -> Self;
    fn grow(self) -> Self;

    fn abs(self) -> Self;
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

    fn reverse(mut self) -> Self {
        self.flex_direction = match self.flex_direction {
            FlexDirection::Row => FlexDirection::RowReverse,
            FlexDirection::Column => FlexDirection::ColumnReverse,
            FlexDirection::RowReverse => FlexDirection::Row,
            FlexDirection::ColumnReverse => FlexDirection::Column,
        };
        self
    }

    fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    fn height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }

    fn size(mut self, width: Val, height: Val) -> Self {
        self.width = width;
        self.height = height;
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

    fn full_size(mut self) -> Self {
        self.width = Percent(100.0);
        self.height = Percent(100.0);
        self
    }

    fn grow(mut self) -> Self {
        self.flex_grow = 1.0;
        self
    }

    fn abs(mut self) -> Self {
        self.position_type = PositionType::Absolute;
        self
    }
}
