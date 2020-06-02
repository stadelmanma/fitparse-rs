//! Provides an alternative Key, Value mapping of a FitDataRecord for serializating the data into
//another format ! that may be more useful than a simple vector of values in each message.
use crate::Value;
use serde::ser;
use serde::Serialize;
use std::collections::BTreeMap;

/// Generic struct that allows FIT data to be handled via a key-value mapping and serialized as such
#[derive(Clone, Debug, Serialize)]
pub struct FitDataRecordSerializer<N: ser::Serialize, K: Ord + ser::Serialize, V: ser::Serialize> {
    kind: N,
    fields: BTreeMap<K, V>,
}

impl<N: ser::Serialize, K: Ord + ser::Serialize, V: ser::Serialize>
    FitDataRecordSerializer<N, K, V>
{
    /// Create a new data record serializer
    pub fn new(kind: N) -> Self {
        FitDataRecordSerializer {
            kind,
            fields: BTreeMap::new(),
        }
    }

    /// Return the kind of data record read
    pub fn kind(&self) -> &N {
        &self.kind
    }

    /// Access a field value using the provided key
    pub fn get(&self, key: &K) -> Option<&V> {
        self.fields.get(key)
    }

    /// Access a field value using the provided key
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.fields.insert(key, value)
    }
}

/// Describes a field value along with it's defined units (if any)
#[derive(Clone, Debug, Serialize)]
pub struct ValueWithUnits {
    value: Value,
    units: String,
}

impl ValueWithUnits {
    /// Create a new data field with the given information
    pub fn new(value: Value, units: String) -> Self {
        ValueWithUnits { value, units }
    }
}

/// Describes how the fields in the mapping should be stored and accessed
pub enum RecordFormat {
    /// Access fields by their `name`, this is preferable if the consumer is not aware of the defined
    /// FIT profile and therefore cannot decode the name from the message number + definition number
    /// combination. Values are provided without units.
    StringKeysValueOnly,
    /// Same as the `StringKeysValueOnly` format, except each value is stored with the units defined
    /// by the FIT profile (if any). This is the most verbose format and is ideal when the consumer
    /// has no knowledge of the FIT profile in use.
    StringKeysValueWithUnits,
}
