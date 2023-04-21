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

        impl<T> $class_name<T> {
            pub fn new($($attr_name: T, )+) -> Self {
                Self {
                    $($attr_name: $attr_name,)+
                }
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
        }

        impl_op!($class_name, Add, add, + $(, $attr_name)+);
        impl_op!($class_name, Sub, sub, - $(, $attr_name)+);
        impl_op!($class_name, Mul, mul, * $(, $attr_name)+);
        impl_op!($class_name, Div, div, / $(, $attr_name)+);

    };
}

def_genvec!(Vec2, x, y);
def_genvec!(Vec3, x, y, z);
def_genvec!(Vec4, x, y, z, w);
