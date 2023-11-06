use super::macros::doc_and_define;

// SYSTEM_USER
doc_and_define!(
    DEFINE_USER_FIELDS,
    r#"
DEFINE FIELD email ON TABLE system_user TYPE string
    ASSERT string::is::email($value);
DEFINE FIELD password ON TABLE system_user TYPE string
    ASSERT string::len(string::trim($input)) >= 8
    VALUE crypto::argon2::generate(string::trim($value));
DEFINE FIELD name ON TABLE system_user TYPE option<string>
    VALUE string::trim($value);
DEFINE FIELD admin ON TABLE system_user TYPE bool
    DEFAULT false;
DEFINE FIELD created_at ON TABLE system_user TYPE datetime
    DEFAULT time::now();
DEFINE FIELD sessions.* ON TABLE system_user TYPE record<system_session>;
"#
);

// SYSTEM_SESSION
doc_and_define!(
    DEFINE_SESSION_FIELDS,
    r#"
DEFINE FIELD tokens.* ON TABLE system_session TYPE record<system_session_token>;
"#
);

// SYSTEM_SESSION_TOKEN
doc_and_define!(
    DEFINE_SESSION_TOKEN_FIELDS,
    r#"
DEFINE FIELD token ON TABLE system_session_token TYPE string
    DEFAULT rand::ulid();
DEFINE FIELD iterations ON TABLE system_session_token TYPE int;
DEFINE FIELD created_at ON TABLE system_session_token TYPE datetime
    DEFAULT time::now();
DEFINE FIELD expires_at ON TABLE system_session_token TYPE datetime;
"#
);

// SYSTEM_PERMISSION
doc_and_define!(
    DEFINE_PERMISSION_FIELDS,
    r#"
DEFINE FIELD name ON TABLE system_permission TYPE string;
DEFINE FIELD description ON TABLE system_permission TYPE string;
"#
);

// SYSTEM_RESOURCE
doc_and_define!(
    DEFINE_RESOURCE_FIELDS,
    r#"
DEFINE FIELD name ON TABLE system_resource TYPE string;
DEFINE FIELD type ON TABLE system_resource TYPE string
    ASSERT ['system_table', 'system_field', 'table', 'field'] CONTAINS $value;
DEFINE FIELD description ON TABLE system_resource TYPE string;
DEFINE FIELD created_at ON TABLE system_resource TYPE datetime
    DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE system_resource TYPE datetime
    DEFAULT time::now();
"#
);
