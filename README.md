# clever-cloud-logs

`clever-cloud-logs` is a [CLI](https://en.wikipedia.org/wiki/Command-line_interface) created to stream [Clever Cloud](https://www.clever-cloud.com/) app logs in a terminal.

## Introduction

> // TODO: Add Introduction

## Usage

```sh
clever-cloud-logs <APP_ID> --consumer_key <OAUTH_CONSUMER_KEY> --consumer_secret <OAUTH_CONSUMER_SECRET>
```

* `APP_ID` is your "Application ID", found in the [Console](https://console.clever-cloud.com/) header, or in the "Information" section of your app.
* `OAUTH_CONSUMER_KEY` is a [OAuth 1.0](https://oauth.net/core/1.0/) consumer key used for [Clever Cloud API](https://www.clever-cloud.com/doc/clever-cloud-apis/cc-api/) authentication. For more information, visit the ["Authentication" section of the docs](https://www.clever-cloud.com/doc/clever-cloud-apis/cc-api/#-create-consumers-tokens-).
* `OAUTH_CONSUMER_SECRET` is a [OAuth 1.0](https://oauth.net/core/1.0/) consumer secret used for [Clever Cloud API](https://www.clever-cloud.com/doc/clever-cloud-apis/cc-api/) authentication. For more information, visit the ["Authentication" section of the docs](https://www.clever-cloud.com/doc/clever-cloud-apis/cc-api/#-create-consumers-tokens-).

## Requirements

* [Rust](https://www.rust-lang.org/) (developed and tested with [Rust 1.47.0 stable](https://blog.rust-lang.org/2020/10/08/Rust-1.47.html))

## Installation

```sh
git pull https://github.com/RemiBardon/clever-cloud-logs
cargo run -- <APP_ID> --consumer_key <OAUTH_CONSUMER_KEY> --consumer_secret <OAUTH_CONSUMER_SECRET>
```

> All of the trailing arguments are passed to the binary to run. If you're passing arguments to both Cargo and the binary, the ones after `--` go to the binary, the ones before go to Cargo.
