use crate::tools::divide;

#[test]
fn division_by_zero() {
    let dividend = 42.0;
    let divisor = 0.0;

    let result = divide(dividend, divisor);
    assert!(result.is_err());
}

#[test]
fn division_by_infinite() {
    let dividend = 42.0;
    let divisor = 42.0/0.0;

    let result = divide(dividend, divisor);
    assert!(result.is_err());
}

#[test]
fn division_by_nan() {
    let dividend = 42.0;
    let divisor = 0.0/0.0;

    let result = divide(dividend, divisor);
    assert!(result.is_err());
}

#[test]
fn division_valid() {
    let dividend = 42.0;
    let divisor = 2.0;

    let result = divide(dividend, divisor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 21.0);
}