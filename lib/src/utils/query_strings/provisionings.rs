use super::macros::{doc_and_define, doc_and_define_with_bindings};

doc_and_define_with_bindings!(
    CREATE_ADMIN,
    r#"
email: String,
password: String,
    "#,
    r#"
LET $password = $password || rand::string();
LET $email = $email || string::lowercase(string::concat('admin', '@', rand::string(10), '.com'));

IF !(SELECT * FROM system_user:admin) {
    CREATE system_user:admin CONTENT {
        email: $email,
        password: $password,
        name: "Admin",
        admin: true,
    };
};

RETURN $password;
"#
);

doc_and_define!(
    CREATE_SYSTEM_RESOURCES,
    r#"
IF !(SELECT * FROM system_resource:system_user) {
    CREATE system_resource:system_user CONTENT {
        name: "system_user",
        type: "system_table",
        description: "Contains system users",
    };
};
IF !(SELECT * FROM system_resource:system_session) {
    CREATE system_resource:system_session CONTENT {
        name: "system_session",
        type: "system_table",
        description: "Contains user sessions",
    };
};
IF !(SELECT * FROM system_resource:system_session_token) {
    CREATE system_resource:system_session_token CONTENT {
        name: "system_session_token",
        type: "system_table",
        description: "Contains user session tokens",
    };
};
IF !(SELECT * FROM system_resource:system_role) {
    CREATE system_resource:system_role CONTENT {
        name: "system_role",
        type: "system_table",
        description: "Contains user roles",
    };
};
IF !(SELECT * FROM system_resource:system_permission) {
    CREATE system_resource:system_permission CONTENT {
        name: "system_permission",
        type: "system_table",
        description: "Contains permission definitions",
    };
};
IF !(SELECT * FROM system_resource:system_resource) {
    CREATE system_resource:system_resource CONTENT {
        name: "system_resource",
        type: "system_table",
        description: "Contains resource definitions",
    };
};
"#
);
