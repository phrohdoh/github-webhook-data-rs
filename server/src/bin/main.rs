extern crate server as lib;

extern crate structopt;
use structopt::StructOpt;

#[macro_use]
extern crate structopt_derive;

#[derive(StructOpt, Debug)]
#[structopt(name = "server", about = "An HTTP server that accepts GitHub webhook API data (as JSON).")]
struct CliOptions {
    #[structopt(long = "port", help = "The port number to accept JSON at", default_value = "3000")]
    port: u16,

    #[structopt(long = "address", help = "The IP/URL/etc. to accept JSON at (not including the protocol)", default_value = "127.0.0.1")]
    addr: String,
}

fn main() {
    let opts = CliOptions::from_args();
    let addr = format!("{}:{}", opts.addr, opts.port);

    println!("Running server on {}", &addr);
    lib::run(&addr);
}