# Discord Bot

* This Rust program implements a Discord bot using the `poise` framework and `serenity` library.
* connects to Discord, registers slash commands, and logs command usage into a SQLite database.
* includes commands to register application commands, display user account age, and generate random codenames.
* uses the `rusqlite` crate for SQLite interactions and `dotenvy` for environment variable management.
* has a web frontend built in Svelte for displaying command usage in realtime

## build, run, watch, test

* for reruning during development `cargo watch -x run`
* for running manually `cargo run`
* for building `cargo build`
* for testing `cargo test`
  * with tarpaulin: `cargo tarpaulin` or for and html file `cargo tarpaulin --out Html`

## features

### command

commands are implemented as slash commands.

1) `/codename` generates a random codename
2) `/register` *admin use* manually register slash commands
