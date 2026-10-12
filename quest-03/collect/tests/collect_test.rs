use collect::bubble_sort;

#[test]
fn test_bubble_sort() {
    let mut arr = [5, 3, 4, 1, 2];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);

    let mut arr = [3, 1, 3, 2, 1];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1, 1, 2, 3, 3]);

    let mut arr = [];
    bubble_sort(&mut arr);
    assert_eq!(arr, []);

    let mut arr = [1];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1]);

    let mut arr = [-5, 3, -1, 0];
    bubble_sort(&mut arr);
    assert_eq!(arr, [-5, -1, 0, 3]);
}
