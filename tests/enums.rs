use serde::{Deserialize, Serialize};
use serde_fields::SerdeField;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
enum BasicEnum {
    Unit,
    Tuple(u32),
    Struct { value: String },
}

#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "kebab-case")]
enum RenamedEnum {
    FirstVariant,
    #[serde(rename = "explicit")]
    SecondVariant,
    #[serde(skip)]
    SkippedVariant,
}

#[test]
fn should_generate_field_list_when_input_is_enum() {
    assert_eq!(BasicEnum::SERDE_FIELDS, &["Unit", "Tuple", "Struct"]);
}

#[test]
fn should_generate_enum_variants_when_input_is_enum() {
    assert_eq!(BasicEnumSerdeField::Unit.as_str(), "Unit");
    assert_eq!(BasicEnumSerdeField::Tuple.as_str(), "Tuple");
    assert_eq!(BasicEnumSerdeField::Struct.as_str(), "Struct");
}

#[test]
fn should_respect_serde_rename_all_rename_and_skip_for_enum_variants() {
    assert_eq!(RenamedEnum::SERDE_FIELDS, &["first-variant", "explicit"]);
    assert_eq!(
        RenamedEnumSerdeField::FirstVariant.as_str(),
        "first-variant"
    );
    assert_eq!(RenamedEnumSerdeField::SecondVariant.as_str(), "explicit");
    assert!(RenamedEnumSerdeField::try_from("skipped-variant").is_err());
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "kebab-case", rename_all_fields = "camelCase")]
enum StructLikeVariantEnum {
    StructVariant {
        first_field: u32,
        #[serde(rename = "explicit")]
        second_field: String,
        #[serde(skip)]
        skipped_field: bool,
    },
    OtherVariant,
}

#[test]
fn should_serialize_and_deserialize_generated_field_enum_for_enum_variants() {
    let serialized = serde_json::to_string(&RenamedEnumSerdeField::FirstVariant).unwrap();
    assert_eq!(serialized, "\"first-variant\"");

    let deserialized: RenamedEnumSerdeField = serde_json::from_str("\"explicit\"").unwrap();
    assert_eq!(deserialized, RenamedEnumSerdeField::SecondVariant);
}

#[test]
fn should_generate_nested_field_enum_for_struct_like_enum_variant() {
    assert_eq!(
        StructLikeVariantEnumStructVariantSerdeField::SERDE_FIELDS,
        &["firstField", "explicit"]
    );
    assert_eq!(
        StructLikeVariantEnumStructVariantSerdeField::FirstField.as_str(),
        "firstField"
    );
    assert_eq!(
        StructLikeVariantEnumStructVariantSerdeField::SecondField.as_str(),
        "explicit"
    );
    assert!(StructLikeVariantEnumStructVariantSerdeField::try_from("skippedField").is_err());
}

#[test]
fn should_serialize_and_deserialize_nested_field_enum_for_struct_like_enum_variant() {
    let serialized =
        serde_json::to_string(&StructLikeVariantEnumStructVariantSerdeField::FirstField).unwrap();
    assert_eq!(serialized, "\"firstField\"");

    let deserialized: StructLikeVariantEnumStructVariantSerdeField =
        serde_json::from_str("\"explicit\"").unwrap();
    assert_eq!(
        deserialized,
        StructLikeVariantEnumStructVariantSerdeField::SecondField
    );
}
