#![doc = include_str!("../README.md")]

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Fields, Ident, parse_macro_input};

struct SerdeItem {
    variant_ident: Ident,
    serde_name: String,
}

fn parse_serde_rename_all(attrs: &[Attribute]) -> Option<String> {
    parse_serde_rename_all_attr(attrs, "rename_all")
}

fn parse_serde_rename_all_fields(attrs: &[Attribute]) -> Option<String> {
    parse_serde_rename_all_attr(attrs, "rename_all_fields")
}

fn parse_serde_rename_all_attr(attrs: &[Attribute], attr_name: &str) -> Option<String> {
    for attr in attrs.iter().filter(|a| a.path().is_ident("serde")) {
        let mut found = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(attr_name) {
                let lit: syn::LitStr = meta.value()?.parse()?;
                found = Some(lit.value());
            } else if meta.input.peek(syn::Token![=]) {
                let _: syn::Lit = meta.value()?.parse()?;
            }
            Ok(())
        });
        if found.is_some() {
            return found;
        }
    }
    None
}

fn apply_rename_all(rename_all_style: Option<&str>, name: &str) -> String {
    match rename_all_style {
        Some("lowercase") => name.to_case(Case::Flat),
        Some("UPPERCASE") => name.to_case(Case::UpperFlat),
        Some("PascalCase") => name.to_case(Case::Pascal),
        Some("camelCase") => name.to_case(Case::Camel),
        Some("snake_case") => name.to_case(Case::Snake),
        Some("SCREAMING_SNAKE_CASE") => name.to_case(Case::UpperSnake),
        Some("kebab-case") => name.to_case(Case::Kebab),
        Some("SCREAMING-KEBAB-CASE") => name.to_case(Case::Cobol),
        _ => name.to_string(),
    }
}

fn parse_serde_name_and_skip(attrs: &[Attribute], default_name: &str) -> (String, bool) {
    let mut rename: Option<String> = None;
    let mut skip = false;

    for attr in attrs.iter().filter(|a| a.path().is_ident("serde")) {
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                rename = Some(lit.value());
            } else if meta.path.is_ident("skip") {
                skip = true;
            } else if meta.input.peek(syn::Token![=]) {
                let _: syn::Lit = meta.value()?.parse()?;
            }
            Ok(())
        });
    }

    (rename.unwrap_or_else(|| default_name.to_string()), skip)
}

fn build_serde_field_enum(
    source_type_name: Option<&Ident>,
    enum_name: &Ident,
    error_name: &Ident,
    serde_items: &[SerdeItem],
) -> TokenStream2 {
    let mut serde_field_literals = Vec::new();
    let mut variant_definitions = Vec::new();
    let mut as_str_arms = Vec::new();
    let mut try_from_arms = Vec::new();

    for item in serde_items {
        let variant_ident = &item.variant_ident;
        let rename_literal = item.serde_name.clone();

        serde_field_literals.push(quote! { #rename_literal });
        variant_definitions.push(quote! {
            #[serde(rename = #rename_literal)]
            #variant_ident
        });
        as_str_arms.push(quote! {
            #enum_name::#variant_ident => #rename_literal,
        });
        try_from_arms.push(quote! {
            #rename_literal => Ok(#enum_name::#variant_ident),
        });
    }

    let source_type_serde_fields = source_type_name
        .map(|source_type_name| {
            quote! {
                impl #source_type_name {
                    pub const SERDE_FIELDS: &'static [&'static str] = #enum_name::SERDE_FIELDS;
                }
            }
        })
        .unwrap_or_default();

    let error_name_str = error_name.to_string();

    quote! {
        #source_type_serde_fields

        #[derive(::serde::Serialize, ::serde::Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum #enum_name {
            #( #variant_definitions ),*
        }

        impl #enum_name {
            pub const SERDE_FIELDS: &'static [&'static str] = &[
                #( #serde_field_literals ),*
            ];

            pub const fn as_str(&self) -> &'static str {
                match self {
                    #( #as_str_arms )*
                }
            }
        }

        impl ::std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl From<#enum_name> for &'static str {
            fn from(field: #enum_name) -> Self {
                field.as_str()
            }
        }

        impl From<&#enum_name> for &'static str {
            fn from(field: &#enum_name) -> Self {
                (*field).into()
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct #error_name(pub String);

        impl ::std::fmt::Display for #error_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    "{}: Got '{}'. Expected any of {:?}.",
                    #error_name_str,
                    self.0,
                    #enum_name::SERDE_FIELDS
                )
            }
        }

        impl ::std::error::Error for #error_name {}

        impl ::core::convert::TryFrom<&str> for #enum_name {
            type Error = #error_name;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    #( #try_from_arms )*
                    other => Err(#error_name(other.to_string())),
                }
            }
        }

        impl ::core::convert::TryFrom<String> for #enum_name {
            type Error = #error_name;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                <#enum_name as ::core::convert::TryFrom<&str>>::try_from(value.as_str())
            }
        }

        impl ::std::str::FromStr for #enum_name {
            type Err = #error_name;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl AsRef<str> for #enum_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    }
}

/// Derive enum and constants for Serde field/variant names.
///
/// This macro generates:
/// 1. A `const SERDE_FIELDS: &'static [&'static str]` on the type, containing the
///    serialized names of all struct fields or enum variants (taking
///    `#[serde(rename = "...")]`, `#[serde(rename_all = "...")]`, and
///    `#[serde(skip)]` into account).
/// 2. An enum named `{TypeName}SerdeField` with variants for each field or variant:
///    - Struct field variants are named after the Rust field name (PascalCase).
///    - Enum variant names are preserved.
///    - Each generated variant is annotated with `#[serde(rename = "...")]`.
/// 3. For each struct-like enum variant, a nested enum named
///    `{EnumName}{VariantName}SerdeField` is generated for that variant's fields.
/// 4. Implementations for:
///    - `as_str() -> &'static str`
///    - `Display`
///    - `From<{TypeName}SerdeField> for &'static str`
///    - `From<&{TypeName}SerdeField> for &'static str`
///    - `TryFrom<&str>` and `TryFrom<String>` with error `Invalid{TypeName}SerdeField`
///    - `FromStr`
///    - `AsRef<str>`
///
/// # Example
///
/// ```rust
/// use serde_fields::SerdeField;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize, SerdeField)]
/// #[serde(rename_all = "camelCase")]
/// struct User {
///     #[serde(rename = "id")]
///     user_id: u32,
///     email: String,
/// }
///
/// // Access field-names as string slice
/// assert_eq!(User::SERDE_FIELDS, &["id", "email"]);
///
/// // Use the generated enum
/// let f = UserSerdeField::UserId;
/// assert_eq!(f.as_str(), "id");
/// assert_eq!(f.to_string(), "id");
///
/// // TryFrom & FromStr
/// let parsed: UserSerdeField = "id".parse().unwrap();
/// assert_eq!(parsed, UserSerdeField::UserId);
/// ```
#[proc_macro_derive(SerdeField, attributes(serde))]
pub fn derive_serde_field(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = input.ident;
    let enum_name = format_ident!("{}SerdeField", type_name);
    let error_name = format_ident!("Invalid{}SerdeField", type_name);

    let rename_all_style = parse_serde_rename_all(&input.attrs);
    let rename_all_fields_style = parse_serde_rename_all_fields(&input.attrs);
    let mut serde_items: Vec<SerdeItem> = Vec::new();
    let mut nested_field_enums = Vec::new();

    match input.data {
        Data::Struct(ref data) => {
            let fields = match data.fields {
                Fields::Named(ref named) => &named.named,
                _ => panic!("SerdeField only supports structs with named fields and enums"),
            };

            for field in fields {
                let ident = field.ident.as_ref().unwrap();
                let rust_name = ident.to_string();
                let default_serde_name = apply_rename_all(rename_all_style.as_deref(), &rust_name);
                let (serde_name, skip) =
                    parse_serde_name_and_skip(&field.attrs, &default_serde_name);

                if !skip {
                    let variant_ident = format_ident!("{}", rust_name.to_case(Case::Pascal));
                    serde_items.push(SerdeItem {
                        variant_ident,
                        serde_name,
                    });
                }
            }
        }
        Data::Enum(ref data) => {
            for variant in &data.variants {
                let variant_ident = variant.ident.clone();
                let rust_name = variant_ident.to_string();
                let default_serde_name = apply_rename_all(rename_all_style.as_deref(), &rust_name);
                let (serde_name, skip) =
                    parse_serde_name_and_skip(&variant.attrs, &default_serde_name);

                if !skip {
                    serde_items.push(SerdeItem {
                        variant_ident: variant_ident.clone(),
                        serde_name,
                    });
                }

                let Fields::Named(named_fields) = &variant.fields else {
                    continue;
                };

                let variant_rename_all_style = parse_serde_rename_all(&variant.attrs)
                    .or_else(|| rename_all_fields_style.clone());
                let mut nested_serde_items = Vec::new();

                for field in &named_fields.named {
                    let ident = field.ident.as_ref().unwrap();
                    let rust_name = ident.to_string();
                    let default_serde_name =
                        apply_rename_all(variant_rename_all_style.as_deref(), &rust_name);
                    let (serde_name, skip) =
                        parse_serde_name_and_skip(&field.attrs, &default_serde_name);

                    if !skip {
                        let variant_ident = format_ident!("{}", rust_name.to_case(Case::Pascal));
                        nested_serde_items.push(SerdeItem {
                            variant_ident,
                            serde_name,
                        });
                    }
                }

                let nested_enum_name = format_ident!("{}{}SerdeField", type_name, variant_ident);
                let nested_error_name =
                    format_ident!("Invalid{}{}SerdeField", type_name, variant_ident);
                nested_field_enums.push(build_serde_field_enum(
                    None,
                    &nested_enum_name,
                    &nested_error_name,
                    &nested_serde_items,
                ));
            }
        }
        _ => panic!("SerdeField only supports structs with named fields and enums"),
    }

    let top_level_enum =
        build_serde_field_enum(Some(&type_name), &enum_name, &error_name, &serde_items);

    let expanded = quote! {
        #top_level_enum
        #( #nested_field_enums )*
    };

    TokenStream::from(expanded)
}
