use std::{fmt::Debug, ops::*};

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

        impl<T: Mul<f32, Output = T>> Mul<f32> for $class_name<T> {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self{
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
