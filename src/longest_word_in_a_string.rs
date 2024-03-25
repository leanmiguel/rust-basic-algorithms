pub fn find_longest_word_length(word: &str) -> i32 {
    match word.split(" ").into_iter().map(|e| e.len()).max() {
        Some(num) => num as i32,
        None => 0,
    }
}

#[test]
fn test_longest_length_quick_brown_fox() {
    let test = find_longest_word_length("The quick brown fox jumped over the lazy dog");
    let expected = 6;
    assert!(test == expected, "got: {}, expected: {}", test, expected)
}

#[test]
fn test_longest_length_may_the_force() {
    let test = find_longest_word_length("May the force be with you");
    let expected = 5;
    assert!(test == expected, "got: {}, expected: {}", test, expected);
}

#[test]
fn test_longest_length_google_barrel_roll() {
    let test = find_longest_word_length("Google do a barrel roll");
    let expected = 6;
    assert!(test == expected, "got: {}, expected: {}", test, expected);
}

#[test]
fn test_longest_length_unladen_swallow() {
    let test =
        find_longest_word_length("What is the average airspeed velocity of an unladen swallow");
    let expected = 8;
    assert!(test == expected, "got: {}, expected: {}", test, expected);
}

#[test]
fn test_longest_length_super_long_word() {
    let test =
        find_longest_word_length("What if we try a super-long word such as otorhinolaryngology");
    let expected = 19;
    assert!(test == expected, "got: {}, expected: {}", test, expected);
}
