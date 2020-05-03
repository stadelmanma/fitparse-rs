//! Defines the FIT profile used to convert an AST into final values
use crate::objects::{DataField, DataFieldValue, FitDataRecord};
use crate::parser::FitDataRecordNode;
use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone};
use std::collections::HashMap;
use std::convert::Into;

pub mod field_types;
use field_types::{get_field_variant_as_string, FieldDataType, MesgNum};

pub mod messages;

/// Convert a collection of FIT data record AST nodes into a proper FitDataRecord object by
/// applying the defined FIT data profile. Some fields are accumulated across multiple messages
/// we store those in a special mapping where the key is (global_message_number << 8 | def_number)
/// to avoid needing a nested hash map. The value in the mapping is the raw DataFieldValue parsed
/// prior to any field rescaling.
pub fn apply_data_profile(nodes: Vec<FitDataRecordNode>) -> Vec<FitDataRecord> {
    let mut accumulator = Accumlator::new();
    let mut records = Vec::new();

    for node in nodes {
        let mesg_num = MesgNum::from_u16(node.global_message_number);
        let mesg_info = mesg_num.message_info();

        // set the message number and check if we have an actual timestamp field defined
        accumulator.current_mesg_number = node.global_message_number;
        if let Some(value) = node.fields.get(&253) {
            accumulator.base_timestamp =
                if let Some(info) = mesg_info.get_field(253, &HashMap::new()) {
                    if let FieldDataType::LocalDateTime = info.field_type {
                        TimestampField::Local(value.as_i64().unwrap_or(0))
                    } else {
                        TimestampField::Utc(value.as_i64().unwrap_or(0))
                    }
                } else {
                    // default to assuming we have a UTC timestamp
                    TimestampField::Utc(value.as_i64().unwrap_or(0))
                }
        }

        // process defined fields and add timstamp field if the header has a time offset
        let mut record = FitDataRecord {
            kind: mesg_num.to_string(),
            fields: build_data_fields_from_map(mesg_info, node.fields, &mut accumulator),
        };
        if let Some(time_offset) = node.time_offset {
            record.fields.push(DataField {
                name: String::from("timestamp"),
                units: String::new(),
                scale: 1.0,
                offset: 0.0,
                value: accumulator.increment_timestamp(time_offset),
                raw_value: DataFieldValue::UInt8(time_offset),
            });
        }
        // TODO process developer fields

        records.push(record);
    }

    records
}

/// Describes a single message pulled from the FIT profile.
#[derive(Clone, Debug)]
pub struct MessageInfo {
    name: &'static str,
    global_message_number: u16,
    fields: HashMap<u8, FieldInfo>,
}

impl MessageInfo {
    /// Fetch the information for a specific field, if the field contains subfields then
    /// we use the data values provided to try and de-reference it and return the subfield
    /// info instead
    fn get_field(&self, key: u8, data_map: &HashMap<u8, DataFieldValue>) -> Option<&FieldInfo> {
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
struct FieldInfo {
    name: &'static str,
    field_type: FieldDataType,
    def_number: u8,
    scale: f64,
    offset: f64,
    units: &'static str,
    accumulate: bool,
    subfields: Vec<(u8, i64, FieldInfo)>, // ref_def_num, ref_value, subfield_info
    components: Vec<ComponentFieldInfo>,
}

impl FieldInfo {
    fn components(&self) -> &[ComponentFieldInfo] {
        &self.components
    }

    fn expand_components(
        &self,
        value: &DataFieldValue,
    ) -> Vec<(&ComponentFieldInfo, DataFieldValue)> {
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
                } else {
                    bit_pos += 1;
                }
            }
            components.push((comp_fld, DataFieldValue::UInt64(tmp)));
        }

        components
    }

    /// convert the value into a "output" form applying any scaling or enum conversions
    fn convert_value(&self, value: &DataFieldValue) -> DataFieldValue {
        // for array types just map and return
        if let DataFieldValue::Array(vals) = value {
            return DataFieldValue::Array(vals.iter().map(|v| self.convert_value(v)).collect());
        }

        // handle time types specially, if for some reason I can't convert to an integer we will
        // just dump the reference timestamp by passing it a 0
        match self.field_type {
            FieldDataType::DateTime => {
                return TimestampField::Utc(value.as_i64().unwrap_or(0)).into()
            }
            FieldDataType::LocalDateTime => {
                return TimestampField::Local(value.as_i64().unwrap_or(0)).into()
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
}

/// Describes a componet field within a largest field pulled from the FIT profile
#[derive(Clone, Debug)]
struct ComponentFieldInfo {
    dest_def_number: u8,
    scale: f64,
    offset: f64,
    units: &'static str,
    bits: u8,
    accumulate: bool,
}

/// Stores the timestamp offset from the FIT reference date in seconds
#[derive(Debug, Copy, Clone)]
enum TimestampField {
    Local(i64),
    Utc(i64),
}

impl TimestampField {
    fn value(&self) -> i64 {
        match self {
            Self::Local(value) => *value,
            Self::Utc(value) => *value,
        }
    }

    /// converts offset value into a proper timestamp
    fn to_date_time(self) -> DateTime<Local> {
        // reference date defined in FIT profile, it's either in UTC or local TZ
        let ref_date = NaiveDate::from_ymd(1989, 12, 31).and_hms(0, 0, 0);
        match self {
            Self::Local(value) => {
                TimeZone::from_local_datetime(&Local, &ref_date).unwrap() + Duration::seconds(value)
            }
            Self::Utc(value) => {
                TimeZone::from_utc_datetime(&Local, &ref_date) + Duration::seconds(value)
            }
        }
    }
}

impl Into<DataFieldValue> for TimestampField {
    fn into(self) -> DataFieldValue {
        return DataFieldValue::Timestamp(self.to_date_time());
    }
}

/// Handles values that need to accumlate across multiple messages as well as applying the
/// time offset to the current base timestamp
struct Accumlator {
    base_timestamp: TimestampField,
    current_mesg_number: u16,
    field_values: HashMap<u32, DataFieldValue>,
}

impl Accumlator {
    fn new() -> Self {
        Accumlator {
            base_timestamp: TimestampField::Utc(0),
            current_mesg_number: 0,
            field_values: HashMap::new(),
        }
    }

    fn increment(&mut self, def_num: u8, value: &DataFieldValue) -> &DataFieldValue {
        let key = (self.current_mesg_number as u32) << 8 | def_num as u32;
        self.field_values
            .entry(key)
            .and_modify(|v| *v += value.clone())
            .or_insert(value.clone())
    }

    fn increment_timestamp(&mut self, offset: u8) -> DataFieldValue {
        let offset: i64 = offset as i64;
        let mask: i64 = 31; // last 5 significant bits of value
        let mut value = offset + (self.base_timestamp.value() & !mask);
        // account for rollover if needed
        if offset < (self.base_timestamp.value() & mask) {
            value += 32;
        }

        // update stored value and return
        match self.base_timestamp {
            TimestampField::Local(_) => {
                self.base_timestamp = TimestampField::Local(value);
            }
            TimestampField::Utc(_) => {
                self.base_timestamp = TimestampField::Utc(value);
            }
        }
        self.base_timestamp.clone().into()
    }
}

/// Add processed data fields from raw values in the data mapping
fn build_data_fields_from_map(
    mesg_info: MessageInfo,
    mut data_map: HashMap<u8, DataFieldValue>,
    accumulator: &mut Accumlator,
) -> Vec<DataField> {
    // initialize process queue with field info for decoded, valid fields.
    let mut data_fields = Vec::new();
    let mut process_queue: Vec<(u8, Option<FieldInfo>)> = data_map
        .keys()
        .map(|k| (*k, mesg_info.get_field(*k, &data_map).cloned()))
        .collect();

    while !process_queue.is_empty() {
        let (def_num, field_info) = process_queue.remove(0);
        let mut value = &data_map[&def_num];

        if let Some(field_info) = field_info {
            // check for components, the decomposition is profile specific so
            // we dont store the parent field because we want the JSON to be
            // profile agnostic
            if field_info.components().is_empty() {
                if field_info.accumulate {
                    value = accumulator.increment(def_num, value)
                }
                data_fields.push(data_field_with_info(&field_info, value));
            } else {
                let (infos, values): (Vec<_>, Vec<_>) =
                    field_info.expand_components(&value).into_iter().unzip();
                // add all data to map first and then update process queue since reference fields
                // are data dependent
                for (comp_info, comp_value) in infos.iter().zip(values.into_iter()) {
                    data_map.insert(comp_info.dest_def_number, comp_value);
                }
                for comp_info in infos {
                    let old_field_info = mesg_info.get_field(comp_info.dest_def_number, &data_map);
                    let new_field_info = match old_field_info {
                        Some(info) => Some(FieldInfo {
                            name: info.name,
                            field_type: info.field_type,
                            def_number: info.def_number,
                            scale: comp_info.scale,
                            offset: comp_info.offset,
                            units: comp_info.units,
                            accumulate: comp_info.accumulate,
                            subfields: info.subfields.clone(),
                            // double component expansion breaks scale/offset adjustment
                            components: Vec::new(), // info.components.clone(),
                        }),
                        None => None,
                    };
                    process_queue.push((comp_info.dest_def_number, new_field_info));
                }
            }
        } else {
            data_fields.push(unknown_field(def_num, &value));
        }
    }

    data_fields
}

/// Build a data field using the provided FIT profile information
fn data_field_with_info(field_info: &FieldInfo, value: &DataFieldValue) -> DataField {
    DataField {
        name: field_info.name.to_string(),
        units: field_info.units.to_string(),
        scale: field_info.scale,
        offset: field_info.offset,
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
