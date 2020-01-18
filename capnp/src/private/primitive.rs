pub trait Primitive {
    type Raw;

    /// Reads the value, swapping bytes on big-endian processors.
    fn get(raw: &Self::Raw) -> Self;

    /// Writes the value, swapping bytes on big-endian processors.
    fn set(raw: &mut Self::Raw, value: Self);
}

macro_rules! primitive_impl(
    ($typ:ty, $n:expr) => (
        impl Primitive for $typ {
            type Raw = [u8; $n];

            #[inline]
            fn get(raw: &Self::Raw) -> Self {
                <$typ>::from_le_bytes(*raw)
            }

            #[inline]
            fn set(raw: &mut Self::Raw, value: Self) {
                *raw = value.to_le_bytes();
            }
        }
        );
    );

primitive_impl!(u8, 1);
primitive_impl!(i8, 1);
primitive_impl!(u16, 2);
primitive_impl!(i16, 2);
primitive_impl!(u32, 4);
primitive_impl!(i32, 4);
primitive_impl!(u64, 8);
primitive_impl!(i64, 8);
primitive_impl!(f32, 4);
primitive_impl!(f64, 8);


/// A value casted directly from a little-endian byte buffer. On big-endian
/// processors, the bytes of the value need to be swapped upon reading and writing.
#[repr(C)]
pub struct WireValue<T> where T: Primitive {
    value: <T as Primitive>::Raw,
}

impl<T> WireValue<T> where T: Primitive {
    /// Reads the value, swapping bytes on big-endian processors.
    #[inline]
    pub fn get(&self) -> T { <T as Primitive>::get(&self.value) }

    /// Writes the value, swapping bytes on big-endian processors.
    #[inline]
    pub fn set(&mut self, value: T) { <T as Primitive>::set(&mut self.value, value) }
}
