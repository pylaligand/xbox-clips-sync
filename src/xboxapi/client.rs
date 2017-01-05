// Copyright (c) 2016 P.Y. Laligand

use hyper::Client as HttpClient;
use serde_json::{self, Map, Value};
use std::io::Read;

use super::clip::Clip;
use super::error::XboxError;
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
    pub fn get_xuid(&self, gamertag: &String) -> Result<Xuid, XboxError> {
        let path = format!("xuid/{}", gamertag);
        self.send_request(&path)
            .and_then(|json| { json.as_u64().ok_or(XboxError::from("Unexpected id format")) })
            .map(|value| { Xuid(value) })
    }

    /// Builds a clip object from its JSON representation.
    fn create_clip(json: &mut Map<String, Value>) -> Result<Clip, XboxError> {
        let get_value = |key: &str| -> Result<String, XboxError> {
            json.get(key)
                .and_then(|value| { value.as_str() })
                .map(|string| { string.to_owned() })
                .ok_or(XboxError::new(format!("Could not get key {}", key)))
        };
        let id = try!(get_value("gameClipId"));
        let date = try!(get_value("datePublished"));
        let url = try!(json.get("gameClipUris")
            .and_then(|uris_value| { uris_value.as_array() })
            .and_then(|uris| { uris[0].as_object() })
            .and_then(|clip_uri| { clip_uri.get("uri") })
            .and_then(|uri_value| { uri_value.as_str() })
            .map(|uri| { uri.to_owned() })
            .ok_or(XboxError::from("Could not find clip URIs")));
        Ok(Clip { id: id, url: url, date: date })
    }

    /// Returns the list of available clips for the given user.
    pub fn get_clips(&self, xuid: &Xuid) -> Result<Vec<Clip>, XboxError> {
        let path = format!("{}/game-clips", xuid);
        let mut json = try!(self.send_request(&path));
        let json_string = json.to_string();
        let clips = try!(json.as_array_mut().ok_or(
                XboxError::new(format!("Unexpected clip array: {}", json_string))));
        clips.iter_mut().map(|clip| {
            let clip_string = clip.to_string();
            let content = try!(clip.as_object_mut().ok_or(
                    XboxError::new(format!("Unexpected clip: {}", clip_string))));
            Client::create_clip(content)
        }).collect()
    }

    /// Issues a request to the xboxapi.com API.
    ///
    /// Returns None if the request failed.
    fn send_request(&self, path: &String) -> Result<Value, XboxError> {
        let url = format!("{}/{}", BASE_URL, path);
        let client = HttpClient::new();
        let mut response = try!(client.get(&url)
            .header(XAuth(self.api_key.clone()))
            .send());
        let mut buffer = String::new();
        try!(response.read_to_string(&mut buffer));
        let result: Value = try!(serde_json::from_str(&buffer));
        Ok(result)
    }
}
