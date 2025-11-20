use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "camelCase")]
struct Camel {
    first_field: i32,
    second_field: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "snake_case")]
struct Snake {
    first_field: i32,
    second_field: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "kebab-case")]
struct Kebab {
    first_field: i32,
    second_field: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamSnake {
    first_field: i32,
    second_field: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
struct ScreamKebab {
    first_field: i32,
    second_field: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "lowercase")]
struct Lower {
    first_field: i32,
    second_field: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "UPPERCASE")]
struct Upper {
    first_field: i32,
    second_field: String,
}

#[test]
fn should_respect_rename_all_camel() {
    assert_eq!(CamelSerdeField::FirstField.as_str(), "firstField");
}

#[test]
fn should_respect_rename_all_snake() {
    assert_eq!(SnakeSerdeField::FirstField.as_str(), "first_field");
}

#[test]
fn should_respect_rename_all_kebab() {
    assert_eq!(KebabSerdeField::FirstField.as_str(), "first-field");
}

#[test]
fn should_respect_rename_all_scream_snake() {
    assert_eq!(ScreamSnakeSerdeField::FirstField.as_str(), "FIRST_FIELD");
}

#[test]
fn should_respect_rename_all_scream_kebab() {
    assert_eq!(ScreamKebabSerdeField::FirstField.as_str(), "FIRST-FIELD");
}

#[test]
fn should_respect_rename_all_lower() {
    assert_eq!(LowerSerdeField::FirstField.as_str(), "firstfield");
}

#[test]
fn should_respect_rename_all_upper() {
    assert_eq!(UpperSerdeField::FirstField.as_str(), "FIRSTFIELD");
}
