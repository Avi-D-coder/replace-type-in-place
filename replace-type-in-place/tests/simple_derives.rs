use replace_type_in_place::{Replace, ReplaceInPlace};

#[derive(ReplaceInPlace, Debug, PartialEq)]
struct NamedStruct<T> {
    field1: T,
    field2: Vec<T>,
}

#[derive(ReplaceInPlace, Debug, PartialEq)]
struct TupleStruct<T>(T, Vec<T>);

#[derive(ReplaceInPlace, Debug, PartialEq)]
enum TestEnum<T> {
    Variant1 { field: T },
    Variant2(T, Vec<T>),
    Variant3,
}

#[derive(ReplaceInPlace, Debug, PartialEq)]
struct GenericStruct<T, U> {
    field_t: T,
    field_u: U,
}

// #[test]
// fn test_replace_named_struct() {
//     let original = NamedStruct {
//         field1: 1,
//         field2: vec![2, 3],
//     };
//     let replaced = original.replace_1(&mut |x| x.to_string());
//     assert_eq!(
//         replaced,
//         NamedStruct {
//             field1: "1".to_string(),
//             field2: vec!["2".to_string(), "3".to_string()]
//         }
//     );
// }

#[test]
fn test_replace_in_place_named_struct() {
    let original = NamedStruct {
        field1: 1,
        field2: vec![2, 3],
    };
    let replaced = original.replace_in_place_1(&mut |x| x * 2);
    assert_eq!(
        replaced,
        NamedStruct {
            field1: 2,
            field2: vec![4, 6]
        }
    );
}

#[test]
fn test_replace_tuple_struct() {
    let original = TupleStruct(1, vec![2, 3]);
    let replaced = original.replace(&mut |x| x.to_string());
    assert_eq!(
        replaced,
        TupleStruct("1".to_string(), vec!["2".to_string(), "3".to_string()])
    );
}

#[test]
fn test_replace_in_place_tuple_struct() {
    let original = TupleStruct(1, vec![2, 3]);
    let replaced = original.replace_in_place_1(&mut |x| x * 2);
    assert_eq!(replaced, TupleStruct(2, vec![4, 6]));
}

// #[test]
// fn test_replace_enum() {
//     let original1 = TestEnum::Variant1 { field: 1 };
//     let original2 = TestEnum::Variant2(2, vec![3, 4]);
//     let original3 = TestEnum::Variant3;

//     let replaced1 = original1.replace(&mut |x| x.to_string());
//     let replaced2 = original2.replace(&mut |x| x.to_string());
//     let replaced3 = original3.replace(&mut |x: i32| x.to_string());

//     assert_eq!(
//         replaced1,
//         TestEnum::Variant1 {
//             field: "1".to_string()
//         }
//     );
//     assert_eq!(
//         replaced2,
//         TestEnum::Variant2("2".to_string(), vec!["3".to_string(), "4".to_string()])
//     );
//     assert_eq!(replaced3, TestEnum::Variant3);
// }

#[test]
fn test_replace_in_place_enum() {
    let original1 = TestEnum::Variant1 { field: 1 };
    let original2 = TestEnum::Variant2(2, vec![3, 4]);
    let original3 = TestEnum::Variant3;

    let replaced1 = original1.replace_in_place_1(&mut |x| x * 2);
    let replaced2 = original2.replace_in_place_1(&mut |x| x * 2);
    let replaced3 = original3.replace_in_place_1(&mut |x: i32| x * 2);

    assert_eq!(replaced1, TestEnum::Variant1 { field: 2 });
    assert_eq!(replaced2, TestEnum::Variant2(4, vec![6, 8]));
    assert_eq!(replaced3, TestEnum::Variant3);
}

// #[test]
// fn test_replace_generic_struct() {
//     let original = GenericStruct {
//         field_t: 1,
//         field_u: "hello".to_string(),
//         phantom: PhantomData,
//     };
//     let replaced = original.replace(&mut |x| x.to_string());
//     assert_eq!(
//         replaced,
//         GenericStruct {
//             field_t: "1".to_string(),
//             field_u: "hello".to_string(),
//             phantom: PhantomData
//         }
//     );
// }

// #[test]
// fn test_replace_in_place_generic_struct() {
//     let original = GenericStruct {
//         field_t: 1,
//         field_u: 2.5f32,
//         phantom: PhantomData,
//     };
//     let replaced = original.replace_in_place_1(&mut |x| x * 2);
//     assert_eq!(
//         replaced,
//         GenericStruct {
//             field_t: 2,
//             field_u: 5.0f32,
//             phantom: PhantomData
//         }
//     );
// }

#[test]
#[should_panic(expected = "The Old type is smaller than the New type you tried to replace it with")]
fn test_replace_in_place_size_check() {
    let original = NamedStruct {
        field1: 1i32,
        field2: vec![2, 3],
    };
    let _replaced = original.replace_in_place_1(&mut |x| x as i64);
}

#[test]
#[should_panic(
    expected = "The Old type has a different alignment than the New type you tried to replace it with"
)]
fn test_replace_in_place_alignment_check() {
    #[repr(align(8))]
    struct Aligned(i32);

    let original = NamedStruct {
        field1: Aligned(1),
        field2: vec![Aligned(2), Aligned(3)],
    };
    let _replaced = original.replace_in_place_1(&mut |Aligned(x)| x);
}
