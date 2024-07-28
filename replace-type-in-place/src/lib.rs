#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

pub mod const_layout_asserts;

// #[cfg(feature = "derive")]
// pub use replace_type_in_place_derive::ReplaceInPlace;

use core::{
    mem::{self, ManuallyDrop},
    ptr,
};

use const_layout_asserts::{AssertAlignments, AssertSizes};

#[macro_export]
macro_rules! replace_in_place {
    ($self:expr) => {
        <_ as $crate::ReplaceInPlace>::replace_in_place_8(
            $self,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
        )
    };
    ($self:expr, $a:expr) => {
        <_ as $crate::ReplaceInPlace>::replace_in_place_8(
            $self,
            $a,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
        )
    };
    ($self:expr, $a:expr, $b:expr) => {
        <_ as $crate::ReplaceInPlace>::replace_in_place_8(
            $self,
            $a,
            $b,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
        )
    };
    ($self:expr, $a:expr, $b:expr, $c:expr) => {
        <_ as $crate::ReplaceInPlace>::replace_in_place_8(
            $self,
            $a,
            $b,
            $c,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
        )
    };
    ($self:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {
        <_ as $crate::ReplaceInPlace>::replace_in_place_8(
            $self,
            $a,
            $b,
            $c,
            $d,
            &|t| t,
            &|t| t,
            &|t| t,
            &|t| t,
        )
    };
    ($self:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        <_ as $crate::ReplaceInPlace>::replace_in_place_8(
            $self,
            $a,
            $b,
            $c,
            $d,
            $e,
            &|t| t,
            &|t| t,
            &|t| t,
        )
    };
    ($self:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        <_ as $crate::ReplaceInPlace>::replace_in_place_8(
            $self,
            $a,
            $b,
            $c,
            $d,
            $e,
            $f,
            &|t| t,
            &|t| t,
        )
    };
    ($self:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => {
        <_ as $crate::ReplaceInPlace>::replace_in_place_8($self, $a, $b, $c, $d, $e, $f, $g, &|t| t)
    };
    ($self:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr) => {
        <_ as $crate::ReplaceInPlace>::replace_in_place_8($self, $a, $b, $c, $d, $e, $f, $g, $h)
    };
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

    fn replace_in_place_8<A, B, C, D, E, F, G, H>(
        self,
        fa: &impl Fn(Self::AOld) -> A,
        fb: &impl Fn(Self::BOld) -> B,
        fc: &impl Fn(Self::COld) -> C,
        fd: &impl Fn(Self::DOld) -> D,
        fe: &impl Fn(Self::EOld) -> E,
        ff: &impl Fn(Self::FOld) -> F,
        fg: &impl Fn(Self::GOld) -> G,
        fh: &impl Fn(Self::HOld) -> H,
    ) -> Self::OutputSelf<A, B, C, D, E, F, G, H>;

    /// # Safety
    /// TODO
    unsafe fn replace_ptr_in_place_8<A, B, C, D, E, F, G, H>(
        self_ptr: *mut Self,
        fa: &impl Fn(Self::AOld) -> A,
        fb: &impl Fn(Self::BOld) -> B,
        fc: &impl Fn(Self::COld) -> C,
        fd: &impl Fn(Self::DOld) -> D,
        fe: &impl Fn(Self::EOld) -> E,
        ff: &impl Fn(Self::FOld) -> F,
        fg: &impl Fn(Self::GOld) -> G,
        fh: &impl Fn(Self::HOld) -> H,
    ) {
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

        unsafe {
            let self_owned = ptr::read(self_ptr);
            ptr::write(
                self_ptr as *mut Self::OutputSelf<A, B, C, D, E, F, G, H>,
                self_owned.replace_in_place_8(fa, fb, fc, fd, fe, ff, fg, fh),
            );
        }
    }
}

impl<AOld, BOld, COld, DOld> ReplaceInPlace for (AOld, BOld, COld, DOld) {
    type AOld = AOld;
    type BOld = BOld;
    type COld = COld;
    type DOld = DOld;
    type EOld = ();
    type FOld = ();
    type GOld = ();
    type HOld = ();
    type OutputSelf<A, B, C, D, E, F, G, H> = (A, B, C, D);

    #[inline(always)]
    fn replace_in_place_8<A, B, C, D, E, F, G, H>(
        self,
        fa: &impl Fn(Self::AOld) -> A,
        fb: &impl Fn(Self::BOld) -> B,
        fc: &impl Fn(Self::COld) -> C,
        fd: &impl Fn(Self::DOld) -> D,
        _fe: &impl Fn(Self::EOld) -> E,
        _ff: &impl Fn(Self::FOld) -> F,
        _fg: &impl Fn(Self::GOld) -> G,
        _fh: &impl Fn(Self::HOld) -> H,
    ) -> Self::OutputSelf<A, B, C, D, E, F, G, H> {
        #[allow(clippy::let_unit_value)]
        let _assert_new_self_type_size_less_or_equal = <Self::OutputSelf<
            A,
            B,
            C,
            D,
            Self::EOld,
            Self::FOld,
            Self::GOld,
            Self::HOld,
        > as AssertSizes<Self>>::ASSERT_LESS_OR_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_self_type_alignment_less_or_equal = <Self::OutputSelf<
            A,
            B,
            C,
            D,
            Self::EOld,
            Self::FOld,
            Self::GOld,
            Self::HOld,
        > as AssertAlignments<Self>>::ASSERT_LESS_OR_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_a_type_size_less_or_equal =
            <A as AssertSizes<Self::AOld>>::ASSERT_LESS_OR_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_a_type_alignment_less_or_equal =
            <A as AssertAlignments<Self::AOld>>::ASSERT_LESS_OR_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_b_type_size_less_or_equal =
            <B as AssertSizes<Self::BOld>>::ASSERT_LESS_OR_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_b_type_alignment_less_or_equal =
            <B as AssertAlignments<Self::BOld>>::ASSERT_LESS_OR_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_c_type_size_less_or_equal =
            <C as AssertSizes<Self::COld>>::ASSERT_LESS_OR_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_c_type_alignment_less_or_equal =
            <C as AssertAlignments<Self::COld>>::ASSERT_LESS_OR_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_d_type_size_less_or_equal =
            <D as AssertSizes<Self::DOld>>::ASSERT_LESS_OR_EQUAL;

        #[allow(clippy::let_unit_value)]
        let _assert_new_d_type_alignment_less_or_equal =
            <D as AssertAlignments<Self::DOld>>::ASSERT_LESS_OR_EQUAL;

        unsafe {
            let mut tuple = mem::ManuallyDrop::new(self);

            let old_a = ptr::addr_of_mut!(tuple.0);
            let new_a = fa(ptr::read(old_a));
            ptr::write(old_a as *mut A, new_a);

            let old_b = ptr::addr_of_mut!(tuple.1);
            let new_b = fb(ptr::read(old_b));
            ptr::write(old_b as *mut B, new_b);

            let old_c = ptr::addr_of_mut!(tuple.2);
            let new_c = fc(ptr::read(old_c));
            ptr::write(old_c as *mut C, new_c);

            let old_d = ptr::addr_of_mut!(tuple.3);
            let new_d = fd(ptr::read(old_d));
            ptr::write(old_d as *mut D, new_d);

            ptr::read(
                &tuple as *const _
                    as *const Self::OutputSelf<
                        A,
                        B,
                        C,
                        D,
                        Self::EOld,
                        Self::FOld,
                        Self::GOld,
                        Self::HOld,
                    >,
            )
        }
    }
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

    #[inline(always)]
    fn replace_in_place_8<A, B, C, D, E, F, G, H>(
        self,
        f: &impl Fn(Self::AOld) -> A,
        _fb: &impl Fn(Self::BOld) -> B,
        _fc: &impl Fn(Self::COld) -> C,
        _fd: &impl Fn(Self::DOld) -> D,
        _fe: &impl Fn(Self::EOld) -> E,
        _ff: &impl Fn(Self::FOld) -> F,
        _fg: &impl Fn(Self::GOld) -> G,
        _fh: &impl Fn(Self::HOld) -> H,
    ) -> Self::OutputSelf<A, B, C, D, E, F, G, H> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_in_place_vec() {
        #[repr(align(8))]
        struct A(u32, u32);

        let v = vec![A(1, 2), A(3, 4)];
        let v = replace_in_place!(v, &|A(x, y)| x as u64 * y as u64);
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

        let vec: Vec<AB> = replace_in_place!(vec, &|abc| match abc {
            ABC::A(s) => AB::A(s),
            ABC::B(b) => AB::B(b),
            ABC::C(c) => AB::A(c.to_string()),
        });

        assert_eq!(
            vec,
            vec![AB::A("a".to_string()), AB::B(true), AB::A("1".to_string())]
        );

        let vec: Vec<A> = replace_in_place!(vec, &|ab| match ab {
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
    //     let v = v.replace_in_place_1(&|x| (x, x));
    //     assert_eq!(v, vec![(1, 1), (2, 2), (3, 3)]);
    // }

    #[test]
    fn test_named_struct() {
        let named_struct = NamedStruct {
            field1: "a".to_string(),
            field2: vec!["b".to_string(), "c".to_string()],
            field3: 1,
        };

        let named_struct = replace_in_place!(named_struct, &|s| s.to_uppercase());
        assert_eq!(
            named_struct,
            NamedStruct {
                field1: "A".to_string(),
                field2: vec!["B".to_string(), "C".to_string()],
                field3: 1
            }
        );
    }

    #[derive(Debug, PartialEq)]
    struct NamedStruct<T> {
        field1: T,
        field2: Vec<T>,
        field3: u64,
    }

    impl<T> ReplaceInPlace for NamedStruct<T> {
        type AOld = T;
        type BOld = ();
        type COld = ();
        type DOld = ();
        type EOld = ();
        type FOld = ();
        type GOld = ();
        type HOld = ();
        type OutputSelf<A, B, C, D, E, F, G, H> = NamedStruct<A>;

        #[inline(always)]

        fn replace_in_place_8<A, B, C, D, E, F, G, H>(
            self,
            fa: &impl Fn(Self::AOld) -> A,
            _fb: &impl Fn(Self::BOld) -> B,
            _fc: &impl Fn(Self::COld) -> C,
            _fd: &impl Fn(Self::DOld) -> D,
            _fe: &impl Fn(Self::EOld) -> E,
            _ff: &impl Fn(Self::FOld) -> F,
            _fg: &impl Fn(Self::GOld) -> G,
            _fh: &impl Fn(Self::HOld) -> H,
        ) -> Self::OutputSelf<A, B, C, D, E, F, G, H> {
            #[allow(clippy::let_unit_value)]
            let _assert_new_self_type_size_less_or_equal = <Self::OutputSelf<
                A,
                Self::BOld,
                Self::COld,
                Self::DOld,
                Self::EOld,
                Self::FOld,
                Self::GOld,
                Self::HOld,
            > as AssertSizes<Self>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_self_type_alignment_less_or_equal = <Self::OutputSelf<
                A,
                Self::BOld,
                Self::COld,
                Self::DOld,
                Self::EOld,
                Self::FOld,
                Self::GOld,
                Self::HOld,
            > as AssertAlignments<Self>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_a_type_size_less_or_equal =
                <A as AssertSizes<Self::AOld>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_a_type_alignment_less_or_equal =
                <A as AssertAlignments<Self::AOld>>::ASSERT_LESS_OR_EQUAL;

            unsafe {
                let mut named_struct = mem::ManuallyDrop::new(self);
                let old_field1 = ptr::addr_of_mut!(named_struct.field1);
                let new_field1 = fa(ptr::read(old_field1));
                ptr::write(old_field1 as *mut A, new_field1);

                // Handle Vec<T> separately
                let old_field2 = ptr::addr_of_mut!(named_struct.field2);
                let new_field2 = replace_in_place!(ptr::read(old_field2), fa);
                ptr::write(old_field2 as _, new_field2);

                ptr::read(
                    &named_struct as *const _
                        as *const Self::OutputSelf<
                            A,
                            Self::BOld,
                            Self::COld,
                            Self::DOld,
                            Self::EOld,
                            Self::FOld,
                            Self::GOld,
                            Self::HOld,
                        >,
                )
            }
        }
    }

    #[test]
    fn test_tuple_struct() {
        let tuple_struct = TupleStruct(true, vec![1u32, 2, 3]);

        #[derive(Debug, PartialEq)]
        #[repr(align(4))]
        struct A(u16, u16);

        let tuple_struct = replace_in_place!(tuple_struct, &|b| A(b as u16, b as u16));
        assert_eq!(
            tuple_struct,
            TupleStruct(true, vec![A(1, 1), A(2, 2), A(3, 3)])
        );
    }

    #[derive(Debug, PartialEq)]
    struct TupleStruct<T>(bool, Vec<T>);

    impl<T> ReplaceInPlace for TupleStruct<T> {
        type AOld = T;
        type BOld = ();
        type COld = ();
        type DOld = ();
        type EOld = ();
        type FOld = ();
        type GOld = ();
        type HOld = ();
        type OutputSelf<A, B, C, D, E, F, G, H> = TupleStruct<A>;

        #[inline(always)]

        fn replace_in_place_8<A, B, C, D, E, F, G, H>(
            self,
            fa: &impl Fn(Self::AOld) -> A,
            _fb: &impl Fn(Self::BOld) -> B,
            _fc: &impl Fn(Self::COld) -> C,
            _fd: &impl Fn(Self::DOld) -> D,
            _fe: &impl Fn(Self::EOld) -> E,
            _ff: &impl Fn(Self::FOld) -> F,
            _fg: &impl Fn(Self::GOld) -> G,
            _fh: &impl Fn(Self::HOld) -> H,
        ) -> Self::OutputSelf<A, B, C, D, E, F, G, H> {
            #[allow(clippy::let_unit_value)]
            let _assert_new_self_type_size_less_or_equal = <Self::OutputSelf<
                A,
                Self::BOld,
                Self::COld,
                Self::DOld,
                Self::EOld,
                Self::FOld,
                Self::GOld,
                Self::HOld,
            > as AssertSizes<Self>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_self_type_alignment_less_or_equal = <Self::OutputSelf<
                A,
                Self::BOld,
                Self::COld,
                Self::DOld,
                Self::EOld,
                Self::FOld,
                Self::GOld,
                Self::HOld,
            > as AssertAlignments<Self>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_a_type_size_less_or_equal =
                <A as AssertSizes<Self::AOld>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_a_type_alignment_less_or_equal =
                <A as AssertAlignments<Self::AOld>>::ASSERT_LESS_OR_EQUAL;

            unsafe {
                let mut tuple_struct = mem::ManuallyDrop::new(self);
                let old_vec = ptr::addr_of_mut!(tuple_struct.1);
                let new_vec = replace_in_place!(ptr::read(old_vec), fa);
                ptr::write(old_vec as *mut Vec<A>, new_vec);

                ptr::read(
                    &tuple_struct as *const _
                        as *const Self::OutputSelf<
                            A,
                            Self::BOld,
                            Self::COld,
                            Self::DOld,
                            Self::EOld,
                            Self::FOld,
                            Self::GOld,
                            Self::HOld,
                        >,
                )
            }
        }
    }

    #[test]
    fn test_enum() {
        let test_enum: TestEnum<(u128, u128), bool, u64> = TestEnum::Variant3;

        let test_enum = replace_in_place!(test_enum, &|x| (x.0, x.1), &|x| !x, &|x| x * 2);

        assert_eq!(test_enum, TestEnum::Variant3);

        let test_enum: TestEnum<(u128, u128), bool, u64> = TestEnum::Variant1 { field: (1, 2) };

        let test_enum = replace_in_place!(test_enum, &|x| (x.0, x.1), &|x| !x, &|x| x * 2);

        assert_eq!(test_enum, TestEnum::Variant1 { field: (1, 2) });

        let test_enum: TestEnum<(u128, u128), bool, u64> = TestEnum::Variant2(true, vec![(1, 2)]);
        let test_enum = replace_in_place!(test_enum, &|x| (x.0, x.1), &|x| !x, &|x| x * 2);
        assert_eq!(test_enum, TestEnum::Variant2(false, vec![(1, 2)]));

        let test_enum: TestEnum<(u128, u128), bool, u64> = TestEnum::Variant4(1);
        let test_enum = replace_in_place!(test_enum, &|x| (x.0, x.1), &|x| !x, &|x| x != 0);
        assert_eq!(test_enum, TestEnum::Variant4(true));

        let test_enum: TestEnum<(u128, u128), bool, u64> = TestEnum::Variant5((1, 2, 3, (4, 5)));
        let test_enum = replace_in_place!(test_enum, &|x| (x.0, x.1), &|x| !x, &|x| x * 2);
        assert_eq!(test_enum, TestEnum::Variant5((1, 4, 6, (4, 5))));
    }

    #[derive(Debug, PartialEq)]
    enum TestEnum<A, B, C> {
        Variant1 { field: A },
        Variant2(B, Vec<A>),
        Variant3,
        Variant4(C),
        Variant5((u32, C, C, A)),
    }

    impl<A, B, C> ReplaceInPlace for TestEnum<A, B, C> {
        type AOld = A;
        type BOld = B;
        type COld = C;
        type DOld = ();
        type EOld = ();
        type FOld = ();
        type GOld = ();
        type HOld = ();
        type OutputSelf<NewA, NewB, NewC, D, E, F, G, H> = TestEnum<NewA, NewB, NewC>;

        #[inline(always)]

        fn replace_in_place_8<NewA, NewB, NewC, D, E, F, G, H>(
            self,
            fa: &impl Fn(Self::AOld) -> NewA,
            fb: &impl Fn(Self::BOld) -> NewB,
            fc: &impl Fn(Self::COld) -> NewC,
            _fd: &impl Fn(Self::DOld) -> D,
            _fe: &impl Fn(Self::EOld) -> E,
            _ff: &impl Fn(Self::FOld) -> F,
            _fg: &impl Fn(Self::GOld) -> G,
            _fh: &impl Fn(Self::HOld) -> H,
        ) -> Self::OutputSelf<NewA, NewB, NewC, D, E, F, G, H> {
            #[allow(clippy::let_unit_value)]
            let _assert_new_self_type_size_less_or_equal = <Self::OutputSelf<
                NewA,
                NewB,
                NewC,
                Self::DOld,
                Self::EOld,
                Self::FOld,
                Self::GOld,
                Self::HOld,
            > as AssertSizes<Self>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_self_type_alignment_less_or_equal = <Self::OutputSelf<
                NewA,
                NewB,
                NewC,
                Self::DOld,
                Self::EOld,
                Self::FOld,
                Self::GOld,
                Self::HOld,
            > as AssertAlignments<Self>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_a_type_size_less_or_equal =
                <NewA as AssertSizes<Self::AOld>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_a_type_alignment_less_or_equal =
                <NewA as AssertAlignments<Self::AOld>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_b_type_size_less_or_equal =
                <NewB as AssertSizes<Self::BOld>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_b_type_alignment_less_or_equal =
                <NewB as AssertAlignments<Self::BOld>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_c_type_size_less_or_equal =
                <NewC as AssertSizes<Self::COld>>::ASSERT_LESS_OR_EQUAL;

            #[allow(clippy::let_unit_value)]
            let _assert_new_c_type_alignment_less_or_equal =
                <NewC as AssertAlignments<Self::COld>>::ASSERT_LESS_OR_EQUAL;

            match self {
                TestEnum::Variant1 { field } => TestEnum::Variant1 { field: fa(field) },
                // Note
                TestEnum::Variant2(b, vec) => TestEnum::Variant2(
                    fb(b),
                    // Note: The parameter A is in vec so we can use replace_in_place_1(fa)
                    // You have to match arity of replace_in_place_1 and the order of the type parameters in the field type.
                    replace_in_place!(vec, fa),
                ),
                TestEnum::Variant3 => TestEnum::Variant3,
                TestEnum::Variant4(c) => TestEnum::Variant4(fc(c)),
                TestEnum::Variant5(tuple) => {
                    TestEnum::Variant5(replace_in_place!(tuple, &|x| x, fc, fc, fa))
                }
            }
        }
    }
}
