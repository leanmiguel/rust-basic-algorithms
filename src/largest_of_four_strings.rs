pub fn largest_of_four_nums(arr: &[[i32; 4]]) -> Vec<i32> {
    match arr
        .into_iter()
        .enumerate()
        .map(|(i, list)| (i, list.into_iter().sum::<i32>()))
        .max_by(|(_, x), (_, y)| x.cmp(y))
    {
        Some((i, _)) => arr[i].into_iter().collect(),
        None => panic!("not valid"), // this should never be possible, how should this be handled?
    }
}

#[test]
fn test_largest_of_four_nums_1() {
    let test = largest_of_four_nums(&[
        [4, 5, 1, 3],
        [13, 27, 18, 26],
        [32, 35, 37, 39],
        [1000, 1001, 857, 1],
    ]);
    let expected = vec![5, 27, 39, 1001];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_largest_of_four_nums_2() {
    let test = largest_of_four_nums(&[
        [13, 27, 18, 26],
        [4, 5, 1, 3],
        [32, 35, 37, 39],
        [1000, 1001, 857, 1],
    ]);
    let expected = vec![27, 5, 39, 1001];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_largest_of_four_nums_3() {
    let test = largest_of_four_nums(&[
        [4, 9, 1, 3],
        [13, 35, 18, 26],
        [32, 35, 97, 39],
        [1000000, 1001, 857, 1],
    ]);
    let expected = vec![9, 35, 97, 1000000];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_largest_of_four_nums_4() {
    let test = largest_of_four_nums(&[
        [17, 23, 25, 12],
        [25, 7, 34, 48],
        [4, -10, 18, 21],
        [-72, -3, -17, -10],
    ]);
    let expected = vec![25, 48, 21, -3];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}
