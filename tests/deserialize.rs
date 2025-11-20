use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
struct DeserializeTest {
    #[serde(rename = "hello-world")]
    hello_world: u32,
    foo: u32,
}

#[test]
fn should_deserialize() {
    let actual: DeserializeTestSerdeField = serde_json::from_str("\"hello-world\"").unwrap();
    assert_eq!(DeserializeTestSerdeField::HelloWorld, actual);
}
