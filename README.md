# Sparkle Cache Postgres

Discord cache implementation using PostgresSQL for the Twilight ecosystem

It provides a cache struct that
implements [Sparkle Cache](https://github.com/laralove143/sparke-cache)'s traits and
uses [SQLx](https://github.com/launchbadge/sqlx) as the backend driver

It also provides access to the cache's backend, meaning you can use the same database
for your custom data

## Stability

This is a new library, meaning there will be bugs. Tests for Sparkle Cache are WIP,
if there's a bug directly related to the PostgresSQL implementation,
please create an issue

The library casts unsigned integers except for IDs internally, so please don't use
integer-based functions such as `SUM` or equality/size checks on them in SQL,
instead query the data and use Rust equivalents

## Features

### TLS Backend

- `rustls` (Default): Enables the `runtime-tokio-rustls` feature of SQLx
- `native-tls`: Enables the `runtime-tokio-native-tls` feature of SQLx

When they're both enabled, `native-tls` takes precedence, meaning you don't have to
disable the default features. One of these features is required