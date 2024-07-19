#[cfg(feature = "derive")]
pub use replace_type_in_place_derive::{Replace, ReplaceInPlace};

use std::{any::type_name, mem, ptr};

pub trait Replace<Old> {
    type OutputSelf<New>;
    fn replace<New>(self, f: &mut impl FnMut(Old) -> New) -> Self::OutputSelf<New>;
}

pub trait ReplaceInPlace<Old> {
    type OutputSelf<New>;
    fn replace_in_place<New>(self, f: &mut impl FnMut(Old) -> New) -> Self::OutputSelf<New>;
}

macro_rules! impl_replace_for_primitives {
    ($($t:ty),*) => {
        $(
            impl Replace<$t> for $t {
                type OutputSelf<New> = New;
                fn replace<New>(self, f: &mut impl FnMut($t) -> New) -> Self::OutputSelf<New> {
                    f(self)
                }
            }

            impl ReplaceInPlace<$t> for $t {
                type OutputSelf<New> = New;
                fn replace_in_place<New>(self, f: &mut impl FnMut($t) -> New) -> Self::OutputSelf<New> {
                    // Size and alignment checks
                    if std::mem::size_of::<$t>() < std::mem::size_of::<New>() {
                        panic!(
                            "The Old type is smaller than the New type you tried to replace it with: \n\
                            Old: {} size: {}\n\
                            New: {} size: {}",
                            std::any::type_name::<$t>(),
                            std::mem::size_of::<$t>(),
                            std::any::type_name::<New>(),
                            std::mem::size_of::<New>()
                        );
                    }

                    if std::mem::align_of::<$t>() != std::mem::align_of::<New>() {
                        panic!(
                            "The Old type has a different alignment than the New type you tried to replace it with: \n\
                            Old: {} alignment: {}\n\
                            New: {} alignment: {}",
                            std::any::type_name::<$t>(),
                            std::mem::align_of::<$t>(),
                            std::any::type_name::<New>(),
                            std::mem::align_of::<New>()
                        );
                    }

                    f(self)
                }
            }
        )*
    }
}

impl_replace_for_primitives!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char
);

impl<T> Replace<T> for Vec<T> {
    type OutputSelf<New> = Vec<New>;
    fn replace<New>(self, f: &mut impl FnMut(T) -> New) -> Self::OutputSelf<New> {
        self.into_iter().map(f).collect()
    }
}

impl<Old> ReplaceInPlace<Old> for Vec<Old> {
    type OutputSelf<New> = Vec<New>;

    fn replace_in_place<New>(mut self, f: &mut impl FnMut(Old) -> New) -> Self::OutputSelf<New> {
        if mem::size_of::<Old>() != mem::size_of::<New>() {
            panic!(
                "The Old type has a different size than the New type you tried to replace it with: \n\
                Old: {} size: {}\n\
                New: {} size: {}",
                type_name::<Old>(),
                mem::size_of::<Old>(),
                type_name::<New>(),
                mem::size_of::<New>()
            );
        };

        if mem::align_of::<Old>() != mem::align_of::<New>() {
            panic!(
                "The Old type has a different alignment than the New type you tried to replace it with: \n\
                Old: {} alignment: {}\n\
                New: {} alignment: {}",
                type_name::<Old>(),
                mem::align_of::<Old>(),
                type_name::<New>(),
                mem::align_of::<New>()
            );
        };

        let size_of_old_self = mem::size_of::<Self>();
        let size_of_new_self = mem::size_of::<<Self as ReplaceInPlace<Old>>::OutputSelf<New>>();

        if size_of_old_self != size_of_new_self {
            panic!(
                "The size of the Vec<Old> is not the same as the size of the Vec<New>: \n\
                Vec<Old> size: {}\n\
                Vec<New> size: {}",
                size_of_old_self, size_of_new_self
            );
        };

        let align_of_old_self = mem::align_of::<Self>();
        let align_of_new_self = mem::align_of::<<Self as ReplaceInPlace<Old>>::OutputSelf<New>>();

        if align_of_old_self != align_of_new_self {
            panic!(
                "The alignment of the Vec<Old> is not the same as the alignment of the Vec<New>: \n\
                Vec<Old> alignment: {}\n\
                Vec<New> alignment: {}",
                align_of_old_self,
                align_of_new_self
            );
        };

        // This is safe because we are checking size and alignment of the types above.
        unsafe {
            for old in self.iter_mut() {
                let old = old as *mut Old;
                let new = f(ptr::read(old));

                ptr::write(old as *mut New, new);
            }

            mem::transmute::<Vec<Old>, Vec<New>>(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_vec() {
        let v = vec![1, 2, 3];
        let v = v.replace(&mut |x| x.to_string());
        assert_eq!(v, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_replace_in_place_vec() {
        #[repr(align(8))]
        struct A(u32, u32);

        let v = vec![A(1, 2), A(3, 4)];
        let v = v.replace_in_place(&mut |A(x, y)| x as u64 * y as u64);
        assert_eq!(v, vec![2, 12]);
    }

    #[derive(Debug, PartialEq)]
    enum A {
        A(String),
    }

    #[derive(Debug, PartialEq)]
    enum AB {
        A(String),
        B(bool),
    }

    #[derive(Debug, PartialEq)]
    #[allow(clippy::upper_case_acronyms)]
    enum ABC {
        A(String),
        B(bool),
        C(u64),
    }

    #[test]
    fn test_refine_enum_inplace() {
        let vec = vec![ABC::A("a".to_string()), ABC::B(true), ABC::C(1)];

        let vec: Vec<AB> = vec.replace_in_place(&mut |abc| match abc {
            ABC::A(s) => AB::A(s),
            ABC::B(b) => AB::B(b),
            ABC::C(c) => AB::A(c.to_string()),
        });

        assert_eq!(
            vec,
            vec![AB::A("a".to_string()), AB::B(true), AB::A("1".to_string())]
        );

        let vec: Vec<A> = vec.replace_in_place(&mut |ab| match ab {
            AB::A(s) => A::A(s),
            AB::B(b) => A::A(b.to_string()),
        });

        assert_eq!(
            vec,
            vec![
                A::A("a".to_string()),
                A::A("true".to_string()),
                A::A("1".to_string())
            ]
        );
    }
}
