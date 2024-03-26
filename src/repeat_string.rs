pub fn repeat_string(word: &str, times: i32) -> String {
    let mut repeated = String::new();

    for _ in 0..times {
        repeated.push_str(word)
    }

    repeated
}

#[test]
fn test_repeat_string_star_3() {
    let test = repeat_string("*", 3);
    let expected = "***";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_repeat_string_abc_3() {
    let test = repeat_string("abc", 3);
    let expected = "abcabcabc";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_repeat_string_abc_4() {
    let test = repeat_string("abc", 4);
    let expected = "abcabcabcabc";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_repeat_string_abc_1() {
    let test = repeat_string("abc", 1);
    let expected = "abc";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_repeat_string_star_8() {
    let test = repeat_string("*", 8);
    let expected = "********";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_repeat_string_abc_negative_2() {
    let test = repeat_string("abc", -2);
    let expected = "";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}
