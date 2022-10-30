//! Functions to generate the message definitions in Rust from the fit profile.
use std::fs::File;
use std::io::prelude::*;

use crate::parse::{FitProfile, MessageDefinition, MessageFieldComponent, MessageFieldDefinition};

impl MessageDefinition {
    fn get_field_by_name(&self, name: &str) -> &MessageFieldDefinition {
        for field in self.field_map().values() {
            if field.name() == name {
                return field;
            }
        }
        panic!("No field with name: {:?}", name);
    }

    fn function_name(&self) -> String {
        format!("{}_message", self.name())
    }

    fn write_function_def(&self, out: &mut File) -> Result<(), std::io::Error> {
        if let Some(v) = self.comment() {
            writeln!(out, "/// {}", v)?;
        }
        writeln!(out, "pub fn {}() -> MessageInfo {{", self.function_name())?;
        writeln!(out, "    let mut fields = HashMap::new();")?;
        for field in self.field_map().values() {
            field.generate_field_info_struct(out, self, "field")?;
            writeln!(out, "fields.insert({}, field);", field.def_number())?;
        }
        writeln!(out, "    MessageInfo {{")?;
        writeln!(out, "        name: \"{}\",", self.name())?;
        writeln!(
            out,
            "        global_message_number: MesgNum::{},",
            self.titlized_name()
        )?;
        writeln!(out, "        fields")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }
}

impl MessageFieldDefinition {
    pub fn generate_field_info_struct(
        &self,
        out: &mut File,
        mesg: &MessageDefinition,
        var_name: &str,
    ) -> Result<(), std::io::Error> {
        let subfield_var: &'static str;
        if self.subfields().is_empty() {
            subfield_var = "Vec::new()";
        } else {
            subfield_var = "subfields";
            writeln!(out, "let mut {} = Vec::new();", subfield_var)?;
            for (fld_name, fld_value, sub_info) in self.subfields() {
                sub_info.generate_field_info_struct(out, mesg, "sub_fld")?;
                let ref_field = mesg.get_field_by_name(fld_name);
                writeln!(
                    out,
                    "subfields.push(({}, {}::{}.as_i64(), sub_fld));",
                    ref_field.def_number(),
                    ref_field.field_type(),
                    fld_value,
                )?;
            }
        }

        let components_var: &'static str;
        if self.components().is_empty() {
            components_var = "Vec::new()";
        } else {
            components_var = "components";
            writeln!(out, "let mut {} = Vec::new();", components_var)?;
            for comp_info in self.components() {
                comp_info.generate_comp_field_info_struct(out, mesg, "comp_fld")?;
                writeln!(out, "components.push(comp_fld);")?;
            }
        }

        if let Some(v) = self.comment() {
            writeln!(out, "// {}", v)?;
        }
        writeln!(
            out,
            "    let {} = FieldInfo {{
            name: \"{}\",
            field_type: FieldDataType::{},
            def_number: {},
            scale: {:.6},
            offset: {:.6},
            units: \"{}\",
            accumulate: {},
            subfields: {},
            components: {},
        }};",
            var_name,
            self.name(),
            self.field_type(),
            self.def_number(),
            self.scale(),
            self.offset(),
            self.units(),
            self.accumulate(),
            subfield_var,
            components_var
        )?;

        Ok(())
    }
}

impl MessageFieldComponent {
    fn generate_comp_field_info_struct(
        &self,
        out: &mut File,
        mesg: &MessageDefinition,
        var_name: &str,
    ) -> Result<(), std::io::Error> {
        let dest_def_number = mesg.get_field_by_name(self.name()).def_number();
        writeln!(
            out,
            "let {} = ComponentFieldInfo {{
            dest_def_number: {},
            scale: {:.6},
            offset: {:.6},
            units: \"{}\",
            bits: {},
            accumulate: {},
        }};",
            var_name,
            dest_def_number,
            self.scale(),
            self.offset(),
            self.units(),
            self.bits(),
            self.accumulate(),
        )?;

        Ok(())
    }
}

fn create_mesg_num_to_mesg_info_fn(
    messages: &[MessageDefinition],
    out: &mut File,
) -> Result<(), std::io::Error> {
    writeln!(out, "impl MesgNum {{")?;
    writeln!(out, "    pub fn message_info(self) -> MessageInfo {{")?;
    writeln!(out, "        match self {{")?;
    for msg in messages {
        writeln!(
            out,
            "            MesgNum::{} => {}(),",
            msg.titlized_name(),
            msg.function_name()
        )?;
    }
    writeln!(out, "            _ => unknown_message(self),")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    Ok(())
}

pub fn write_messages_file(profile: &FitProfile, out: &mut File) -> Result<(), std::io::Error> {
    writeln!(
        out,
        "//! Auto generated profile messages from FIT SDK Release: {}",
        profile.version()
    )?;
    writeln!(out, "#![allow(missing_docs)]")?;
    writeln!(out, "#![allow(clippy::redundant_field_names)]")?;
    writeln!(out, "#![allow(clippy::unreadable_literal)]")?;
    writeln!(out, "use std::collections::HashMap;")?;
    writeln!(
        out,
        "use super::{{ComponentFieldInfo, FieldDataType, FieldInfo, MessageInfo}};"
    )?;
    writeln!(out, "use super::field_types::*;")?;
    writeln!(out, "pub const VERSION: &str = \"{}\";", profile.version())?;

    // output all message functions
    for msg in profile.messages() {
        msg.write_function_def(out)?;
    }

    // output an unknown_message() fn that has no fields
    writeln!(
        out,
        "fn unknown_message(global_message_number: MesgNum) -> MessageInfo {{"
    )?;
    writeln!(out, "    MessageInfo {{")?;
    writeln!(out, "        name: \"unknown\",")?;
    writeln!(out, "        global_message_number,")?;
    writeln!(out, "        fields: HashMap::new()")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    // output MesgNum implementation to allow parser to fetch info
    create_mesg_num_to_mesg_info_fn(profile.messages(), out)?;

    Ok(())
}
