use super::macros::doc_and_define_with_bindings;

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
