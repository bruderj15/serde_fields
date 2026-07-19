# serde_fields

[![Crates.io](https://img.shields.io/crates/v/serde_fields)](https://crates.io/crates/serde_fields)
[![Docs.rs](https://img.shields.io/docsrs/serde_fields)](https://docs.rs/serde_fields)
[![License](https://img.shields.io/crates/l/serde_fields)](https://github.com/bruderj15/serde_fields/blob/main/LICENSE)

A procedural macro to generate **field/variant name enums and constants** for structs and enums using Serde, respecting `#[serde(rename = "...")]` and `#[serde(rename_all = "...")]`.

---

## Features

- Automatically generate a `const SERDE_FIELDS: &'static [&'static str]` array containing the serialized names of all non-skipped struct fields or enum variants.
- Generate an enum named `{TypeName}SerdeField` for all non-skipped fields or variants.
- Generated enum variants match Rust field names (PascalCase) for structs and original variant names for enums. They are annotated with `#[serde(rename = "...")]`, matching the serialized names of the original type, and are (de)serializable.
- For struct-like enum variants, generate nested enums named `{EnumName}{VariantName}SerdeField` for the variant's non-skipped fields.
- Provides convenient methods and trait implementations:
  - `as_str() -> &'static str`
  - `Display` implementation
  - `From<Enum>` and `From<&Enum>` for `&'static str`
  - `TryFrom<&str>` and `TryFrom<String>` with custom error `Invalid{TypeName}SerdeField`
  - `FromStr` implementation
  - `AsRef<str>` for ergonomic usage
- Supports skipped fields/variants via `#[serde(skip)]` and renaming via `#[serde(rename = "...")]`.
- Fully respects type-level `#[serde(rename_all = "...")]`, and enum `#[serde(rename_all_fields = "...")]` for struct-like variant fields.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["serde_derive"] }
serde_fields = "0.1"
```

## Usage

```rust
use serde::{Serialize, Deserialize};
use serde_fields::SerdeField;

#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u32,
    #[serde(rename = "eMail")]
    email: String,
    foo_bar: String,
}

// Access serialized field names as a slice
assert_eq!(User::SERDE_FIELDS, &["userId", "eMail", "fooBar"]);

// Use the generated enum
let field = UserSerdeField::UserId;
assert_eq!(field.as_str(), "userId");
assert_eq!(field.to_string(), "userId");

// Parse enum from string
let parsed: UserSerdeField = "userId".parse().unwrap();
assert_eq!(parsed, UserSerdeField::UserId);

// Convert enum to string slice
let name: &str = UserSerdeField::Email.into();
assert_eq!(name, "eMail");

// Serialize
let serialized = serde_json::to_string(&UserSerdeField::FooBar).unwrap();
assert_eq!("\"fooBar\"", serialized);

#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "kebab-case", rename_all_fields = "camelCase")]
enum Event {
    UserCreated,
    #[serde(rename = "deleted")]
    UserDeleted,
    PayloadReceived {
        payload_id: u32,
        #[serde(rename = "data")]
        payload_data: String,
    },
}

assert_eq!(
    Event::SERDE_FIELDS,
    &["user-created", "deleted", "payload-received"]
);
assert_eq!(EventSerdeField::UserCreated.as_str(), "user-created");
assert_eq!(
    EventPayloadReceivedSerdeField::SERDE_FIELDS,
    &["payloadId", "data"]
);
assert_eq!(
    EventPayloadReceivedSerdeField::PayloadId.as_str(),
    "payloadId"
);
```
