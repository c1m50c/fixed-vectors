pub mod floating;


#[macro_export(local_inner_macros)]
macro_rules! impl_vector {
    ( $struct: ident { $($field: ident), + }, ( $($generic: ident), + ), $size: expr ) => {
        impl<T> $struct<T> {
            /// Constructs a new vector with the specified values for each field.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(0, 0);
            /// 
            /// assert_eq!(vec2.x, 0);
            /// assert_eq!(vec2.y, 0);
            /// ```
            #[inline(always)]
            pub const fn new( $($field: T), + ) -> Self {
                Self {
                    $( $field ), +
                }
            }

            /// Consumes the vector and returns its values as an array.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(0, 0);
            /// let array = vec2.to_array();
            /// 
            /// assert_eq!(array, [0, 0]);
            /// ```
            #[inline(always)]
            pub fn to_array(self) -> [T; $size] {
                [ $(self.$field), + ]
            }

            /// Consumes the vector and returns its values as a tuple.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(0, 0);
            /// let tuple = vec2.to_tuple();
            /// 
            /// assert_eq!(tuple, (0, 0));
            /// ```
            #[inline(always)]
            pub fn to_tuple(self) -> ( $($generic), + ) {
                ( $(self.$field), + )
            }

            /// Consumes the vector and returns a new vector with the given function applied on each field.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::new(1, 2)
            ///     .map(|i| i * 2);
            /// 
            /// assert_eq!(vec2, Vector2::new(2, 4));
            /// ```
            #[inline]
            pub fn map<F, U>(self, f: F) -> $struct<U>
            where
                F: Fn(T) -> U
            {
                $struct {
                    $( $field: f(self.$field) ), +
                }
            }
        }

        impl<T: Copy> $struct<T> {
            /// Constructs a vector using the given `value` as the value for all of its fields.
            /// 
            /// # Example
            /// 
            /// ```
            /// use fixed_vectors::Vector2;
            /// 
            /// let vec2 = Vector2::from_value(0);
            /// 
            /// assert_eq!(vec2, Vector2::new(0, 0));
            /// ```
            #[inline(always)]
            pub const fn from_value(value: T) -> Self {
                Self {
                    $( $field: value ), +
                }
            }
        }

        impl<T: core::fmt::Debug> core::fmt::Debug for $struct<T> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let identifier = core::stringify!($struct);

                f.debug_struct(identifier)
                    $( .field( core::stringify!($field), &self.$field ) ) +
                    .finish()
            }
        }

        impl<T: PartialEq> PartialEq for $struct<T> {
            fn eq(&self, other: &Self) -> bool {
                $( self.$field == other.$field ) && +
            }
        }

        impl<T: Eq> Eq for $struct<T> {  }

        impl<T: core::ops::Neg<Output = T>> core::ops::Neg for $struct<T> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self.map(|x| -x)
            }
        }

        impl<T: core::ops::Add<Output = T>> core::ops::Add<Self> for $struct<T> {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                Self {
                    $( $field: self.$field + other.$field ), +
                }
            }
        }

        impl<T: core::ops::AddAssign> core::ops::AddAssign<Self> for $struct<T> {
            fn add_assign(&mut self, other: Self)  {
                $( self.$field += other.$field ); +
            }
        }

        impl<T: core::ops::Add<Output = T> + Copy> core::ops::Add<T> for $struct<T> {
            type Output = Self;

            fn add(self, other: T) -> Self::Output {
                Self {
                    $( $field: self.$field + other ), +
                }
            }
        }

        impl<T: core::ops::AddAssign + Copy> core::ops::AddAssign<T> for $struct<T> {
            fn add_assign(&mut self, other: T)  {
                $( self.$field += other); +
            }
        }

        impl<T: core::ops::Sub<Output = T>> core::ops::Sub for $struct<T> {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                Self {
                    $( $field: self.$field - other.$field ), +
                }
            }
        }

        impl<T: core::ops::SubAssign> core::ops::SubAssign for $struct<T> {
            fn sub_assign(&mut self, other: Self)  {
                $( self.$field -= other.$field ); +
            }
        }

        impl<T: core::ops::Sub<Output = T> + Copy> core::ops::Sub<T> for $struct<T> {
            type Output = Self;

            fn sub(self, other: T) -> Self::Output {
                Self {
                    $( $field: self.$field - other ), +
                }
            }
        }

        impl<T: core::ops::SubAssign + Copy> core::ops::SubAssign<T> for $struct<T> {
            fn sub_assign(&mut self, other: T)  {
                $( self.$field -= other); +
            }
        }


        impl<T: core::ops::Mul<Output = T>> core::ops::Mul for $struct<T> {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                Self {
                    $( $field: self.$field * other.$field ), +
                }
            }
        }

        impl<T: core::ops::MulAssign> core::ops::MulAssign for $struct<T> {
            fn mul_assign(&mut self, other: Self)  {
                $( self.$field *= other.$field ); +
            }
        }

        impl<T: core::ops::Mul<Output = T> + Copy> core::ops::Mul<T> for $struct<T> {
            type Output = Self;

            fn mul(self, other: T) -> Self::Output {
                Self {
                    $( $field: self.$field * other ), +
                }
            }
        }

        impl<T: core::ops::MulAssign + Copy> core::ops::MulAssign<T> for $struct<T> {
            fn mul_assign(&mut self, other: T)  {
                $( self.$field *= other); +
            }
        }

        impl<T: core::ops::Div<Output = T>> core::ops::Div for $struct<T> {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output {
                Self {
                    $( $field: self.$field / other.$field ), +
                }
            }
        }

        impl<T: core::ops::DivAssign> core::ops::DivAssign for $struct<T> {
            fn div_assign(&mut self, other: Self)  {
                $( self.$field /= other.$field ); +
            }
        }

        impl<T: core::ops::Div<Output = T> + Copy> core::ops::Div<T> for $struct<T> {
            type Output = Self;

            fn div(self, other: T) -> Self::Output {
                Self {
                    $( $field: self.$field / other ), +
                }
            }
        }

        impl<T: core::ops::DivAssign + Copy> core::ops::DivAssign<T> for $struct<T> {
            fn div_assign(&mut self, other: T)  {
                $( self.$field /= other); +
            }
        }

        impl<T: core::ops::Rem<Output = T>> core::ops::Rem for $struct<T> {
            type Output = Self;

            fn rem(self, other: Self) -> Self::Output {
                Self {
                    $( $field: self.$field % other.$field ), +
                }
            }
        }

        impl<T: core::ops::RemAssign> core::ops::RemAssign for $struct<T> {
            fn rem_assign(&mut self, other: Self)  {
                $( self.$field %= other.$field ); +
            }
        }

        impl<T: core::ops::Rem<Output = T> + Copy> core::ops::Rem<T> for $struct<T> {
            type Output = Self;

            fn rem(self, other: T) -> Self::Output {
                Self {
                    $( $field: self.$field % other ), +
                }
            }
        }

        impl<T: core::ops::RemAssign + Copy> core::ops::RemAssign<T> for $struct<T> {
            fn rem_assign(&mut self, other: T)  {
                $( self.$field %= other); +
            }
        }

        impl<T: core::hash::Hash> core::hash::Hash for $struct<T> {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                $( self.$field.hash(state); ) +
            }
        }

        impl<T: Clone> Clone for $struct<T> {
            fn clone(&self) -> Self {
                Self {
                    $( $field: self.$field.clone() ), +
                }
            }
        }

        impl<T: Default> Default for $struct<T> {
            fn default() -> Self {
                Self {
                    $( $field: T::default() ), +
                }
            }
        }

        $crate::impl_cross_type_floating_point_operations!( $struct { $($field), + }, $size );
        $crate::impl_floating_point_operations!( $struct { $($field), + }, $size, f32 );
        $crate::impl_floating_point_operations!( $struct { $($field), + }, $size, f64 );
    };
}