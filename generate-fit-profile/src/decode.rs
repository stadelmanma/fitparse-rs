//! Functions to generate the message decoding functions from the fit profile.
use crate::parse::{FitProfile, MessageDefinition, MessageFieldDefinition};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

// TODO: make the larger "write" and "generate" functinos
// not be part of the impl and just pass the field/message
// in a parameter

fn bare_number_literal(value: u8) -> Literal {
    Literal::u8_unsuffixed(value)
}

impl MessageDefinition {
    fn function_name(&self) -> Ident {
        format_ident!("{}_message", self.name())
    }

    fn decode_function_def(&self) -> TokenStream {
        let mut comments = self
            .comment()
            .map_or_else(|| vec![TokenStream::new()], |v| vec![quote!(#[doc = #v])]);
        // TODO: add field comments here as well?
        let fn_name = self.function_name();
        let match_arms = self
            .field_map()
            .values()
            .map(|fld| {
                (
                    fld.def_number(),
                    fld.field_decode_block(
                        self,
                        &Ident::new("value", Span::call_site()).to_token_stream(),
                        None,
                        None,
                    ),
                )
            })
            .map(|(dfn, expr)| quote!(#dfn => {#expr}));
        let mut sub_field_fns = Vec::new();
        for field in self.field_map().values() {
            sub_field_fns.push(field.create_fn_def(self));
            if let Some(v) = field.comment() {
                let comment = format!(" * {}: {}", field.name(), v);
                comments.push(quote!(#[doc = #comment]));
            }
            let mut created_subfield_fns = HashSet::new();
            for (_, _, sub_field_info) in field.subfields() {
                if !created_subfield_fns.contains(sub_field_info.name()) {
                    // only create the function once, even if multiple values reference
                    // the subfield
                    sub_field_fns.push(sub_field_info.create_fn_def(self));
                    if let Some(v) = sub_field_info.comment() {
                        let comment = format!(" * {}: {}", sub_field_info.name(), v);
                        comments.push(quote!(#[doc = #comment]));
                    }
                }
                created_subfield_fns.insert(sub_field_info.name());
            }
        }

        quote! {
            #(#comments)*
            fn #fn_name(mesg_num: MesgNum, data_map: &mut HashMap<u8, Value>, accumlators: &mut HashMap<u32, Value>, options: &HashSet<DecodeOption>) -> Result<Vec<FitDataField>> {
                let mut fields = Vec::new();
                let mut entries: VecDeque<(u8, Value)> = data_map.iter().map(|(k, v)| (*k, v.clone())).collect();
                while let Some((def_num, value)) = entries.pop_front() {
                    match def_num {
                        #(#match_arms)*
                        _ => {
                            if !options.contains(&DecodeOption::DropUnknownFields) {
                                fields.push(unknown_field(def_num, value));
                            }
                        }
                    }
                }
                Ok(fields)
            }
            #(#sub_field_fns)*
        }
    }
}

impl MessageFieldDefinition {
    fn field_decode_block(
        &self,
        mesg_def: &MessageDefinition,
        val_str: &TokenStream,
        alt_scale: Option<f64>,
        alt_offset: Option<f64>,
    ) -> TokenStream {
        let body = if !self.components().is_empty() {
            self.component_exp(mesg_def, val_str, alt_scale, alt_offset)
        } else if !self.subfields().is_empty() {
            self.subfield_deref(mesg_def, val_str)
        } else {
            let fn_call = self.generate_create_fn_call(mesg_def, val_str, alt_scale, alt_offset);
            quote!(fields.push(#fn_call?);)
        };

        quote! {
            #body
        }
    }

    fn component_exp(
        &self,
        mesg_def: &MessageDefinition,
        val_expr: &TokenStream,
        alt_scale: Option<f64>,
        alt_offset: Option<f64>,
    ) -> TokenStream {
        // detect array fields so we can append a counter to variable name
        let mut array_flds = HashMap::new();
        for (_, fld) in self.components() {
            *array_flds.entry(fld.def_number()).or_insert(0u8) += 1;
        }
        array_flds = array_flds
            .into_iter()
            .filter(|(_, v)| *v > 1)
            .map(|(k, _)| (k, 0))
            .collect();

        // generate suffix'd var names for the vec -> variable expansion
        let mut var_names = Vec::new();
        for (_, fld) in self.components() {
            if array_flds.contains_key(&fld.def_number()) {
                array_flds.entry(fld.def_number()).and_modify(|v| *v += 1);
                var_names.push(format_ident!(
                    "{}_{}",
                    fld.name(),
                    array_flds
                        .get(&fld.def_number())
                        .expect("array_flds should have entry")
                ));
            } else {
                var_names.push(Ident::new(fld.name(), Span::call_site()));
            }
        }

        let keep_comp_fn_call = self.generate_create_fn_call(
            mesg_def,
            &quote!(#val_expr.clone()),
            alt_scale,
            alt_offset,
        );

        let name_and_size = var_names
            .iter()
            .zip(self.components().iter().map(|(n, _)| n));
        let mut comp_exp_chain = Vec::new();
        // build component expansion calls
        for (i, (vn, csize)) in name_and_size.enumerate() {
            let csize = bare_number_literal(*csize);
            if i == 0 {
                comp_exp_chain.push(
                    quote!(let ((input, offset), #vn) = extract_component(&input, 0usize, #csize);),
                )
            } else {
                comp_exp_chain.push(
                    quote!(let ((input, offset), #vn) = extract_component(input, offset, #csize);),
                )
            }
        }

        let mut comps_decoded = HashSet::new();
        let mut comp_decode_block = Vec::new();
        for (_, comp) in self.components() {
            let name = Ident::new(comp.name(), Span::call_site());
            let def_num = comp.def_number();
            // array components show up more than once, but we collect those
            // into an array value above so we only want to decode it once
            if comps_decoded.contains(&comp.def_number()) {
                continue;
            }

            // generate a vec! macro to build the array field
            if array_flds.contains_key(&comp.def_number()) {
                let vec_macro_vars = (0..*array_flds
                    .get(&comp.def_number())
                    .expect("array_flds should have entry"))
                    .map(|i| format_ident!("{}_{}", comp.name(), i + 1))
                    .collect::<Vec<Ident>>();
                comp_decode_block
                    .push(quote!(let #name = Value::Array(vec![#(#vec_macro_vars,)*]);));
            }

            // insert back into datamap for subfield look ups and then generate
            // a decode block incase the component has subfields/nested comps
            comp_decode_block.push(quote! {
                data_map.insert(#def_num, #name.clone());
            });

            // When we are expanding to a field that has a single component
            // use the scale and offset defined here instead of what that
            // component defines, example case is:
            // compressed_speed_distance => speed => enhanced_speed.
            // We want to apply the same the same scale for going from CSD to speed
            // (100) when expanding the value for enchanced_speed instead of
            // using the speed to enchanced_speed scale of 1000.
            let alt_scale = if alt_scale.is_some() {
                alt_scale
            } else if comp.components().len() == 1 {
                Some(comp.scale())
            } else {
                None
            };
            let alt_offset = if alt_scale.is_some() {
                alt_offset
            } else if comp.components().len() == 1 {
                Some(comp.offset())
            } else {
                None
            };

            comp_decode_block.push(comp.field_decode_block(
                mesg_def,
                &name.to_token_stream(),
                alt_scale,
                alt_offset,
            ));
            comps_decoded.insert(comp.def_number());
        }

        quote! {
            // if the decode option is present add the parent field prior to expansion
            if options.contains(&DecodeOption::KeepCompositeFields) {
                fields.push(#keep_comp_fn_call?);
            }
            let input = #val_expr.to_ne_bytes();
            #(#comp_exp_chain)*
            #(#comp_decode_block)*
        }
    }

    fn subfield_deref(&self, mesg_def: &MessageDefinition, val_expr: &TokenStream) -> TokenStream {
        let mut deref_branches = Vec::new();
        for (idx, (ref_name, ref_val_str, sub_field_info)) in self.subfields().iter().enumerate() {
            let ref_field = mesg_def.get_field_by_name(ref_name);
            let ref_field_ident = Ident::new(ref_field.field_type(), Span::call_site());
            let ref_val_ident = Ident::new(ref_val_str, Span::call_site());
            let ref_def_num = ref_field.def_number();
            let elif = if idx == 0 {
                quote!(if)
            } else {
                quote!(else if)
            };
            let body = if sub_field_info.components().is_empty() {
                // if the field is "terminal", i.e. has no components generate a create_fn call
                let fn_call =
                    sub_field_info.generate_create_fn_call(mesg_def, val_expr, None, None);
                quote!(fields.push(#fn_call?);)
            } else {
                // generate a nested component expansion
                sub_field_info.component_exp(mesg_def, val_expr, None, None)
            };

            deref_branches.push(quote!{
                #elif #ref_field_ident::#ref_val_ident.as_i64() == data_map.get(&#ref_def_num).map(|v| v.try_into().ok()).flatten().unwrap_or(-1i64) {
                    #body
                }
            });
        }
        let else_fn_call = self.generate_create_fn_call(mesg_def, val_expr, None, None);
        quote! {
            #(#deref_branches)*
            else {
                fields.push(#else_fn_call?);
            }
        }
    }

    fn create_fn_def(&self, mesg_def: &MessageDefinition) -> TokenStream {
        let def_number = self.def_number();
        let name = self.name();
        let fld_type_variant = Ident::new(self.field_type(), Span::call_site());
        let fld_type = quote!(FieldDataType::#fld_type_variant);
        let field_fn_name = format_ident!("{}_{}_field", mesg_def.function_name(), self.name());
        // tokens to generate data field
        let data_field_call = if let Some(parent) = self.parent_field() {
            let parent_name = parent.name();
            quote! {
                let name = if options.contains(&DecodeOption::UseGenericSubFieldName) {
                    #parent_name
                } else {
                    #name
                };
                data_field_with_info(#def_number, name, #fld_type, scale, offset, units, value, options)
            }
        } else {
            quote! {
                data_field_with_info(#def_number, #name, #fld_type, scale, offset, units, value, options)
            }
        };

        quote! {
            fn #field_fn_name(mesg_num: MesgNum,
                accumlators: &mut HashMap<u32, Value>,
                options: &HashSet<DecodeOption>,
                data_map: &HashMap<u8, Value>,
                accumulate: bool,
                scale: f64,
                offset: f64,
                units: &'static str,
                value: Value
            ) -> Result<FitDataField> {
                // accumlator field code
                let value = if accumulate {
                    calculate_cumulative_value(accumlators, mesg_num.as_u16(), #def_number, value)?
                } else {
                    value
                };
                #data_field_call
            }
        }
    }

    fn generate_create_fn_call(
        &self,
        mesg_def: &MessageDefinition,
        val_expr: &TokenStream,
        alt_scale: Option<f64>,
        alt_offset: Option<f64>,
    ) -> TokenStream {
        let fn_name = format_ident!("{}_{}_field", mesg_def.function_name(), self.name());
        let acc = self.accumulate();
        let scale = alt_scale.unwrap_or_else(|| self.scale());
        let offset = alt_offset.unwrap_or_else(|| self.offset());
        let units = self.units();
        quote! {
            #fn_name(mesg_num, accumlators, options, data_map, #acc, #scale, #offset, #units, #val_expr)
        }
    }
}

fn unknown_mesg_fn() -> TokenStream {
    quote! {
        fn unknown_message(
            data_map: &HashMap<u8, Value>,
            options: &HashSet<DecodeOption>,
        ) -> Result<Vec<FitDataField>> {
            // since it's an unknown message all the fields are unknown
            if options.contains(&DecodeOption::DropUnknownFields) {
                return Ok(Vec::new());
            }
            let fields = data_map.iter()
                .map(|(k, v)| unknown_field(*k, v.clone()))
                .collect();
            Ok(fields)
        }
    }
}

fn mesg_num_to_mesg_decode_fn(messages: &[MessageDefinition]) -> TokenStream {
    let msg_variants = messages.iter().map(MessageDefinition::struct_ident);
    let fn_names = messages.iter().map(MessageDefinition::function_name);
    quote! {
        impl MesgNum {
            /// Decode the raw values from a FitDataMessage based on the Global Message Number
            pub fn decode_message(self, data_map: &mut HashMap<u8, Value>, accumlators: &mut HashMap<u32, Value>, options: &HashSet<DecodeOption>) -> Result<Vec<FitDataField>> {
                match self {
                    #(MesgNum::#msg_variants => #fn_names(self, data_map, accumlators, options),)*
                    _ => unknown_message(data_map, options)
                }
            }
        }
    }
}

pub fn write_decode_file(profile: &FitProfile, out: &mut File) -> Result<(), std::io::Error> {
    let version = profile.version();
    let comment = format!(
        "//! Auto generated profile messages from FIT SDK Release: {}",
        version
    );
    // output all message functions
    let decode_fn_defs = profile.messages().iter().map(|m| m.decode_function_def());
    let unknown_fn = unknown_mesg_fn();
    let main_decode_fn = mesg_num_to_mesg_decode_fn(profile.messages());
    let output = quote! {
        #![doc = #comment]
        #![allow(unused_variables)]
        use std::collections::{HashMap, HashSet, VecDeque};
        use std::convert::TryInto;
        use crate::{{FitDataField, Value}};
        use crate::de::{{DecodeOption}};
        use crate::error::{{Result}};
        use super::{
            calculate_cumulative_value,
            data_field_with_info,
            extract_component,
            unknown_field
        };
        use super::field_types::*;
        #[doc = "FIT SDK version used to generate profile decoder"]
        pub const VERSION: &str = #version;

        #(#decode_fn_defs)*

        #unknown_fn
        #main_decode_fn

    };
    write!(out, "{}", output)
}
