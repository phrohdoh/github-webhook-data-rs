extern crate server as lib;

fn main() {
    let addr = "127.0.0.1:3000";
    println!("Running server on {}", addr);
    lib::run(addr);
}