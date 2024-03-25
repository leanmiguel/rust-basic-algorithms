pub fn find_longest_word_length(word: &str) -> i32 {
    return 1;
}

#[test]
fn test_longest_length_quick_brown_fox() {
    let test = find_longest_word_length("The quick brown fox jumped over the lazy dog");
    let expected = 6;
    assert!(test == expected, "got: {}, expected: {}", test, expected)
}
