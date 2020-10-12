// JSON Parsing
use serde::{Deserialize};

// Terminal Formatting
extern crate term;

/* === Public Functions === */

pub fn log(line: &str) {
    let deserialized: CCEventMessage = {
        match serde_json::from_str(line) {
            Ok(d) => d,
            Err(_) => return,
        }
    };

    let source = &deserialized.source;

    // Print timestamp
    print!("{}: ", source.timestamp);

    // Set terminal format according to emphasis
    let mut terminal = term::stdout().unwrap();
    if let Some((fg, attr)) = get_term_emphasis(source) {
        terminal.fg(fg).unwrap();
        terminal.attr(attr).unwrap();
    }

    // Print message
    println!("{}", source.message);

    // Reset terminal format
    terminal.reset().unwrap();
}

/* === Private Functions === */

fn is_clever_message(source: &CCEventSource) -> bool {
    return source.syslog_program == "/home/bas/rubydeployer/deployer.rb";
}

fn get_term_emphasis(source: &CCEventSource) -> Option<(term::color::Color, term::Attr)> {
    if is_clever_message(source) {
        let lower_message = source.message.to_lowercase();
        if lower_message.starts_with("successfully deployed in") {
            return Some((term::color::GREEN, term::Attr::Bold))
        } else if lower_message.starts_with("deploy failed in") {
            return Some((term::color::RED, term::Attr::Bold))
        } else if lower_message.starts_with("build succeeded in") {
            return Some((term::color::BLUE, term::Attr::Bold))
        }
    }
    return None
}

/* === Data Models === */

#[derive(Deserialize)]
struct CCEventMessage {
    #[serde(rename = "_source")]
    source: CCEventSource,
}

#[derive(Deserialize)]
struct CCEventSource {
    syslog_program: String,

    #[serde(rename = "@timestamp")]
    timestamp: String,
    #[serde(rename = "@message")]
    message: String,
}
