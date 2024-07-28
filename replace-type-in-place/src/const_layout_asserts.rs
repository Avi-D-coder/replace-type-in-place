use core::mem;

pub trait AssertSizes<T: Sized>: Sized {
    const ASSERT_EQUAL: () = {
        if mem::size_of::<Self>() != mem::size_of::<T>() {
            panic!("The size of the Self type is not equal to the size of T");
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
