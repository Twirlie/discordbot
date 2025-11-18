# Discord Bot

* This Rust program implements a Discord bot using the `poise` framework and `serenity` library.
* It connects to Discord, registers slash commands, and logs command usage into a SQLite database.
* It includes commands to register application commands, display user account age, and generate random codenames.
* It uses the `rusqlite` crate for SQLite interactions and `dotenvy` for environment variable management.

## build, run, watch

* for reruning during development `cargo watch -x run`
* for running manually `cargo run`
* for building `cargo build`

## features

### command

commands are implemented as slash commands.

1) `/codename` generates a random codename
2) `/age` *temporary* returns age of your discord account
3) `/register` *admin use* manually register slash commands
