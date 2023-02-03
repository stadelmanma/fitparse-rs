//! Functions to generate the message decoding functions from the fit profile.
use crate::parse::{FitProfile, MessageDefinition, MessageFieldDefinition};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

impl MessageDefinition {
    fn function_name(&self) -> String {
        format!("{}_message", self.name())
    }

    fn get_field_by_name(&self, name: &str) -> &MessageFieldDefinition {
        for field in self.field_map().values() {
            if field.name() == name {
                return field;
            }
        }
        panic!("No field with name: {:?}", name);
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
            if let Some(v) = field.comment() {
                writeln!(out, "// {}", v)?;
            }
            if !field.components().is_empty() {
                writeln!(
                    out,
                    "let component_values = expand_components(value, &[{}]);",
                    field
                        .components()
                        .iter()
                        .map(|c| c.bits().to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )?;
                for (idx, comp) in field.components().iter().enumerate() {
                    let dest_def_number = self.get_field_by_name(comp.name()).def_number();

                    // --------------------------------------------------------
                    // TODO: components expanding into array fields are not
                    //       handled correctly see
                    //       hr_message_event_timestamp_field
                    // TODO: components expanding into other components might
                    //       also no be handled correctly. I think for that to
                    //       work I would need to do component expansion in the
                    //       field function and mutate the data map there.
                    // TODO: recursive subfield resolution won't work how I have
                    //       it right now, but would if I do the same thing as above.
                    //       However, the profile I'm using doesn't have recursive
                    //       subfields
                    // --------------------------------------------------------

                    // insert back into datamap for subfield look ups
                    writeln!(
                        out,
                        "data_map.insert({}, component_values[{}].clone());",
                        dest_def_number, idx
                    )?;

                    // according to some sample code in the FIT SDK when a component expands
                    // to a composite field (i.e. one with 2 or more components) the scale and
                    // offset are not applied but this seems inconsistent or maybe I'm
                    // misunderstaning the code
                    writeln!(out, "fields.push({0}_{1}_field(mesg_num, accumlators, {2}, {3:.6}, {4:.6}, \"{5}\", component_values[{6}].clone())?);",
                        self.function_name(),
                        comp.name(),
                        comp.accumulate(),
                        comp.scale(),
                        comp.offset(),
                        comp.units(),
                        idx
                    )?;
                }
            } else if !field.subfields().is_empty() {
                for (idx, (ref_name, val_str, sub_field_info)) in
                    field.subfields().iter().enumerate()
                {
                    let ref_field = self.get_field_by_name(ref_name);
                    if idx == 0 {
                        writeln!(out, "if {}::{}.as_i64() == data_map.get(&{}).map(|v| v.try_into().ok()).flatten().unwrap_or(-1i64) {{", ref_field.field_type(), val_str, ref_field.def_number())?;
                    } else {
                        writeln!(out, "else if {}::{}.as_i64() == data_map.get(&{}).map(|v| v.try_into().ok()).flatten().unwrap_or(-1i64) {{", ref_field.field_type(), val_str, ref_field.def_number())?;
                    }
                    writeln!(out, "fields.push({}_{}_field(mesg_num, accumlators, {}, {3:.6}, {4:.6}, \"{5}\", value)?);",
                        self.function_name(),
                        sub_field_info.name(),
                        sub_field_info.accumulate(),
                        sub_field_info.scale(),
                        sub_field_info.offset(),
                        sub_field_info.units()
                    )?;
                    writeln!(out, "}}")?;
                }
                writeln!(out, "else {{")?;
                writeln!(out, "fields.push({}_{}_field(mesg_num, accumlators, {}, {3:.6}, {4:.6}, \"{5}\", value)?);",
                        self.function_name(),
                        field.name(),
                        field.accumulate(),
                        field.scale(),
                        field.offset(),
                        field.units()
                    )?;
                writeln!(out, "}}")?;
            } else {
                writeln!(out, "fields.push({}_{}_field(mesg_num, accumlators, {}, {3:.6}, {4:.6}, \"{5}\", value)?);",
                    self.function_name(),
                    field.name(),
                    field.accumulate(),
                    field.scale(),
                    field.offset(),
                    field.units()
                )?;
            }
            writeln!(out, "}}")?;
        }
        writeln!(out, "_ => fields.push(unknown_field(def_num, value))")?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(out, "Ok(fields)")?;
        writeln!(out, "}}")?;

        for field in self.field_map().values() {
            self.write_create_field_fn_def(out, field)?;
            let mut created_subfield_fns = HashSet::new();
            for (_, _, sub_field_info) in field.subfields() {
                if !created_subfield_fns.contains(sub_field_info.name()) {
                    // only create the function once, even if multiple values reference
                    // the subfield
                    self.write_create_field_fn_def(out, sub_field_info)?;
                }
                created_subfield_fns.insert(sub_field_info.name());
            }
        }

        Ok(())
    }

    fn write_create_field_fn_def(
        &self,
        out: &mut File,
        field: &MessageFieldDefinition,
    ) -> Result<(), std::io::Error> {
        writeln!(out, "fn {}_{}_field(mesg_num: MesgNum, accumlators: &mut HashMap<u32, Value>, accumulate: bool, scale: f64, offset: f64, units: &'static str, value: Value) -> Result<FitDataField> {{", self.function_name(), field.name())?;
        // generate acccumated field code
        writeln!(out, "let value = if accumulate {{")?;
        writeln!(
            out,
            "  calculate_cumulative_value(accumlators, mesg_num.as_u16(), {0}, value)?",
            field.def_number()
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "else {{")?;
        writeln!(out, "  value")?;
        writeln!(out, "}};")?;
        // generate field
        writeln!(
            out,
            "data_field_with_info({0}, \"{1}\", FieldDataType::{2}, scale, offset, units, value)",
            field.def_number(),
            field.name(),
            field.field_type()
        )?;
        //
        writeln!(out, "}}")?;
        Ok(())
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

    writeln!(out, "use std::collections::{{HashMap, VecDeque}};")?;
    writeln!(out, "use std::convert::TryInto;")?;
    writeln!(out, "use crate::{{FitDataField, Value}};")?;
    writeln!(out, "use crate::error::{{Result}};")?;
    writeln!(
        out,
        "use super::{{calculate_cumulative_value, data_field_with_info, expand_components, unknown_field}};"
    )?;
    writeln!(out, "use super::field_types::*;")?;
    writeln!(out, "pub const VERSION: &str = \"{}\";", profile.version())?;

    // output all message functions
    for msg in profile.messages() {
        msg.write_decode_function_def(out)?;
    }

    write_unknown_mesg_fn(out)?;
    create_mesg_num_to_mesg_decode_fn(profile.messages(), out)?;

    Ok(())
}
