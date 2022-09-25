# Twilight Cache Postgres

Discord cache implementation using PostgresSQL for the Twilight ecosystem

It provides a cache struct that
implements [Twilight Cache with Any Backend](https://github.com/laralove143/twilight-cache-any-backed)'s traits and
uses [SQLx](https://github.com/launchbadge/sqlx) as the backend driver

## Features

### TLS Backend

- `rustls` (Default): Enables the `runtime-tokio-rustls` feature of SQLx
- `native-tls`: Enables the `runtime-tokio-native-tls` feature of SQLx

When they're both enabled, `native-tls` takes precedence, meaning you don't have to disable the default features. One of
these features is required