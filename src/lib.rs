// Copyright (c) 2016 P.Y. Laligand

#![feature(proc_macro)]

extern crate chrono;
extern crate google_drive3;
#[macro_use] extern crate hyper;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate yup_oauth2;

pub mod drive;
pub mod xboxapi;
