use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, DeriveInput, Fields, parse_macro_input};

fn parse_serde_rename_all(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs.iter().filter(|a| a.path().is_ident("serde")) {
        let mut found = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename_all") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                found = Some(lit.value());
            }
            Ok(())
        });
        if found.is_some() {
            return found;
        }
    }
    None
}

fn parse_field_serde_name_and_skip(attrs: &[Attribute], default_name: &str) -> (String, bool) {
    let mut rename: Option<String> = None;
    let mut skip = false;

    for attr in attrs.iter().filter(|a| a.path().is_ident("serde")) {
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                rename = Some(lit.value());
            } else if meta.path.is_ident("skip") {
                skip = true;
            }
            Ok(())
        });
    }

    (rename.unwrap_or_else(|| default_name.to_string()), skip)
}

/// Derive enum and constants for Serde field-names.
///
/// This macro generates:
/// 1. A `const SERDE_FIELDS: &'static [&'static str]` on the struct, containing the
///    serialized names of all fields (taking `#[serde(rename = "...")]` and
///    `#[serde(rename_all = "...")]` into account).
/// 2. An enum named `{StructName}SerdeFields` with variants for each field:
///    - Each variant is named after the Rust field name (PascalCase).
///    - Each variant is annotated with `#[serde(rename = "...")]`.
/// 3. Implementations for:
///    - `as_str() -> &'static str`
///    - `Display`
///    - `From<FooFields> for &'static str`
///    - `From<&FooFields> for &'static str`
///    - `TryFrom<&str>` and `TryFrom<String>` with error `InvalidSerdeFieldName`
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

    let struct_name = input.ident;
    let enum_name = format_ident!("{}SerdeField", struct_name);

    let rename_all_style = parse_serde_rename_all(&input.attrs);
    let apply_rename_all = |name: &str| -> String {
        match rename_all_style.as_deref() {
            Some("lowercase") => name.to_case(Case::Lower),
            Some("UPPERCASE") => name.to_case(Case::Upper),
            Some("PascalCase") => name.to_case(Case::Pascal),
            Some("camelCase") => name.to_case(Case::Camel),
            Some("snake_case") => name.to_case(Case::Snake),
            Some("SCREAMING_SNAKE_CASE") => name.to_case(Case::ScreamingSnake),
            Some("kebab-case") => name.to_case(Case::Kebab),
            Some("SCREAMING-KEBAB-CASE") => name.to_case(Case::Cobol),
            _ => name.to_string(),
        }
    };

    let fields = match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            Fields::Named(ref named) => &named.named,
            _ => panic!("SerdeField only supports structs with named fields"),
        },
        _ => panic!("SerdeField only supports structs"),
    };

    let mut serde_field_literals = Vec::new();
    let mut variant_definitions = Vec::new();
    let mut as_str_arms = Vec::new();
    let mut try_from_arms = Vec::new();

    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        let rust_name = ident.to_string();
        let default_serde_name = apply_rename_all(&rust_name);

        let (serde_name, skip) = parse_field_serde_name_and_skip(&field.attrs, &default_serde_name);
        if skip {
            continue;
        }

        let variant_ident = format_ident!("{}", rust_name.to_case(Case::Pascal));
        let rename_literal = serde_name.clone();

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

    let expanded = quote! {
        impl #struct_name {
            pub const SERDE_FIELDS: &'static [&'static str] = &[
                #( #serde_field_literals ),*
            ];
        }

        #[derive(::serde::Serialize, ::serde::Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum #enum_name {
            #( #variant_definitions ),*
        }

        impl #enum_name {
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
        pub struct InvalidSerdeFieldName(pub String);

        impl ::std::fmt::Display for InvalidSerdeFieldName {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "invalid serde field name: {}", self.0)
            }
        }

        impl ::std::error::Error for InvalidSerdeFieldName {}

        impl ::core::convert::TryFrom<&str> for #enum_name {
            type Error = InvalidSerdeFieldName;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value {
                    #( #try_from_arms )*
                    other => Err(InvalidSerdeFieldName(other.to_string())),
                }
            }
        }

        impl ::core::convert::TryFrom<String> for #enum_name {
            type Error = InvalidSerdeFieldName;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                <#enum_name as ::core::convert::TryFrom<&str>>::try_from(value.as_str())
            }
        }

        impl ::std::str::FromStr for #enum_name {
            type Err = InvalidSerdeFieldName;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s)
            }
        }

        impl AsRef<str> for #enum_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    };

    TokenStream::from(expanded)
}
