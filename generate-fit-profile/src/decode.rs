//! Functions to generate the message decoding functions from the fit profile.
use crate::parse::{FitProfile, MessageDefinition, MessageFieldDefinition};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

// TODO: make the larger "write" and "generate" functinos
// not be part of the impl and just pass the field/message
// in a parameter

impl MessageDefinition {
    fn function_name(&self) -> String {
        format!("{}_message", self.name())
    }

    fn write_decode_function_def(&self, out: &mut File) -> Result<(), std::io::Error> {
        if let Some(v) = self.comment() {
            writeln!(out, "/// {}", v)?;
        }
        writeln!(out, "fn {}(mesg_num: MesgNum, data_map: &mut HashMap<u8, Value>, accumlators: &mut HashMap<u32, Value>) -> Result<Vec<FitDataField>> {{", self.function_name())?;
        writeln!(out, "let mut fields = Vec::new();")?;
        writeln!(out, "let mut entries: VecDeque<(u8, Value)> = data_map.iter().map(|(k, v)| (*k, v.clone())).collect();")?;

        writeln!(
            out,
            "while let Some((def_num, value)) = entries.pop_front() {{"
        )?;
        writeln!(out, "match def_num {{")?;
        for field in self.field_map().values() {
            writeln!(out, "{0} => {{", field.def_number())?;
            field.write_field_decode_block(out, &self, "value")?;
            writeln!(out, "}}")?;
        }
        writeln!(out, "_ => fields.push(unknown_field(def_num, value))")?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(out, "Ok(fields)")?;
        writeln!(out, "}}")?;

        for field in self.field_map().values() {
            field.write_create_fn_def(out, &self)?;
            let mut created_subfield_fns = HashSet::new();
            for (_, _, sub_field_info) in field.subfields() {
                if !created_subfield_fns.contains(sub_field_info.name()) {
                    // only create the function once, even if multiple values reference
                    // the subfield
                    sub_field_info.write_create_fn_def(out, &self)?;
                }
                created_subfield_fns.insert(sub_field_info.name());
            }
        }

        Ok(())
    }
}

impl MessageFieldDefinition {
    fn write_field_decode_block(
        &self,
        out: &mut File,
        mesg_def: &MessageDefinition,
        val_str: &str,
    ) -> Result<(), std::io::Error> {
        if let Some(v) = self.comment() {
            writeln!(out, "// {}", v)?;
        }
        if !self.components().is_empty() {
            self.write_component_exp(out, mesg_def, val_str)?;
        } else if !self.subfields().is_empty() {
            self.write_subfield_deref(out, mesg_def, val_str)?;
        } else {
            writeln!(
                out,
                "fields.push({}?);",
                self.generate_create_fn_call(mesg_def, val_str)
            )?;
        }

        Ok(())
    }

    fn write_component_exp(
        &self,
        out: &mut File,
        mesg_def: &MessageDefinition,
        val_str: &str,
    ) -> Result<(), std::io::Error> {
        // this can be recursive and might have multiple levels of
        // component expansion and subfields derefs
        // this could cause some variable naming collisions if
        // im not careful. I can't push the fields back onto the
        // vecdeq since then I'd need to track field info changes.
        // it might be useful to append some kind prefix/suffix
        // if it looks like I'll have name collisions
        // it looks like we also have instances where components might
        // expand into an array field. The signal for that might be
        // multiple components with the same destiniation def_num
        // and that destination being an array field, an example
        // is the hr_message_event_timestamp_field

        writeln!(
            out,
            "let [{}] = expand_components({}, &[{}])[..];",
            self.components()
                .iter()
                .map(|(_, f)| f.name())
                .collect::<Vec<&str>>()
                .join(","),
            val_str,
            self.components()
                .iter()
                .map(|(n, _)| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )?;
        for (_, comp) in self.components().iter() {
            // insert back into datamap for subfield look ups
            writeln!(
                out,
                "data_map.insert({}, {}.clone());",
                comp.def_number(),
                comp.name()
            )?;

            // to make this work, I'll do something similar to what I do
            // in my old decoding logic. I'll take the component info and
            // the destination field and create a new pseduo-field from it
            // and use that to recursively handle generating stuff
            // let comp_field = MessageFieldDefinition::new(dest_fld: &MessageFieldDefinition, comp_info: &MessageFieldComponent) -> MessageFieldDefinition;
            comp.write_field_decode_block(out, mesg_def, comp.name())?;
        }

        Ok(())
    }

    fn write_subfield_deref(
        &self,
        out: &mut File,
        mesg_def: &MessageDefinition,
        val_str: &str,
    ) -> Result<(), std::io::Error> {
        for (idx, (ref_name, ref_val_str, sub_field_info)) in self.subfields().iter().enumerate() {
            let ref_field = mesg_def.get_field_by_name(ref_name);
            if idx == 0 {
                writeln!(out, "if")?;
            } else {
                writeln!(out, "else if")?;
            }
            writeln!(out, "{}::{}.as_i64() == data_map.get(&{}).map(|v| v.try_into().ok()).flatten().unwrap_or(-1i64) {{", ref_field.field_type(), ref_val_str, ref_field.def_number())?;

            // if the field is "terminal", i.e. has no components generate a create_fn call
            if sub_field_info.components().is_empty() {
                writeln!(
                    out,
                    "fields.push({}?);",
                    sub_field_info.generate_create_fn_call(mesg_def, val_str)
                )?;
            } else {
                // generate a nested component expansion
                sub_field_info.write_component_exp(out, mesg_def, val_str)?;
            }
            writeln!(out, "}}")?;
        }
        writeln!(out, "else {{")?;
        writeln!(
            out,
            "fields.push({}?);",
            self.generate_create_fn_call(mesg_def, val_str)
        )?;
        writeln!(out, "}}")?;

        Ok(())
    }

    fn write_create_fn_def(
        &self,
        out: &mut File,
        mesg_def: &MessageDefinition,
    ) -> Result<(), std::io::Error> {
        if !self.components().is_empty() {
            // fields with components are always expanded
            // and never actually created
            return Ok(());
        }

        writeln!(out,
            "fn {}_{}_field(mesg_num: MesgNum, accumlators: &mut HashMap<u32, Value>, data_map: &HashMap<u8, Value>, accumulate: bool, scale: f64, offset: f64, units: &'static str, value: Value) -> Result<FitDataField> {{",
            mesg_def.function_name(), self.name())?;

        // generate acccumated field code
        writeln!(out, "let value = if accumulate {{")?;
        writeln!(
            out,
            "  calculate_cumulative_value(accumlators, mesg_num.as_u16(), {0}, value)?",
            self.def_number()
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "else {{")?;
        writeln!(out, "  value")?;
        writeln!(out, "}};")?;
        // generate field
        writeln!(
            out,
            "data_field_with_info({0}, \"{1}\", FieldDataType::{2}, scale, offset, units, value)",
            self.def_number(),
            self.name(),
            self.field_type()
        )?;
        //
        writeln!(out, "}}")?;
        Ok(())
    }

    fn generate_create_fn_call(&self, mesg_def: &MessageDefinition, val_str: &str) -> String {
        format!(
            "{0}_{1}_field(mesg_num, accumlators, data_map, {2}, {3:.6}, {4:.6}, \"{5}\", {6})",
            mesg_def.function_name(),
            self.name(),
            self.accumulate(),
            self.scale(),
            self.offset(),
            self.units(),
            val_str,
        )
    }
}

fn write_unknown_mesg_fn(out: &mut File) -> Result<(), std::io::Error> {
    writeln!(
        out,
        "{}",
        "
        fn unknown_message(
        data_map: &HashMap<u8, Value>
    ) -> Result<Vec<FitDataField>> {
        let fields = data_map.iter()
            .map(|(k, v)| unknown_field(*k, v.clone()))
            .collect();
        Ok(fields)
    }
    "
    )
}

fn create_mesg_num_to_mesg_decode_fn(
    messages: &[MessageDefinition],
    out: &mut File,
) -> Result<(), std::io::Error> {
    writeln!(out, "impl MesgNum {{")?;
    writeln!(
        out,
        "  /// Decode the raw values from a FitDataMessage based on the Global Message Number"
    )?;
    writeln!(out, "  pub fn decode_message(self, data_map: &mut HashMap<u8, Value>, accumlators: &mut HashMap<u32, Value>) -> Result<Vec<FitDataField>> {{")?;
    writeln!(out, "    match self {{")?;
    for msg in messages {
        writeln!(
            out,
            "MesgNum::{} => {}(self, data_map, accumlators),",
            msg.titlized_name(),
            msg.function_name()
        )?;
    }
    writeln!(out, "_ => unknown_message(data_map),")?;
    writeln!(out, "    }}")?;
    writeln!(out, "  }}")?;
    writeln!(out, "}}")?;

    Ok(())
}

pub fn write_decode_file(profile: &FitProfile, out: &mut File) -> Result<(), std::io::Error> {
    writeln!(
        out,
        "//! Auto generated profile messages from FIT SDK Release: {}",
        profile.version()
    )?;
    writeln!(out, "#![allow(unused_variables)]")?;
    writeln!(out, "use std::collections::{{HashMap, VecDeque}};")?;
    writeln!(out, "use std::convert::TryInto;")?;
    writeln!(out, "use crate::{{FitDataField, Value}};")?;
    writeln!(out, "use crate::error::{{Result}};")?;
    writeln!(
        out,
        "use super::{{calculate_cumulative_value, data_field_with_info, expand_components, unknown_field}};"
    )?;
    writeln!(out, "use super::field_types::*;")?;
    writeln!(out, "/// FIT SDK version used to generate profile decoder")?;
    writeln!(out, "pub const VERSION: &str = \"{}\";", profile.version())?;

    // output all message functions
    for msg in profile.messages() {
        msg.write_decode_function_def(out)?;
    }

    write_unknown_mesg_fn(out)?;
    create_mesg_num_to_mesg_decode_fn(profile.messages(), out)?;

    Ok(())
}
