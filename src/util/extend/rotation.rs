use crate::prelude::*;

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/14525>.
pub trait QuatExtConvert {
    fn radians(radians: f32) -> Self;
    fn degrees(degrees: f32) -> Self;

    fn to_radians(self) -> f32;
    fn to_degrees(self) -> f32;
    fn to_dir2(self) -> Dir2;
    fn to_rot2(self) -> Rot2;
}

impl QuatExtConvert for Quat {
    fn radians(radians: f32) -> Self {
        Self::from_rotation_z(radians)
    }

    fn degrees(degrees: f32) -> Self {
        Self::from_rotation_z(degrees.to_radians())
    }

    fn to_radians(self) -> f32 {
        self.to_scaled_axis().z
    }

    fn to_degrees(self) -> f32 {
        self.to_scaled_axis().z.to_degrees()
    }

    fn to_dir2(self) -> Dir2 {
        Dir2::new_unchecked(Vec2::from_angle(self.to_scaled_axis().z))
    }

    fn to_rot2(self) -> Rot2 {
        Rot2::radians(self.to_scaled_axis().z)
    }
}

pub trait Rot2ExtConvert {
    fn to_radians(self) -> f32;
    fn to_degrees(self) -> f32;
    fn to_dir2(self) -> Dir2;
    fn to_quat(self) -> Quat;
}

impl Rot2ExtConvert for Rot2 {
    fn to_radians(self) -> f32 {
        self.as_radians()
    }

    fn to_degrees(self) -> f32 {
        self.as_degrees()
    }

    fn to_dir2(self) -> Dir2 {
        self.normalize();
        Dir2::from_xy_unchecked(self.cos, self.sin)
    }

    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.as_radians())
    }
}

pub trait Dir2ExtConvert {
    fn radians(radians: f32) -> Self;
    fn degrees(degrees: f32) -> Self;

    fn to_radians(self) -> f32;
    fn to_degrees(self) -> f32;
    fn to_rot2(self) -> Rot2;
    fn to_quat(self) -> Quat;
}

impl Dir2ExtConvert for Dir2 {
    fn radians(radians: f32) -> Self {
        Dir2::new_unchecked(Vec2::from_angle(radians))
    }

    fn degrees(degrees: f32) -> Self {
        Dir2::new_unchecked(Vec2::from_angle(degrees.to_radians()))
    }

    fn to_radians(self) -> f32 {
        self.to_angle()
    }

    fn to_degrees(self) -> f32 {
        self.to_angle().to_degrees()
    }

    fn to_rot2(self) -> Rot2 {
        self.rotation_from_x()
    }

    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.to_angle())
    }
}
