use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;
use std::process::{Command, Stdio};

use crate::Result;

pub fn process_status(status: ExitStatus, cmd: &[&str]) -> Result<()> {
    if !status.success() {
        let reason: String;

        if let Some(code) = status.code() {
            reason = format!("with return code {}", code);
        } else {
            let signal = status.signal().expect(
                "process terminated not with retutn code nor by a signal",
            );
            reason = format!("by signal {}", signal);
        }

        let msg = format!("command {} was failded {}", cmd.join(" "), reason);
        return Err(msg.into());
    }

    Ok(())
}

pub fn check_call(cmd: &[&str]) -> Result<()> {
    let status = Command::new(cmd[0])
        .args(&cmd[1..])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    process_status(status, &cmd)
}

pub fn get_output(cmd: &[&str]) -> Result<String> {
    let output = Command::new(cmd[0]).args(&cmd[1..]).output()?;

    process_status(output.status, &cmd)?;

    let stdout = String::from_utf8(output.stdout)?;
    return Ok(stdout);
}

pub fn get_code(cmd: &[&str]) -> Result<i32> {
    let output = Command::new(cmd[0]).args(&cmd[1..]).output()?;

    if let Some(code) = output.status.code() {
        return Ok(code);
    } else {
        let signal = output
            .status
            .signal()
            .expect("process terminated not with retutn code nor by a signal");
        let msg = format!(
            "command {} terminated by signal {}",
            cmd.join(" "),
            signal
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            msg,
        )));
    }
}
