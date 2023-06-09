use std::{fmt::Debug, ops::*};

trait Numeric<T>: Copy + Default + Mul<T> + Add<T> {}

macro_rules! impl_op {
    ($class_name:ident, $op_trait_name:ident, $func_name:ident, $op:tt ,$($attr_name: ident), +) => {
        impl<T: $op_trait_name<Output = T>> $op_trait_name for $class_name<T> {
            type Output = $class_name<T>;
            fn $func_name(self, rhs: $class_name<T>) -> Self::Output {
                $class_name {
                    $(
                        $attr_name: self.$attr_name $op rhs.$attr_name,
                    )+
                }
            }
        }
    };
}

macro_rules! impl_op_assign {
    ($class_name:ident, $op_trait_name:ident, $func_name:ident, $op:tt, $($attr_name:ident),+) => {
        impl<T: $op_trait_name>  $op_trait_name for $class_name<T> {
            fn $func_name(&mut self, rhs: Self) {
                $(
                    self.$attr_name $op rhs.$attr_name;
                )+
            }
        }
    };
}

macro_rules! def_genvec {
    ($class_name:ident, $($attr_name:ident), +) => {
        #[derive(Debug, PartialEq, Copy, Clone, Default)]
        pub struct $class_name<T>
        {
            $(
                pub $attr_name: T,
            )+
        }

        impl<T: Copy + Default + Mul<Output = T> + Add<Output = T>> $class_name<T> {
            pub fn new($($attr_name: T, )+) -> Self {
                Self {
                    $($attr_name: $attr_name,)+
                }
            }

            pub fn from(value: T) -> Self {
                Self {
                    $($attr_name: value,)+
                }
            }

            pub fn dot(&self, rhs: Self) -> T {
                $(self.$attr_name * rhs.$attr_name+)+ T::default()
            }
        }

        impl $class_name<f32> {
            pub fn len_square(&self) -> f32 {
                $(self.$attr_name * self.$attr_name +)+ 0.
            }

            pub fn len(&self) -> f32 {
                self.len_square().sqrt()
            }

            pub fn normalize(&self) -> Self {
                let ls = self.len();
                Self {
                    $($attr_name: self.$attr_name / ls,)+
                }
            }

            pub fn to_u8(&self) -> $class_name<u8> {
                $class_name::new($(self.$attr_name as u8,)+)
            }
        }

        impl_op!($class_name, Add, add, + $(, $attr_name)+);
        impl_op!($class_name, Sub, sub, - $(, $attr_name)+);
        impl_op!($class_name, Mul, mul, * $(, $attr_name)+);
        impl_op!($class_name, Div, div, / $(, $attr_name)+);

        impl_op_assign!($class_name, AddAssign, add_assign, += $(, $attr_name)+);

        impl<T: Div<f32, Output = T>> Div<f32> for $class_name<T> {
            type Output = Self;
            fn div(self, rhs: f32) -> Self {
                Self {
                    $($attr_name: self.$attr_name / rhs,)+
                }
            }
        }

        impl<T: Mul<f32, Output = T>> Mul<f32> for $class_name<T> {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self {
                Self {
                    $($attr_name: self.$attr_name * rhs,)+
                }
            }
        }
    };
}

pub fn to_u8_vec(from: Vec3<f32>) -> Vec3<u8> {
    Vec3::new(from.x as u8, from.y as u8, from.z as u8)
}

def_genvec!(Vec2, x, y);
def_genvec!(Vec3, x, y, z);
def_genvec!(Vec4, x, y, z, w);

impl<T> Vec4<T>
where
    T: Copy + Default + Mul<Output = T> + Add<Output = T>,
{
    pub fn from_array(from: &[T]) -> Self {
        Self {
            x: from[0],
            y: from[1],
            z: from[2],
            w: from[3],
        }
    }

    pub fn from_vec3(xyz: Vec3<T>, w: T) -> Self {
        Self {
            x: xyz.x,
            y: xyz.y,
            z: xyz.z,
            w: w,
        }
    }

    pub fn xyz(&self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy,
{
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mat4<T> {
    rows: [Vec4<T>; 4],
}

impl<T: Copy + Default + Mul<Output = T> + Add<Output = T>> Mat4<T> {
    pub fn new(rows: [Vec4<T>; 4]) -> Self {
        Self { rows: rows }
    }

    pub fn from_array(from: &[T; 16]) -> Self {
        let mut rows: [Vec4<T>; 4] = Default::default();
        for i in 0..4 {
            rows[i] = Vec4::from_array(&from[i * 4..(i + 1) * 4]);
        }
        Self { rows: rows }
    }

    pub fn col(&self, i: usize) -> Vec4<T> {
        if i == 0 {
            Vec4::new(
                self.rows[0].x,
                self.rows[1].x,
                self.rows[2].x,
                self.rows[3].x,
            )
        } else if i == 1 {
            Vec4::new(
                self.rows[0].y,
                self.rows[1].y,
                self.rows[2].y,
                self.rows[3].y,
            )
        } else if i == 2 {
            Vec4::new(
                self.rows[0].z,
                self.rows[1].z,
                self.rows[2].z,
                self.rows[3].z,
            )
        } else if i == 3 {
            Vec4::new(
                self.rows[0].w,
                self.rows[1].w,
                self.rows[2].w,
                self.rows[3].w,
            )
        } else {
            panic!("wrong i")
        }
    }

    pub fn transpose(&self) -> Self {
        Self::new([self.col(0), self.col(1), self.col(2), self.col(3)])
    }
}

#[rustfmt::skip]
impl Mat4<f32> {
    pub fn identity() -> Self {
        Mat4::from_array(&[
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Mat4::from_array(&[
            1., 0., 0., x,
            0., 1., 0., y,
            0., 0., 1., z,
            0., 0., 0., 1.,
        ])
    }
}

impl<T> Mul<Vec4<T>> for Mat4<T>
where
    T: Copy + Default + Mul<Output = T> + Add<Output = T>,
{
    type Output = Vec4<T>;
    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        let mut rows: [T; 4] = Default::default();
        for i in 0..4 {
            rows[i] = self.rows[i].dot(rhs);
        }
        Vec4::from_array(&rows)
    }
}

impl<T> Mul<f32> for Mat4<T>
where
    T: Mul<f32, Output = T> + Copy + Default + Mul<Output = T> + Add<Output = T>,
{
    type Output = Mat4<T>;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut rows: [Vec4<T>; 4] = Default::default();
        for i in 0..4 {
            rows[i] = self.rows[i] * rhs;
        }
        Mat4::new(rows)
    }
}

impl<T> Mul<Mat4<T>> for f32
where
    T: Mul<f32, Output = T> + Default + Copy + Mul<Output = T> + Add<Output = T>,
{
    type Output = Mat4<T>;
    fn mul(self, rhs: Mat4<T>) -> Self::Output {
        let mut rows: [Vec4<T>; 4] = Default::default();
        for i in 0..4 {
            rows[i] = rhs.rows[i] * self;
        }
        Mat4::new(rows)
    }
}

impl<T> Mul for Mat4<T>
where
    T: Default + Copy + Mul<Output = T> + Add<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut array: [T; 16] = Default::default();
        for i in 0..4 {
            for j in 0..4 {
                array[i * 4 + j] = self.rows[i].dot(rhs.col(j));
            }
        }
        Self::from_array(&array)
    }
}

pub fn saturate<T>(from: T, to: T, value: T) -> T
where
    T: Copy + PartialOrd,
{
    if value < from {
        from
    } else if value > to {
        to
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::{Mat4, Vec4};

    #[test]
    fn mat_test() {
        let I = Mat4::identity();
        let v1 = Vec4::from_array(&[1., 2., 3., 4.]);
        assert_eq!(I * v1, v1);
        let I = 2.0f32 * I;
        let I = I * 2.;
        assert_eq!(I * v1, v1 * 4.);
        assert_eq!(I * Mat4::identity(), I);
    }
}
