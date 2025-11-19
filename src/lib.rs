use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, DeriveInput, Fields, parse_macro_input};

fn parse_serde_rename_all(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

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

    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

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

#[proc_macro_derive(SerdeFieldNames, attributes(serde))]
pub fn derive_serde_field_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident.clone();
    let enum_name = format_ident!("{}Fields", struct_name);

    let rename_all_style = parse_serde_rename_all(&input.attrs);

    // Apply struct-level `rename_all`
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
        syn::Data::Struct(data) => match data.fields {
            Fields::Named(named) => named.named,
            _ => panic!("SerdeFieldNames only supports structs with named fields"),
        },
        _ => panic!("SerdeFieldNames only supports structs"),
    };

    let mut serde_fields = Vec::new();
    let mut enum_variants = Vec::new();
    let mut enum_match_arms = Vec::new();

    for field in fields {
        let ident = field.ident.unwrap();
        let rust_field_name = ident.to_string();

        let default_serde_name = apply_rename_all(&rust_field_name);

        let (serde_name, skip) = parse_field_serde_name_and_skip(&field.attrs, &default_serde_name);

        if skip {
            continue;
        }

        let variant_ident = format_ident!("{}", rust_field_name.to_case(Case::Pascal));
        let rename_str = serde_name.clone();

        serde_fields.push(quote! { #rename_str });

        enum_variants.push(quote! {
            #[serde(rename = #rename_str)]
            #variant_ident
        });
        enum_match_arms.push(quote! {
            #enum_name::#variant_ident => #rename_str,
        });
    }

    let expanded = quote! {
        impl #struct_name {
            pub const SERDE_FIELDS: &'static [&'static str] = &[
                #( #serde_fields ),*
            ];
        }

        #[derive(::serde::Serialize, ::serde::Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum #enum_name {
            #( #enum_variants ),*
        }

        impl #enum_name {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    #( #enum_match_arms )*
                }
            }
        }

        impl ::std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let s: &'static str = self.into();
                write!(f, "{s}")
            }
        }

        impl From<#enum_name> for &'static str {
            fn from(field: #enum_name) -> Self {
                match field {
                    #(#enum_match_arms)*
                }
            }
        }

        impl From<&#enum_name> for &'static str {
            fn from(field: &#enum_name) -> Self {
                (*field).into()
            }
        }
    };

    TokenStream::from(expanded)
}
