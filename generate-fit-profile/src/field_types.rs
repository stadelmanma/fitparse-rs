//! Functions to generate the field-types in Rust from the fit profile.
use crate::parse::{FieldTypeDefintion, FieldTypeVariant, FitProfile};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use std::{
    fs::File,
    io::{Error, Write},
};

fn field_type_enum_is_named_variant(field_type: &FieldTypeDefintion) -> TokenStream {
    let variant_values = field_type.variant_map().keys();
    quote! {
        pub fn is_named_variant(value: i64) -> bool {
            match value {
                #( #variant_values => true,)*
                _ => false
            }
        }
    }
}

fn field_type_enum_as_type(field_type: &FieldTypeDefintion) -> TokenStream {
    let ident = field_type.ident();
    let fn_ident = format_ident!("as_{}", field_type.base_type());
    let rtype = field_type.base_type();
    let variant_idents = field_type.variant_map().values().map(|v| v.ident());
    let variant_values = field_type
        .variant_map()
        .values()
        .map(FieldTypeVariant::value);
    let other_value_ident = field_type.other_value_field_name();

    quote! {
        pub fn #fn_ident(self) -> #rtype {
            match self {
                #( #ident::#variant_idents => #variant_values,)*
                #ident::#other_value_ident(value) => value

            }
        }

       pub fn as_i64(self) -> i64 {
        self.#fn_ident() as i64
       }
    }
}

fn field_type_enum_impl(field_type: &FieldTypeDefintion) -> TokenStream {
    let ident = field_type.ident();
    let is_named_variant = field_type_enum_is_named_variant(field_type);
    let as_numeric_types = field_type_enum_as_type(field_type);

    quote! {
        impl #ident {
            #is_named_variant
            #as_numeric_types
        }
    }
}

fn field_type_enum_impl_display(field_type: &FieldTypeDefintion) -> TokenStream {
    let ident = field_type.ident();
    let variant_idents = field_type.variant_map().values().map(|v| v.ident());
    let variant_names = field_type.variant_map().values().map(|v| v.name());
    let other_val_ident = field_type.other_value_field_name();
    let other_match_arm = if field_type.is_true_enum() {
        quote!(#ident::#other_val_ident(value) => write!(f, "unknown_variant_{}", value))
    } else {
        quote!(#ident::#other_val_ident(value) => write!(f, "{}", value))
    };

    quote! {
        impl fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match &self {
                    #( #ident::#variant_idents => write!(f, #variant_names),)*
                    #other_match_arm
                }
            }
        }
    }
}

fn field_type_enum_impl_from(field_type: &FieldTypeDefintion) -> TokenStream {
    let ident = field_type.ident();
    let base_type = field_type.base_type();
    let variant_idents = field_type.variant_map().values().map(|v| v.ident());
    let variant_values = field_type
        .variant_map()
        .values()
        .map(FieldTypeVariant::value);
    let other_val_ident = field_type.other_value_field_name();

    quote! {
            impl convert::From<#base_type> for #ident {
                fn from(value: #base_type) -> Self {
                    match value {
                        #( #variant_values => #ident::#variant_idents, )*
                        _ => #ident::#other_val_ident(value)
                    }
                }
            }

        impl convert::From<i64> for #ident {
            fn from(value: i64) -> Self {
                #ident::from(value as #base_type)
            }
        }
    }
}

fn field_type_enum_impl_serialize_fn_body(field_type: &FieldTypeDefintion) -> TokenStream {
    let ident = field_type.ident();
    if field_type.is_true_enum() {
        quote!(serializer.serialize_str(&self.to_string()))
    } else {
        let serialize_fn = format_ident!("serialize_{}", field_type.base_type());
        let other_val_ident = field_type.other_value_field_name();
        quote! {
            match &self {
                #ident::#other_val_ident(value) => serializer.#serialize_fn(*value),
                _ => serializer.serialize_str(&self.to_string())
            }
        }
    }
}

fn field_type_enum_impl_serialize(field_type: &FieldTypeDefintion) -> TokenStream {
    let ident = field_type.ident();
    let fn_body = field_type_enum_impl_serialize_fn_body(field_type);

    quote! {
        impl Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                #fn_body
            }
        }
    }
}

fn field_type_enum_variant_line(variant: &FieldTypeVariant) -> TokenStream {
    let comment = variant.comment();
    let ident = variant.ident();
    quote! {
        #comment
        #ident
    }
}

fn field_type_enum_other_value(field_type: &FieldTypeDefintion) -> TokenStream {
    let ident = field_type.other_value_field_name();
    let base_type = field_type.base_type();

    quote! {
        #ident(#base_type)
    }
}

fn field_type_enum(field_type: &FieldTypeDefintion) -> TokenStream {
    if field_type.variant_map().is_empty() {
        return TokenStream::new();
    }
    let comment = field_type.comment();
    let ident = field_type.ident();
    let variants = field_type
        .variant_map()
        .values()
        .map(field_type_enum_variant_line);
    let other_val = field_type_enum_other_value(field_type);
    let enum_impl = field_type_enum_impl(field_type);
    let impl_display = field_type_enum_impl_display(field_type);
    let impl_from = field_type_enum_impl_from(field_type);
    let impl_serialize = field_type_enum_impl_serialize(field_type);

    quote! {
        #comment
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
        pub enum #ident {
            #(#variants,)*
            #other_val
        }
        #enum_impl
        #impl_display
        #impl_from
        #impl_serialize
    }
}

fn generate_main_field_type_enum(field_types: &[FieldTypeDefintion]) -> TokenStream {
    let base_types = vec![
        "Bool", "SInt8", "UInt8", "SInt16", "UInt16", "SInt32", "UInt32", "String", "Float32",
        "Float64", "UInt8z", "UInt16z", "UInt32z", "Byte", "SInt64", "UInt64", "UInt64z",
    ]
    .into_iter()
    .map(|s| format_ident!("{}", s));
    let variants = field_types.iter().map(|f| f.ident());
    let mut is_enum_force_false = HashSet::new();
    is_enum_force_false.insert("date_time".to_string());
    is_enum_force_false.insert("local_date_time".to_string());
    let filtered_field_idents: Vec<_> = field_types
        .iter()
        .filter(|f| !f.variant_map().is_empty())
        .filter(|f| !is_enum_force_false.contains(f.name()))
        .map(|f| f.ident())
        .collect();

    quote! {
        /// Describe all possible data types of a field
        ///
        /// The Enum type's value is actually an enum of enums.
        #[derive(Clone, Copy, Debug)]
        pub enum FieldDataType {
            #( #base_types ,)*
            #( #variants ,)*
        }

        impl FieldDataType {
            #[allow(clippy::match_like_matches_macro)]
            pub fn is_enum_type(self) -> bool {
                match self {
                    #( FieldDataType::#filtered_field_idents => true, )*
                    _ => false
                }
            }
            pub fn is_named_variant(self, value: i64) -> bool {
                match self {
                    #( FieldDataType::#filtered_field_idents => #filtered_field_idents::is_named_variant(value), )*
                    _ => false
                }
            }
        }
        pub fn get_field_variant_as_string(field_type: FieldDataType , value: i64) -> String {
            match field_type {
                #( FieldDataType::#filtered_field_idents => #filtered_field_idents::from(value).to_string(), )*
                _ => format!("Undefined{}", value),
            }
        }
    }
}

pub fn write_types_file(profile: &FitProfile, out: &mut File) -> Result<(), Error> {
    let comment = format!(
        "Auto generated profile field types from FIT SDK Release: {}",
        profile.version()
    );
    let main_enum = generate_main_field_type_enum(profile.field_types());
    let field_type_enums = profile.field_types().iter().map(field_type_enum);
    let output = quote! {
        #![allow(missing_docs)]
        #![allow(dead_code)]
        #![allow(clippy::unreadable_literal)]
        #![doc = #comment]
        #![doc = "Not all of these may be used by the defined set of FIT messages"]

        use serde::{Serialize, ser::Serializer};
        use std::{convert, fmt};

        #main_enum

        #( #field_type_enums )*
    };

    write!(out, "{}", output)
}
