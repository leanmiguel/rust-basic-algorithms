pub fn largest_array_in_array(arr: &[Vec<i32>]) -> &Vec<i32> {
    match arr
        .into_iter()
        .enumerate()
        .map(|(i, list)| (i, list.into_iter().sum::<i32>()))
        .max_by(|(_, x), (_, y)| x.cmp(y))
    {
        Some((i, _)) => &arr[i],
        None => panic!("not valid"), // this should never be possible, how should this be handled?
    }
}
#[test]
fn test_largest_array_in_array_1() {
    let test_input = &[
        vec![4, 5, 1, 3],
        vec![13, 27, 18, 26],
        vec![32, 35, 37, 39],
        vec![1000, 1001, 857, 1],
    ];
    let test = largest_array_in_array(test_input);
    let expected = &vec![1000, 1001, 857, 1];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}
#[test]
fn test_largest_array_in_array_2() {
    let test_input = &[
        vec![13, 27, 18, 26],
        vec![4, 5, 1, 3],
        vec![32, 35, 37, 39],
        vec![1000, 1001, 857, 1],
    ];
    let test = largest_array_in_array(test_input);
    let expected = &vec![1000, 1001, 857, 1];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_largest_array_in_array_3() {
    let test_input = &[
        vec![4, 9, 1, 3],
        vec![13, 35, 18, 26],
        vec![32, 35, 97, 39],
        vec![1000000, 1001, 857, 1],
    ];
    let test = largest_array_in_array(test_input);
    let expected = &vec![1000000, 1001, 857, 1];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_largest_array_in_array_4() {
    let test_input = &[
        vec![17, 23, 25, 12],
        vec![25, 7, 34, 48],
        vec![4, -10, 18, 21],
        vec![-72, -3, -17, -10],
    ];
    let test = largest_array_in_array(test_input);
    let expected = &vec![25, 7, 34, 48];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}
