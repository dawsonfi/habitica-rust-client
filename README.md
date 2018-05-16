[![Build Status](https://travis-ci.org/dawsonfi/habitica-rust-client.svg?branch=master)](https://travis-ci.org/dawsonfi/habitica-rust-client)
[![Current Crates.io Version](https://img.shields.io/crates/v/habitica_rust_client.svg)](https://crates.io/crates/habitica_rust_client)
[![Docs.rs](https://docs.rs/habitica_rust_client/badge.svg)](https://docs.rs/habitica_rust_client/)
[![Code Coverage](https://codecov.io/github/dawsonfi/habitica-rust-client/coverage.svg?branch=master)](https://codecov.io/github/dawsonfi/habitica-rust-client)


# Habitica Api Rust Client

This is a unnoficial [Habitica V3 Api](https://habitica.com/apidoc) Client for Rust.

Feel free to use, open an issue or a PR.

## Supported Operations

### List user tasks

Method: `client.get_all_tasks()`

Reference: [Task - Get a user's tasks](https://habitica.com/apidoc/#api-Task-GetUserTasks)

## Usage
In order to use the api, you will need an active account on [Habitica](https://habitica.com/), with that, get the `user_id` and `api_token` from the [Api Configurations Page](https://habitica.com/user/settings/api).

With the `user_id` and `api_token` create a new instance of `ApiCredentials` with the following command:

`ApiCredentials::new(user_id, api_token)`

Having created the credentials, you can create the `HabiticaClient`:

`HabiticaClient::new(api_credentials)`

And then use it to call the supported api methods:

`habitica_client.get_all_tasks()`

## Examples

```
extern crate habitica_rust_client;

use habitica_rust_client::task::api_credentials::ApiCredentials;
use habitica_rust_client::task::habitica_client::HabiticaClient;

pub fn main() {
    let user_id: String = "you_user_id".to_string();
    let api_token: String = "you_api_token".to_string();

    let api_credentials = ApiCredentials::new(user_id, api_token);
    let habitica_client = HabiticaClient::new(api_credentials);

    let tasks = habitica_client.get_all_tasks();

    print("{:?}", tasks);
}

```
