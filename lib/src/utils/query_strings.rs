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

doc_and_define!(
    DEFINE_TABLES,
    r#"
DEFINE TABLE system_user SCHEMAFULL;
DEFINE TABLE system_session SCHEMAFULL;
DEFINE TABLE system_session_token SCHEMAFULL;
"#
);

doc_and_define!(
    DEFINE_FIELDS,
    r#"
-- system_user
DEFINE FIELD email ON TABLE system_user TYPE string
    ASSERT string::is::email($value);
DEFINE FIELD password ON TABLE system_user TYPE string
    VALUE crypto::argon2::generate(string::trim($value));
DEFINE FIELD name ON TABLE system_user TYPE option<string>
    VALUE string::trim($value);
DEFINE FIELD created_at ON TABLE system_user TYPE datetime
    DEFAULT time::now();
DEFINE FIELD sessions.* ON TABLE system_user TYPE record<system_session>;

-- system_session
DEFINE FIELD tokens.* ON TABLE system_session TYPE record<system_session_token>;

-- system_session_token
DEFINE FIELD token ON TABLE system_session_token TYPE string
    DEFAULT rand::ulid();
DEFINE FIELD iterations ON TABLE system_session_token TYPE int;
DEFINE FIELD created_at ON TABLE system_session_token TYPE datetime
    DEFAULT time::now();
DEFINE FIELD expires_at ON TABLE system_session_token TYPE datetime;
"#
);

doc_and_define!(
    DEFINE_INDEXES,
    r#"
-- system_user
DEFINE INDEX system_userEmailIndex ON TABLE system_user COLUMNS email UNIQUE;

-- system_session
DEFINE INDEX system_sessionTokenIndex ON TABLE system_session COLUMNS token UNIQUE;
"#
);

doc_and_define_with_bindings!(
    CREATE_ADMIN,
    r#"
email: String,
password: String,
    "#,
    r#"
LET $password = $password || rand::string();
LET $email = $email || string::lowercase(string::concat('admin', '@', rand::string(10), '.com'));

CREATE system_user:admin CONTENT {
    email: $email,
    password: $password,
    name: "Admin",
};

RETURN $password;
"#
);

doc_and_define_with_bindings!(
    FIND_USER_BY_CREDS,
    r#"
email: String,
password: String,
    "#,
    r#"
SELECT * FROM system_user WHERE 
    email = $email AND 
    crypto::argon2::compare(password, $password);
"#
);

doc_and_define!(
    DEV_CLEAR,
    r#"
REMOVE DATABASE cms;
"#
);
