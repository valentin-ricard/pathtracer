#[macro_export]
macro_rules! operator_impl {
    ($element:ident, $value_creator:path, $($field:ident),+) => {
        impl std::ops::Add for $element {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                $value_creator(
                    $(
                        self.$field + rhs.$field
                    ),*
                )
            }
        }

        impl std::ops::AddAssign for $element {
            fn add_assign(&mut self, rhs: Self) {
                $(
                        self.$field += rhs.$field;
                )*
            }
        }

        impl core::cmp::PartialEq for $element {
            fn eq(&self, rhs: &Self) -> bool {
                $(
                        self.$field == rhs.$field
                )&&*
            }
        }

        impl core::cmp::Eq for $element {}

        impl std::ops::Mul for $element {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                $value_creator(
                    $(
                        self.$field * rhs.$field
                    ),*
                )
            }
        }

        impl std::ops::Neg for $element {
            type Output = Self;

            fn neg(self) -> Self::Output {
                $value_creator(
                    $(
                        self.$field * -1.0
                    ),*
                )
            }
        }

    };
}


#[macro_export]
macro_rules! scalar_ops {
    ($element:ident, $value_creator:path, $scalar_type:ty, $field_type:ty,$($field:ident),+) => {
        impl std::ops::Mul<$scalar_type> for $element {
            type Output = Self;

            fn mul(self, rhs: $scalar_type) -> Self::Output {
                $value_creator(
                    $(
                        self.$field * (rhs as $field_type)
                    ),*
                )
            }
        }

        impl std::ops::Mul<$element> for $scalar_type {
            type Output = $element;

            fn mul(self, rhs: $element) -> Self::Output {
                rhs * (self as $field_type)
            }
        }

        impl std::ops::MulAssign<$scalar_type> for $element {
            fn mul_assign(&mut self, rhs: $scalar_type) {
                $(
                    self.$field *= (rhs as $field_type);
                )*
            }
        }

        impl std::ops::Div<$scalar_type> for $element {
            type Output = Self;

            fn div(self, rhs: $scalar_type) -> Self::Output {
                self * (1.0/(rhs as $field_type))
            }
        }

        impl std::ops::DivAssign<$scalar_type> for $element {
            fn div_assign(&mut self, rhs: $scalar_type) {
                *self *= 1.0/(rhs as $field_type)
            }
        }
    };
}