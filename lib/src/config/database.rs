use crate::utils::query_strings::*;
use serde::Deserialize;
use surrealdb::{engine::any::Any, Error, Surreal};

#[derive(Deserialize)]
struct Created {
    email: String,
}

pub async fn configure(db: &Surreal<Any>) -> Result<(), Error> {
    // dev_clear(db).await?;

    let mut admin_credentials_supplied = true;

    let admin_email = std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| {
        admin_credentials_supplied = false;
        "".to_string()
    });
    let admin_passw = match std::env::var("ADMIN_PASSWORD") {
        Ok(passw) => passw,
        Err(_) => {
            admin_credentials_supplied = false;
            "".to_string()
        }
    };

    db.query(DEFINE_TABLES).await?;
    db.query(DEFINE_FIELDS).await?;
    db.query(DEFINE_INDEXES).await?;

    // Create admin user if it doesn't exist
    let mut result = db
        .query(CREATE_ADMIN)
        .bind(("email", &admin_email))
        .bind(("password", admin_passw))
        .await?;

    let created: Result<Option<Created>, Error> = result.take(2);
    let generated_password: Option<String> = result.take(3).unwrap();
    let generated_password = generated_password.unwrap();

    // Print admin credentials if they were generated
    if let Ok(created) = created {
        if !admin_credentials_supplied {
            let email = created.unwrap().email;
            let email_spaces = if email.len() < generated_password.len() {
                " ".repeat(generated_password.len() - email.len())
            } else {
                "".to_string()
            };
            let password_spaces = if generated_password.len() < email.len() {
                " ".repeat(email.len() - generated_password.len())
            } else {
                "".to_string()
            };

            println!("##############################################");
            println!("#        Generated Admin Credentials:        #");
            println!("# Email:    {email}{email_spaces} #");
            println!("# Password: {generated_password}{password_spaces} #");
            println!("##############################################");
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn dev_clear(db: &Surreal<Any>) -> Result<(), Error> {
    db.query(DEV_CLEAR).await?;
    Ok(())
}
