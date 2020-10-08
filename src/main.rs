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
        .get_matches();

    // Calling .unwrap() is safe here because "APP_ID" is required
    let app_id = matches.value_of("APP_ID").unwrap();

    // Create HTTP endpoint
    let endpoint = format!("https://api.clever-cloud.com:443/v2/logs/{}/sse", app_id);
    println!("endpoint: {}", endpoint);

    // More program logic goes here...
}
