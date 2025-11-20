use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
struct TryFromTest {
    value: u32,
}

#[test]
fn should_convert_from_str_when_value_is_valid() {
    use TryFromTestSerdeField::*;
    let result = TryFromTestSerdeField::try_from("value");
    assert_eq!(result.unwrap(), Value);
}

#[test]
fn should_return_error_when_try_from_receives_invalid_name() {
    let result = TryFromTestSerdeField::try_from("invalid");
    assert!(result.is_err());
}
