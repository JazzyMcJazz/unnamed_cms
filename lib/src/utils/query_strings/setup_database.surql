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
DEFINE FIELD resource_type ON TABLE system_resource TYPE string
    ASSERT ['SystemTable', 'SystemField', 'Table', 'Field'] CONTAINS $value;
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

#################
# DEFINE EVENTS #
#################
DEFINE EVENT session_deleted ON TABLE system_session WHEN $event = "DELETE" THEN (
    DELETE FROM system_session_token WHERE session = $before.id
); 

DEFINE EVENT user_deleted ON TABLE system_user WHEN $event = "DELETE" THEN (
    DELETE FROM system_session WHERE user = $before.id
);

###########################
# CREATE SYSTEM RESOURCES #
###########################
-- system_user
IF !(SELECT * FROM system_resource:system_user) {
    CREATE system_resource:system_user CONTENT {
        name: "system_user",
        resource_type: "SystemTable",
        description: "Contains system users",
    };
};
IF !(SELECT * FROM system_resource:system_user_email) {
    CREATE system_resource:system_user_email CONTENT {
        name: "system_user|email",
        resource_type: "SystemField",
        description: "Email address",
    };
};
IF !(SELECT * FROM system_resource:system_user_password) {
    CREATE system_resource:system_user_password CONTENT {
        name: "system_user|password",
        resource_type: "SystemField",
        description: "Password",
    };
};
IF !(SELECT * FROM system_resource:system_user_name) {
    CREATE system_resource:system_user_name CONTENT {
        name: "system_user|name",
        resource_type: "SystemField",
        description: "Name",
    };
};
IF !(SELECT * FROM system_resource:system_user_admin) {
    CREATE system_resource:system_user_admin CONTENT {
        name: "system_user|admin",
        resource_type: "SystemField",
        description: "Admin flag",
    };
};
IF !(SELECT * FROM system_resource:system_user_created_at) {
    CREATE system_resource:system_user_created_at CONTENT {
        name: "system_user|created_at",
        resource_type: "SystemField",
        description: "Created at timestamp",
    };
};
-- system_session
IF !(SELECT * FROM system_resource:system_session) {
    CREATE system_resource:system_session CONTENT {
        name: "system_session",
        resource_type: "SystemTable",
        description: "Contains user sessions",
    };
};
IF !(SELECT * FROM system_resource:system_session_user) {
    CREATE system_resource:system_session_user CONTENT {
        name: "system_session|user",
        resource_type: "SystemField",
        description: "User",
    };
};
-- system_session_token
IF !(SELECT * FROM system_resource:system_session_token) {
    CREATE system_resource:system_session_token CONTENT {
        name: "system_session_token",
        resource_type: "SystemTable",
        description: "Contains user session tokens",
    };
};
IF !(SELECT * FROM system_resource:system_session_token_session) {
    CREATE system_resource:system_session_token_session CONTENT {
        name: "system_session_token|session",
        resource_type: "SystemField",
        description: "Session",
    };
};
IF !(SELECT * FROM system_resource:system_session_token_token) {
    CREATE system_resource:system_session_token_token CONTENT {
        name: "system_session_token|token",
        resource_type: "SystemField",
        description: "Token",
    };
};
IF !(SELECT * FROM system_resource:system_session_token_created_at) {
    CREATE system_resource:system_session_token_created_at CONTENT {
        name: "system_session_token|created_at",
        resource_type: "SystemField",
        description: "Created at timestamp",
    };
};
IF !(SELECT * FROM system_resource:system_session_token_expires_at) {
    CREATE system_resource:system_session_token_expires_at CONTENT {
        name: "system_session_token|expires_at",
        resource_type: "SystemField",
        description: "Expires at timestamp",
    };
};
-- system_role
IF !(SELECT * FROM system_resource:system_role) {
    CREATE system_resource:system_role CONTENT {
        name: "system_role",
        resource_type: "SystemTable",
        description: "Contains user roles",
    };
};
IF !(SELECT * FROM system_resource:system_role_name) {
    CREATE system_resource:system_role_name CONTENT {
        name: "system_role|name",
        resource_type: "SystemField",
        description: "Name",
    };
};
IF !(SELECT * FROM system_resource:system_role_description) {
    CREATE system_resource:system_role_description CONTENT {
        name: "system_role|description",
        resource_type: "SystemField",
        description: "Description",
    };
};
-- system_permission
IF !(SELECT * FROM system_resource:system_permission) {
    CREATE system_resource:system_permission CONTENT {
        name: "system_permission",
        resource_type: "SystemTable",
        description: "Contains permission definitions",
    };
};
IF !(SELECT * FROM system_resource:system_permission_name) {
    CREATE system_resource:system_permission_name CONTENT {
        name: "system_permission|name",
        resource_type: "SystemField",
        description: "Name",
    };
};
IF !(SELECT * FROM system_resource:system_permission_description) {
    CREATE system_resource:system_permission_description CONTENT {
        name: "system_permission|description",
        resource_type: "SystemField",
        description: "Description",
    };
};
-- system_resource
IF !(SELECT * FROM system_resource:system_resource) {
    CREATE system_resource:system_resource CONTENT {
        name: "system_resource",
        resource_type: "SystemTable",
        description: "Contains resource definitions",
    };
};
IF !(SELECT * FROM system_resource:system_resource_name) {
    CREATE system_resource:system_resource_name CONTENT {
        name: "system_resource|name",
        resource_type: "SystemField",
        description: "Name",
    };
};
IF !(SELECT * FROM system_resource:system_resource_type) {
    CREATE system_resource:system_resource_type CONTENT {
        name: "system_resource|resource_type",
        resource_type: "SystemField",
        description: "Type",
    };
};
IF !(SELECT * FROM system_resource:system_resource_description) {
    CREATE system_resource:system_resource_description CONTENT {
        name: "system_resource|description",
        resource_type: "SystemField",
        description: "Description",
    };
};
IF !(SELECT * FROM system_resource:system_resource_created_at) {
    CREATE system_resource:system_resource_created_at CONTENT {
        name: "system_resource|created_at",
        resource_type: "SystemField",
        description: "Created at timestamp",
    };
};
IF !(SELECT * FROM system_resource:system_resource_updated_at) {
    CREATE system_resource:system_resource_updated_at CONTENT {
        name: "system_resource|updated_at",
        resource_type: "SystemField",
        description: "Last time that resource was updated",
    };
};

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