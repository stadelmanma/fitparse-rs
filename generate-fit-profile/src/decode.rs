//! Functions to generate the message decoding functions from the fit profile.
use crate::parse::{FitProfile, MessageDefinition, MessageFieldDefinition};
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
            if let Some(v) = field.comment() {
                writeln!(out, "// {}", v)?;
            }
            if !field.components().is_empty() {
                writeln!(out, "let mut bytes = value.to_ne_bytes().into_iter();")?;
                let mut bit_offset = 0u8;
                for comp in field.components() {
                    let dest_def_number = self.get_field_by_name(comp.name()).def_number();
                    writeln!(
                        out,
                        "let {}_value = expand_components(bytes, {}, {});",
                        comp.name(),
                        bit_offset,
                        bit_offset + comp.bits()
                    )?;

                    // --------------------------------------------------------
                    // TODO: components expanding into array fields are not
                    //       handled correctly see
                    //       hr_message_event_timestamp_field
                    // TODO: components expanding into other components might
                    //       also no behandled correctly, I think for that to
                    //       work I would need todo component expansion in the
                    //       field function and mmutate the data map there
                    // --------------------------------------------------------

                    // insert back into datamap for subfield look ups
                    writeln!(
                        out,
                        "data_map.insert({}, {}_value);",
                        dest_def_number,
                        comp.name()
                    )?;

                    if field.components().len() > 1 {
                        // according to some sample code in the FIT SDK when a component expands
                        // to a composite field (i.e. one with 2 or more components) the scale and
                        // offset are not applied but this seems inconsistent or maybe I'm
                        // misunderstaning the code
                        writeln!(out, "fields.push({}_{}_field(mesg_num, data_map, accumlators, {}, {3:.6}, {4:.6}, \"{5}\", value.clone())?);",
                            self.function_name(),
                            comp.name(),
                            comp.accumulate(),
                            comp.scale(),
                            comp.offset(),
                            comp.units()
                        )?;
                    }
                    bit_offset += comp.bits();
                }
            } else if !field.subfields().is_empty() {
                // for this to work we'll probably need to drop into a separate subfield expansion
                // function using an if/else block since subfields might look at different fields to
                // do the resolution
                writeln!(out, "panic!(\"subfields not supported yet!\")")?;
            } else {
                writeln!(out, "fields.push({}_{}_field(mesg_num, data_map, accumlators, {}, {3:.6}, {4:.6}, \"{5}\", value.clone())?);",
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
        writeln!(
            out,
            "_ => fields.push(unknown_field(*def_num, value.clone()))"
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(out, "Ok(fields)")?;
        writeln!(out, "}}")?;

        for field in self.field_map().values() {
            self.write_create_field_fn_def(out, field)?;
        }

        Ok(())
    }

    fn write_create_field_fn_def(
        &self,
        out: &mut File,
        field: &MessageFieldDefinition,
    ) -> Result<(), std::io::Error> {
        writeln!(out, "fn {}_{}_field(mesg_num: MesgNum, data_map: HashMap<u8, Option<Value>>, accumlators: &mut HashMap<u32, Value>, accumulate: bool, scale: f64, offset: f64, units: &'static str, value: Value) -> Result<FitDataField> {{", self.function_name(), field.name())?;
        // generate acccumated field code
        writeln!(out, "let value = if accumulate {{")?;
        writeln!(
            out,
            "  calculate_cumulative_value(accumlators, mesg_num.as_u16(), {0}, value)?;",
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
