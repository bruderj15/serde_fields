use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[derive(Serialize, Deserialize, SerdeField)]
struct Basic {
    a: u32,
    b: String,
}

#[test]
fn should_generate_field_list_when_struct_is_basic() {
    assert_eq!(Basic::SERDE_FIELDS, &["a", "b"]);
}

#[test]
fn should_generate_enum_variants_when_struct_is_basic() {
    use BasicSerdeField::*;
    assert_eq!(A.as_str(), "a");
    assert_eq!(B.as_str(), "b");
}
