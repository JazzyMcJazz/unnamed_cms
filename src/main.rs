use std::env;

mod server;
extern crate unnamed_cms;

fn main() {
    // TODO: Remove this when deploying
    env::set_var("RUST_BACKTRACE", "full");
    env::set_var("SURREAL_URL", "ws://localhost:8000");
    env::set_var("SURREAL_USER", "root");
    env::set_var("SURREAL_PASS", "root");
    env::set_var("ADMIN_EMAIL", "admin@test.com");
    env::set_var("ADMIN_PASSWORD", "asdf1234");

    if env::var("JWT_SECRET").is_err() {
        env::set_var("JWT_SECRET", "secret");
    }

    server::main();
}