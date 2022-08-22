macro_rules! impl_vector {
    ($struct: ident { $($field: ident), + }, $len: expr) => {
        impl<T> $struct<T> {
            pub const fn new($($field: T), +) -> Self {
                return Self {
                    $($field: $field), +
                }
            }
        }
    };
}

pub(crate) use impl_vector;