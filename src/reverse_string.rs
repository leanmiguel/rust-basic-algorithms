pub fn reverse(word: &str) -> String {
    let mut reversed = String::new();
    for letter in word.chars().rev() {
        reversed.push(letter)
    }

    reversed
}

#[test]
fn test_reverse_hello() {
    let test = reverse("hello");
    let expected = "olleh";
    assert!(test == expected, "got: {}, expected: {}", test, expected)
}

#[test]
fn test_reverse_howdy() {
    let test = reverse("Howdy");
    let expected = "ydwoH";
    assert!(test == expected, "got: {}, expected: {}", test, expected)
}

#[test]
fn test_reverse_greetings_from_earth() {
    let test = reverse("Greetings from Earth");
    let expected = "htraE morf sgniteerG";
    assert!(test == expected, "got: {}, expected: {}", test, expected)
}
