extern crate sorting_lib;
use sorting_lib::sorting::*;
use std::cmp::Ordering;

fn num_cmp(a: &i32, b: &i32) -> Ordering {
    a.cmp(b)
}

#[test]
fn test_empty_vectors() {
    let mut vec: Vec<i32> = vec![];
    quick_sort(&mut vec, &num_cmp);
    assert_eq!(vec, []);

    let mut vec2 = vec![];
    merge_sort(&mut vec2, &num_cmp);
    assert_eq!(vec2, []);

    let mut vec3 = vec![];
    insertion_sort(&mut vec3, &num_cmp);
    assert_eq!(vec3, []);

    let mut vec4 = vec![];
    selection_sort(&mut vec4, &num_cmp);
    assert_eq!(vec4, []);
}

#[test]
fn test_repeated_elements() {
    let mut vec = vec![2, 2, 2, 2];
    quick_sort(&mut vec, &num_cmp);
    assert_eq!(vec, [2, 2, 2, 2]);

    let mut vec2 = vec![2, 2, 2, 2];
    merge_sort(&mut vec2, &num_cmp);
    assert_eq!(vec2, [2, 2, 2, 2]);

    let mut vec3 = vec![2, 2, 2, 2];
    insertion_sort(&mut vec3, &num_cmp);
    assert_eq!(vec3, [2, 2, 2, 2]);

    let mut vec4 = vec![2, 2, 2, 2];
    selection_sort(&mut vec4, &num_cmp);
    assert_eq!(vec4, [2, 2, 2, 2]);
}

#[test]
fn test_reverse_order() {
    let mut vec = vec![5, 4, 3, 2, 1];
    quick_sort(&mut vec, &num_cmp);
    assert_eq!(vec, [1, 2, 3, 4, 5]);

    let mut vec2 = vec![5, 4, 3, 2, 1];
    merge_sort(&mut vec2, &num_cmp);
    assert_eq!(vec2, [1, 2, 3, 4, 5]);

    let mut vec3 = vec![5, 4, 3, 2, 1];
    insertion_sort(&mut vec3, &num_cmp);
    assert_eq!(vec3, [1, 2, 3, 4, 5]);

    let mut vec4 = vec![5, 4, 3, 2, 1];
    selection_sort(&mut vec4, &num_cmp);
    assert_eq!(vec4, [1, 2, 3, 4, 5]);
}

#[test]
fn test_mixed_numbers() {
    let mut vec = vec![-1, 0, 3, -2, 5, -3];
    quick_sort(&mut vec, &num_cmp);
    assert_eq!(vec, [-3, -2, -1, 0, 3, 5]);

    let mut vec2 = vec![-1, 0, 3, -2, 5, -3];
    merge_sort(&mut vec2, &num_cmp);
    assert_eq!(vec2, [-3, -2, -1, 0, 3, 5]);

    let mut vec3 = vec![-1, 0, 3, -2, 5, -3];
    insertion_sort(&mut vec3, &num_cmp);
    assert_eq!(vec3, [-3, -2, -1, 0, 3, 5]);

    let mut vec4 = vec![-1, 0, 3, -2, 5, -3];
    selection_sort(&mut vec4, &num_cmp);
    assert_eq!(vec4, [-3, -2, -1, 0, 3, 5]);
}
