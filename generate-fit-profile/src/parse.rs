//! Code used to parse the Profile.xlsx file into useful data structures
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

// the fields in these structs are mostly duplicated from code in src/profile/parser.rs
#[derive(Clone, Debug)]
pub struct FitProfile {
    version: String,
    field_types: Vec<FieldTypeDefintion>,
    messages: Vec<MessageDefinition>,
}

impl FitProfile {
    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn field_types(&self) -> &[FieldTypeDefintion] {
        &self.field_types
    }

    pub fn messages(&self) -> &[MessageDefinition] {
        &self.messages
    }
}

#[derive(Clone, Debug)]
pub struct FieldTypeDefintion {
    name: String,
    titlized_name: String,
    base_type: &'static str,
    is_true_enum: bool,
    comment: Option<String>,
    variant_map: BTreeMap<i64, FieldTypeVariant>,
}

impl FieldTypeDefintion {
    fn new(name: String, base_type: &'static str, comment: Option<String>) -> Self {
        let is_true_enum = base_type == "enum";
        let base_type = if is_true_enum { "u8" } else { base_type };

        FieldTypeDefintion {
            name: name.clone(),
            titlized_name: titlecase_string(&name),
            base_type,
            is_true_enum,
            comment,
            variant_map: BTreeMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn titlized_name(&self) -> &str {
        &self.titlized_name
    }

    pub fn base_type(&self) -> &'static str {
        &self.base_type
    }

    pub fn is_true_enum(&self) -> bool {
        self.is_true_enum
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    pub fn variant_map(&self) -> &BTreeMap<i64, FieldTypeVariant> {
        &self.variant_map
    }

    pub fn other_value_field_name(&self) -> &'static str {
        if self.is_true_enum() {
            "UnknownVariant"
        } else {
            "Value"
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldTypeVariant {
    name: String,
    titlized_name: String,
    value: i64,
    comment: Option<String>,
}

impl FieldTypeVariant {
    fn new(name: String, value: i64, comment: Option<String>) -> Self {
        // First letter isn't between A-Z in ASCII
        let mut titlized_name = titlecase_string(&name);
        let first_let = titlized_name.as_bytes()[0];
        if !first_let.is_ascii_alphabetic() {
            titlized_name = format!("Name{}", titlized_name);
        }

        FieldTypeVariant {
            name: name.clone(),
            titlized_name: titlized_name,
            value,
            comment,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn titlized_name(&self) -> &str {
        &self.titlized_name
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }
}

#[derive(Clone, Debug)]
pub struct MessageDefinition {
    name: String,
    titlized_name: String,
    comment: Option<String>,
    field_map: BTreeMap<u8, MessageFieldDefinition>,
}

impl MessageDefinition {
    fn new(name: &str, comment: Option<String>) -> Self {
        MessageDefinition {
            name: name.to_string(),
            titlized_name: titlecase_string(&name),
            comment,
            field_map: BTreeMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn titlized_name(&self) -> &str {
        &self.titlized_name
    }

    pub fn field_map(&self) -> &BTreeMap<u8, MessageFieldDefinition> {
        &self.field_map
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    pub fn get_field_by_name(&self, name: &str) -> &MessageFieldDefinition {
        self.field_map()
            .values()
            .find(|f| f.name() == name)
            .unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct MessageFieldDefinition {
    def_number: u8,
    name: String,
    field_type: String,
    is_array: bool,
    scale: f64,
    offset: f64,
    units: String,
    accumulate: bool,
    parent_field: Option<Box<MessageFieldDefinition>>,
    subfields: Vec<(String, String, MessageFieldDefinition)>,
    components: Vec<(u8, MessageFieldDefinition)>,
    raw_components: Vec<MessageFieldComponent>,
    comment: Option<String>,
}

impl MessageFieldDefinition {
    fn new(
        def_number: u8,
        name: &str,
        field_type: &str,
        is_array: bool,
        scale: f64,
        offset: f64,
        units: &str,
        accumulate: bool,
        raw_components: Vec<MessageFieldComponent>,
        comment: Option<String>,
    ) -> Self {
        MessageFieldDefinition {
            def_number,
            name: name.to_string(),
            field_type: field_type_str_to_field_type(field_type),
            is_array,
            scale,
            offset,
            units: units.to_string(),
            accumulate,
            parent_field: None,
            subfields: Vec::new(),
            components: Vec::new(),
            raw_components,
            comment,
        }
    }

    pub fn def_number(&self) -> u8 {
        self.def_number
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn field_type(&self) -> &str {
        &self.field_type
    }

    pub fn is_array(&self) -> bool {
        self.is_array
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn offset(&self) -> f64 {
        self.offset
    }

    pub fn units(&self) -> &str {
        &self.units
    }

    pub fn accumulate(&self) -> bool {
        self.accumulate
    }

    pub fn parent_field(&self) -> &Option<Box<MessageFieldDefinition>> {
        &self.parent_field
    }

    pub fn set_parent_field(&mut self, field: MessageFieldDefinition) {
        self.parent_field = Some(Box::new(field));
    }

    pub fn subfields(&self) -> &[(String, String, MessageFieldDefinition)] {
        &self.subfields
    }

    pub fn subfields_mut(&mut self) -> &mut [(String, String, MessageFieldDefinition)] {
        &mut self.subfields
    }

    pub fn raw_components(&self) -> &[MessageFieldComponent] {
        &self.raw_components
    }

    pub fn components(&self) -> &[(u8, MessageFieldDefinition)] {
        &self.components
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }
}

#[derive(Clone, Debug)]
pub struct MessageFieldComponent {
    name: String,
    scale: f64,
    offset: f64,
    units: String,
    bits: u8,
    accumulate: bool,
}

impl MessageFieldComponent {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn offset(&self) -> f64 {
        self.offset
    }

    pub fn units(&self) -> &str {
        &self.units
    }

    pub fn accumulate(&self) -> bool {
        self.accumulate
    }

    pub fn bits(&self) -> u8 {
        self.bits
    }
}

macro_rules! split_csv_string ( ($value:expr) => ( {$value.split(',').map(|v| v.trim().to_string())} ););

/// Match a base type string to a rust type for enum generation
fn base_type_to_rust_type(base_type_str: &str) -> &'static str {
    match base_type_str {
        "enum" => "enum", // "pseduo-type" we use to detect real enums, changed to u8 later on
        "sint8" => "i8",
        "uint8" => "u8",
        "uint8z" => "u8",
        "sint16" => "i16",
        "uint16" => "u16",
        "uint16z" => "u16",
        "sint32" => "i32",
        "uint32" => "u32",
        "uint32z" => "u32",
        _ => panic!("unsupported base_type for enum field: {}", base_type_str),
    }
}

/// match the field type string to a simple type or an enum
fn field_type_str_to_field_type(field_type_str: &str) -> String {
    match field_type_str {
        "sint8" => "SInt8".to_string(),
        "uint8" => "UInt8".to_string(),
        "sint16" => "SInt16".to_string(),
        "uint16" => "UInt16".to_string(),
        "sint32" => "SInt32".to_string(),
        "uint32" => "UInt32".to_string(),
        "string" => "String".to_string(),
        "float32" => "Float32".to_string(),
        "float64" => "Float64".to_string(),
        "uint8z" => "UInt8z".to_string(),
        "uint16z" => "UInt16z".to_string(),
        "uint32z" => "UInt32z".to_string(),
        "byte" => "Byte".to_string(),
        "sint64" => "SInt64".to_string(),
        "uint64" => "UInt64".to_string(),
        "uint64z" => "UInt64z".to_string(),
        _ => titlecase_string(field_type_str),
    }
}

fn titlecase_string(value: &str) -> String {
    let mut words: Vec<String> = value.split('_').map(|v| v.to_string()).collect();

    for word in &mut words {
        if let Some(l) = word.get_mut(0..1) {
            l.make_ascii_uppercase();
        }
    }

    words.join("")
}

fn process_types(sheet: Range<DataType>) -> Vec<FieldTypeDefintion> {
    let mut field_types: Vec<FieldTypeDefintion> = Vec::new();

    for row in sheet.rows().skip(1) {
        if !row[0].is_empty() {
            // extract enum name
            let enum_name = match row[0].get_string() {
                Some(v) => v.to_string(),
                None => panic!("Enum type name must be a string row={:?}.", row),
            };

            // extract base type and convert to its rust equivalent
            let rust_type = match row[1].get_string() {
                Some(v) => base_type_to_rust_type(v),
                None => panic!("Base type name must be a string row={:?}.", row),
            };
            let comment = row[4].get_string().map(|v| v.to_string());
            field_types.push(FieldTypeDefintion::new(enum_name, rust_type, comment));
        } else if !row[2].is_empty() {
            let field_type = match field_types.last_mut() {
                Some(v) => v,
                None => panic!("field_types vector was empty!"),
            };
            // add enum variant
            // extract enum name
            let name = match row[2].get_string() {
                Some(v) => v.to_string(),
                None => panic!("Enum variant name must be a string row={:?}.", row),
            };

            // handle mix of numeric and hex string data values
            let value = match &row[3] {
                DataType::Float(v) => *v as i64,
                DataType::Int(v) => *v,
                DataType::String(v) => i64::from_str_radix(&v[2..], 16).unwrap(),
                _ => {
                    panic!("Unsupported enum variant value data type row={:?}.", row);
                }
            };
            let comment = row[4].get_string().map(|v| v.to_string());
            field_type
                .variant_map
                .insert(value, FieldTypeVariant::new(name, value, comment));
        }
    }

    field_types
}

fn parse_message_field_components(row: &[DataType]) -> Vec<MessageFieldComponent> {
    let mut components = Vec::new();

    // parse out the fields into iterators
    let names: Vec<String> = match row[5].get_string() {
        Some(v) => split_csv_string!(v).collect(),
        None => {
            return components;
        }
    };
    let cols: Vec<String> = row[6..=10].iter().map(|v| v.to_string()).collect();
    let mut scales = split_csv_string!(cols[0]).map(|s| s.parse::<f64>().ok());
    let mut offsets = split_csv_string!(cols[1]).map(|s| s.parse::<f64>().ok());
    let mut units = split_csv_string!(cols[2]);
    let mut bits = split_csv_string!(cols[3]).map(|s| s.parse::<u8>().ok());
    let mut accumulate = split_csv_string!(cols[4]).map(|s| s == "1");

    // build each component
    for name in names {
        components.push(MessageFieldComponent {
            name,
            scale: scales.next().flatten().unwrap_or(1.0),
            offset: offsets.next().flatten().unwrap_or(0.0),
            units: units.next().unwrap_or_default(),
            bits: bits
                .next()
                .flatten()
                .unwrap_or_else(|| panic!("Could not parse bits value for row: {:?}", row)),
            accumulate: accumulate.next().unwrap_or(false),
        });
    }
    components
}

fn post_process_message(msg: MessageDefinition) -> MessageDefinition {
    // destructure and rebuild to fully resolve components, we do
    // this to appease the mighty borrow checker and deal with
    // recursive component expansion. A field with components gets it's own
    // chain of field def copies since the values for certain members change
    // depending on how we get to the field
    let MessageDefinition {
        name,
        titlized_name,
        comment,
        field_map,
    } = msg;
    // we need this lookup to map components back to original field info without
    // trying to do an immutable borrow against the mapping we're updating
    let name_to_field: HashMap<String, MessageFieldDefinition> = field_map
        .values()
        .map(|v| (v.name().to_owned(), v.clone()))
        .collect();

    // this all seems horrendously over complicated and inefficient but it's "run once"
    // code, not "application code" that's regularly executed so we'll clean it up later.
    let mut updated_field_map = BTreeMap::new();
    for (def_num, mut field_def) in field_map.into_iter() {
        if !field_def.raw_components().is_empty() {
            field_def.components = process_components(&field_def, &name_to_field);
        }
        let extra_fld_def = field_def.clone();
        for (_, _, sub_fld) in field_def.subfields_mut() {
            sub_fld.set_parent_field(extra_fld_def.clone());
        }
        updated_field_map.insert(def_num, field_def);
    }

    MessageDefinition {
        name,
        titlized_name,
        comment,
        field_map: updated_field_map,
    }
}

fn process_components(
    field: &MessageFieldDefinition,
    field_lookup: &HashMap<String, MessageFieldDefinition>,
) -> Vec<(u8, MessageFieldDefinition)> {
    let mut components = Vec::new();
    for comp_info in field.raw_components() {
        let dest_field = field_lookup.get(comp_info.name()).unwrap();
        let comp_fld = MessageFieldDefinition {
            def_number: dest_field.def_number(),
            name: dest_field.name().to_owned(),
            field_type: dest_field.field_type().to_owned(),
            is_array: dest_field.is_array(),
            scale: comp_info.scale(),
            offset: comp_info.offset(),
            units: comp_info.units().to_owned(),
            accumulate: comp_info.accumulate(),
            parent_field: dest_field.parent_field().clone(),
            subfields: dest_field.subfields().to_vec(),
            components: process_components(dest_field, field_lookup),
            raw_components: Vec::new(),
            comment: dest_field.comment().map(|s| s.to_owned()),
        };
        components.push((comp_info.bits(), comp_fld))
    }
    components
}

fn new_message_field_definition(row: &[DataType]) -> MessageFieldDefinition {
    let def_number = match row[1] {
        DataType::Float(v) => v as u8,
        DataType::Int(v) => v as u8,
        _ => panic!("Field defintiton number must be an integer, row={:?}.", row),
    };
    let name = row[2]
        .get_string()
        .unwrap_or_else(|| panic!("Field name must be a string, row={:?}.", row));
    let ftype = row[3]
        .get_string()
        .unwrap_or_else(|| panic!("Field type must be a string, row={:?}.", row));
    let components = parse_message_field_components(row);
    let comment = row[13].get_string().map(|v| v.to_string());

    MessageFieldDefinition::new(
        def_number,
        name,
        ftype,
        row[4].is_empty(),
        row[6].get_float().unwrap_or(1.0),
        row[7].get_float().unwrap_or(0.0),
        row[8].get_string().unwrap_or(""),
        row[10].to_string() == "1",
        components,
        comment,
    )
}

fn process_messages(sheet: Range<DataType>) -> Vec<MessageDefinition> {
    let mut rows = sheet.rows().skip(1);
    let mut messages: Vec<MessageDefinition> = Vec::new();
    let mut msg: MessageDefinition;
    let mut field: MessageFieldDefinition;
    let mut last_def_number: u8 = 0;

    // parse first message row to initialize first message to prevent unitialized compile error in loop
    let row = rows.next().unwrap();
    if let Some(v) = row[0].get_string() {
        msg = MessageDefinition::new(v, row[13].get_string().map(|v| v.to_string()));
    } else {
        panic!("Message name must be a string row={:?}.", row);
    }

    // process messages and fields
    for row in rows {
        // begin new message function
        if !row[0].is_empty() {
            if let Some(v) = row[0].get_string() {
                messages.push(msg);
                msg = MessageDefinition::new(v, row[13].get_string().map(|v| v.to_string()));
            } else {
                panic!("Message name must be a string row={:?}.", row);
            }
        } else if !row[1].is_empty() {
            field = new_message_field_definition(row);
            last_def_number = field.def_number;
            msg.field_map.insert(field.def_number, field);
        } else if !row[2].is_empty() {
            // process sub field
            let parent = msg
                .field_map
                .get_mut(&last_def_number)
                .expect("No parent field defined for subfield!");
            let mut temp_row: Vec<DataType> = Vec::from(row);
            temp_row[1] = DataType::Int(last_def_number as i64);
            field = new_message_field_definition(&temp_row);
            // store subfield ref_field, ref_field_value and defintion, if multiple values can
            // trigger this subfield we simply duplicate them
            let ref_field_names = row[11].get_string().expect("No reference field name(s)");
            let ref_field_vals = row[12].get_string().expect("No reference field value(s)");
            for (name, value) in
                split_csv_string!(ref_field_names).zip(split_csv_string!(ref_field_vals))
            {
                parent
                    .subfields
                    .push((name, titlecase_string(&value), field.clone()));
            }
        }
    }
    // push last message since the iter completes
    messages.push(msg);
    // post process messages once we have "all" the
    // information present
    messages
        .into_iter()
        .map(|m| post_process_message(m))
        .collect()
}

pub fn parse_profile(
    profile_fname: &PathBuf,
    version: String,
) -> Result<FitProfile, Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = open_workbook(&profile_fname)?;

    // process Types sheet
    let field_types = if let Some(Ok(sheet)) = excel.worksheet_range("Types") {
        process_types(sheet)
    } else {
        panic!("Could not access workbook sheet 'Types'");
    };

    // process Messages sheet
    let messages = if let Some(Ok(sheet)) = excel.worksheet_range("Messages") {
        process_messages(sheet)
    } else {
        panic!("Could not access workbook sheet 'Messages'");
    };

    Ok(FitProfile {
        version,
        field_types,
        messages,
    })
}
