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
        for field in record.into_vec() {
            match field.number() {
                #( #field_match_cases )*
                _ => if !options.ignore_unexpected_fields {
                    return Err(TryFromRecordError::unexpected_field(&field));
                }
            }
        }
        Ok(Self {
            #( #field_idents ),*
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
        }

        impl FitMessage for #ident {
            const NAME: &'static str = #name;
            const KIND: MesgNum = MesgNum::#ident;

            fn parse_with_options(
                record: FitDataRecord,
                options: MessageParseOptions,
            ) -> Result<Self, TryFromRecordError> {
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
            /// Parse a message from a [`FitDataRecord`][] using the default options.
            pub fn parse(record: FitDataRecord) -> Result<Self, TryFromRecordError> {
                Self::parse_with_options(record, Default::default())
            }

            /// Parse a message from a [`FitDataRecord`][] using the given options.
            pub fn parse_with_options(
                record: FitDataRecord,
                options: MessageParseOptions,
            ) -> Result<Self, TryFromRecordError> {
                match record.kind() {
                    #( #idents::KIND => #idents::parse_with_options(record, options).map(Self::#idents), )*
                    kind => Err(TryFromRecordError::UnsupportedMessageKind(kind)),
                }
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

        use crate::{FitDataRecord, Value, profile::{FitMessage, MessageParseOptions, MesgNum, TryFromRecordError}};
        use serde::Serialize;

        #message_enum

        #( #message_structs )*
    };
    write!(out, "{}", output)
}
