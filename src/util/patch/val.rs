use crate::prelude::*;

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/5893>.
pub trait ValExtAdd: Sized {
    fn add(
        &self,
        other: Self,
        parent_size: f32,
        viewport_size: Vec2,
    ) -> Result<Self, ValArithmeticError>;
}

impl ValExtAdd for Val {
    fn add(
        &self,
        other: Self,
        parent_size: f32,
        viewport_size: Vec2,
    ) -> Result<Self, ValArithmeticError> {
        Ok(Px(self.resolve(parent_size, viewport_size)?
            + other.resolve(parent_size, viewport_size)?))
    }
}
