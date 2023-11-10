use crate::{
    config::database::repo::info::{table_names, TableFilter},
    utils::query_strings::*,
};
use serde::Deserialize;
use surrealdb::{engine::any::Any, Error, Surreal};

#[derive(Debug, Deserialize)]
struct Created {
    email: String,
}

pub async fn init(db: &Surreal<Any>) -> Result<(), Error> {
    // dev_clear(db).await?;

    dbg!(table_names(db, TableFilter::All).await);

    let mut admin_credentials_supplied = true;

    let admin_email = std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| {
        admin_credentials_supplied = false;
        "".to_string()
    });

    let admin_passw = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| {
        admin_credentials_supplied = false;
        "".to_string()
    });

    // Setup system tables and create default admin user
    let mut result = db
        .query(SETUP_DATABASE)
        .bind(("email", &admin_email))
        .bind(("password", admin_passw))
        .await?;

    let statements = result.num_statements();
    let created: Result<Option<Created>, Error> = result.take(statements - 2);
    let generated_password: Option<String> = result.take(statements - 1).unwrap();
    let generated_password = generated_password.unwrap();

    // Print admin credentials if they were generated
    match created {
        Ok(created) => {
            if !admin_credentials_supplied {
                let mut email = format!("# Email:    {} ", created.unwrap().email);
                let mut password = format!("# Password: {} ", generated_password);
                email = match email.len() < 45 {
                    true => format!("{email}{}#", &" ".repeat(45 - email.len())),
                    false => format!("{email}#"),
                };
                password = match password.len() < 45 {
                    true => format!("{password}{}#", &" ".repeat(45 - password.len())),
                    false => format!("{password}#"),
                };

                println!("{h:#>46}", h = "");
                println!("#        Generated Admin Credentials:        #");
                println!("{email}");
                println!("{password}");
                println!("{h:#>46}", h = "");
            }
        }
        Err(e) => {
            panic!("Error creating default admin user: {e}");
        }
    }

    let errors = result.take_errors();
    if !errors.is_empty() {
        println!("[");
        for error in errors {
            println!("\t{}: {}\n", error.0, error.1);
        }
        println!("]");
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn dev_clear(db: &Surreal<Any>) -> Result<(), Error> {
    db.query(DEV_CLEAR).await?;
    Ok(())
}
