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
use std::fs::File;
use std::path::Path;
use std::process::exit;

fn parse_binary(unit: u8) -> u8 {
    if unit != 0 && unit != 1 {
        display_config_err(
        &format!("Boolean option must be 1 (True) or 0 (False). Not {}", unit));
    }
    unit
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
    let al = parse_binary(parse_unit(&get_input("Auto login (0 = False, 1 = True)")));
    let wm = parse_binary(parse_unit(&get_input("WiFi mode (0 = False, 1 = True)")));
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
    validate_config(serde_json::from_str(&cs).unwrap())
}

fn validate_config(conf: Configuration) -> Configuration {
    parse_binary(conf.get_auto_login());
    parse_binary(conf.get_wifi_mode());
    conf
}

fn connect_to_portal(conf: &Configuration, verbose: bool) {
    let mut ec = -2;
    let ssid = SSID::new();
    if conf.get_wifi_mode() == 1 {
        if verbose {
            println!("Connecting to {}...", conf.get_ssid());
        } 
        if !ssid.is_connected_to(conf.get_ssid()) {
            ssid.connect(conf.get_ssid());
        }
        if ssid.is_connected_to(conf.get_ssid()) {
            ec = 0;
        }
    }
    if verbose && ec == 0 {
        println!("OK.");
    } else if verbose {
        println!("FAILED.");
    }
    exit(ec);
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

fn display_config_err(err: &str) {
    println!("Config Error: {}.", err);
    exit(-1);
}

fn display_usage(program: &str, code: i32) {
    println!("cpal (Captive Portal Auto Login).");
    println!("Utility to automatically log in to a captive portal.");
    println!("Copyright 2017 Sam Saint-Pettersen.");
    println!("\nReleased under the MIT License.");
    println!("\nUsage: {} <command> [<options>]", program);
    println!("\nCommands are:\n");
    println!("configure : Configure the settings for auto log in.");
    println!("login : Log in to the captive portal using saved configuration.");
    println!("status : Get the status of the captive portal connection.");
    println!("configuration : Display the loaded configuration.");
    println!("\nOptions are:\n");
    println!("-q | --quiet: Do not be verbose on login and configure commands.");
    println!("-h | --help: Display this usage information and exit.");
    println!("-v | --version: Display version information and exit.");
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
    } else {
        display_error(&program, "No commands or options provided");
    }
    
    if !Path::new(conf_json).exists() {
        write_configuration(&conf_json, verbose);
    }

    let config = load_configuration(&conf_json, verbose);
    match op {
        0 => write_configuration(&conf_json, verbose),
        1 => connect_to_portal(&config, verbose),
        2 => get_status(&config),
        3 => get_configuration(&config),
        _ => {
            display_error(&program,
            &format!("Invalid command(s) or option(s) in:\n{:?}", &cli.get_args()[1..]))
        },
    }
}
