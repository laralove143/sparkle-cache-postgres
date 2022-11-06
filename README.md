# Sparkle Cache Postgres

[GitHub](https://github.com/laralove143/sparkle-cache-postgres)
[crates.io](https://crates.io/crates/sparkle-cache-postgres)
[docs.rs](https://docs.rs/sparkle-cache-postgres/latest)

Discord cache implementation using PostgresSQL for the Twilight ecosystem

It provides a cache struct that
implements [Sparkle Cache](https://github.com/laralove143/sparkle-cache)'s traits and
uses [SQLx](https://github.com/launchbadge/sqlx) as the backend driver

It also provides access to the cache's backend, meaning you can use the same database
for your custom data

## Compile-time verification

To use [SQLx's compile time verification](https://github.com/launchbadge/sqlx#compile-time-verification), set
the `DATABASE_URL` environment variable to the URL of your database, for
example, `DATABASE_URL=postgresql://localhost:5432/sparkle`

## Stability

This is a new, relatively immature library. It passes Sparkle Cache's tests, but of course there may be points not
covered by them. If there's a bug directly related to the PostgresSQL implementation, please create an issue

Sparkle Cache's tests currently don't cover stickers because
of [a bug in Twilight](https://github.com/twilight-rs/twilight/issues/1954)

The library casts unsigned integers except for IDs internally, so please don't use
integer-based functions such as `SUM` or equality/size checks on them in SQL,
instead query the data and use Rust equivalents

## Features

### TLS Backend

- `rustls` (Default): Enables the `runtime-tokio-rustls` feature of SQLx
- `native-tls`: Enables the `runtime-tokio-native-tls` feature of SQLx

Only one of these features can be selected