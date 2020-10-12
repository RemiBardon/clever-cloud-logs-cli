// JSON Parsing
use serde::{Deserialize};

// Terminal Formatting
extern crate term;

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

fn is_clever_message(source: &CCEventSource) -> bool {
    return source.syslog_program == "/home/bas/rubydeployer/deployer.rb";
}

fn is_deployment_success_message(source: &CCEventSource) -> bool {
    return is_clever_message(source)
        && source.message
            .to_lowercase().as_str()
            .starts_with("successfully deployed in");
}

fn is_deployment_failed_message(source: &CCEventSource) -> bool {
    return is_clever_message(source)
        && source.message
            .to_lowercase().as_str()
            .starts_with("deploy failed in");
}

fn is_build_sucess_message(source: &CCEventSource) -> bool {
    return source.message
        .to_lowercase().as_str()
        .starts_with("build succeeded in");
}

pub fn log(line: &str) {
    let deserialized: CCEventMessage = {
        match serde_json::from_str(line) {
            Ok(d) => d,
            Err(_) => return,
        }
    };

    let source = &deserialized.source;

    print!("{}: ", source.timestamp.as_str());

    let message = source.message.as_str();
    let mut terminal = term::stdout().unwrap();
    if is_deployment_success_message(source) {
        terminal.fg(term::color::GREEN).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        println!("{}", message);
    } else if is_deployment_failed_message(source) {
        terminal.fg(term::color::RED).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        println!("{}", message);
    } else if is_build_sucess_message(source) {
        terminal.fg(term::color::BLUE).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        println!("{}", message);
    } else {
        println!("{}", message);
    }
    terminal.reset().unwrap();
}
