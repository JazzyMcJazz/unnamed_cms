// Setup
pub static SETUP_DATABASE: &str = include_str!("./setup_database.surql");
pub static DEV_CLEAR: &str = "REMOVE DATABASE cms;";

// Create
pub static CREATE_SESSION: &str = include_str!("./create_session.surql");

// Read
pub static FIND_USER_BY_CREDENTIALS: &str = include_str!("./select_user_by_credentials.surql");

// Update
pub static REFRESH_SESSION: &str = include_str!("./refresh_session.surql");

// Delete
pub static DELETE_SESSION: &str = include_str!("./delete_session.surql");
