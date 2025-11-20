use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
struct DisplayTest {
    val: u32,
}

#[test]
fn should_display_serde_field_name_when_display_is_called() {
    use DisplayTestSerdeField::*;
    assert_eq!(format!("{}", Val), "val");
}
