pub trait Alignable: Sized {
    #[must_use]
    fn align_to<T: Into<Self>>(self, align: T) -> Self;
}

macro_rules! impl_alignable {
    ($id:ident) => {
        impl Alignable for $id {
            #[inline]
            fn align_to<T: Into<Self>>(self, align: T) -> Self {
                let align = align.into();

                if !align.is_power_of_two() {
                    panic!("align is not a power-of-two");
                }

                (self + (align - 1)) & !(align - 1)
            }
        }
    };
}

impl_alignable!(u32);
impl_alignable!(u64);
impl_alignable!(usize);
