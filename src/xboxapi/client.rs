// Copyright (c) 2016 P.Y. Laligand

use hyper::Client as HttpClient;
use serde_json::{self, Value};
use std::io::Read;

use super::xuid::Xuid;

/// Header definition for the API key.
header! { (XAuth, "X-AUTH") => [String] }

/// Interface to the xboxapi.com API.
pub struct Client {

    /// The API key to authenticate with the service.
    api_key: String,
}

/// Base URL for API calls.
const BASE_URL: &'static str = "https://xboxapi.com/v2";

impl Client {

    /// Creates a new instance with the given API key.
    pub fn new(api_key: String) -> Client {
        Client {api_key: api_key}
    }

    /// Matches a gamertag with its internal user identifier.
    pub fn get_xuid(&self, gamertag: &String) -> Option<Xuid> {
        let path = format!("xuid/{}", gamertag);
        self.send_request(&path)
            .and_then(|json| {
                json.as_u64()
            })
            .and_then(|value| {
                Some(Xuid(value))
            })
    }

    /// Issues a request to the xboxapi.com API.
    ///
    /// Returns None if the request failed.
    fn send_request(&self, path: &String) -> Option<Value> {
        let url = format!("{}/{}", BASE_URL, path);
        let client = HttpClient::new();
        client.get(&url)
            .header(XAuth(self.api_key.clone()))
            .send()
            .ok()
            .and_then(|mut response| {
                let mut buffer : String = String::new();
                response.read_to_string(&mut buffer).ok().map(|_| buffer)
            })
            .and_then(|body| {
                serde_json::from_str(&body).ok()
            })
    }
}
