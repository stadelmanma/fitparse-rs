//! Functions to generate the message decoding functions from the fit profile.
use crate::parse::{FitProfile, MessageDefinition, MessageFieldComponent, MessageFieldDefinition};
use std::fs::File;
use std::io::prelude::*;

impl MessageDefinition {
    fn function_name(&self) -> String {
        format!("{}_message", self.name())
    }

    fn write_decode_function_def(&self, out: &mut File) -> Result<(), std::io::Error> {
        if let Some(v) = self.comment() {
            writeln!(out, "/// {}", v)?;
        }
        writeln!(out, "fn {}(mesg_num: MesgNum, data_map: HashMap<u8, Option<Value>>, accumlators: &mut HashMap<u32, Value>) -> Result<Vec<FitDataField>> {{", self.function_name())?;
        writeln!(out, "let mut fields = Vec::new();")?;
        writeln!(out, "let mut entries: VecDeque<(&u8, &Value)> = data_map.iter().filter_map(|(k, v)| v.as_ref().map(|v| (k, v))).collect();")?;

        writeln!(
            out,
            "while let Some((def_num, value)) = entries.pop_front() {{"
        )?;
        writeln!(out, "match def_num {{")?;
        for field in self.field_map().values() {
            writeln!(out, "{0} => {{", field.def_number())?;
            if (field.accumulate()
                || !field.subfields().is_empty()
                || !field.components().is_empty())
            {
                if (field.accumulate()) {
                    writeln!(out, "let value = calculate_cumulative_value(accumlators, mesg_num.as_u16(), {0}, value.clone())?;", field.def_number())?;
                    writeln!(out, "fields.push(data_field_with_info({0}, \"{1}\", FieldDataType::{2}, {3:.6}, {4:.6}, \"{5}\", value)?);",
                    field.def_number(),
                    field.name(),
                    field.field_type(),
                    field.scale(),
                    field.offset(),
                    field.units())?;
                } else if (!field.subfields().is_empty() || !field.components().is_empty()) {
                    writeln!(out, "panic!(\"subfields/components not supported yet!\")")?;
                }
            } else {
                // simple field with out extra dereferencing necessary
                writeln!(out, "fields.push(data_field_with_info({0}, \"{1}\", FieldDataType::{2}, {3:.6}, {4:.6}, \"{5}\", value.clone())?);",
                field.def_number(),
                field.name(),
                field.field_type(),
                field.scale(),
                field.offset(),
                field.units())?;
            }
            writeln!(out, "}}")?;
        }
        writeln!(
            out,
            "_ => fields.push(unknown_field(*def_num, value.clone()))"
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(out, "Ok(fields)")?;
        writeln!(out, "}}")?;

        Ok(())
    }
}

pub fn write_decode_file(profile: &FitProfile, out: &mut File) -> Result<(), std::io::Error> {
    writeln!(
        out,
        "//! Auto generated profile messages from FIT SDK Release: {}",
        profile.version()
    )?;
    writeln!(out, "use super::field_types::*;")?;
    writeln!(out, "pub const VERSION: &str = \"{}\";", profile.version())?;

    // output all message functions
    for msg in profile.messages() {
        msg.write_decode_function_def(out)?;
    }

    Ok(())
}
