use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub enum DataType {
    Boolean,
    DateTime,
    Decimal,
    Float,
    Int,
    Number,
    String,
    Unknown,
}

impl DataType {
    pub fn from_string(data_type: String) -> Self {
        match data_type.as_str() {
            "bool" => Self::Boolean,
            "datetime" => Self::DateTime,
            "decimal" => Self::Decimal,
            "float" => Self::Float,
            "int" => Self::Int,
            "number" => Self::Number,
            "string" => Self::String,
            _ => {
                dbg!(data_type);
                Self::Unknown
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TableInfo {
    #[allow(dead_code)]
    events: HashMap<String, String>,
    #[allow(dead_code)]
    indexes: HashMap<String, String>,
    fields: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct Field {
    name: String,
    data_type: DataType,
    default: Option<String>,
    nullable: bool,
    unique: bool,
}

impl Field {
    pub fn from_info(info: &TableInfo) -> Vec<Self> {
        let mut fields = Vec::new();
        for (key, value) in info.fields.iter() {
            let data_type = value
                .split("TYPE")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap();

            let nullable = data_type.contains("option<");
            let data_type = data_type.replace("option<", "").replace('>', "");

            let field = Self {
                name: key.clone(),
                data_type: DataType::from_string(data_type),
                default: None,
                nullable,
                unique: false,
            };
            fields.push(field);
        }
        fields
    }
}
