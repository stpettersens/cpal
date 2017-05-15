/*
    cpal (Captive Portal Auto Login).
    Utility to automatically log in to a captive portal.
    Copyright 2017 Sam Saint-Pettersen.

    Released under the MIT License.
*/

extern crate clioptions;
extern crate ssid;
extern crate curl;
//extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use clioptions::CliOptions;
use ssid::SSID;
use curl::easy::Easy as CurlRequest;
//use rustc_serialize::json;
//use rustc_serialize::json::Json;
use std::io::{stdin, stdout, Read, Write};
use std::fs;
use std::fs::File;
use std::process::exit;

//#[derive(RustcEncodable, RustcDecodable)]
#[derive(Serialize, Deserialize)]
struct Configuration {
    ssid: String,
    username: String,
    password: String,
    auto_login: u8,
    wifi_mode: u8,
}

impl Configuration {
    fn new(ssid: &str, username: &str, password: &str,
    auto_login: u8, wifi_mode: u8) -> Configuration {
        Configuration {
            ssid: ssid.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
            auto_login: auto_login,
            wifi_mode: wifi_mode,
        }
    }
}

fn parse_unit(unit: &str) -> u8 {
    let n = unit.parse::<u8>().ok();
    let unit = match n {
        Some(unit) => unit as u8,
        None => 0 as u8,
    };
    unit
}

fn get_input(prompt: &str) -> String {
    println!("{}?", prompt);
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(error) => {
            println!("Stdin Error: {}", error);
            exit(-1);
        }
    }
    input.trim().to_owned()
}

fn write_configuration(conf: &str) {
    let ssid = get_input("SSID");
    let username = get_input("Username");
    let password = get_input("Password");
    let al = parse_unit(&get_input("Auto login (0 = False, 1 = True)"));
    let wm = parse_unit(&get_input("WiFi mode (0 = False, 1 = True)"));
    let config = Configuration::new(&ssid, &username, &password, al, wm);
    let mut w = File::create(conf).unwrap();
    //let o = json::encode(&config).unwrap();
    let j = serde_json::to_string(&config).unwrap();
    let fo = format!("{:#}\n", j);
    let _ = w.write_all(fo.as_bytes());
}

fn display_version() {
    println!("cpal v. 0.1.0");
    println!("This program uses libcurl (https://curl.haxx.se)");
    exit(0);
}

fn display_error(program: &str, err: &str) {
    println!("Error: {}.\n", err);
    display_usage(program, -1);
}

fn display_usage(program: &str, code: i32) {
    println!("cpal (Captive Portal Auto Login).");
    println!("Utility to automatically log in to a captive portal.");
    println!("Copyright 2017 Sam Saint-Pettersen.");
    println!("\nReleased under the MIT License.");
    println!("\nUsage: {} <command> [<options>]", program);
    exit(code);
}

fn main() {
    let cli = CliOptions::new("cpal");
    let program = cli.get_program();

    // -------------------------------
    let conf_json = ".cpal.json";
    // -------------------------------

    let mut verbose = true;
    let mut op = -1;

    if cli.get_num() > 1 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-h" | "--help" => display_usage(&program, 0),
                "-v" | "--version" => display_version(),
                "-q" | "--quiet" => verbose = false,
                "configure" => op = 0,
                _ => continue,
            }
        }
    }
    match op {
        0 => write_configuration(conf_json),
        _ => {},
    }
}
