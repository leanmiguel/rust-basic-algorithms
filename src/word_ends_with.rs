pub fn word_ends_with(word: &str, ending: &str) -> bool {
    return &word[word.len() - ending.len()..] == ending;
}

#[test]
fn test_confirm_ending_congratulation() {
    let test = word_ends_with("Congratulation", "on");
    let expected = true;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_confirm_ending_connor() {
    let test = word_ends_with("Connor", "n");
    let expected = false;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_confirm_ending_walking_on_water() {
    let test = word_ends_with(
        "Walking on water and developing software from a specification are easy if both are frozen",
        "specification",
    );
    let expected = false;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_confirm_ending_he_has_to_give_me_a_new_name() {
    let test = word_ends_with("He has to give me a new name", "name");
    let expected = true;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_confirm_ending_open_sesame_same() {
    let test = word_ends_with("Open sesame", "same");
    let expected = true;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_confirm_ending_open_sesame_sage() {
    let test = word_ends_with("Open sesame", "sage");
    let expected = false;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_confirm_ending_open_sesame_game() {
    let test = word_ends_with("Open sesame", "game");
    let expected = false;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_confirm_ending_long_sentence_mountain() {
    let test = word_ends_with("If you want to save our world, you must hurry. We dont know how much longer we can withstand the nothing", "mountain");
    let expected = false;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_confirm_ending_abstraction() {
    let test = word_ends_with("Abstraction", "action");
    let expected = true;
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}
