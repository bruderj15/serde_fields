use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
struct ErrorTest {
    x: u32,
    foo: String,
}

#[test]
fn should_store_input_string_in_error_when_invalid_field_name_provided() {
    let err = ErrorTestSerdeField::try_from("unknown").unwrap_err();
    assert_eq!(err.0, "unknown");
}

#[test]
fn should_format_error_correctly_when_displayed() {
    let err = InvalidErrorTestSerdeField("oops".into());
    assert_eq!(
        format!("{}", err),
        "InvalidErrorTestSerdeField: Got 'oops'. Expected any of [\"x\", \"foo\"]."
    );
}
