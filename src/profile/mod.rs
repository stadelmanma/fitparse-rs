use crate::parser::FitDataRecordNode;
use crate::objects::{DataField, DataFieldValue, FitDataRecord};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, TimeZone};
use std::collections::HashMap;

pub mod field_types;
use field_types::{get_field_variant_as_string, FieldDataType, MesgNum};

pub mod messages;

/// Describes a single message pulled from the FIT profile.
#[derive(Clone, Debug)]
pub struct MessageInfo {
    name: &'static str,
    fields: HashMap<u8, FieldInfo>,
}

impl MessageInfo {
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Fetch the information for a specific field, if the field contains subfields then
    /// we use the data values provided to try and de-reference it and return the subfield
    /// info instead
    pub fn get_field(&self, key: u8, data_map: &HashMap<u8, DataFieldValue>) -> Option<&FieldInfo> {
        if let Some(field) = self.fields.get(&key) {
            // check against subfields
            for (num, val, sub_info) in &field.subfields {
                if let Some(v) = data_map.get(num) {
                    if v.as_i64().map_or(false, |v| v == *val) {
                        return Some(sub_info);
                    }
                }
            }
            // fallback to initial field info as default return
            return Some(field);
        }
        None
    }
}

/// Describes a single field within a message pulled from the FIT profile
#[derive(Clone, Debug)]
pub struct FieldInfo {
    name: &'static str,
    field_type: FieldDataType,
    def_number: u8,
    scale: f64,
    offset: f64,
    units: &'static str,
    subfields: Vec<(u8, i64, FieldInfo)>, // ref_def_num, ref_value, subfield_info
    components: Vec<ComponentFieldInfo>,
}

impl FieldInfo {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn units(&self) -> &'static str {
        self.units
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn offset(&self) -> f64 {
        self.offset
    }

    pub fn subfields(&self) -> &[(u8, i64, FieldInfo)] {
        &self.subfields
    }

    pub fn components(&self) -> &[ComponentFieldInfo] {
        &self.components
    }

    pub fn expand_components(&self, value: &DataFieldValue) -> Vec<(&ComponentFieldInfo, DataFieldValue)> {
        // extract out each field by masking specific bits, spanning 1 or more bytes
        let bit_mask = [1u8, 2u8, 4u8, 8u8, 16u8, 32u8, 64u8, 128u8];
        let mut bytes = value.to_ne_bytes().into_iter();
        let mut components = Vec::new();
        let mut byte = bytes.next().unwrap_or(0);
        let mut bit_pos = 0;
        for comp_fld in &self.components {
            let mut tmp: u64 = 0;
            for pos in 0..comp_fld.bits {
                tmp |= (((byte & bit_mask[bit_pos]) >> bit_pos) as u64) << pos;
                if bit_pos == 7 {
                    byte = bytes.next().unwrap_or(0);
                    bit_pos = 0;
                }
                else {
                    bit_pos += 1;
                }
            }
            components.push((comp_fld, DataFieldValue::UInt64(tmp)));
        }

        components
    }

    /// convert the value into a "output" form applying any scaling or enum conversions
    pub fn convert_value(&self, value: &DataFieldValue) -> DataFieldValue {
        // for array types just map and return
        if let DataFieldValue::Array(vals) = value {
            return DataFieldValue::Array(vals.iter().map(|v| self.convert_value(v)).collect());
        }

        // handle time types specially
        match self.field_type {
            FieldDataType::DateTime | FieldDataType::LocalDateTime => {
                return self.parse_timestamp(value)
            }
            _ => (),
        }

        // convert enum or rescale integer value into floating point
        if self.field_type.is_enum_type() {
            if let Some(val) = value.as_i64() {
                DataFieldValue::String(get_field_variant_as_string(self.field_type, val))
            } else {
                panic!(format!(
                    "Cannot convert non-integer data type to enum variant - {:?}",
                    value
                ));
            }
        } else if self.scale != 1.0 || self.offset != 0.0 {
            if let Some(val) = value.as_f64() {
                self.rescale_value(val)
            } else {
                panic!(format!(
                    "Cannot convert non-numeric data type to float value - {:?}",
                    value
                ));
            }
        } else {
            value.clone()
        }
    }

    /// Rescale value using the scale and offset into a floating point number
    fn rescale_value(&self, value: f64) -> DataFieldValue {
        DataFieldValue::Float64(value / self.scale - self.offset)
    }

    /// converts value into a proper timestamp
    fn parse_timestamp(&self, value: &DataFieldValue) -> DataFieldValue {
        // reference date defined in FIT profile, it's either in UTC or local TZ
        let ref_date: NaiveDateTime = NaiveDate::from_ymd(1989, 12, 31).and_hms(0, 0, 0);
        let sec_since: Duration;
        if let Some(val) = value.as_i64() {
            sec_since = Duration::seconds(val);
        } else {
            // return raw value as fallback if we can't get seconds as int
            return value.clone();
        }

        // process convert ref date to UTC/local TZ and add Duration, we want to return a local
        // timestamp with TZ info
        let ref_date = match self.field_type {
            FieldDataType::DateTime => TimeZone::from_utc_datetime(&Local, &ref_date),
            FieldDataType::LocalDateTime => {
                TimeZone::from_local_datetime(&Local, &ref_date).unwrap()
            }
            _ => panic!("Invalid field type in self.parse_timestamp!"),
        };

        DataFieldValue::Timestamp(ref_date + sec_since)
    }
}

/// Describes a componet field within a largest field pulled from the FIT profile
#[derive(Clone, Debug)]
pub struct ComponentFieldInfo {
    dest_def_number: u8,
    scale: f64,
    offset: f64,
    units: &'static str,
    bits: u8,
    accumulate: bool,
}

impl ComponentFieldInfo {
    pub fn dest_def_number(&self) -> u8 {
        self.dest_def_number
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn offset(&self) -> f64 {
        self.offset
    }

    pub fn units(&self) -> &str {
        self.units
    }

    pub fn bits(&self) -> u8 {
        self.bits
    }

    pub fn accumulate(&self) -> bool {
        self.accumulate
    }
}

/// Convert a collection of FIT data record AST nodes into a proper FitDataRecord object by
/// applying the defined FIT data profile
pub fn apply_data_profile(nodes: Vec<FitDataRecordNode>) -> Vec<FitDataRecord> {
    let mut accumlated_values: HashMap<u16, HashMap<u8, DataFieldValue>> = HashMap::new();
    let mut records = Vec::new();

    for mut node in nodes {
        let mesg_num = MesgNum::from_u16(node.global_message_number);
        let mesg_info = mesg_num.message_info();
        let mut rec_fields = Vec::new();

        // initialize process queue with field info for decoded, valid fields.
        let mut process_queue: Vec<(u8, Option<FieldInfo>)> = node.fields.keys().map(|k| (*k, mesg_info.get_field(*k, &node.fields).cloned())).collect();

        // process data mapping populating the record fields
        build_data_fields_from_map(&mesg_info, &mut process_queue, &mut node.fields, &mut rec_fields);

        // todo process developer fields

        records.push(FitDataRecord{
            kind: mesg_num.to_string(),
            time_offset: node.time_offset,
            fields: rec_fields,
        });
    }

    records
}

/// Add processed data fields from raw values in the data mapping
fn build_data_fields_from_map(
    mesg_info: &MessageInfo,
    process_queue: &mut Vec<(u8, Option<FieldInfo>)>,
    data_map: &mut HashMap<u8, DataFieldValue>,
    data_fields: &mut Vec<DataField>,
) {
    while !process_queue.is_empty() {
        let (def_num, field_info) = process_queue.remove(0);
        let value = &data_map[&def_num];

        if let Some(field_info) = field_info {
            // check for components, the decomposition is profile specific so
            // we dont store the parent field because we want the JSON to be
            // profile agnostic
            if field_info.components().is_empty() {
                data_fields.push(data_field_with_info(&field_info, &value));
            } else {
                let (infos, values): (Vec<_>, Vec<_>) = field_info.expand_components(&value).into_iter().unzip();
                // add all data to map first and then update process queue since reference fields
                // are data dependent
                for (comp_info, comp_value) in infos.iter().zip(values.into_iter()) {
                    data_map.insert(comp_info.dest_def_number, comp_value);
                }
                for comp_info in infos {
                    let old_field_info = mesg_info.get_field(comp_info.dest_def_number, data_map);
                    let new_field_info = match old_field_info {
                        Some(info) => {
                            Some(FieldInfo {
                                name: info.name,
                                field_type: info.field_type,
                                def_number: info.def_number,
                                scale: comp_info.scale,
                                offset: comp_info.offset,
                                units: comp_info.units,
                                subfields: info.subfields.clone(),
                                components: info.components.clone(),
                            })
                        },
                        None => None
                    };
                    process_queue.push((comp_info.dest_def_number, new_field_info));
                }
            }
        } else {
            data_fields.push(unknown_field(def_num, &value));
        }
    }
}

/// Build a data field using the provided FIT profile information
fn data_field_with_info(field_info: &FieldInfo, value: &DataFieldValue) -> DataField {
    DataField {
        name: field_info.name().to_string(),
        units: field_info.units().to_string(),
        scale: field_info.scale(),
        offset: field_info.offset(),
        value: field_info.convert_value(value),
        raw_value: value.clone(),
    }
}

/// Create an "unknown" field as a placeholder if we don't have any field information
fn unknown_field(field_def_num: u8, value: &DataFieldValue) -> DataField {
    DataField {
        name: format!("unknown_field_{}", field_def_num),
        units: "".to_string(),
        scale: 1.0,
        offset: 0.0,
        value: value.clone(),
        raw_value: value.clone(),
    }
}
