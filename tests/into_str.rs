use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
struct IntoStrTest {
    h: u32,
}

#[test]
fn should_convert_enum_into_str_when_using_from_trait() {
    use IntoStrTestSerdeField::*;
    let s: &str = H.into();
    assert_eq!(s, "h");
}
