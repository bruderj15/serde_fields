use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
struct AsRefTest {
    my_field: u32,
}

#[test]
fn should_return_str_when_as_ref_is_used() {
    use AsRefTestSerdeField::*;
    let s: &str = MyField.as_ref();
    assert_eq!(s, "my_field");
}
