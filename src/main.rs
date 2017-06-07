/*
    cpal (Captive Portal Auto Login).
    Utility to automatically log in to a captive portal.
    Copyright 2017 Sam Saint-Pettersen.

    Released under the MIT License.
*/

mod config;
extern crate clioptions;
extern crate ssid;
extern crate curl;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use config::Configuration;
use clioptions::CliOptions;
use ssid::SSID;
use curl::easy::Easy as CurlRequest;
use std::io::{stdin, stdout, Read, Write};
use std::fs::{self, File};
use std::path::Path;
use std::process::exit;

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

fn get_configuration(config: &Configuration) {
    println!("{:#?}", config);
}

fn get_status(config: &Configuration) {
    let ssid = SSID::new();
    println!("Configured SSID is {}", config.get_ssid());
    if ssid.is_connected_to(config.get_ssid()) {
        println!("Connected to that SSID.");
    } else if ssid.is_connected() {
        println!("Connected to SSID: {}", ssid.get_id());
    } else {
        println!("Disconnected.");
    }
}

fn write_configuration(conf: &str, verbose: bool) {
    let ssid = get_input("SSID");
    let portal = get_input("Portal");
    let username = get_input("Username");
    let password = get_input("Password");
    let al = parse_unit(&get_input("Auto login (0 = False, 1 = True)"));
    let wm = parse_unit(&get_input("WiFi mode (0 = False, 1 = True)"));
    let config = Configuration::new(&ssid, &portal, &username, &password, al, wm);
    let mut w = File::create(conf).unwrap();
    let j = serde_json::to_string(&config).unwrap();
    let fo = format!("{:#}\n", j);
    let _ = w.write_all(fo.as_bytes());
    if verbose {
        println!("Wrote configuration -> {}", conf);
    }
    exit(0);
}

fn load_configuration(conf: &str, verbose: bool) -> Configuration {
    let mut cs = String::new();
    let mut file = File::open(conf).unwrap();
    let _ = file.read_to_string(&mut cs);
    if verbose {
        println!("Loaded configuration <- {}", conf);
    }
    serde_json::from_str(&cs).unwrap()
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
        for a in cli.get_args().iter() {
            match a.trim() {
                "-h" | "--help" => display_usage(&program, 0),
                "-v" | "--version" => display_version(),
                "-q" | "--quiet" => verbose = false,
                "configure" => op = 0,
                "login" => op = 1,
                "status" => op = 2,
                "configuration" => op = 3,
                _ => continue,
            }
        }
    }
    
    if !Path::new(conf_json).exists() {
        write_configuration(&conf_json, verbose);
    }
    
    let config = load_configuration(&conf_json, verbose);
    match op {
        0 => write_configuration(&conf_json, verbose),
        1 => {},
        2 => { 
            let _ = get_status(&config);
        },
        3 => get_configuration(&config),
        _ => {},
    }
}
