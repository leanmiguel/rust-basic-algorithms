// quite a bit messy, and the logic can be cleaned up but the fcc test cases work lol
pub fn chunk_array_in_groups(arr: &[i32], size: i32) -> Vec<Vec<i32>> {
    let mut chunked:Vec<Vec<i32>> = Vec::new();

    let mut current_chunk:Vec<i32> = Vec::new();
    
    let mut current_count = 0;

    for item in arr {
        if current_count != 0 && current_count % size == 0 {
            chunked.push(current_chunk); 
            current_chunk = Vec::new(); // seems that even if you move the value, if you reinit it, it's fine
            // the error is use of moved value, which means we're using a pointer to something that's been dropped (in this case, chunked.push)
        }
        current_count += 1;
        current_chunk.push(*item);    
    }

    if current_chunk.len() != 0 {
        chunked.push(current_chunk);
    }

    
    chunked
}

#[test]
fn test_chunk_array_in_groups_2() {
    let test = chunk_array_in_groups(&[0, 1, 2, 3, 4, 5], 3);
    let expected = vec![vec![0, 1, 2], vec![3, 4, 5]];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_chunk_array_in_groups_3() {
    let test = chunk_array_in_groups(&[0, 1, 2, 3, 4, 5], 2);
    let expected = vec![vec![0, 1], vec![2, 3], vec![4, 5]];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_chunk_array_in_groups_4() {
    let test = chunk_array_in_groups(&[0, 1, 2, 3, 4, 5], 4);
    let expected = vec![vec![0, 1, 2, 3], vec![4, 5]];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_chunk_array_in_groups_5() {
    let test = chunk_array_in_groups(&[0, 1, 2, 3, 4, 5, 6], 3);
    let expected = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6]];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_chunk_array_in_groups_6() {
    let test = chunk_array_in_groups(&[0, 1, 2, 3, 4, 5, 6, 7, 8], 4);
    let expected = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8]];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_chunk_array_in_groups_7() {
    let test = chunk_array_in_groups(&[0, 1, 2, 3, 4, 5, 6, 7, 8], 2);
    let expected = vec![vec![0, 1], vec![2, 3], vec![4, 5], vec![6, 7], vec![8]];
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}
