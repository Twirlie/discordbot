# GitHub Workflows Documentation

This directory contains CI/CD workflows for the Discord bot project. Both workflows are designed to build and test a Rust backend with an integrated Svelte frontend.

## Workflows Overview

### `rust-tests.yml` - Continuous Integration (CI)

Triggers on: `push` to `master` and `pull_request` against `master`

**Purpose:** Run tests and generate coverage reports for every commit and pull request.

**Key Steps:**
1. Checkout code
2. Install Rust (stable)
3. Install Node.js (LTS)
4. Install pnpm v8
5. Install frontend dependencies (`pnpm install --filter frontend`) - **required for Svelte build**
6. Build and run tests (`cargo test --verbose`)
7. Generate coverage reports (`cargo tarpaulin`)

**Important Notes:**
- Frontend dependencies MUST be installed before `cargo build`/`cargo test`, as `build.rs` calls `npm run build` (via pnpm)
- The `build.rs` script is cross-platform and uses `npm` command (which resolves to `npm.cmd` on Windows or `npm` on Unix)
- Coverage step uses `cargo-tarpaulin` which may fail in Docker environments due to ASLR permission limitations - this is **not** a real failure and will work on actual GitHub runners

**Files Modified During Build:**
- `frontend/build/` - Svelte output (generated, not committed)
- `target/` - Rust build artifacts
- `coverage/` - Coverage reports (XML and HTML)

### `release.yml` - Release Build

Triggers on: `push` with tags matching `v*.*.*` (semantic versioning)

**Purpose:** Build optimized release binary, package assets, and create GitHub release with downloadable archive.

**Key Steps:**
1. Checkout code
2. Install Rust (stable)
3. Install Node.js (LTS)
4. Install pnpm v8
5. Install frontend dependencies (`pnpm install --filter frontend`) - **required for Svelte build**
6. Build release binary (`cargo build --release`)
7. Prepare release directory (copy `target/release/` and `assets/`)
8. Create zip archive
9. Upload to GitHub release

**Important Notes:**
- Frontend dependencies MUST be installed before `cargo build --release`
- The release build is optimized (`--release` flag) and will take longer than debug builds
- Upload step requires GitHub token (automatically provided by GitHub Actions, but may fail in local testing with `act`)
- The archive includes compiled binaries, assets, and fingerprints

**Files Modified During Build:**
- `target/release/` - Optimized Rust build artifacts
- Generated `.zip` archive ready for distribution

## Frontend Build Integration

### Why Frontend Dependencies Are Required

The project uses a Svelte frontend that must be built as part of the Rust build process:

1. **`build.rs`** runs during `cargo build`/`cargo test`/`cargo build --release`
2. **`build.rs`** executes `npm run build` (resolved via `pnpm`)
3. **`pnpm`** needs `frontend/node_modules` to be populated
4. Without frontend dependencies, the build fails with "vite: not found"

**Dependency Installation:**
```bash
pnpm install --filter frontend
```

This uses pnpm's workspace feature to install only the `frontend` package dependencies (not the entire monorepo if there were multiple packages).

## Cross-Platform Compatibility

- **Windows:** `build.rs` detects OS and uses `npm.cmd`
- **Linux/macOS:** `build.rs` uses `npm`
- Workflows run on `ubuntu-latest`, so `npm` is used (no `.cmd` suffix needed)

## Limitations & Known Issues

### Tarpaulin Coverage (CI Workflow)

**Issue:** The `Run coverage` step may fail with `ASLR disable failed: EPERM: Operation not permitted`

**Why:** Docker containers don't have permissions to disable ASLR (Address Space Layout Randomization), which `tarpaulin` requires to accurately measure code coverage.

**Impact:** 
- Local testing with `act` may fail at this step
- Actual GitHub Actions runners DO have the necessary permissions and coverage will work

**Workaround:** 
- Skip coverage locally: Use `cargo test` without tarpaulin
- Coverage will be generated successfully in actual GitHub Actions CI

### Release Upload (Release Workflow)

**Issue:** Upload step may fail locally when testing with `act` because GitHub token and release context aren't available in the local Docker environment.

**Why:** The `softprops/action-gh-release@v2` action requires:
- A valid GitHub token (`GITHUB_TOKEN`)
- A GitHub release context (only available when pushing tags)

**Impact:** 
- Local testing with `act` may fail at upload step
- Actual GitHub Actions will succeed because the token and context are automatically available

**Workaround:** 
- Don't test release workflow locally; verify it when you push an actual release tag to GitHub

## Maintenance Guidelines

### Adding New Dependencies

If you add dependencies:
1. Update `frontend/package.json` - triggers pnpm install
2. Update `Cargo.toml` - triggers Rust dependency download
3. Both workflows will automatically pick up changes on next run

### Modifying Build Script

The `build.rs` script is critical. If modifying:
1. Ensure it remains cross-platform (Windows/Unix)
2. Keep `current_dir("frontend")` so it runs from the frontend directory
3. Test locally: `cargo build` and `cargo build --release`
4. Verify with `act push` before committing

### Updating Node/pnpm Versions

Workflows specify fixed versions:
- **Node.js:** `lts/*` (latest LTS) - consider pinning to specific version like `20.*.*` if needed
- **pnpm:** `version: 8` - update when upgrading pnpm major version

To update:
1. Edit version in both workflow files
2. Test with `act push` locally
3. Commit and push

## Testing Workflows Locally

Use `act` to simulate GitHub Actions locally:

```bash
# Requires Docker
# Install: https://github.com/nektos/act

# Test both workflows (push event)
act push --rm

# Test release workflow (push tag event)
# Note: You'll need to modify event context, or just verify on GitHub

# Clean up containers after testing
act push --rm
```

**Expected Behavior:**
- Both build and test steps should succeed
- Coverage step may fail (ASLR limitation in Docker) - ignore this
- Upload step may fail (no token) - ignore this; will work on GitHub

## Environment Variables

Both workflows set:
- `CARGO_TERM_COLOR: always` - Colored output in logs

Local testing can override via:
```bash
DISCORD_TOKEN=your_token cargo build
```

The bot token is needed for tests that interact with Discord (check `tests/` for conditional bot-token tests).
