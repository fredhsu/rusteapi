#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;

use std::io::Read;
use serde::json;

use hyper::Client;
use hyper::header::Basic;
use hyper::header::Authorization;

#[derive(Serialize)]
pub struct Request <T> {
    jsonrpc: String,
    method: String,
    params: T,
    id: String
}

#[derive(Deserialize)]
pub struct Response <T> {
    jsonrpc: String,
    result: T,
    error: Option <String>,
    id: String
}

#[derive(Deserialize)]
pub struct Error <E> {
    code: u32,
    message: String,
    data: E
}
/*
#[derive(Deserialize)]
pub struct EapiResult <E> {
    message: String,
    data: E
}
*/

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct EapiVersion {
    modelName: String,
    internalVersion: String,
    systemMacAddress: String,
    serialNumber: String,
    memTotal: u32,
    bootupTimestamp: f32,
    memFree: u32,
    version: String,
    architecture: String,
    internalBuildId: String,
    hardwareRevision: String
}

#[derive(Serialize)]
pub struct EapiParams {
    version: u32,
    cmds: Vec<String>,
    format: String,
    timestamps: bool,
}


#[test]
fn it_works() {
}
