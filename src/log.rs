use serde::{Deserialize};

#[derive(Deserialize)]
struct CCEventMessage {
    #[serde(rename = "_source")]
    source: CCEventSource,
}

#[derive(Deserialize)]
struct CCEventSource {
    #[serde(rename = "@timestamp")]
    timestamp: String,
    #[serde(rename = "@message")]
    message: String,
}

fn format_log_line(line: &str) -> Option<String> {
    let deserialized: CCEventMessage = {
        match serde_json::from_str(line) {
            Ok(d) => d,
            Err(_) => return None,
        }
    };
    let source = &deserialized.source;
    let (timestamp, message) = (source.timestamp.as_str(), source.message.as_str());

    Some(format!("{}: {}", timestamp, message))
}

pub fn log(line: &str) {
    if let Some(formatted) = format_log_line(line) {
        println!("{}", formatted);
    }
}
