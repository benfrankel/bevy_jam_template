use crate::prelude::*;

// TODO: Workaround for https://github.com/bevyengine/bevy/issues/14525.
pub trait Dir2ExtToQuat {
    fn to_quat(self) -> Quat;
}

impl Dir2ExtToQuat for Dir2 {
    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.to_angle())
    }
}
