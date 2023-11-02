use std::env;

mod server;
extern crate unnamed_cms_lib;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    env::set_var("SURREAL_URL", "ws://localhost:8000");
    env::set_var("SURREAL_USER", "root");
    env::set_var("SURREAL_PASS", "root");

    server::main();
}
