pub fn get_index_to_ins(arr: &[i32], insert_num: i32) -> i32 {
    let mut copy: Vec<i32> = Vec::new();
    arr.clone_into(&mut copy);
    copy.sort();

    for (i, val) in copy.iter().enumerate() {
        if insert_num <= *val {
            return i as i32;
        }
    }
    
    copy.iter().len() as i32
}

#[test]
fn test_get_index_to_ins_1() {
    let test = get_index_to_ins(&[10, 20, 30, 40, 50], 35);
    let expected = 3;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_get_index_to_ins_2() {
    let test = get_index_to_ins(&[10, 20, 30, 40, 50], 30);
    let expected = 2;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_get_index_to_ins_3() {
    let test = get_index_to_ins(&[40, 60], 50);
    let expected = 1;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_get_index_to_ins_4() {
    let test = get_index_to_ins(&[3, 10, 5], 3);
    let expected = 0;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_get_index_to_ins_5() {
    let test = get_index_to_ins(&[5, 3, 20, 3], 5);
    let expected = 2;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_get_index_to_ins_6() {
    let test = get_index_to_ins(&[2, 20, 10], 19);
    let expected = 2;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_get_index_to_ins_7() {
    let test = get_index_to_ins(&[2, 5, 10], 15);
    let expected = 3;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_get_index_to_ins_8() {
    let test = get_index_to_ins(&[], 1);
    let expected = 0;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}
