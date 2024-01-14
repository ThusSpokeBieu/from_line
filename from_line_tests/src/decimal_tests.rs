use from_line::traits::from_string::FromString;
use rust_decimal::Decimal;

#[test]
fn test_decimal_from_string() {
    assert_eq!(
        Decimal::from_string("3.14").unwrap(),
        Decimal::from_str_exact("3.14").unwrap()
    );
}
