extern crate clap;
use clap::{Arg, App};

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

    // Calling .unwrap() is safe here because "APP_ID" is required
    let app_id = matches.value_of("APP_ID").unwrap();

    // Create HTTP endpoint
    let mut endpoint = format!("https://api.clever-cloud.com:443/v2/logs/{}/sse", app_id);

    // Calling .unwrap() is safe here because "token" is required
    let access_token = matches.value_of("token").unwrap();

    // Add HTTP query parameters
    // FIXME: Find how is built o_auth_authorization_string
    let o_auth_authorization_string = access_token;
    endpoint = format!("{}?authorization={}", endpoint, o_auth_authorization_string);
    println!("endpoint: {}", endpoint);

    // More program logic goes here...
}
