// Copyright (c) 2016 P.Y. Laligand

extern crate getopts;
extern crate xbox_clips_sync;

use getopts::Options;
use xbox_clips_sync::xboxapi::Client;
use std::env;

fn main() {
    let mut options = Options::new();
    options.reqopt("a", "api-key", "xboxapi API key", "KEY");
    options.reqopt("g", "gamertag", "Gamertag", "GAMERTAG");
    let args: Vec<String> = env::args().collect();
    let matches = options.parse(&args[1..]).expect("Could not parse parameters");
    let api_key = matches.opt_str("a").unwrap();
    let gamertag = matches.opt_str("g").unwrap();

    let client = Client::new(api_key);

    let xuid = client.get_xuid(&gamertag).expect("Could not identify this gamertag");
    println!("ID for {} is {}", gamertag, xuid);

    let clips = client.get_clips(&xuid).unwrap();
    let size = clips.len();
    println!("Found {} clips", size);
    for clip in clips.iter().take(5) {
        println!("{}", clip);
    }
}
