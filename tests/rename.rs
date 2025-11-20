use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[derive(Serialize, Deserialize, SerdeField)]
struct Renamed {
    #[serde(rename = "x")]
    a: u32,
}

#[test]
fn should_use_explicit_serde_rename_when_present() {
    assert_eq!(Renamed::SERDE_FIELDS, &["x"]);
}

#[test]
fn should_generate_variant_with_renamed_value_when_field_is_renamed() {
    use RenamedSerdeField::*;
    assert_eq!(A.as_str(), "x");
}
