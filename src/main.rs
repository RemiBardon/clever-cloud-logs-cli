// Error Handling
use anyhow::{Result, Error};

// Command Line Argument Parsing
extern crate clap;
use clap::{Arg, App};

// HTTP Client
extern crate surf;

// Server Sent Event
use async_sse::{decode, Event};
use async_std::prelude::*;

// Clever Cloud CLI Config
mod config;

// Pretty Logs
mod log;

#[async_std::main]
async fn main() -> Result<()> {
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

    // Note: Should never throw "APP_ID" is required
    let app_id = matches.value_of("APP_ID")
        .ok_or(Error::msg("Argument 'APP_ID' not found"))?;
    // Note: Should never throw because "consumer_key" is required
    let consumer_key = matches.value_of("consumer_key")
        .ok_or(Error::msg("Argument 'consumer_key' not found"))?;
    // Note: Should never throw because "consumer_secret" is required
    let consumer_secret = matches.value_of("consumer_secret")
        .ok_or(Error::msg("Argument 'consumer_secret' not found"))?;

    // Build SSE endpoint
    let endpoint = sse_endpoint(app_id, consumer_key, consumer_secret)?;

    // Connect to endpoint
    let res = surf::get(&endpoint).await
        .map_err(|e| Error::msg(format!("Could not send GET request to {}: {}", &endpoint, e)))?;

    // Create Decoder from AsyncBufRead
    let mut decoder = decode(res);

    // Print status information
    println!("Streaming logs from {}...\n", app_id);

    loop {
        // Handle event or log errors if any (we don't want to bubble up errors here, but rather skip loop cycle)
        if let Err(e) = log_next(&mut decoder).await {
            eprintln!("{}", e);
        }
    }
}

fn sse_endpoint(app_id: &str, consumer_key: &str, consumer_secret: &str) -> Result<String> {
    // First, read Clever CLoud CLI config file, which aborts if error
    let (access_token, token_secret) = config::clever_config()?;

    // Create HTTP endpoint
    let mut endpoint = format!("https://api.clever-cloud.com/v2/logs/{}/sse", app_id);

    // Create OAuth 1 consumer and token with secrets
    let consumer = oauth1::Token::new(consumer_key, consumer_secret);
    let token = oauth1::Token::new(access_token, token_secret);

    // Create OAuth 1 HTTP Authorization header
    let authorization = oauth1::authorize("GET", &endpoint, &consumer, Some(&token), None);

    // Base64 encode OAuth 1 HTTP Authorization header
    let base64_authorization = base64::encode(authorization);

    // Add OAuth 1 authorization string query parameter
    endpoint = format!("{}?authorization={}", endpoint, base64_authorization);

    Ok(endpoint)
}

async fn log_next(decoder: &mut async_sse::Decoder<surf::Response>) -> Result<()> {
    // Wait for new data
    let event = decoder.next().await
        .ok_or(Error::msg("Could not read next value from buffer"))?        // Unwrap Option
        .map_err(|e| Error::msg(format!("Error getting data: {}", e)))?;    // Unwrap Result

    // Match and handle the event
    match event {
        Event::Message(message) => {
            match std::str::from_utf8(message.data()) {
                Ok(line) => log::log(line),
                Err(e) => Err(Error::msg(format!("Invalid UTF-8 sequence: {}", e))),
            }
        },
        Event::Retry(duration) => Err(Error::msg(format!("Retry: {}s", duration.as_secs()))),
    }
}
