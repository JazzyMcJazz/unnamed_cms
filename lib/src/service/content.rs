use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Number,
    Datetime,
    Boolean,
    Object,
    List,
    Relation,
    MultipleRelation,
}

impl FieldType {
    pub fn as_list() -> Vec<FieldType> {
        vec![
            FieldType::Text,
            FieldType::Number,
            FieldType::Datetime,
            FieldType::Boolean,
            FieldType::Object,
            FieldType::List,
            FieldType::Relation,
        ]
    }

    fn from_string(s: &str) -> Self {
        match s {
            "Text" => FieldType::Text,
            "Number" => FieldType::Number,
            "Datetime" => FieldType::Datetime,
            "Boolean" => FieldType::Boolean,
            "Object" => FieldType::Object,
            "List" => FieldType::List,
            "Relation" => FieldType::Relation,
            _ => FieldType::Text,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Table {
    name: String,
    display_name: String,
    description: String,
    pub fields: Vec<TableField>,
    errors: HashMap<&'static str, bool>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            display_name: String::new(),
            description: String::new(),
            fields: vec![TableField::new(0)],
            errors: HashMap::new(),
        }
    }

    pub fn from_form(form: &HashMap<String, String>) -> Self {
        let name = form
            .get("name")
            .unwrap_or(&String::new())
            .to_string()
            .to_lowercase()
            .replace(' ', "_");

        let display_name = form
            .get("display_name")
            .unwrap_or(&String::new())
            .to_string();

        let mut errors = HashMap::<&str, bool>::new();

        if display_name.is_empty() {
            errors.insert("display_name", true);
        }

        let fields = TableField::from_form(form);

        Self {
            name,
            display_name,
            description: form
                .get("description")
                .unwrap_or(&String::new())
                .to_string(),
            fields,
            errors,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TableField {
    id: u32,
    name: String,
    description: String,
    field_type: FieldType,
    required: bool,
    // unique: bool,
    default_value: Option<String>,
    errors: HashMap<&'static str, bool>,
}

impl TableField {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: String::new(),
            description: String::new(),
            field_type: FieldType::Text,
            required: true,
            // unique: false,
            default_value: None,
            errors: HashMap::new(),
        }
    }

    pub fn from_form(form: &HashMap<String, String>) -> Vec<Self> {
        let mut ids = Vec::<u32>::new();
        for k in form.keys() {
            let mut chunks = k.split('-');
            let id = chunks.next().unwrap_or("").parse::<u32>();
            if k.eq("display_name") || k.eq("description") || id.is_err() {
                continue;
            }

            let id = id.unwrap_or(0);
            if !ids.contains(&id) {
                ids.push(id);
            }
        }

        let mut fields = Vec::<Self>::new();
        for id in ids {
            let mut field = Self::new(0);
            field.id = id;
            field.required = false;

            let f = form
                .iter()
                .filter(|(k, _)| k.starts_with(&format!("{}-", id)))
                .map(|(k, v)| (k.split('-').last().unwrap_or("").to_string(), v.to_string()))
                .collect::<HashMap<_, _>>();

            for (k, v) in f {
                match k.as_str() {
                    "name" => field.name = v,
                    "description" => field.description = v,
                    "type" => field.field_type = FieldType::from_string(v.as_str()),
                    "default_value" => {
                        field.default_value = if v.is_empty() { None } else { Some(v) }
                    }
                    "required" => field.required = v.eq("on"),
                    _ => continue,
                }
            }

            let mut errors = HashMap::<&str, bool>::new();
            if field.name.is_empty() {
                errors.insert("name", true);
            }

            field.errors = errors;

            fields.push(field);
        }
        fields.sort_by(|a, b| a.id.cmp(&b.id));
        fields
    }
}

// pub struct Content;
