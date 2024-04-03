// not efficient, but easy lol
pub fn is_mutated(mutated: &str, original: &str) -> bool {
    for letter in original.to_lowercase().chars() {
        if !mutated.to_lowercase().contains(letter) {
            return false;
        }
    }

    true
}

#[test]
fn test_mutation_hello_hey() {
    assert_eq!(
        is_mutated("hello", "hey"),
        false,
        "Test failed: 'hello' vs 'hey'"
    );
}

#[test]
fn test_mutation_hello_hello() {
    assert_eq!(
        is_mutated("hello", "Hello"),
        true,
        "Test failed: 'hello' vs 'Hello'"
    );
}

#[test]
fn test_mutation_zyxwvutsrqponmlkjihgfedcba_qrstu() {
    assert_eq!(
        is_mutated("zyxwvutsrqponmlkjihgfedcba", "qrstu"),
        true,
        "Test failed: 'zyxwvutsrqponmlkjihgfedcba' vs 'qrstu'"
    );
}

#[test]
fn test_mutation_mary_army() {
    assert_eq!(
        is_mutated("Mary", "Army"),
        true,
        "Test failed: 'Mary' vs 'Army'"
    );
}

#[test]
fn test_mutation_mary_aarmy() {
    assert_eq!(
        is_mutated("Mary", "Aarmy"),
        true,
        "Test failed: 'Mary' vs 'Aarmy'"
    );
}

#[test]
fn test_mutation_alien_line() {
    assert_eq!(
        is_mutated("Alien", "line"),
        true,
        "Test failed: 'Alien' vs 'line'"
    );
}

#[test]
fn test_mutation_floor_for() {
    assert_eq!(
        is_mutated("floor", "for"),
        true,
        "Test failed: 'floor' vs 'for'"
    );
}

#[test]
fn test_mutation_hello_neo() {
    assert_eq!(
        is_mutated("hello", "neo"),
        false,
        "Test failed: 'hello' vs 'neo'"
    );
}

#[test]
fn test_mutation_voodoo_no() {
    assert_eq!(
        is_mutated("voodoo", "no"),
        false,
        "Test failed: 'voodoo' vs 'no'"
    );
}

#[test]
fn test_mutation_ate_date() {
    assert_eq!(
        is_mutated("ate", "date"),
        false,
        "Test failed: 'ate' vs 'date'"
    );
}

#[test]
fn test_mutation_tiger_zebra() {
    assert_eq!(
        is_mutated("Tiger", "Zebra"),
        false,
        "Test failed: 'Tiger' vs 'Zebra'"
    );
}

#[test]
fn test_mutation_noel_ole() {
    assert_eq!(
        is_mutated("Noel", "Ole"),
        true,
        "Test failed: 'Noel' vs 'Ole'"
    );
}
