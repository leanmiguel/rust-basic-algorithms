pub fn largest_sum_in_array(arr: &[Vec<i32>]) -> Vec<i32> {
    arr.into_iter()
        .map(|e| *e.iter().max().unwrap_or(&0))
        .collect()
}

#[test]
fn test_largest_sum_in_array_1() {
    let test = largest_sum_in_array(&[
        vec![4, 5, 1, 3],
        vec![13, 27, 18, 26],
        vec![32, 35, 37, 39],
        vec![1000, 1001, 857, 1],
    ]);
    let expected = vec![5, 27, 39, 1001];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_largest_sum_in_array_2() {
    let test = largest_sum_in_array(&[
        vec![13, 27, 18, 26],
        vec![4, 5, 1, 3],
        vec![32, 35, 37, 39],
        vec![1000, 1001, 857, 1],
    ]);
    let expected = vec![27, 5, 39, 1001];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_largest_sum_in_array_3() {
    let test = largest_sum_in_array(&[
        vec![4, 9, 1, 3],
        vec![13, 35, 18, 26],
        vec![32, 35, 97, 39],
        vec![1000000, 1001, 857, 1],
    ]);
    let expected = vec![9, 35, 97, 1000000];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_largest_sum_in_array_4() {
    let test = largest_sum_in_array(&[
        vec![17, 23, 25, 12],
        vec![25, 7, 34, 48],
        vec![4, -10, 18, 21],
        vec![-72, -3, -17, -10],
    ]);
    let expected = vec![25, 48, 21, -3];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}
