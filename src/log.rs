// Error Handling
use anyhow::{Result, Error};

// JSON Parsing
use serde::{Deserialize};

// Terminal Formatting
extern crate term;

/* === Public Functions === */

pub fn log(line: &str) -> Result<()> {
    // Skip log if heartbeat
    if line.contains("\"type\":\"heartbeat\"") { return Ok(()) }

    // Deserialize JSON data
    let deserialized: CCEventMessage = serde_json::from_str(line)
        .map_err(|e| Error::msg(format!("Could not deserialize JSON from {}: {}", line, e)))?;

    // Get "_source" field
    let source = &deserialized.source;

    // Print timestamp
    print!("{}: ", source.timestamp);

    // Set terminal format according to emphasis
    let mut terminal = term::stdout().ok_or(Error::msg("Could not get terminal stdout"))?;
    if let Some((fg, attr)) = get_term_emphasis(source) {
        terminal.fg(fg)
            .map_err(|e| Error::msg(format!("Could not set terminal foreground: {}", e)))?;
        terminal.attr(attr)
            .map_err(|e| Error::msg(format!("Could not set terminal attribute (bold...): {}", e)))?;
    }

    // Print message
    println!("{}", source.message);

    // Reset terminal format
    terminal.reset()
        .map_err(|e| Error::msg(format!("Could not reset terminal format: {}", e)))?;

    Ok(())
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
