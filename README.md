# serde_fields

[![Crates.io](https://img.shields.io/crates/v/serde_fields)](https://crates.io/crates/serde_fields)
[![Docs.rs](https://img.shields.io/docsrs/serde_fields)](https://docs.rs/serde_fields)
[![License](https://img.shields.io/crates/l/serde_fields)](LICENSE)

A procedural macro to generate **field name enums and constants** for structs using Serde, respecting `#[serde(rename = "...")]` and `#[serde(rename_all = "...")]`.

---

## Features

- Automatically generate a `const SERDE_FIELDS: &'static [&'static str]` array containing the serialized names of all struct fields.
- Generate an enum named `{StructName}SerdeField` for all non-skipped fields.
- Enum variants match Rust field names (PascalCase) and are annotated with `#[serde(rename = "...")]`.
- Provides convenient methods and trait implementations:
  - `as_str() -> &'static str`
  - `Display` implementation
  - `From<Enum>` and `From<&Enum>` for `&'static str`
  - `TryFrom<&str>` and `TryFrom<String>` with custom error `InvalidSerdeFieldName`
  - `FromStr` implementation
  - `AsRef<str>` for ergonomic usage
- Supports skipped fields via `#[serde(skip)]` and renaming via `#[serde(rename = "...")]`.
- Fully respects struct-level `#[serde(rename_all = "...")]`.

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
use serde_fields::SerdeFieldNames;

#[derive(Serialize, Deserialize, SerdeField)]
#[serde(rename_all = "camelCase")]
struct User {
    user_id: u32,
    #[serde(rename = "eMail")]
    email: String,
}

fn main() {
    // Access serialized field names as a slice
    assert_eq!(User::SERDE_FIELDS, &["userId", "eMail"]);

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
}
```
