use crate::prelude::*;

pub trait NodeExtLayout {
    // Direction:
    const ROW: Self;
    const COLUMN: Self;
    fn reverse(self) -> Self;

    // Position:
    fn top_left(self) -> Self;
    fn top_center(self) -> Self;
    fn top_right(self) -> Self;
    fn center_left(self) -> Self;
    fn center(self) -> Self;
    fn center_right(self) -> Self;
    fn bottom_left(self) -> Self;
    fn bottom_center(self) -> Self;
    fn bottom_right(self) -> Self;

    // Size:
    fn width(self, width: Val) -> Self;
    fn height(self, height: Val) -> Self;
    fn size(self, width: Val, height: Val) -> Self;
    fn full_width(self) -> Self;
    fn full_height(self) -> Self;
    fn full_size(self) -> Self;
    fn grow(self) -> Self;

    // Other:
    fn abs(self) -> Self;
}

impl NodeExtLayout for Node {
    const ROW: Self = Self::DEFAULT;

    const COLUMN: Self = {
        let mut x = Self::DEFAULT;
        x.flex_direction = FlexDirection::Column;
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

    fn top_left(mut self) -> Self {
        self.align_items = AlignItems::Start;
        self.justify_content = JustifyContent::Start;
        self
    }

    fn top_center(mut self) -> Self {
        (self.align_items, self.justify_content) = match self.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                (AlignItems::Start, JustifyContent::Center)
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                (AlignItems::Center, JustifyContent::Start)
            },
        };
        self
    }

    fn top_right(mut self) -> Self {
        (self.align_items, self.justify_content) = match self.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                (AlignItems::Start, JustifyContent::End)
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                (AlignItems::End, JustifyContent::Start)
            },
        };
        self
    }

    fn center_left(mut self) -> Self {
        (self.align_items, self.justify_content) = match self.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                (AlignItems::Center, JustifyContent::Start)
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                (AlignItems::Start, JustifyContent::Center)
            },
        };
        self
    }

    fn center(mut self) -> Self {
        self.align_items = AlignItems::Center;
        self.justify_content = JustifyContent::Center;
        self
    }

    fn center_right(mut self) -> Self {
        (self.align_items, self.justify_content) = match self.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                (AlignItems::Center, JustifyContent::End)
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                (AlignItems::End, JustifyContent::Center)
            },
        };
        self
    }

    fn bottom_left(mut self) -> Self {
        (self.align_items, self.justify_content) = match self.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                (AlignItems::End, JustifyContent::Start)
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                (AlignItems::Start, JustifyContent::End)
            },
        };
        self
    }

    fn bottom_center(mut self) -> Self {
        (self.align_items, self.justify_content) = match self.flex_direction {
            FlexDirection::Row | FlexDirection::RowReverse => {
                (AlignItems::End, JustifyContent::Center)
            },
            FlexDirection::Column | FlexDirection::ColumnReverse => {
                (AlignItems::Center, JustifyContent::End)
            },
        };
        self
    }

    fn bottom_right(mut self) -> Self {
        self.align_items = AlignItems::End;
        self.justify_content = JustifyContent::End;
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
