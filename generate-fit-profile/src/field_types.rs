//! Functions to generate the field-types in Rust from the fit profile.
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use crate::parse::{FieldTypeDefintion, FieldTypeVariant, FitProfile};

impl FieldTypeDefintion {
    /// Generate an enum from the field type variants
    fn generate_enum(&self, out: &mut File) -> Result<(), std::io::Error> {
        if let Some(v) = self.comment() {
            writeln!(out, "/// {v}")?;
        }
        writeln!(
            out,
            "#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]"
        )?;
        writeln!(out, "pub enum {} {{", self.titlized_name())?;
        for variant in self.variant_map().values() {
            variant.write_variant_line(out)?;
        }
        writeln!(
            out,
            "{}({}),",
            self.other_value_field_name(),
            self.base_type()
        )?;
        writeln!(out, "}}")?;

        self.generate_impl(out)
    }

    /// generate to/from integer and `to_string` implementation for field type enum
    #[allow(clippy::too_many_lines)]
    fn generate_impl(&self, out: &mut File) -> Result<(), std::io::Error> {
        writeln!(out, "impl {} {{", self.titlized_name())?;

        writeln!(out, "pub fn is_named_variant(value: i64) -> bool {{")?;
        writeln!(out, "match value {{")?;
        for variant in self.variant_map().values() {
            writeln!(out, "{} => true,", variant.value())?;
        }
        writeln!(out, "_ => false",)?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(out, "pub fn as_{0}(self) -> {0} {{", self.base_type())?;
        writeln!(out, "match self {{")?;
        for variant in self.variant_map().values() {
            writeln!(
                out,
                "{}::{} => {},",
                self.titlized_name(),
                variant.titlized_name(),
                variant.value()
            )?;
        }
        writeln!(
            out,
            "{}::{}(value) => value",
            self.titlized_name(),
            self.other_value_field_name(),
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(out, "pub fn as_i64(self) -> i64 {{")?;
        writeln!(out, "self.as_{}() as i64", self.base_type())?;
        writeln!(out, "}}")?;

        writeln!(out, "}}")?;

        writeln!(out, "impl fmt::Display for {} {{", self.titlized_name())?;
        writeln!(
            out,
            "fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
        )?;
        writeln!(out, "match &self {{")?;
        for variant in self.variant_map().values() {
            writeln!(
                out,
                "{}::{} => write!(f, \"{}\"),",
                self.titlized_name(),
                variant.titlized_name(),
                variant.name()
            )?;
        }
        if self.is_true_enum() {
            writeln!(
                out,
                "{}::{}(value) => write!(f, \"unknown_variant_{{}}\", *value)",
                self.titlized_name(),
                self.other_value_field_name(),
            )?;
        } else {
            writeln!(
                out,
                "{}::{}(value) => write!(f, \"{{}}\", value),",
                self.titlized_name(),
                self.other_value_field_name(),
            )?;
        }
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(
            out,
            "impl convert::From<{}> for {} {{",
            self.base_type(),
            self.titlized_name()
        )?;
        writeln!(out, "fn from(value: {0}) -> Self {{", self.base_type())?;
        writeln!(out, "match value {{")?;
        for variant in self.variant_map().values() {
            writeln!(
                out,
                "{} => {}::{},",
                variant.value(),
                self.titlized_name(),
                variant.titlized_name()
            )?;
        }
        writeln!(
            out,
            " _ => {}::{}(value)",
            self.titlized_name(),
            self.other_value_field_name()
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(
            out,
            "impl convert::From<i64> for {} {{",
            self.titlized_name()
        )?;
        writeln!(out, "fn from(value: i64) -> Self {{")?;
        writeln!(
            out,
            "{0}::from(value as {1})",
            self.titlized_name(),
            self.base_type()
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(out, "impl Serialize for {} {{", self.titlized_name())?;
        writeln!(
            out,
            "fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>"
        )?;
        writeln!(out, "where")?;
        writeln!(out, "S: Serializer,")?;
        writeln!(out, "{{")?;
        if self.is_true_enum() {
            writeln!(out, "serializer.serialize_str(&self.to_string())")?;
        } else {
            writeln!(out, "match &self {{")?;
            writeln!(
                out,
                "{}::{}(value) => serializer.serialize_{}(*value),",
                self.titlized_name(),
                self.other_value_field_name(),
                self.base_type(),
            )?;
            writeln!(out, "_ => serializer.serialize_str(&self.to_string())")?;
            writeln!(out, "}}")?;
        }
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        Ok(())
    }
}

impl FieldTypeVariant {
    fn write_variant_line(&self, out: &mut File) -> Result<(), std::io::Error> {
        if let Some(v) = self.comment() {
            writeln!(out, "/// {}\n{},", v, self.titlized_name())
        } else {
            writeln!(out, "{},", self.titlized_name())
        }
    }
}

fn generate_main_field_type_enum(
    field_types: &[FieldTypeDefintion],
    out: &mut File,
) -> Result<(), std::io::Error> {
    let base_types = vec![
        "Bool", "SInt8", "UInt8", "SInt16", "UInt16", "SInt32", "UInt32", "String", "Float32",
        "Float64", "UInt8z", "UInt16z", "UInt32z", "Byte", "SInt64", "UInt64", "UInt64z",
    ];
    let mut is_enum_force_false = HashSet::new();
    is_enum_force_false.insert("date_time".to_string());
    is_enum_force_false.insert("local_date_time".to_string());

    writeln!(
        out,
        "
/// Describe all possible data types of a field
///
/// The Enum type's value is actually an enum of enums.
#[derive(Clone, Copy, Debug)]
pub enum FieldDataType {{
"
    )?;
    for type_name in base_types {
        writeln!(out, " {type_name},")?;
    }
    for field_type in field_types {
        writeln!(out, "{},", field_type.titlized_name())?;
    }

    writeln!(out, "}}")?;

    writeln!(out, "impl FieldDataType {{")?;
    writeln!(out, "    #[allow(clippy::match_like_matches_macro)]")?;
    writeln!(out, "    pub fn is_enum_type(self) -> bool {{")?;
    writeln!(out, "        match self {{")?;
    for field_type in field_types {
        if !field_type.variant_map().is_empty() && !is_enum_force_false.contains(field_type.name())
        {
            writeln!(
                out,
                "FieldDataType::{} => true,",
                field_type.titlized_name()
            )?;
        }
    }
    writeln!(out, "            _ => false,")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;

    writeln!(
        out,
        "    pub fn is_named_variant(self, value: i64) -> bool {{"
    )?;
    writeln!(out, "        match self {{")?;
    for field_type in field_types {
        if !field_type.variant_map().is_empty() && !is_enum_force_false.contains(field_type.name())
        {
            writeln!(
                out,
                "FieldDataType::{0} => {0}::is_named_variant(value),",
                field_type.titlized_name(),
            )?;
        }
    }
    writeln!(out, "            _ => false,")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(
        out,
        "pub fn get_field_variant_as_string(field_type: FieldDataType , value: i64) -> String {{"
    )?;
    writeln!(out, "    match field_type {{")?;
    for field_type in field_types {
        if !field_type.variant_map().is_empty() && !is_enum_force_false.contains(field_type.name())
        {
            writeln!(
                out,
                "        FieldDataType::{0} => {0}::from(value).to_string(),",
                field_type.titlized_name()
            )?;
        }
    }
    writeln!(out, "        _ => format!(\"Undefined{{}}\", value),")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    Ok(())
}

pub fn write_types_file(profile: &FitProfile, out: &mut File) -> Result<(), std::io::Error> {
    writeln!(
        out,
        "//! Auto generated profile field types from FIT SDK Release: {}",
        profile.version()
    )?;
    writeln!(
        out,
        "//! Not all of these may be used by the defined set of FIT messages"
    )?;
    writeln!(out, "#![allow(missing_docs)]")?;
    writeln!(out, "#![allow(dead_code)]")?;
    writeln!(out, "#![allow(clippy::unreadable_literal)]")?;
    writeln!(out, "use serde::Serialize;")?;
    writeln!(out, "use serde::ser::Serializer;")?;
    writeln!(out, "use std::convert;")?;
    writeln!(out, "use std::fmt;")?;

    // output enums and implementations
    for field_type in profile.field_types() {
        if !field_type.variant_map().is_empty() {
            field_type.generate_enum(out)?;
        }
    }
    generate_main_field_type_enum(profile.field_types(), out)?;

    Ok(())
}
