use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
struct WithSkip {
    a: u32,
    #[serde(skip)]
    skipped: u32,
}

#[test]
fn should_skip_field_when_serde_skip_is_present() {
    assert_eq!(WithSkip::SERDE_FIELDS, &["a"]);
}

#[test]
fn should_not_generate_enum_variant_when_field_is_skipped() {
    use WithSkipSerdeField::*;
    assert_eq!(A.as_str(), "a");
}
