use std::process::Command;

fn main() {
    // Trigger the Svelte build process
    let status = Command::new(r"C:\Program Files\nodejs\npm.cmd")
        .args(&["run", "build"])
        .current_dir("frontend")
        .status()
        .expect("Failed to execute npm build command");

    if !status.success() {
        panic!("Frontend build failed");
    }
}
