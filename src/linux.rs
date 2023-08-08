
use std::process::{Command, Stdio};
use std::fs::OpenOptions;
pub fn exe_cmd_linux(command: String) -> Result<(), Box<dyn std::error::Error>> {
    let log_file = OpenOptions::new()
    .create(true)
    .write(true)
    .append(true)
    .open("child_process.log")
    .expect("Failed to open log file");
    // start child cmd
    Command::new("nohup")
        .args(&[&command])
        .stdout(Stdio::from(log_file.try_clone().unwrap()))
        .stderr(Stdio::from(log_file))
        .spawn()
        .expect("Failed to execute command");
    Ok(())
}
