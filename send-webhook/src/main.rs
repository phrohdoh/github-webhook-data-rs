extern crate github_webhook_data as lib;
use lib::events::pull_request::{Event, Action, PullRequest, Head};

extern crate reqwest;
use reqwest::Client;

fn main() {
    let mut args = std::env::args().skip(1);

    let send_to_addr = if let Some(send_to_addr) = args.nth(0) {
        send_to_addr
    } else {
        eprintln!("Please provide the address to send the webhook data to. See the following example.");
        eprintln!("send-webhook http://127.0.0.1:3000/endpoint");
        std::process::exit(1);
    };

    let data = Event {
        // TODO: Get the `action` from args.
        action: Action::Opened,
        number: 1,
        pull_request: PullRequest {
            head: Head {
                label: String::from("Phrohdoh:test-1"),
                ref_name: String::from("test-1"),
                sha: String::from("abc123def456"),
            },
        },
    };

    // TODO: Do not use unwrap/expect. Prefer proper error handling.
    let json = data.to_json().expect("Failed to serialize test data into JSON");

    let client = Client::new();
    let mut resp = client.post(&send_to_addr)
        .body(json)
        .send()
        .expect("The remote endpoint did not respond");

    let body = resp.text().expect("Unable to read the response's body");

    println!("Status: {}", resp.status());
    println!("Body: {}", body);
}
