#[allow(dead_code)]
pub fn shell_exec(command: &str) -> String {
    let output = std::process::Command::new("/bin/bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    let output = String::from_utf8_lossy(&output.stdout);
    output.to_string()
}
