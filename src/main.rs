extern crate clap;
use clap::{Arg, App};

use directories::UserDirs;
use std::io::prelude::*;
use std::fs::File;

use serde_json::Value;

fn main() {
    let matches = App::new("Clever Cloud App Logs CLI")
        .version("1.0")
        .author("Rémi B. <remi.bardon.dev@gmail.com>")
        .about("Streams app logs from Clever Cloud API")
        .arg(Arg::with_name("APP_ID")
            .long("id")
            .help("Sets the app ID to use")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name("token")
            .short("t")
            .long("token")
            .value_name("ACCESS_TOKEN")
            .help("OAuth1 access token used for Clever Cloud API authentication. For more information, visit https://www.clever-cloud.com/doc/clever-cloud-apis/cc-api/#http-calls.")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

    let (access_token, token_secret) = clever_config();

    // Calling .unwrap() is safe here because "APP_ID" is required
    let app_id = matches.value_of("APP_ID").unwrap();

    // Create HTTP endpoint
    let mut endpoint = format!("https://api.clever-cloud.com:443/v2/logs/{}/sse", app_id);

    // Add HTTP query parameters
    // FIXME: Find how is built o_auth_authorization_string
    let o_auth_authorization_string: &str = access_token.as_str();
    endpoint = format!("{}?authorization={}", endpoint, o_auth_authorization_string);
    println!("endpoint: {}", endpoint);

    // More program logic goes here...
}

fn parsed_config() -> serde_json::Value {
    let user_dirs = UserDirs::new().expect("Could not find user home directory");
    let config_file_path = format!("{}/.config/clever-cloud", user_dirs.home_dir().display());

    let mut config_file = File::open(config_file_path)
        .expect("No file found at '~/.config/clever-cloud'. Please follow instructions at https://github.com/CleverCloud/clever-tools to install 'clever-tools', then run `clever login`.");
    
    let mut json_config = String::new();
    config_file.read_to_string(&mut json_config)
        .expect("Unable to read the file at '~/.config/clever-cloud'");

    // Parse the string of data into serde_json::Value
    serde_json::from_str(json_config.as_str())
        .expect("Invalid content in file '~/.config/clever-cloud'")
}

fn clever_config() -> (String, String) {
    let config = parsed_config();

    // Get OAuth1 Access Token
    let access_token: String;
    let token_key = "token";
    if let Value::String(token) = config[token_key].clone() {
        access_token = token;
    } else {
        println!("No value '{}' found in '~/.config/clever-cloud'. Please follow instructions at https://github.com/CleverCloud/clever-tools to install 'clever-tools', then run `clever login`.", token_key);
        std::process::exit(1);
    }
    
    // Get OAuth1 API Secret
    let token_secret: String;
    let secret_key = "secret";
    if let Value::String(secret) = config[secret_key].clone() {
        token_secret = secret;
    } else {
        println!("No value '{}' found in '~/.config/clever-cloud'. Please follow instructions at https://github.com/CleverCloud/clever-tools to install 'clever-tools', then run `clever login`.", secret_key);
        std::process::exit(1);
    }

    (access_token, token_secret)
}
