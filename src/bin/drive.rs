// Copyright (c) 2016 P.Y. Laligand

extern crate getopts;
extern crate xbox_clips_sync;

use std::env;

use getopts::Options;

use xbox_clips_sync::drive::Driver;

fn main() {
    let mut options = Options::new();
    options.reqopt("c", "config", "oAuth app configuration", "CONFIG_FILE");
    options.reqopt("x", "credentials", "oAuth credentials", "CREDENTIALS_FILE");
    let args: Vec<String> = env::args().collect();
    let matches = options.parse(&args[1..]).expect("Could not parse parameters");
    let config = matches.opt_str("c").unwrap();
    let credentials = matches.opt_str("x").unwrap();

    let driver = Driver::new(&config, &credentials).unwrap();
    let details = driver.get_user_details().unwrap();
    println!("Got: {}", details);
}
