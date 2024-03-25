pub fn factorialize(num: i64) -> i64 {
    if num == 0 {
        return 1;
    }

    num * factorialize(num - 1)
}

#[test]
fn test_factorialize_five() {
    let test = factorialize(5);
    let expected = 120;
    assert!(test == expected, "got: {}, expected: {}", test, expected)
}

#[test]
fn test_factorialize_ten() {
    let test = factorialize(10);
    let expected = 3628800;
    assert!(test == expected, "got: {}, expected: {}", test, expected);
}

#[test]
fn test_factorialize_twenty() {
    let test = factorialize(20);
    let expected = 2432902008176640000;
    assert!(test == expected, "got: {}, expected: {}", test, expected);
}

#[test]
fn test_factorialize_zero() {
    let test = factorialize(0);
    let expected = 1;
    assert!(test == expected, "got: {}, expected: {}", test, expected);
}
