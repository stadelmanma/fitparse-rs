//! Functions to generate the message decoding functions from the fit profile.
use crate::parse::{FitProfile, MessageDefinition, MessageFieldDefinition};
use std::collections::{HashMap, HashSet};
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

        if self.field_map().values().any(|f| f.subfields().len() != 0) {
            // only certain messages need this, it's the specific case where a subfield
            // references a field that can come from a component and that component has
            // a def_number greater than the field doing the lookup, see the Monitoring
            // message, field #3 references field #5 and when not explicitly defined field
            // #5 can be derived from field #24.
            writeln!(out, "let mut retry: HashSet<u8> = HashSet::new();")?;
        }

        writeln!(out, "let mut fields = Vec::new();")?;
        writeln!(out, "let mut entries: VecDeque<(u8, Value)> = data_map.iter().map(|(k, v)| (*k, v.clone())).collect();")?;
        writeln!(
            out,
            "while let Some((def_num, value)) = entries.pop_front() {{"
        )?;
        writeln!(out, "match def_num {{")?;
        for field in self.field_map().values() {
            writeln!(out, "{0} => {{", field.def_number())?;
            field.write_field_decode_block(out, &self, "value", None, None)?;
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
        alt_scale: Option<f64>,
        alt_offset: Option<f64>,
    ) -> Result<(), std::io::Error> {
        if let Some(v) = self.comment() {
            writeln!(out, "// {}", v)?;
        }
        if !self.components().is_empty() {
            self.write_component_exp(out, mesg_def, val_str, alt_scale, alt_offset)?;
        } else if !self.subfields().is_empty() {
            self.write_subfield_deref(out, mesg_def, val_str)?;
        } else {
            writeln!(
                out,
                "fields.push({}?);",
                self.generate_create_fn_call(mesg_def, val_str, alt_scale, alt_offset)
            )?;
        }

        Ok(())
    }

    fn write_component_exp(
        &self,
        out: &mut File,
        mesg_def: &MessageDefinition,
        val_str: &str,
        alt_scale: Option<f64>,
        alt_offset: Option<f64>,
    ) -> Result<(), std::io::Error> {
        // detect array fields so we can append a counter to variable name
        let mut array_flds = HashMap::new();
        for (_, fld) in self.components() {
            *array_flds.entry(fld.def_number()).or_insert(0) += 1;
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
                var_names.push(format!(
                    "{}_{}",
                    fld.name(),
                    array_flds.get(&fld.def_number()).unwrap()
                ));
            } else {
                var_names.push(fld.name().to_owned());
            }
        }

        // expand them and then pop to individual values to avoid cloning twice
        writeln!(
            out,
            "let mut {}_component_values = expand_components({}, &[{}]);",
            self.name(),
            val_str,
            self.components()
                .iter()
                .map(|(n, _)| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )?;
        for vn in var_names.iter().rev() {
            writeln!(
                out,
                "let {} = {}_component_values.pop().unwrap();",
                vn,
                self.name()
            )?;
        }

        let mut comps_decoded = HashSet::new();
        for (_, comp) in self.components().iter() {
            // array components show up more than once, but we collect those
            // into an array value above so we only want to decode it once
            if comps_decoded.contains(&comp.def_number()) {
                continue;
            }

            // generate a vec! macro to build the array field
            if array_flds.contains_key(&comp.def_number()) {
                writeln!(
                    out,
                    "let {} = Value::Array(vec![{}]);",
                    comp.name(),
                    (0..*array_flds.get(&comp.def_number()).unwrap())
                        .into_iter()
                        .map(|i| format!("{}_{}", comp.name(), i + 1))
                        .collect::<Vec<String>>()
                        .join(",")
                )?;
            }

            // insert back into datamap for subfield look ups and then generate
            // a decode block incase the component has subfields/nested comps
            writeln!(
                out,
                "data_map.insert({}, {}.clone());",
                comp.def_number(),
                comp.name()
            )?;

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

            comp.write_field_decode_block(out, mesg_def, comp.name(), alt_scale, alt_offset)?;
            comps_decoded.insert(comp.def_number());
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
                    sub_field_info.generate_create_fn_call(mesg_def, val_str, None, None)
                )?;
            } else {
                // generate a nested component expansion
                sub_field_info.write_component_exp(out, mesg_def, val_str, None, None)?;
            }
            writeln!(out, "}}")?;
        }
        // give it one more chance to be resolved incase a component expansion hadn't occured yet
        writeln!(out, "else if !retry.contains(&def_num) {{")?;
        writeln!(out, "retry.insert({});", self.def_number())?;
        writeln!(
            out,
            "entries.push_back(({}, {}));",
            self.def_number(),
            val_str
        )?;
        writeln!(out, "}}")?;
        //
        writeln!(out, "else {{")?;
        writeln!(
            out,
            "fields.push({}?);",
            self.generate_create_fn_call(mesg_def, val_str, None, None)
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

    fn generate_create_fn_call(
        &self,
        mesg_def: &MessageDefinition,
        val_str: &str,
        alt_scale: Option<f64>,
        alt_offset: Option<f64>,
    ) -> String {
        format!(
            "{0}_{1}_field(mesg_num, accumlators, data_map, {2}, {3:.6}, {4:.6}, \"{5}\", {6})",
            mesg_def.function_name(),
            self.name(),
            self.accumulate(),
            alt_scale.unwrap_or(self.scale()),
            alt_offset.unwrap_or(self.offset()),
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
    writeln!(out, "use std::collections::{{HashMap, HashSet, VecDeque}};")?;
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
