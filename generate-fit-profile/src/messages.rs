//! Functions to generate the message structs in Rust from the fit profile.

use crate::parse::{FitProfile, MessageDefinition, MessageFieldDefinition};
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    fs::File,
    io::{Error, Write},
};

fn message_struct_field(field: &MessageFieldDefinition) -> TokenStream {
    let ident = field.field_ident();
    quote! {
        pub #ident: Option<Value>,
    }
}

fn field_variable(field: &MessageFieldDefinition) -> TokenStream {
    let ident = field.field_ident();
    quote! {
        let mut #ident = None;
    }
}

fn field_match_case(field: &MessageFieldDefinition) -> TokenStream {
    let number = field.def_number();
    let ident = field.field_ident();
    quote! {
        #number => {
            #ident = Some(field.into_value());
        }
    }
}

fn message_parse_impl(message: &MessageDefinition) -> TokenStream {
    let field_idents = message.fields().map(MessageFieldDefinition::field_ident);
    let field_variables = message.fields().map(field_variable);
    let field_match_cases = message.fields().map(field_match_case);

    quote! {
        if record.kind() != Self::KIND {
            return Err(TryFromRecordError::unexpected_message_kind::<Self>(&record));
        }
        #( #field_variables )*
        let mut unknown_fields = Vec::new();
        for field in record.into_vec() {
            match field.number() {
                #( #field_match_cases )*
                _ => {
                    unknown_fields.push(field);
                }
            }
        }
        Ok(Self {
            #( #field_idents ),*,
            unknown_fields,
        })
    }
}

fn message_struct(message: &MessageDefinition) -> TokenStream {
    let name = message.titlized_name();
    let ident = message.struct_ident();
    let comment = message.comment().into_iter();

    let struct_fields = message.field_map().values().map(message_struct_field);
    let parse_impl = message_parse_impl(message);

    quote! {
        #( #[doc = #comment] )*
        #[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
        pub struct #ident {
            #( #struct_fields )*
            /// All fields that are not defined in the profile.
            pub unknown_fields: Vec<FitDataField>,
        }

        impl FitMessage for #ident {
            const NAME: &'static str = #name;
            const KIND: MesgNum = MesgNum::#ident;

            fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
                #parse_impl
            }
        }

        impl TryFrom<FitDataRecord> for #ident {
            type Error = TryFromRecordError;

            fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
                Self::parse(record)
            }
        }
    }
}

fn message_enum(messages: &[MessageDefinition]) -> TokenStream {
    let idents: Vec<_> = messages
        .iter()
        .map(MessageDefinition::struct_ident)
        .collect();

    quote! {
        /// All supported message types.
        #[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
        pub enum Message {
            #( #idents(#idents) ),*
        }

        impl Message {
            /// Parse a message from a [`FitDataRecord`][].
            pub fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
                match record.kind() {
                    #( #idents::KIND => #idents::parse(record).map(Self::#idents), )*
                    kind => Err(TryFromRecordError::UnsupportedMessageKind(kind)),
                }
            }

            /// Return all fields of the message that are not defined by the profile.
            pub fn unknown_fields(&self) -> &[FitDataField] {
                match self {
                    #( Self::#idents(message) => &message.unknown_fields, )*
                }
            }
        }

        impl TryFrom<FitDataRecord> for Message {
            type Error = TryFromRecordError;

            fn try_from(record: FitDataRecord) -> Result<Self, Self::Error> {
                Self::parse(record)
            }
        }
    }
}

pub fn write_messages_file(profile: &FitProfile, out: &mut File) -> Result<(), Error> {
    let comment = format!(
        "Auto generated profile messages from FIT SDK Release: {}",
        profile.version()
    );
    let message_enum = message_enum(profile.messages());
    let message_structs = profile.messages().iter().map(message_struct);
    let output = quote! {
        #![allow(missing_docs)]
        #![doc = #comment]

        use crate::{FitDataField, FitDataRecord, Value, profile::{FitMessage, MesgNum, TryFromRecordError}};
        use serde::Serialize;

        #message_enum

        #( #message_structs )*
    };
    write!(out, "{}", output)
}
