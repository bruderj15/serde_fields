use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[derive(Debug, Clone, Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "camelCase")]
pub struct Foo {
    pub bar: String,
    pub baz_bat: u8,
}

#[test]
fn should_foo() {
    assert_eq!("bazBat", FooSerdeField::BazBat.as_str());
    assert_eq!("bazBat", FooSerdeField::BazBat.to_string());
}
