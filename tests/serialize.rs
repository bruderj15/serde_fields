use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
struct SerializeTest {
    #[serde(rename = "hello-world")]
    hello_world: u32,
    foo: u32,
}

#[test]
fn should_serialize() {
    let actual = serde_json::to_string(&SerializeTestSerdeField::HelloWorld).unwrap();
    assert_eq!("\"hello-world\"", actual);
}
