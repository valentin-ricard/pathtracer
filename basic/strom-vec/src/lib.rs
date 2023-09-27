use std::ops::{Index};
use strom_macro::{operator_impl, scalar_ops};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn of<T>(x: T, y: T, z: T) -> Self
        where
            T: Into<f32>,
    {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of bounds. Vec only contains 3 elements.")
        }
    }
}

operator_impl!(Vec3, Vec3::of, x, y, z);

scalar_ops!(Vec3, Vec3::of, f32, f32, x, y, z);
scalar_ops!(Vec3, Vec3::of, i32, f32, x, y, z);

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.x * v.x
    + u.y * u.y
    + u.z * u.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::of(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x
    )
}

#[cfg(test)]
mod vec_tests {
    use crate::{cross, Vec3};

    #[test]
    fn test_add_and_eq() {
        let add = Vec3::of(1.0, 2.0, 3.0) + Vec3::of(4.0, 5.0, 6.0);
        assert_eq!(add, Vec3::of(5.0, 7.0, 9.0))
    }

    #[test]
    fn test_assign_add() {
        let mut add = Vec3::of(1.0, 2.0, 3.0);
        add += Vec3::of(4.0, 5.0, 6.0);

        assert_eq!(add, Vec3::of(5.0, 7.0, 9.0))
    }

    #[test]
    fn test_scalar_multiplication() {
        let mult = Vec3::of(1.0, 2.0, 3.0) * 3.0;
        assert_eq!(mult, Vec3::of(3.0, 6.0, 9.0))
    }

    #[test]
    fn test_assign_scalar_multiplication() {
        let mut mult = Vec3::of(1.0, 2.0, 3.0);
        mult *= 3.0;
        assert_eq!(mult, Vec3::of(3.0, 6.0, 9.0))
    }

    #[test]
    fn test_length_squared() {
        assert_eq!(Vec3::of(2.0, 3.0, 6.0).length_squared(), 49.0);
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec3::of(2.0, 3.0, 6.0).length(), 7.0);
    }

    #[test]
    fn test_cross() {
        assert_eq!(cross(Vec3::of(1.0,2.0,3.0), Vec3::of(4.0,5.0,6.0)), Vec3::of(-3.0, 6.0, -3.0))
    }

    #[test]
    fn assert_neg() {
        assert_eq!(-Vec3::of(1.0,2.0,3.0), Vec3::of(-1.0, -2.0, -3.0))
    }
}


impl<T: Into<f32>> From<(T, T, T)> for Vec3 {
    fn from((x, y, z): (T, T, T)) -> Self {
        Vec3::of(x, y, z)
    }
}

pub trait ToVec3 {
    fn to_vec(&self) -> Vec3;
}

impl<T: Into<f32> + Clone> ToVec3 for (T, T, T) {
    fn to_vec(&self) -> Vec3 {
        Vec3::of(self.0.clone(), self.1.clone(), self.2.clone())
    }
}