use serde::{Deserialize, Serialize};
use serde_fields::SerdeFieldNames;

#[derive(Debug, Clone, Serialize, Deserialize, SerdeFieldNames)]
#[serde(rename_all = "camelCase")]
pub struct Foo {
    pub bar: String,
    pub baz_bat: u8,
}

#[test]
fn should_foo() {
    assert_eq!("bazBat", FooSerdeFields::BazBat.as_str());
    assert_eq!("bazBat", FooSerdeFields::BazBat.to_string());
}
