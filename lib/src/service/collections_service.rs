use serde_json::Value;

use crate::{
    api::html::CollectionForm,
    config::database::{Field, SystemResource},
    prelude::*,
};

pub struct CollectionService;

impl CollectionService {
    pub async fn create(
        db: &Surreal<Any>,
        form: &CollectionForm,
    ) -> Result<CmsResponse, CmsResponse> {
        let collection = SystemResource::from_form(form);
        let resource = db.create_resource(&collection).await?;
        let target = format!("/collections/{}", resource.id().unwrap());
        Ok(CmsResponse::SeeOther(target))
    }

    pub async fn find_all(
        db: &Surreal<Any>,
        collection: String,
    ) -> Result<(Vec<Field>, Vec<Value>), CmsResponse> {
        dbg!(&collection);

        let data = db.collections().find_all(collection).await?;
        Ok(data)
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub enum FieldType {
//     Text,
//     Number,
//     Datetime,
//     Boolean,
//     Object,
//     List,
//     Relation,
//     MultipleRelation,
// }

// impl FieldType {
//     pub fn as_list() -> Vec<FieldType> {
//         vec![
//             FieldType::Text,
//             FieldType::Number,
//             FieldType::Datetime,
//             FieldType::Boolean,
//             FieldType::Object,
//             FieldType::List,
//             FieldType::Relation,
//         ]
//     }

//     fn from_string(s: &str) -> Self {
//         match s {
//             "Text" => FieldType::Text,
//             "Number" => FieldType::Number,
//             "Datetime" => FieldType::Datetime,
//             "Boolean" => FieldType::Boolean,
//             "Object" => FieldType::Object,
//             "List" => FieldType::List,
//             "Relation" => FieldType::Relation,
//             _ => FieldType::Text,
//         }
//     }
// }

// #[derive(Debug, Serialize)]
// pub struct TableField {
//     id: u32,
//     name: String,
//     description: String,
//     field_type: FieldType,
//     required: bool,
//     // unique: bool,
//     default_value: Option<String>,
//     errors: HashMap<&'static str, bool>,
// }

// impl TableField {
//     pub fn new(id: u32) -> Self {
//         Self {
//             id,
//             name: String::new(),
//             description: String::new(),
//             field_type: FieldType::Text,
//             required: true,
//             // unique: false,
//             default_value: None,
//             errors: HashMap::new(),
//         }
//     }

//     pub fn from_form(form: &HashMap<String, String>) -> Vec<Self> {
//         let mut ids = Vec::<u32>::new();
//         for k in form.keys() {
//             let mut chunks = k.split('-');
//             let id = chunks.next().unwrap_or("").parse::<u32>();
//             if k.eq("display_name") || k.eq("description") || id.is_err() {
//                 continue;
//             }

//             let id = id.unwrap_or(0);
//             if !ids.contains(&id) {
//                 ids.push(id);
//             }
//         }

//         let mut fields = Vec::<Self>::new();
//         for id in ids {
//             let mut field = Self::new(0);
//             field.id = id;
//             field.required = false;

//             let f = form
//                 .iter()
//                 .filter(|(k, _)| k.starts_with(&format!("{}-", id)))
//                 .map(|(k, v)| (k.split('-').last().unwrap_or("").to_string(), v.to_string()))
//                 .collect::<HashMap<_, _>>();

//             for (k, v) in f {
//                 match k.as_str() {
//                     "name" => field.name = v,
//                     "description" => field.description = v,
//                     "type" => field.field_type = FieldType::from_string(v.as_str()),
//                     "default_value" => {
//                         field.default_value = if v.is_empty() { None } else { Some(v) }
//                     }
//                     "required" => field.required = v.eq("on"),
//                     _ => continue,
//                 }
//             }

//             let mut errors = HashMap::<&str, bool>::new();
//             if field.name.is_empty() {
//                 errors.insert("name", true);
//             }

//             field.errors = errors;

//             fields.push(field);
//         }
//         fields.sort_by(|a, b| a.id.cmp(&b.id));
//         fields
//     }
// }
