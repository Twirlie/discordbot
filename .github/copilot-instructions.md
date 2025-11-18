# Copilot / AI agent instructions for this repo

Keep guidance short and actionable — this file contains the minimal, repository-specific facts an AI coding agent needs to be productive.

Project summary

- Small Discord bot written in Rust using `poise` (on top of `serenity`) with command handlers in `src/commands.rs` and startup/DI in `src/main.rs`.
- Uses `rusqlite` (bundled) to persist command history in `history.db` and `CodenameData.json` to generate random codenames.

Big-picture architecture

- Entry point: `src/main.rs` builds a `poise::Framework<BotState, Error>` and a `serenity::Client`.
- Shared state: `BotState` stores only `db_path: String` (not a `rusqlite::Connection`) because `rusqlite::Connection` is not `Sync`.
- DB handling: `db_setup()` creates the `command_history` table. `log_command_usage()` opens a `Connection` per-call inside `tokio::task::spawn_blocking` and writes a row.
- Commands: `src/commands.rs` contains slash/prefix commands (e.g. `register`, `age`, `codename`). A helper `send_and_log(ctx, response)` is used to send messages and record them in the DB.

Key files to inspect when changing behavior

- `src/main.rs`: framework options, `setup()`, DB schema creation, `post_command` hook.
- `src/commands.rs`: command implementations, `get_codenamedata()`, `generate_codename()`, and the `send_and_log` helper.
- `CodenameData.json`: source data for `codename` command.
- `Cargo.toml`: dependency list (poise, serenity, rusqlite, tokio, dotenvy, rand, chrono, colored).

Developer workflows & quick commands

- Build: `cargo build` — verifies compilation and proc-macro expansion compatibility.
- Run locally: set `DISCORD_TOKEN` in environment (e.g. with `.env` and `dotenvy`), then `cargo run`.
- Tests: `cargo test` runs unit tests in `src/commands.rs` that read `CodenameData.json`.
- Editor: rust-analyzer sometimes shows proc-macro metadata-version errors. Fixes: update rust toolchain (`rustup update`) and rust-analyzer extension, or disable proc-macro expansion with `rust-analyzer.procMacro.enable: false` as a temporary workaround.

Project-specific conventions & patterns

- Do NOT store a global `rusqlite::Connection` in `BotState` — use `db_path` and open a connection inside `spawn_blocking` for each DB write. This is intentional to avoid `RefCell`/`!Sync` issues.
- Centralize message sending + logging via `send_and_log(ctx, response)` in `src/commands.rs`. Prefer this helper over mixing direct `ctx.say(...)` + separate logging calls.
- Keep all command functions consistent in their signature. The framework expects command handlers to have compatible concrete types — prefer returning `Result<(), Error>` and using `send_and_log` for messages.

How to add a new command (example)

1. Add a function in `src/commands.rs` with the `#[poise::command(...)]` attribute and signature `async fn foo(ctx: Context<'_>, ...) -> Result<(), Error>`.
2. Compose a `response: String`, call `send_and_log(ctx, response).await?;` and `Ok(())`.
3. Register the command in `src/main.rs` inside the `commands: vec![ ... ]` list.

Examples (canonical patterns)

- send and log helper (already present):
  - `send_and_log(ctx, response).await?;`
- log helper (crate-local) in `src/main.rs`:
  - `log_command_usage(&data.db_path, &ctx, &command_name, &response).await;`

Integration points

- Discord: Discord bot token via `DISCORD_TOKEN` env var; uses `serenity` GatewayIntents set in `main.rs`.
- SQLite: `rusqlite` with `bundled` feature — DB file is `history.db` in repo working dir.
- JSON asset: `CodenameData.json` read at runtime by `get_codenamedata()`.

Debugging tips for AI agents

- If you modify DB code and see `!Sync` errors, remember to avoid placing `Connection` into shared state; use `spawn_blocking` or a single background writer task.
- If rust-analyzer proc-macro errors appear after dependency changes, run `cargo build` in the workspace to ensure toolchain compatibility and to surface the real compiler errors.
- To inspect runtime logging, run the bot locally with `cargo run` and watch console output (messages are colorized with `colored`).

If you need to change logging semantics, prefer introducing a small wrapper (e.g., background writer using `tokio::mpsc`) rather than making `rusqlite::Connection` shared across threads.

Questions for the maintainer

Questions for the maintainer (answered)

- Background DB writer: Not necessary now — keep `send_and_log`.
  - Rationale: current bot is low-traffic and the per-call `Connection::open` inside `spawn_blocking` is simple, robust, and avoids `!Sync` issues from sharing a `rusqlite::Connection` in shared state.
  - When to revisit: implement a background writer (tokio mpsc + single `Connection`) if the bot's throughput increases or profiling shows DB open/close becomes a bottleneck.
- Command return style: Keep `send_and_log` and `Result<(), Error>` command signatures for now.
  - Rationale: this avoids poise macro return-type inference issues and centralizes sending+logging via the helper.

If anything above is unclear or you want the file expanded with more examples, say which area to expand.
