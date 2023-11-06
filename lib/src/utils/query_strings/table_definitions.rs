use super::macros::doc_and_define;

doc_and_define!(
    DEFINE_TABLES,
    r#"
DEFINE TABLE system_user SCHEMAFULL;
DEFINE TABLE system_session SCHEMAFULL;
DEFINE TABLE system_session_token SCHEMAFULL;
DEFINE TABLE system_role SCHEMAFULL;
DEFINE TABLE system_permission SCHEMAFULL;
DEFINE TABLE system_resource SCHEMAFULL;
"#
);
