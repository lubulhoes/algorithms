use super::*;

struct BinarySearchCase<'a> {
    test_name: &'a str,
    arr: &'a [i32],
    target: i32,
    expected: Option<usize>,
}

#[test]
fn test_binary_search_struct_cases() {
    let cases = [
        BinarySearchCase { test_name: "middle element", arr: &[1, 3, 5, 7, 9], target: 5, expected: Some(2) },
        BinarySearchCase { test_name: "first element", arr: &[10, 20, 30, 40], target: 10, expected: Some(0) },
        BinarySearchCase { test_name: "last element", arr: &[2, 4, 6, 8, 10], target: 10, expected: Some(4) },
        BinarySearchCase { test_name: "not found", arr: &[1, 2, 3, 4, 5], target: 6, expected: None },
        BinarySearchCase { test_name: "empty array", arr: &[], target: 1, expected: None },
        BinarySearchCase { test_name: "multiple duplicates", arr: &[1, 2, 2, 2, 3], target: 2, expected: Some(1) }, // could be 1, 2, or 3
        BinarySearchCase { test_name: "single element found", arr: &[42], target: 42, expected: Some(0) },
        BinarySearchCase { test_name: "single element not found", arr: &[42], target: 21, expected: None },
    ];

    for case in cases {
        let result = binary_search(case.arr, case.target);
        if case.arr == &[1, 2, 2, 2, 3] && case.target == 2 {
            assert!(
                result == Some(1) || result == Some(2) || result == Some(3),
                "Failed [{}] for arr={:?}, target={}",
                case.test_name,
                case.arr,
                case.target
            );
        } else {
            assert_eq!(
                result,
                case.expected,
                "Failed [{}] for arr={:?}, target={}",
                case.test_name,
                case.arr,
                case.target
            );
        }
    }
}

struct LowerBoundCase<'a> {
    test_name: &'a str,
    arr: &'a [i32],
    target: i32,
    expected: Option<usize>,
}

#[test]
fn test_lower_bound_struct_cases() {
    let cases = [
        LowerBoundCase { test_name: "middle element", arr: &[1, 3, 5, 7, 9], target: 5, expected: Some(2) },
        LowerBoundCase { test_name: "multiple duplicates", arr: &[1, 3, 5, 5, 5, 7, 9], target: 5, expected: Some(2) },
        LowerBoundCase { test_name: "target less than all", arr: &[10, 20, 30], target: 1, expected: Some(0) },
        LowerBoundCase { test_name: "target greater than all", arr: &[1, 2, 3], target: 10, expected: None },
        LowerBoundCase { test_name: "between elements", arr: &[1, 3, 5, 7], target: 4, expected: Some(2) },
        LowerBoundCase { test_name: "first element", arr: &[2, 4, 6, 8], target: 2, expected: Some(0) },
        LowerBoundCase { test_name: "last element", arr: &[2, 4, 6, 8], target: 8, expected: Some(3) },
        LowerBoundCase { test_name: "all elements equal", arr: &[5, 5, 5, 5], target: 5, expected: Some(0) },
        LowerBoundCase { test_name: "target less than all equal", arr: &[5, 5, 5, 5], target: 4, expected: Some(0) },
        LowerBoundCase { test_name: "target greater than all equal", arr: &[5, 5, 5, 5], target: 6, expected: None },
        LowerBoundCase { test_name: "empty array", arr: &[], target: 5, expected: None },
    ];

    for case in cases {
        assert_eq!(
            lower_bound(case.arr, case.target),
            case.expected,
            "Failed [{}] for arr={:?}, target={}",
            case.test_name,
            case.arr,
            case.target
        );
    }
}

struct UpperBoundCase<'a> {
    test_name: &'a str,
    arr: &'a [i32],
    target: i32,
    expected: Option<usize>,
}

#[test]
fn test_upper_bound_struct_cases() {
    let cases = [
        UpperBoundCase { test_name: "middle element", arr: &[1, 3, 5, 7, 9], target: 5, expected: Some(3) },
        UpperBoundCase { test_name: "multiple duplicates", arr: &[1, 3, 5, 5, 5, 7, 9], target: 5, expected: Some(5) },
        UpperBoundCase { test_name: "target less than all", arr: &[10, 20, 30], target: 1, expected: Some(0) },
        UpperBoundCase { test_name: "target greater than all", arr: &[1, 2, 3], target: 10, expected: None },
        UpperBoundCase { test_name: "between elements", arr: &[1, 3, 5, 7], target: 4, expected: Some(2) },
        UpperBoundCase { test_name: "first element", arr: &[2, 4, 6, 8], target: 2, expected: Some(1) },
        UpperBoundCase { test_name: "last element", arr: &[2, 4, 6, 8], target: 8, expected: None }, // target == max
        UpperBoundCase { test_name: "all elements equal", arr: &[5, 5, 5, 5], target: 5, expected: None }, // target == max
        UpperBoundCase { test_name: "target less than all equal", arr: &[5, 5, 5, 5], target: 4, expected: Some(0) },
        UpperBoundCase { test_name: "target greater than all equal", arr: &[5, 5, 5, 5], target: 6, expected: None },
        UpperBoundCase { test_name: "empty array", arr: &[], target: 5, expected: None },
    ];

    for case in cases {
        assert_eq!(
            upper_bound(case.arr, case.target),
            case.expected,
            "Failed [{}] for arr={:?}, target={}",
            case.test_name,
            case.arr,
            case.target
        );
    }
}