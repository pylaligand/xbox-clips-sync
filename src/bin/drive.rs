// Copyright (c) 2016 P.Y. Laligand

extern crate getopts;
extern crate google_drive3 as drive3;
extern crate hyper;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;

use drive3::Drive;
use getopts::Options;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, ConsoleApplicationSecret, DiskTokenStorage, FlowType};
use std::env;

/// Reads oAuth application config from a given file.
fn read_secret(file: &str) -> std::result::Result<ApplicationSecret, String> {
    use std::fs::File;
    use std::io::Read;
    File::open(file)
        .and_then(|mut f| {
            let mut buffer = String::new();
            f.read_to_string(&mut buffer).map(|_| buffer)
        })
        .map_err(|err| err.to_string())
        .and_then(|content| {
            serde_json::from_str::<ConsoleApplicationSecret>(content.as_str())
                .map_err(|err| err.to_string())
        })
        .and_then(|console_secret| {
            console_secret.installed.ok_or(String::from("No params for installed flow"))
        })
}

fn main() {
    let mut options = Options::new();
    options.reqopt("c", "config", "oAuth app configuration", "CONFIG_FILE");
    options.reqopt("x", "credentials", "oAuth credentials", "CREDENTIALS_FILE");
    let args: Vec<String> = env::args().collect();
    let matches = options.parse(&args[1..]).expect("Could not parse parameters");
    let config = matches.opt_str("c").unwrap();
    let credentials = matches.opt_str("x").unwrap();

    let secret = read_secret(&config).unwrap();
    let storage = DiskTokenStorage::new(&credentials).unwrap();
    let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
        hyper::Client::new(),
        storage,
        Some(FlowType::InstalledInteractive));
    let hub = Drive::new(hyper::Client::new(), auth);
    match hub.about().get().param("fields", "user").doit() {
        Ok(response) => println!("{:?}", response),
        Err(e) => println!("There was an error: {}", e),
    }
}
