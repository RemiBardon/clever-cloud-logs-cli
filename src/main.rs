// Command Line Argument Parsing
extern crate clap;
use clap::{Arg, App};

// File reading & JSON parsing
use directories::UserDirs;
use std::io::prelude::*;
use std::fs::File;
use serde_json::Value;

// HTTP Client
extern crate surf;

// Server Sent Event
use async_sse::{decode, Event};
use async_std::prelude::*;

// Pretty logs
mod log;

#[async_std::main]
async fn main() -> http_types::Result<()> {
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
        .arg(Arg::with_name("consumer_key")
            .short("k")
            .long("consumer_key")
            .value_name("OAUTH_CONSUMER_KEY")
            .help("OAuth1 consumer key used for Clever Cloud API authentication. For more information, visit https://www.clever-cloud.com/doc/clever-cloud-apis/cc-api/#-create-consumers-tokens-.")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("consumer_secret")
            .short("s")
            .long("consumer_secret")
            .value_name("OAUTH_CONSUMER_SECRET")
            .help("OAuth1 consumer secret used for Clever Cloud API authentication. For more information, visit https://www.clever-cloud.com/doc/clever-cloud-apis/cc-api/#-create-consumers-tokens-.")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

    // First, read Clever CLoud CLI config file, which aborts if error
    let (access_token, token_secret) = clever_config();

    // Calling .expect() is safe here because "APP_ID" is required
    let app_id = matches.value_of("APP_ID").expect("Argument 'APP_ID' not found");

    // Create HTTP endpoint
    let mut endpoint = format!("https://api.clever-cloud.com/v2/logs/{}/sse", app_id);

    // Calling .expect() is safe here because "consumer_key" is required
    let consumer_key = matches.value_of("consumer_key").expect("Argument 'consumer_key' not found");
    // Calling .expect() is safe here because "consumer_secret" is required
    let consumer_secret = matches.value_of("consumer_secret").expect("Argument 'consumer_secret' not found");

    // Create OAuth 1 consumer and token with secrets
    let consumer = oauth1::Token::new(consumer_key, consumer_secret);
    let token = oauth1::Token::new(access_token.as_str(), token_secret.as_str());

    // Create OAuth 1 HTTP Authorization header
    let authorization = oauth1::authorize("GET", endpoint.as_str(), &consumer, Some(&token), None);
    // println!("authorization: {}\n", authorization);

    // Base64 encode OAuth 1 HTTP Authorization header
    let base64_authorization = base64::encode(authorization);

    // Add OAuth 1 authorization string query parameter
    endpoint = format!("{}?authorization={}", endpoint, base64_authorization);
    // println!("endpoint: {}\n", endpoint);

    let res = surf::get(endpoint).await?;
    let mut reader = decode(res);

    loop {
        let event = reader.next().await.unwrap()?;
        
        // Match and handle the event
        match event {
            Event::Message(message) => {
                match std::str::from_utf8(message.data()) {
                    Ok(s) => log::log(s),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                }
            },
            Event::Retry(duration) => println!("retry: {}s", duration.as_secs())
        }
    }
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
