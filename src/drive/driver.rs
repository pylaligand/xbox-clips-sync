// Copyright (c) 2016 P.Y. Laligand

use std::fs;
use std::io::Read;
use std::path::Path;

use google_drive3::Drive;
use hyper::Client;
use serde_json;
use yup_oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, ConsoleApplicationSecret, DiskTokenStorage, FlowType};

use super::error::DriveError;

/// Interface to the Google Drive service.
///
/// Takes care of authentication.
pub struct Driver {
    hub: Drive<Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>,
}

impl Driver {
    /// Reads oAuth application config from a given file.
    fn read_secret<P: AsRef<Path>>(path: P) -> Result<ApplicationSecret, DriveError> {
        let mut file = try!(fs::File::open(path));
        let mut content = String::new();
        try!(file.read_to_string(&mut content));
        let secret: ConsoleApplicationSecret = try!(serde_json::from_str(content.as_str()));
        secret.installed.ok_or(DriveError::Described(String::from("No params for installed flow")))
    }

    pub fn new(config_path: &String, credentials_path: &String) -> Result<Driver, DriveError> {
        let secret = try!(Driver::read_secret(config_path));
        let storage = try!(DiskTokenStorage::new(credentials_path));
        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
            Client::new(),
            storage,
            Some(FlowType::InstalledInteractive));
        Ok(Driver { hub: Drive::new(Client::new(), auth) })
    }

    pub fn get_user_details(&self) -> Result<String, DriveError> {
        let response = try!(self.hub.about().get().param("fields", "user").doit());
        Ok(format!("{:?}", response))
    }
}
