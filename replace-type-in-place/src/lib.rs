#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

// #[cfg(feature = "derive")]
// pub use replace_type_in_place_derive::ReplaceInPlace;

use std::{
    mem::{self, ManuallyDrop},
    ptr,
};

pub trait Replace {
    type AOld;
    type BOld;
    type COld;
    type DOld;
    type EOld;
    type FOld;
    type GOld;
    type HOld;
    type OutputSelf<A, B, C, D, E, F, G, H>;

    fn replace_1<A>(
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

    fn replace_2<A, B>(
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

    fn replace_3<A, B, C>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
    ) -> Self::OutputSelf<A, B, C, Self::DOld, Self::EOld, Self::FOld, Self::GOld, Self::HOld>;

    fn replace_4<A, B, C, D>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
        fd: &mut impl FnMut(Self::DOld) -> D,
    ) -> Self::OutputSelf<A, B, C, D, Self::EOld, Self::FOld, Self::GOld, Self::HOld>;

    fn replace_5<A, B, C, D, E>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
        fd: &mut impl FnMut(Self::DOld) -> D,
        fe: &mut impl FnMut(Self::EOld) -> E,
    ) -> Self::OutputSelf<A, B, C, D, E, Self::FOld, Self::GOld, Self::HOld>;

    fn replace_6<A, B, C, D, E, F>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
        fd: &mut impl FnMut(Self::DOld) -> D,
        fe: &mut impl FnMut(Self::EOld) -> E,
        ff: &mut impl FnMut(Self::FOld) -> F,
    ) -> Self::OutputSelf<A, B, C, D, E, F, Self::GOld, Self::HOld>;

    fn replace_7<A, B, C, D, E, F, G>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        fb: &mut impl FnMut(Self::BOld) -> B,
        fc: &mut impl FnMut(Self::COld) -> C,
        fd: &mut impl FnMut(Self::DOld) -> D,
        fe: &mut impl FnMut(Self::EOld) -> E,
        ff: &mut impl FnMut(Self::FOld) -> F,
        fg: &mut impl FnMut(Self::GOld) -> G,
    ) -> Self::OutputSelf<A, B, C, D, E, F, G, Self::HOld>;

    fn replace_8<A, B, C, D, E, F, G, H>(
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

impl<AOld> Replace for Vec<AOld> {
    type AOld = AOld;
    type BOld = ();
    type COld = ();
    type DOld = ();
    type EOld = ();
    type FOld = ();
    type GOld = ();
    type HOld = ();
    type OutputSelf<A, B, C, D, E, F, G, H> = Vec<A>;

    fn replace_1<A>(
        self,
        f: &mut impl FnMut(Self::AOld) -> A,
    ) -> Self::OutputSelf<A, (), (), (), (), (), (), ()> {
        self.into_iter().map(f).collect()
    }

    fn replace_2<A, B>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
    ) -> Self::OutputSelf<A, B, (), (), (), (), (), ()> {
        self.replace_1(fa)
    }

    fn replace_3<A, B, C>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
    ) -> Self::OutputSelf<A, B, C, (), (), (), (), ()> {
        self.replace_1(fa)
    }

    fn replace_4<A, B, C, D>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
        _fd: &mut impl FnMut(Self::DOld) -> D,
    ) -> Self::OutputSelf<A, B, C, D, (), (), (), ()> {
        self.replace_1(fa)
    }

    fn replace_5<A, B, C, D, E>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
        _fd: &mut impl FnMut(Self::DOld) -> D,
        _fe: &mut impl FnMut(Self::EOld) -> E,
    ) -> Self::OutputSelf<A, B, C, D, E, (), (), ()> {
        self.replace_1(fa)
    }

    fn replace_6<A, B, C, D, E, F>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
        _fd: &mut impl FnMut(Self::DOld) -> D,
        _fe: &mut impl FnMut(Self::EOld) -> E,
        _ff: &mut impl FnMut(Self::FOld) -> F,
    ) -> Self::OutputSelf<A, B, C, D, E, F, (), ()> {
        self.replace_1(fa)
    }

    fn replace_7<A, B, C, D, E, F, G>(
        self,
        fa: &mut impl FnMut(Self::AOld) -> A,
        _fb: &mut impl FnMut(Self::BOld) -> B,
        _fc: &mut impl FnMut(Self::COld) -> C,
        _fd: &mut impl FnMut(Self::DOld) -> D,
        _fe: &mut impl FnMut(Self::EOld) -> E,
        _ff: &mut impl FnMut(Self::FOld) -> F,
        _fg: &mut impl FnMut(Self::GOld) -> G,
    ) -> Self::OutputSelf<A, B, C, D, E, F, G, ()> {
        self.replace_1(fa)
    }

    fn replace_8<A, B, C, D, E, F, G, H>(
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
        self.replace_1(fa)
    }
}

pub trait ReplaceInPlace: Sized {
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
    // where
    // Self::OutputSelf<
    //     A,
    //     Self::BOld,
    //     Self::COld,
    //     Self::DOld,
    //     Self::EOld,
    //     Self::FOld,
    //     Self::GOld,
    //     Self::HOld,
    // >: AssertSizes<Self> + AssertAlignments<Self>,
    // A: AssertSizes<Self::AOld> + AssertAlignments<Self::AOld>;

    // fn replace_in_place_2<A, B>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     fb: &mut impl FnMut(Self::BOld) -> B,
    // ) -> Self::OutputSelf<
    //     A,
    //     B,
    //     Self::COld,
    //     Self::DOld,
    //     Self::EOld,
    //     Self::FOld,
    //     Self::GOld,
    //     Self::HOld,
    // >;

    // fn replace_in_place_3<A, B, C>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     fb: &mut impl FnMut(Self::BOld) -> B,
    //     fc: &mut impl FnMut(Self::COld) -> C,
    // ) -> Self::OutputSelf<A, B, C, Self::DOld, Self::EOld, Self::FOld, Self::GOld, Self::HOld>;

    // fn replace_in_place_4<A, B, C, D>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     fb: &mut impl FnMut(Self::BOld) -> B,
    //     fc: &mut impl FnMut(Self::COld) -> C,
    //     fd: &mut impl FnMut(Self::DOld) -> D,
    // ) -> Self::OutputSelf<A, B, C, D, Self::EOld, Self::FOld, Self::GOld, Self::HOld>;

    // fn replace_in_place_5<A, B, C, D, E>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     fb: &mut impl FnMut(Self::BOld) -> B,
    //     fc: &mut impl FnMut(Self::COld) -> C,
    //     fd: &mut impl FnMut(Self::DOld) -> D,
    //     fe: &mut impl FnMut(Self::EOld) -> E,
    // ) -> Self::OutputSelf<A, B, C, D, E, Self::FOld, Self::GOld, Self::HOld>;

    // fn replace_in_place_6<A, B, C, D, E, F>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     fb: &mut impl FnMut(Self::BOld) -> B,
    //     fc: &mut impl FnMut(Self::COld) -> C,
    //     fd: &mut impl FnMut(Self::DOld) -> D,
    //     fe: &mut impl FnMut(Self::EOld) -> E,
    //     ff: &mut impl FnMut(Self::FOld) -> F,
    // ) -> Self::OutputSelf<A, B, C, D, E, F, Self::GOld, Self::HOld>;

    // fn replace_in_place_7<A, B, C, D, E, F, G>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     fb: &mut impl FnMut(Self::BOld) -> B,
    //     fc: &mut impl FnMut(Self::COld) -> C,
    //     fd: &mut impl FnMut(Self::DOld) -> D,
    //     fe: &mut impl FnMut(Self::EOld) -> E,
    //     ff: &mut impl FnMut(Self::FOld) -> F,
    //     fg: &mut impl FnMut(Self::GOld) -> G,
    // ) -> Self::OutputSelf<A, B, C, D, E, F, G, Self::HOld>;

    // fn replace_in_place_8<A, B, C, D, E, F, G, H>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     fb: &mut impl FnMut(Self::BOld) -> B,
    //     fc: &mut impl FnMut(Self::COld) -> C,
    //     fd: &mut impl FnMut(Self::DOld) -> D,
    //     fe: &mut impl FnMut(Self::EOld) -> E,
    //     ff: &mut impl FnMut(Self::FOld) -> F,
    //     fg: &mut impl FnMut(Self::GOld) -> G,
    //     fh: &mut impl FnMut(Self::HOld) -> H,
    // ) -> Self::OutputSelf<A, B, C, D, E, F, G, H>;
}

pub trait AssertSizes<T: Sized>: Sized {
    const ASSERT_EQUAL: () = {
        if mem::size_of::<Self>() != mem::size_of::<T>() {
            match mem::size_of::<Self>() {
                0 => panic!("The size of the Self type `0` is not equal to the size of T"),
                1 => panic!("The size of the Self type `1` is not equal to the size of T"),
                2 => panic!("The size of the Self type `2` is not equal to the size of T"),
                3 => panic!("The size of the Self type `3` is not equal to the size of T"),
                4 => panic!("The size of the Self type `4` is not equal to the size of T"),
                5 => panic!("The size of the Self type `5` is not equal to the size of T"),
                6 => panic!("The size of the Self type `6` is not equal to the size of T"),
                7 => panic!("The size of the Self type `7` is not equal to the size of T"),
                8 => panic!("The size of the Self type `8` is not equal to the size of T"),
                9 => panic!("The size of the Self type `9` is not equal to the size of T"),
                10 => panic!("The size of the Self type `10` is not equal to the size of T"),
                11 => panic!("The size of the Self type `11` is not equal to the size of T"),
                12 => panic!("The size of the Self type `12` is not equal to the size of T"),
                13 => panic!("The size of the Self type `13` is not equal to the size of T"),
                14 => panic!("The size of the Self type `14` is not equal to the size of T"),
                15 => panic!("The size of the Self type `15` is not equal to the size of T"),
                16 => panic!("The size of the Self type `16` is not equal to the size of T"),
                17 => panic!("The size of the Self type `17` is not equal to the size of T"),
                18 => panic!("The size of the Self type `18` is not equal to the size of T"),
                19 => panic!("The size of the Self type `19` is not equal to the size of T"),
                20 => panic!("The size of the Self type `20` is not equal to the size of T"),
                21 => panic!("The size of the Self type `21` is not equal to the size of T"),
                22 => panic!("The size of the Self type `22` is not equal to the size of T"),
                23 => panic!("The size of the Self type `23` is not equal to the size of T"),
                24 => panic!("The size of the Self type `24` is not equal to the size of T"),
                25 => panic!("The size of the Self type `25` is not equal to the size of T"),
                26 => panic!("The size of the Self type `26` is not equal to the size of T"),
                27 => panic!("The size of the Self type `27` is not equal to the size of T"),

                _ => panic!("The size of the Self type is not equal to the size of T"),
            }
        }
    };

    const ASSERT_LESS: () = {
        if !mem::size_of::<Self>() < mem::size_of::<T>() {
            panic!("The size of the Self type is greater than the size of T");
        }
    };

    const ASSERT_GREATER: () = {
        if !mem::size_of::<Self>() > mem::size_of::<T>() {
            panic!("The size of the Self type is less than the size of T");
        }
    };

    const ASSERT_LESS_OR_EQUAL: () = {
        if !mem::size_of::<Self>() <= mem::size_of::<T>() {
            panic!("The size of the Self type is greater than the size of T");
        }
    };

    const ASSERT_GREATER_OR_EQUAL: () = {
        if !mem::size_of::<Self>() >= mem::size_of::<T>() {
            panic!("The size of the Self type is less than the size of T");
        }
    };
}
impl<S: Sized, T: Sized> AssertSizes<T> for S {}

pub trait AssertAlignments<T: Sized>: Sized {
    const ASSERT_EQUAL: () = {
        if mem::align_of::<Self>() != mem::align_of::<T>() {
            panic!("The alignment of the Self type is not equal to the alignment of T");
        }
    };

    const ASSERT_LESS: () = {
        if !mem::align_of::<Self>() < mem::align_of::<T>() {
            panic!("The alignment of the Self type is greater than the alignment of T");
        }
    };

    const ASSERT_GREATER: () = {
        if !mem::align_of::<Self>() > mem::align_of::<T>() {
            panic!("The alignment of the Self type is less than the alignment of T");
        }
    };

    const ASSERT_LESS_OR_EQUAL: () = {
        if !mem::align_of::<Self>() <= mem::align_of::<T>() {
            panic!("The alignment of the Self type is greater than the alignment of T");
        }
    };

    const ASSERT_GREATER_OR_EQUAL: () = {
        if !mem::align_of::<Self>() >= mem::align_of::<T>() {
            panic!("The alignment of the Self type is less than the alignment of T");
        }
    };
}
impl<S: Sized, T: Sized> AssertAlignments<T> for S {}

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

    #[inline(always)]
    fn replace_in_place_1<A>(
        self,
        f: &mut impl FnMut(Self::AOld) -> A,
    ) -> Self::OutputSelf<A, (), (), (), (), (), (), ()> {
        #[allow(clippy::let_unit_value)]
        let _assert_new_self_type_is_same_size_as_old_self_type = <Self::OutputSelf<
            A,
            Self::BOld,
            Self::COld,
            Self::DOld,
            Self::EOld,
            Self::FOld,
            Self::GOld,
            Self::HOld,
        > as AssertSizes<Self>>::ASSERT_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_self_type_is_same_alignment_as_old_self_type = <Self::OutputSelf<
            A,
            Self::BOld,
            Self::COld,
            Self::DOld,
            Self::EOld,
            Self::FOld,
            Self::GOld,
            Self::HOld,
        > as AssertAlignments<Self>>::ASSERT_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_a_type_is_same_size_as_old_a_type =
            <A as AssertSizes<Self::AOld>>::ASSERT_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_a_type_is_same_alignment_as_old_a_type =
            <A as AssertAlignments<Self::AOld>>::ASSERT_EQUAL;

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

    // fn replace_in_place_2<A, B>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     _fb: &mut impl FnMut(Self::BOld) -> B,
    // ) -> Self::OutputSelf<A, B, (), (), (), (), (), ()> {
    //     self.replace_in_place_1(fa)
    // }

    // fn replace_in_place_3<A, B, C>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     _fb: &mut impl FnMut(Self::BOld) -> B,
    //     _fc: &mut impl FnMut(Self::COld) -> C,
    // ) -> Self::OutputSelf<A, B, C, (), (), (), (), ()> {
    //     self.replace_in_place_1(fa)
    // }

    // fn replace_in_place_4<A, B, C, D>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     _fb: &mut impl FnMut(Self::BOld) -> B,
    //     _fc: &mut impl FnMut(Self::COld) -> C,
    //     _fd: &mut impl FnMut(Self::DOld) -> D,
    // ) -> Self::OutputSelf<A, B, C, D, (), (), (), ()> {
    //     self.replace_in_place_1(fa)
    // }

    // fn replace_in_place_5<A, B, C, D, E>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     _fb: &mut impl FnMut(Self::BOld) -> B,
    //     _fc: &mut impl FnMut(Self::COld) -> C,
    //     _fd: &mut impl FnMut(Self::DOld) -> D,
    //     _fe: &mut impl FnMut(Self::EOld) -> E,
    // ) -> Self::OutputSelf<A, B, C, D, E, (), (), ()> {
    //     self.replace_in_place_1(fa)
    // }

    // fn replace_in_place_6<A, B, C, D, E, F>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     _fb: &mut impl FnMut(Self::BOld) -> B,
    //     _fc: &mut impl FnMut(Self::COld) -> C,
    //     _fd: &mut impl FnMut(Self::DOld) -> D,
    //     _fe: &mut impl FnMut(Self::EOld) -> E,
    //     _ff: &mut impl FnMut(Self::FOld) -> F,
    // ) -> Self::OutputSelf<A, B, C, D, E, F, (), ()> {
    //     self.replace_in_place_1(fa)
    // }

    // fn replace_in_place_7<A, B, C, D, E, F, G>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     _fb: &mut impl FnMut(Self::BOld) -> B,
    //     _fc: &mut impl FnMut(Self::COld) -> C,
    //     _fd: &mut impl FnMut(Self::DOld) -> D,
    //     _fe: &mut impl FnMut(Self::EOld) -> E,
    //     _ff: &mut impl FnMut(Self::FOld) -> F,
    //     _fg: &mut impl FnMut(Self::GOld) -> G,
    // ) -> Self::OutputSelf<A, B, C, D, E, F, G, ()> {
    //     self.replace_in_place_1(fa)
    // }

    // fn replace_in_place_8<A, B, C, D, E, F, G, H>(
    //     self,
    //     fa: &mut impl FnMut(Self::AOld) -> A,
    //     _fb: &mut impl FnMut(Self::BOld) -> B,
    //     _fc: &mut impl FnMut(Self::COld) -> C,
    //     _fd: &mut impl FnMut(Self::DOld) -> D,
    //     _fe: &mut impl FnMut(Self::EOld) -> E,
    //     _ff: &mut impl FnMut(Self::FOld) -> F,
    //     _fg: &mut impl FnMut(Self::GOld) -> G,
    //     _fh: &mut impl FnMut(Self::HOld) -> H,
    // ) -> Self::OutputSelf<A, B, C, D, E, F, G, H> {
    //     self.replace_in_place_1(fa)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_vec() {
        let v = vec![1, 2, 3];
        let v = v.replace_1(&mut |x| x.to_string());
        assert_eq!(v, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_replace_in_place_vec() {
        #[repr(align(8))]
        struct A(u32, u32);

        let v = vec![A(1, 2), A(3, 4)];
        let v = v.replace_in_place_1(&mut |A(x, y)| x as u64 * y as u64);
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

        let vec: Vec<AB> = vec.replace_in_place_1(&mut |abc| match abc {
            ABC::A(s) => AB::A(s),
            ABC::B(b) => AB::B(b),
            ABC::C(c) => AB::A(c.to_string()),
        });

        assert_eq!(
            vec,
            vec![AB::A("a".to_string()), AB::B(true), AB::A("1".to_string())]
        );

        let vec: Vec<A> = vec.replace_in_place_1(&mut |ab| match ab {
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

    // TODO use trybuild to test this fails to compile
    // #[test]
    // fn test_different_size_vec_elements() {
    //     let v = vec![1, 2, 3];
    //     let v = v.replace_in_place_1(&mut |x| (x, x));
    //     assert_eq!(v, vec![(1, 1), (2, 2), (3, 3)]);
    // }
}
