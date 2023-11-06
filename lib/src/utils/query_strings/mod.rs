mod index_definition;
mod misc;
mod provisionings;
mod table_definitions;
mod table_field_definitions;
mod user_queries;

pub use index_definition::*;
pub use misc::*;
pub use provisionings::*;
pub use table_definitions::*;
pub use table_field_definitions::*;
pub use user_queries::*;

mod macros {
    macro_rules! doc_and_define {
        ($name:ident, $value:expr) => {
            #[doc = "### Query:\n```sql\n"]
            #[doc = $value]
            #[doc = "\n```"]
            pub const $name: &str = $value;
        };
    }

    macro_rules! doc_and_define_with_bindings {
        ($name:ident, $bindings:expr, $value:expr) => {
            #[doc = "### Required Bindings:\n```sql\n"]
            #[doc = $bindings]
            #[doc = "\n```"]
            #[doc = "### Query:\n```sql\n"]
            #[doc = $value]
            #[doc = "\n```"]
            pub const $name: &str = $value;
        };
    }

    pub(crate) use doc_and_define;
    pub(crate) use doc_and_define_with_bindings;
}
