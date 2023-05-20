//! backend_rest/src/main.rs


use common_lib::init::init;

fn main() {

    // init(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    // get the package name of this package (otherwise init would choose "common_lib")
    init(env!("CARGO_MANIFEST_DIR"));

    println!("Hello");

}
