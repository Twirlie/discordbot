use std::process::Command;

fn main() {
    let npm = if cfg!(windows) { "npm.cmd" } else { "npm" }; // Handle Windows vs Unix-like systems
    // Trigger the Svelte build process
    let status = Command::new(npm)
        .args(["run", "build"])
        .current_dir("frontend")
        .status()
        .expect("Failed to execute npm build command");

    if !status.success() {
        panic!("Frontend build failed");
    }
}
