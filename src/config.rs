// Error Handling
use anyhow::{Result, Error};

// File reading & JSON parsing
use directories::UserDirs;
use std::io::prelude::*;
use std::fs::File;
use serde_json::Value;

/* === Public Functions === */

pub fn clever_config() -> Result<(String, String)> {
    let config = parsed_config()?;

    // Get OAuth1 Access Token
    let access_token: String;
    let token_key = "token";
    if let Value::String(token) = &config[token_key] {
        access_token = token.clone();
    } else {
        return Err(Error::msg(format!("No value '{}' found in '~/.config/clever-cloud'. Please follow instructions at https://github.com/CleverCloud/clever-tools to install 'clever-tools', then run `clever login`.", token_key)))
    }
    
    // Get OAuth1 API Secret
    let token_secret: String;
    let secret_key = "secret";
    if let Value::String(secret) = &config[secret_key] {
        token_secret = secret.clone();
    } else {
        return Err(Error::msg(format!("No value '{}' found in '~/.config/clever-cloud'. Please follow instructions at https://github.com/CleverCloud/clever-tools to install 'clever-tools', then run `clever login`.", secret_key)))
    }

    Ok((access_token, token_secret))
}

/* === Private Functions === */

fn parsed_config() -> Result<serde_json::Value> {
    // Find user's home directory
    let user_dirs = UserDirs::new().ok_or(Error::msg("Could not find user home directory"))?;
    let config_file_path = format!("{}/.config/clever-cloud", user_dirs.home_dir().display());

    // Open config file
    let mut config_file = File::open(config_file_path)
        .map_err(|_| Error::msg("No file found at '~/.config/clever-cloud'. Please follow instructions at https://github.com/CleverCloud/clever-tools to install 'clever-tools', then run `clever login`."))?;
    
    // Read file into a JSON string
    let mut json_config = String::new();
    config_file.read_to_string(&mut json_config)
        .map_err(|e| Error::msg(format!("Unable to read the file at '~/.config/clever-cloud': {}", e)))?;

    // Parse the JSON string into serde_json::Value
    let config = serde_json::from_str(&json_config)
        .map_err(|e| Error::msg(format!("Invalid content in file '~/.config/clever-cloud': {}",e)))?;

    // Return value
    Ok(config)
}
