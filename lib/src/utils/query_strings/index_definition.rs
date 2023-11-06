use super::macros::doc_and_define;

doc_and_define!(
    DEFINE_INDEXES,
    r#"
-- system_user
DEFINE INDEX system_userEmailIndex ON TABLE system_user COLUMNS email UNIQUE;

-- system_session
DEFINE INDEX system_sessionTokenIndex ON TABLE system_session COLUMNS token UNIQUE;

-- system_resource
DEFINE INDEX system_resourceNameIndex ON TABLE system_resource COLUMNS name UNIQUE;
"#
);
