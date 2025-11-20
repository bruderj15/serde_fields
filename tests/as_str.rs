use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
struct TestAsStr {
    foo_bar: u32,
}

#[test]
fn should_return_serde_name_when_as_str_called() {
    use TestAsStrSerdeField::*;
    assert_eq!(FooBar.as_str(), "foo_bar");
}
