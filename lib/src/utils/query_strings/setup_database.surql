-- BEGIN TRANSACTION;

#################
# DEFINE TABLES #
#################
DEFINE TABLE system_user SCHEMAFULL;
DEFINE TABLE system_session SCHEMAFULL;
DEFINE TABLE system_session_token SCHEMAFULL;
DEFINE TABLE system_role SCHEMAFULL;
DEFINE TABLE system_permission SCHEMAFULL;
DEFINE TABLE system_resource SCHEMAFULL;

#################
# DEFINE FIELDS #
#################
-- system_user
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
-- DEFINE FIELD sessions.* ON TABLE system_user TYPE record<system_session>;

-- system_session
DEFINE FIELD user ON TABLE system_session TYPE record<system_user>;
-- DEFINE FIELD tokens.* ON TABLE system_session TYPE record<system_session_token>;

-- system_sesssion_token
DEFINE FIELD session ON TABLE system_session_token TYPE record<system_session>;
DEFINE FIELD token ON TABLE system_session_token TYPE string
    DEFAULT rand::ulid();
DEFINE FIELD created_at ON TABLE system_session_token TYPE datetime
    DEFAULT time::now();
DEFINE FIELD expires_at ON TABLE system_session_token TYPE datetime;

-- system_permission
DEFINE FIELD name ON TABLE system_permission TYPE string;
DEFINE FIELD description ON TABLE system_permission TYPE string;

-- system_resource
DEFINE FIELD name ON TABLE system_resource TYPE string;
DEFINE FIELD type ON TABLE system_resource TYPE string
    ASSERT ['system_table', 'system_field', 'table', 'field'] CONTAINS $value;
DEFINE FIELD description ON TABLE system_resource TYPE string;
DEFINE FIELD created_at ON TABLE system_resource TYPE datetime
    DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE system_resource TYPE datetime
    DEFAULT time::now();

##################
# DEFINE INDEXES #
##################
-- system_user
DEFINE INDEX system_userEmailIndex ON TABLE system_user COLUMNS email UNIQUE;

-- system_session
DEFINE INDEX system_sessionTokenIndex ON TABLE system_session COLUMNS token UNIQUE;

-- system_resource
DEFINE INDEX system_resourceNameIndex ON TABLE system_resource COLUMNS name UNIQUE;

###########################
# CREATE SYSTEM RESOURCES #
###########################
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

#################
# DEFINE EVENTS #
#################
DEFINE EVENT session_deleted ON TABLE system_session WHEN $event = "DELETE" THEN (
    DELETE FROM system_session_token WHERE session = $before.id
); 

DEFINE EVENT user_deleted ON TABLE system_user WHEN $event = "DELETE" THEN (
    DELETE FROM system_session WHERE user = $before.id
);

########################
# CREATE DEFAULT ADMIN #
########################
LET $password = $password || rand::string();
LET $email = $email || string::lowercase(string::concat('admin', '@', rand::string(10), '.com'));

IF !(SELECT * FROM system_user:admin) {
    RETURN CREATE system_user:admin CONTENT {
        email: $email,
        password: $password,
        name: "Admin",
        admin: true,
    };
};

RETURN $password;

-- COMMIT TRANSACTION;