#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

// #[cfg(feature = "derive")]
// pub use replace_type_in_place_derive::{Replace, ReplaceInPlace};
use std::{
    any::type_name,
    mem::{self, ManuallyDrop},
    ptr,
};

pub trait ReplaceInPlace {
    type AOld;
    type BOld;
    type COld;
    type DOld;
    type EOld;
    type FOld;
    type GOld;
    type HOld;
    type OutputSelf<A, B, C, D, E, F, G, H>;

    fn replace_in_place_1<A>(
        self,
        f: &mut impl FnMut(Self::AOld) -> A,
    ) -> Self::OutputSelf<
        A,
        Self::BOld,
        Self::COld,
        Self::DOld,
        Self::EOld,
        Self::FOld,
        Self::GOld,
        Self::HOld,
    >;

    fn replace_in_place_2<A, B>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
    ) -> Self::OutputSelf<
        A,
        B,
        Self::COld,
        Self::DOld,
        Self::EOld,
        Self::FOld,
        Self::GOld,
        Self::HOld,
    >;

    fn replace_in_place_3<A, B, C>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
    ) -> Self::OutputSelf<A, B, C, Self::DOld, Self::EOld, Self::FOld, Self::GOld, Self::HOld>;

    fn replace_in_place_4<A, B, C, D>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
        fd: &mut impl FnMut(Self::DOld) -> D,
    ) -> Self::OutputSelf<A, B, C, D, Self::EOld, Self::FOld, Self::GOld, Self::HOld>;

    fn replace_in_place_5<A, B, C, D, E>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
        fd: &mut impl FnMut(Self::DOld) -> D,
        fe: &mut impl FnMut(Self::EOld) -> E,
    ) -> Self::OutputSelf<A, B, C, D, E, Self::FOld, Self::GOld, Self::HOld>;

    fn replace_in_place_6<A, B, C, D, E, F>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
        fd: &mut impl FnMut(Self::DOld) -> D,
        fe: &mut impl FnMut(Self::EOld) -> E,
        ff: &mut impl FnMut(Self::FOld) -> F,
    ) -> Self::OutputSelf<A, B, C, D, E, F, Self::GOld, Self::HOld>;

    fn replace_in_place_7<A, B, C, D, E, F, G>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
        fd: &mut impl FnMut(Self::DOld) -> D,
        fe: &mut impl FnMut(Self::EOld) -> E,
        ff: &mut impl FnMut(Self::FOld) -> F,
        fg: &mut impl FnMut(Self::GOld) -> G,
    ) -> Self::OutputSelf<A, B, C, D, E, F, G, Self::HOld>;

    fn replace_in_place_8<A, B, C, D, E, F, G, H>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
        fd: &mut impl FnMut(Self::DOld) -> D,
        fe: &mut impl FnMut(Self::EOld) -> E,
        ff: &mut impl FnMut(Self::FOld) -> F,
        fg: &mut impl FnMut(Self::GOld) -> G,
        fh: &mut impl FnMut(Self::HOld) -> H,
    ) -> Self::OutputSelf<A, B, C, D, E, F, G, H>;
}

impl<AOld> ReplaceInPlace for Vec<AOld> {
    type AOld = AOld;
    type BOld = ();
    type COld = ();
    type DOld = ();
    type EOld = ();
    type FOld = ();
    type GOld = ();
    type HOld = ();
    type OutputSelf<A, B, C, D, E, F, G, H> = Vec<A>;

    fn replace_in_place_1<A>(
        self,
        f: &mut impl FnMut(Self::AOld) -> A,
    ) -> Self::OutputSelf<A, (), (), (), (), (), (), ()> {
        if mem::size_of::<Self::AOld>() != mem::size_of::<A>() {
            panic!(
                "The Self::AOld type has a different size than the New type you tried to replace it with: \n\
                Self::AOld: {} size: {}\n\
                New: {} size: {}",
                type_name::<Self::AOld>(),
                mem::size_of::<Self::AOld>(),
                type_name::<A>(),
                mem::size_of::<A>()
            );
        };

        if mem::align_of::<Self::AOld>() != mem::align_of::<A>() {
            panic!(
                "The Self::AOld type has a different alignment than the A type you tried to replace it with: \n\
                Self::AOld: {} alignment: {}\n\
                A: {} alignment: {}",
                type_name::<Self::AOld>(),
                mem::align_of::<Self::AOld>(),
                type_name::<A>(),
                mem::align_of::<A>()
            );
        };

        let size_of_old_self = mem::size_of::<Self>();
        let size_of_new_self = mem::size_of::<
            <Self as ReplaceInPlace>::OutputSelf<
                A,
                Self::BOld,
                Self::COld,
                Self::DOld,
                Self::EOld,
                Self::FOld,
                Self::GOld,
                Self::HOld,
            >,
        >();

        if size_of_old_self != size_of_new_self {
            panic!(
                "The size of the Vec<Self::AOld> is not the same as the size of the Vec<A>: \n\
                Vec<Self::AOld> size: {}\n\
                Vec<A> size: {}",
                size_of_old_self, size_of_new_self
            );
        };

        let align_of_old_self = mem::align_of::<Self>();
        let align_of_new_self = mem::align_of::<
            <Self as ReplaceInPlace>::OutputSelf<
                A,
                Self::BOld,
                Self::COld,
                Self::DOld,
                Self::EOld,
                Self::FOld,
                Self::GOld,
                Self::HOld,
            >,
        >();

        if align_of_old_self != align_of_new_self {
            panic!(
                "The alignment of the Vec<Self::AOld> is not the same as the alignment of the Vec<A>: \n\
                Vec<Self::AOld> alignment: {}\n\
                Vec<A> alignment: {}",
                align_of_old_self,
                align_of_new_self
            );
        };

        // This is safe because we are checking size and alignment of the types above.
        unsafe {
            // The use of ManuallyDrop prevents the vec from being dropped if f panics.
            // Without it the drop could be called on elements of the wrong type.
            let mut vec = mem::ManuallyDrop::new(self);
            for old in vec.iter_mut() {
                let old = old as *mut Self::AOld;
                let new = f(ptr::read(old));

                ptr::write(old as *mut A, new);
            }

            mem::transmute::<ManuallyDrop<Vec<Self::AOld>>, Vec<A>>(vec)
        }
    }

    fn replace_in_place_2<A, B>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
    ) -> Self::OutputSelf<A, B, (), (), (), (), (), ()> {
        self.replace_in_place_1(fa)
    }

    fn replace_in_place_3<A, B, C>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
    ) -> Self::OutputSelf<A, B, C, (), (), (), (), ()> {
        self.replace_in_place_1(fa)
    }

    fn replace_in_place_4<A, B, C, D>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
        _fd: &mut impl FnMut(Self::DOld) -> D,
    ) -> Self::OutputSelf<A, B, C, D, (), (), (), ()> {
        self.replace_in_place_1(fa)
    }

    fn replace_in_place_5<A, B, C, D, E>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
        _fd: &mut impl FnMut(Self::DOld) -> D,
        _fe: &mut impl FnMut(Self::EOld) -> E,
    ) -> Self::OutputSelf<A, B, C, D, E, (), (), ()> {
        self.replace_in_place_1(fa)
    }

    fn replace_in_place_6<A, B, C, D, E, F>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
        _fd: &mut impl FnMut(Self::DOld) -> D,
        _fe: &mut impl FnMut(Self::EOld) -> E,
        _ff: &mut impl FnMut(Self::FOld) -> F,
    ) -> Self::OutputSelf<A, B, C, D, E, F, (), ()> {
        self.replace_in_place_1(fa)
    }

    fn replace_in_place_7<A, B, C, D, E, F, G>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
        _fd: &mut impl FnMut(Self::DOld) -> D,
        _fe: &mut impl FnMut(Self::EOld) -> E,
        _ff: &mut impl FnMut(Self::FOld) -> F,
        _fg: &mut impl FnMut(Self::GOld) -> G,
    ) -> Self::OutputSelf<A, B, C, D, E, F, G, ()> {
        self.replace_in_place_1(fa)
    }

    fn replace_in_place_8<A, B, C, D, E, F, G, H>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
        _fd: &mut impl FnMut(Self::DOld) -> D,
        _fe: &mut impl FnMut(Self::EOld) -> E,
        _ff: &mut impl FnMut(Self::FOld) -> F,
        _fg: &mut impl FnMut(Self::GOld) -> G,
        _fh: &mut impl FnMut(Self::HOld) -> H,
    ) -> Self::OutputSelf<A, B, C, D, E, F, G, H> {
        self.replace_in_place_1(fa)
    }
}

// macro_rules! impl_replace_for_primitives {
//     ($($t:ty),*) => {
//         $(
//             impl Replace<$t> for $t {
//                 type OutputSelf<New> = New;
//                 fn replace<New>(self, f: &mut impl FnMut($t) -> New) -> Self::OutputSelf<New> {
//                     f(self)
//                 }
//             }

//             impl ReplaceInPlace<$t> for $t {
//                 type OutputSelf<New> = New;
//                 fn replace_in_place<New>(self, f: &mut impl FnMut($t) -> New) -> Self::OutputSelf<New> {
//                     // Size and alignment checks
//                     if std::mem::size_of::<$t>() < std::mem::size_of::<New>() {
//                         panic!(
//                             "The Old type is smaller than the New type you tried to replace it with: \n\
//                             Old: {} size: {}\n\
//                             New: {} size: {}",
//                             std::any::type_name::<$t>(),
//                             std::mem::size_of::<$t>(),
//                             std::any::type_name::<New>(),
//                             std::mem::size_of::<New>()
//                         );
//                     }

//                     if std::mem::align_of::<$t>() != std::mem::align_of::<New>() {
//                         panic!(
//                             "The Old type has a different alignment than the New type you tried to replace it with: \n\
//                             Old: {} alignment: {}\n\
//                             New: {} alignment: {}",
//                             std::any::type_name::<$t>(),
//                             std::mem::align_of::<$t>(),
//                             std::any::type_name::<New>(),
//                             std::mem::align_of::<New>()
//                         );
//                     }

//                     f(self)
//                 }

//                 unsafe fn replace_ptr_in_place<New>(
//                     self_: *mut Self,
//                     f: &mut impl FnMut($t) -> New,
//                 ) -> Self::OutputSelf<New> {
//                     // Size and alignment checks
//                     if std::mem::size_of::<$t>() < std::mem::size_of::<New>() {
//                         panic!(
//                             "The Old type is smaller than the New type you tried to replace it with: \n\
//                             Old: {} size: {}\n\
//                             New: {} size: {}",
//                             std::any::type_name::<$t>(),
//                             std::mem::size_of::<$t>(),
//                             std::any::type_name::<New>(),
//                             std::mem::size_of::<New>()
//                         );
//                     }

//                     if std::mem::align_of::<$t>() != std::mem::align_of::<New>() {
//                         panic!(
//                             "The Old type has a different alignment than the New type you tried to replace it with: \n\
//                             Old: {} alignment: {}\n\
//                             New: {} alignment: {}",
//                             std::any::type_name::<$t>(),
//                             std::mem::align_of::<$t>(),
//                             std::any::type_name::<New>(),
//                             std::mem::align_of::<New>()
//                         );
//                     }

//                     f(ptr::read(self_))
//                 }
//             }
//         )*
//     }
// }

// impl_replace_for_primitives!(
//     u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char
// );

// impl<T> Replace<T> for Vec<T> {
//     type OutputSelf<New> = Vec<New>;
//     fn replace<New>(self, f: &mut impl FnMut(T) -> New) -> Self::OutputSelf<New> {
//         self.into_iter().map(f).collect()
//     }
// }

// impl<Old> ReplaceInPlace<Old> for Vec<Old> {
//     type OutputSelf<New> = Vec<New>;

//     fn replace_in_place<New>(self, f: &mut impl FnMut(Old) -> New) -> Self::OutputSelf<New> {
//         if mem::size_of::<Old>() != mem::size_of::<New>() {
//             panic!(
//                 "The Old type has a different size than the New type you tried to replace it with: \n\
//                 Old: {} size: {}\n\
//                 New: {} size: {}",
//                 type_name::<Old>(),
//                 mem::size_of::<Old>(),
//                 type_name::<New>(),
//                 mem::size_of::<New>()
//             );
//         };

//         if mem::align_of::<Old>() != mem::align_of::<New>() {
//             panic!(
//                 "The Old type has a different alignment than the New type you tried to replace it with: \n\
//                 Old: {} alignment: {}\n\
//                 New: {} alignment: {}",
//                 type_name::<Old>(),
//                 mem::align_of::<Old>(),
//                 type_name::<New>(),
//                 mem::align_of::<New>()
//             );
//         };

//         let size_of_old_self = mem::size_of::<Self>();
//         let size_of_new_self = mem::size_of::<<Self as ReplaceInPlace<Old>>::OutputSelf<New>>();

//         if size_of_old_self != size_of_new_self {
//             panic!(
//                 "The size of the Vec<Old> is not the same as the size of the Vec<New>: \n\
//                 Vec<Old> size: {}\n\
//                 Vec<New> size: {}",
//                 size_of_old_self, size_of_new_self
//             );
//         };

//         let align_of_old_self = mem::align_of::<Self>();
//         let align_of_new_self = mem::align_of::<<Self as ReplaceInPlace<Old>>::OutputSelf<New>>();

//         if align_of_old_self != align_of_new_self {
//             panic!(
//                 "The alignment of the Vec<Old> is not the same as the alignment of the Vec<New>: \n\
//                 Vec<Old> alignment: {}\n\
//                 Vec<New> alignment: {}",
//                 align_of_old_self,
//                 align_of_new_self
//             );
//         };

//         // This is safe because we are checking size and alignment of the types above.
//         unsafe {
//             // The use of ManuallyDrop prevents the vec from being dropped if f panics.
//             // Without it the drop could be called on elements of the wrong type.
//             let mut vec = mem::ManuallyDrop::new(self);
//             for old in vec.iter_mut() {
//                 let old = old as *mut Old;
//                 let new = f(ptr::read(old));

//                 ptr::write(old as *mut New, new);
//             }

//             mem::transmute::<ManuallyDrop<Vec<Old>>, Vec<New>>(vec)
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_replace_vec() {
//         let v = vec![1, 2, 3];
//         let v = v.replace(&mut |x| x.to_string());
//         assert_eq!(v, vec!["1", "2", "3"]);
//     }

//     #[test]
//     fn test_replace_in_place_vec() {
//         #[repr(align(8))]
//         struct A(u32, u32);

//         let v = vec![A(1, 2), A(3, 4)];
//         let v = v.replace_in_place(&mut |A(x, y)| x as u64 * y as u64);
//         assert_eq!(v, vec![2, 12]);
//     }

//     #[derive(Debug, PartialEq)]
//     enum A {
//         A(String),
//     }

//     #[derive(Debug, PartialEq)]
//     enum AB {
//         A(String),
//         B(bool),
//     }

//     #[derive(Debug, PartialEq)]
//     #[allow(clippy::upper_case_acronyms)]
//     enum ABC {
//         A(String),
//         B(bool),
//         C(u64),
//     }

//     #[test]
//     fn test_refine_enum_inplace() {
//         let vec = vec![ABC::A("a".to_string()), ABC::B(true), ABC::C(1)];

//         let vec: Vec<AB> = vec.replace_in_place(&mut |abc| match abc {
//             ABC::A(s) => AB::A(s),
//             ABC::B(b) => AB::B(b),
//             ABC::C(c) => AB::A(c.to_string()),
//         });

//         assert_eq!(
//             vec,
//             vec![AB::A("a".to_string()), AB::B(true), AB::A("1".to_string())]
//         );

//         let vec: Vec<A> = vec.replace_in_place(&mut |ab| match ab {
//             AB::A(s) => A::A(s),
//             AB::B(b) => A::A(b.to_string()),
//         });

//         assert_eq!(
//             vec,
//             vec![
//                 A::A("a".to_string()),
//                 A::A("true".to_string()),
//                 A::A("1".to_string())
//             ]
//         );
//     }
// }
