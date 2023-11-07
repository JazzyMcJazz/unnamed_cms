pub static SETUP_DATABASE: &str = include_str!("./setup_database.surql");
pub static FIND_USER_BY_CREDENTIALS: &str = include_str!("./select_user_by_credentials.surql");
pub static DEV_CLEAR: &str = "REMOVE DATABASE cms;";
