//! backend_rest/src/main.rs


use common_lib::init::init;

fn main() {

    init(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));

    println!("Hello");

}
