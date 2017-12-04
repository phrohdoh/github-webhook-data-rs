extern crate github_webhook_data as lib;
use lib::events::pull_request::{Event, Action, PullRequest, Head};

extern crate reqwest;
use reqwest::Client;

#[macro_use]
extern crate failure;

#[derive(Debug, Fail)]
enum GeneralError {
    #[fail(display = r#"Please provide the address to send the webhook data to. 
Example: send-webhook http://127.0.0.1:3000/endpoint"#)]
    AddressNotProvided,

    #[fail(display = "Please provide the action type to send.")]
    ActionNotProvided,

    #[fail(display = "The provided action `{}` is invalid.", action)]
    UnknownAction {
        action: String,
    },

    #[fail(display = "Failed to serialize test data into JSON. Perhaps an invalid string?")]
    FailedToCreateJson,

    #[fail(display = "The remote endpoint did not respond. Is your server running?")]
    EndpointDidNotRespond,

    #[fail(display = "The response's body was not valid UTF-8 data so could not be presented.")]
    NonUtf8ResponseBody,
}

fn run() -> Result<(reqwest::StatusCode, String), GeneralError> {
    let mut args = std::env::args().skip(1);
    let send_to_addr = args.nth(0).ok_or(GeneralError::AddressNotProvided)?;

    let action = {
        let action = args.nth(0).ok_or(GeneralError::ActionNotProvided)?;
        action.parse::<Action>().map_err(|_| GeneralError::UnknownAction { action })?
    };

    let data = Event {
        action: action,
        number: 1,
        pull_request: PullRequest {
            head: Head {
                label: "Phrohdoh:test-1".into(),
                ref_name: "test-1".into(),
                sha: "abc123def456".into(),
            },
        },
    };

    let json = data.to_json().map_err(|_| GeneralError::FailedToCreateJson)?;

    let client = Client::new();
    let mut resp = client.post(&send_to_addr).body(json).send().map_err(|_| GeneralError::EndpointDidNotRespond)?;

    let body = resp.text().map_err(|_| GeneralError::NonUtf8ResponseBody)?;

    Ok((resp.status(), body))
}

fn main() {
    match run() {
        Ok((status_code, resp_body)) => {
            println!("Status: {}", status_code);
            println!("Body: {}", resp_body);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        },
    }
}
