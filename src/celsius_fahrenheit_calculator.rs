pub fn convert_c_to_f(celsius: f32) -> f32 {
    (celsius * (9. / 5.)) + 32.
}

#[test]
fn test_convert_c_to_f_minus_thirty() {
    let test = convert_c_to_f(-30.);
    let expected = -22.;
    assert!(
        test == expected,
        "failed: got: {}, expected: {}",
        test,
        expected
    );
}

#[test]
fn test_convert_c_to_f_minus_ten() {
    let test = convert_c_to_f(-10.);
    let expected = 14.;
    assert!(
        test == expected,
        "failed: got: {}, expected: {}",
        test,
        expected
    );
}

#[test]
fn test_convert_c_to_f_twenty() {
    let test = convert_c_to_f(20.);
    let expected = 68.;
    assert!(
        test == expected,
        "failed: got: {}, expected: {}",
        test,
        expected
    );
}

#[test]
fn test_convert_c_to_f_thirty() {
    let test = convert_c_to_f(30.);
    let expected = 86.;
    assert!(
        test == expected,
        "failed: got: {}, expected: {}",
        test,
        expected
    );
}
