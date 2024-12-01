use std::process::{Command, Stdio};

use crate::template::Day;

/// # Panics
///
/// Will panic if the Cargo command spawned by this function fails to start or
/// panics itself during execution.
pub fn handle(day: Day, release: bool, dhat: bool, submit_part: Option<u8>) {
    let mut cmd_args = vec!["run".to_string(), "--bin".to_string(), day.to_string()];

    if dhat {
        cmd_args.extend([
            "--profile".to_string(),
            "dhat".to_string(),
            "--features".to_string(),
            "dhat-heap".to_string(),
        ]);
    } else if release {
        cmd_args.push("--release".to_string());
    }

    cmd_args.push("--".to_string());

    if let Some(submit_part) = submit_part {
        cmd_args.push("--submit".to_string());
        cmd_args.push(submit_part.to_string());
    }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
