//! Helper functions and structures needed to decode a FIT file using the defined profile.
use super::parser::FitDataMessage;
use crate::error::Result;
use crate::profile::{MesgNum, TimestampField};
use crate::{FitDataField, FitDataRecord, Value};
use std::collections::HashMap;
use std::convert::{From, TryInto};

/// Decodes a raw FitDataMessage using the defined profile. Additional logic is used to handle
/// values that need to accumlate across multiple messages as well as applying the
/// time offset to the current base timestamp.
pub struct Decoder {
    base_timestamp: TimestampField,
    accumulate_fields: HashMap<u32, Value>,
}

impl Decoder {
    /// Create a new decoder
    pub fn new() -> Self {
        Decoder {
            base_timestamp: TimestampField::Utc(0),
            accumulate_fields: HashMap::new(),
        }
    }

    /// Reset accumation related fields
    pub fn reset(&mut self) {
        self.base_timestamp = TimestampField::Utc(0);
        self.accumulate_fields = HashMap::new();
    }

    /// Decode a raw FIT data message by applying the defined profile
    pub fn decode_message(&mut self, mut message: FitDataMessage) -> Result<FitDataRecord> {
        let mesg_num = MesgNum::from(message.global_message_number());
        let mut record = FitDataRecord::new(mesg_num);

        // check if we have a real timestamp field to set the reference
        // field id 253 always appears to be a timestamp with the type
        // FieldDataType::DateTime
        if let Some(value) = message.fields().get(&253) {
            self.base_timestamp = TimestampField::Utc(value.clone().try_into().unwrap_or(0));
        }

        // process raw data
        let mut fields =
            mesg_num.decode_message(message.fields_mut(), &mut self.accumulate_fields)?;
        fields.sort_by_key(|f| f.number());
        record.extend(fields);

        // Add a timestamp field if we have a time offset
        if let Some(time_offset) = message.time_offset() {
            record.push(FitDataField::new(
                String::from("timestamp"),
                253,
                self.update_timestamp(time_offset),
                String::new(),
            ));
        }

        // TODO: process developer fields

        Ok(record)
    }

    /// Update the timestamp with a new offset and return the value
    fn update_timestamp(&mut self, offset: u8) -> Value {
        let offset: i64 = offset as i64;
        let mask: i64 = 31; // last 5 significant bits of value
        let mut value = offset + (self.base_timestamp.as_i64() & !mask);
        // account for rollover if needed
        if offset < (self.base_timestamp.as_i64() & mask) {
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

        Value::from(self.base_timestamp)
    }
}
