pub mod floating;


#[doc(hidden)]
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

        impl<T> From<[T; $size]> for $struct<T> {
            fn from(from: [T; $size]) -> Self {
                let mut iterator = from.into_iter();

                Self {
                    // SAFETY: We know the size of `from` so `iterator.next()` is always `Some(..)`
                    $( $field: unsafe { iterator.next().unwrap_unchecked() } ), +
                }
            }
        }

        impl<T> From<($($generic), +)> for $struct<T> {
            fn from(from: ($($generic), +)) -> Self {
                let ( $($field), + ) = from;
    
                Self {
                    $( $field ), +
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

        impl<T: core::ops::Neg<Output = T>> core::ops::Neg for $struct<T> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self {
                    $( $field: -self.$field ), +
                }
            }
        }

        // Impl arithmetic pperators
        $crate::impl_operator!( $struct { $($field), + }, AddAssign, add_assign );
        $crate::impl_operator!( $struct { $($field), + }, Add, add, Self );
        $crate::impl_operator!( $struct { $($field), + }, SubAssign, sub_assign );
        $crate::impl_operator!( $struct { $($field), + }, Sub, sub, Self );
        $crate::impl_operator!( $struct { $($field), + }, MulAssign, mul_assign );
        $crate::impl_operator!( $struct { $($field), + }, Mul, mul, Self );
        $crate::impl_operator!( $struct { $($field), + }, DivAssign, div_assign );
        $crate::impl_operator!( $struct { $($field), + }, Div, div, Self );
        $crate::impl_operator!( $struct { $($field), + }, RemAssign, rem_assign );
        $crate::impl_operator!( $struct { $($field), + }, Rem, rem, Self );

        // Impl bitwise operators
        $crate::impl_operator!( $struct { $($field), + }, BitAndAssign, bitand_assign );
        $crate::impl_operator!( $struct { $($field), + }, BitAnd, bitand, Self );
        $crate::impl_operator!( $struct { $($field), + }, BitOrAssign, bitor_assign );
        $crate::impl_operator!( $struct { $($field), + }, BitOr, bitor, Self );
        $crate::impl_operator!( $struct { $($field), + }, BitXorAssign, bitxor_assign );
        $crate::impl_operator!( $struct { $($field), + }, BitXor, bitxor, Self );

        // Impl floating-point based methods
        $crate::impl_cross_type_floating_point_operations!( $struct { $($field), + }, $size );
        $crate::impl_floating_point_operations!( $struct { $($field), + }, $size, f32 );
        $crate::impl_floating_point_operations!( $struct { $($field), + }, $size, f64 );
    };
}


#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! impl_operator {
    ( $struct: ident { $($field: ident), + }, $trait: ident, $method: ident, $output: ty ) => {
        impl<T: core::ops::$trait<Output = T>> core::ops::$trait<Self> for $struct<T> {
            type Output = $output;

            fn $method(self, other: Self) -> Self::Output {
                Self {
                    $( $field: self.$field.$method(other.$field) ), +
                }
            }
        }

        impl<T: core::ops::$trait<Output = T> + Copy> core::ops::$trait<T> for $struct<T> {
            type Output = $output;

            fn $method(self, other: T) -> Self::Output {
                Self {
                    $( $field: self.$field.$method(other) ), +
                }
            }
        }
    };

    ( $struct: ident { $($field: ident), + }, $trait: ident, $method: ident ) => {
        impl<T: core::ops::$trait> core::ops::$trait for $struct<T> {
            fn $method(&mut self, other: Self)  {
                $( self.$field.$method(other.$field) ); +
            }
        }

        impl<T: core::ops::$trait + Copy> core::ops::$trait<T> for $struct<T> {
            fn $method(&mut self, other: T)  {
                $( self.$field.$method(other) ); +
            }
        }
    };
}