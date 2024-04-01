pub fn title_case(sentence: &str) -> String {
    let mut title_cased = String::new();

    let mut words = sentence.split_ascii_whitespace().peekable();

    while let Some(word) = words.next() {
        let mut title_case = String::new();

        let first = &word[..=0];
        title_case.push_str(first.to_uppercase().as_str());

        let rest = &word[1..];
        title_case.push_str(rest.to_lowercase().as_str());

        if words.peek().is_some() {
            title_case.push(' ');
        }

        title_cased.push_str(title_case.as_str());
    }

    title_cased
}

// it'd be nice to have tests which push the boundaries here
// what about japanese characters? (title sense doesn't make sense, but we should guard against it)
// what about chars with accents?
// what about what about 1 letter strings

#[test]
fn test_title_case_tea_pot() {
    let test = title_case("I'm a little tea pot");
    let expected = "I'm A Little Tea Pot";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_title_case_short_and_stout() {
    let test = title_case("sHoRt AnD sToUt");
    let expected = "Short And Stout";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}

#[test]
fn test_title_case_handle_spout() {
    let test = title_case("HERE IS MY HANDLE HERE IS MY SPOUT");
    let expected = "Here Is My Handle Here Is My Spout";
    assert_eq!(
        test, expected,
        "Test failed: got {:?}, expected {:?}",
        test, expected
    );
}
