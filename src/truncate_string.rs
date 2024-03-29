pub fn truncate_string(word: &str, cap: i32) -> String {
    let mut truncated = String::new();

    truncated.push_str(
        &word
            .chars()
            .take(cap as usize)
            .collect::<String>()
            .to_string(),
    );

    if word.len() > cap as usize {
        truncated.push_str("...");
    }

    return truncated;
}

#[test]
fn test_truncate_string_a_tisket_8() {
    let test = truncate_string("A-tisket a-tasket A green and yellow basket", 8);
    let expected = "A-tisket...";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_truncate_string_peter_piper_11() {
    let test = truncate_string("Peter Piper picked a peck of pickled peppers", 11);
    let expected = "Peter Piper...";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_truncate_string_full_length() {
    let input = "A-tisket a-tasket A green and yellow basket";
    let test = truncate_string(input, input.len() as i32);
    let expected = input;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_truncate_string_length_plus_two() {
    let input = "A-tisket a-tasket A green and yellow basket";
    let test = truncate_string(input, input.len() as i32 + 2);
    let expected = input;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_truncate_string_a_1() {
    let test = truncate_string("A-", 1);
    let expected = "A...";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_truncate_string_absolutely_longer_2() {
    let test = truncate_string("Absolutely Longer", 2);
    let expected = "Ab...";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}
