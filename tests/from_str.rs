use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
struct FromStrTest {
    has_thing: u32,
}

#[test]
fn should_parse_enum_when_using_from_str_for_valid_name() {
    use FromStrTestSerdeField::*;
    let parsed: FromStrTestSerdeField = "has_thing".parse().unwrap();
    assert_eq!(parsed, HasThing);
}

#[test]
fn should_fail_parsing_when_using_from_str_for_invalid_name() {
    let parsed: Result<FromStrTestSerdeField, _> = "nope".parse();
    assert!(parsed.is_err());
}
