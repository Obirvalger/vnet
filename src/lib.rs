use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;
use std::process::{Command, Stdio};

use caps::{CapSet, Capability};

pub type ExResult<T> = Result<T, Box<dyn std::error::Error + 'static>>;

fn prefix_name(name: &str) -> String {
    format!("vnet_{}", name)
}

pub fn set_ambient_cap() -> ExResult<()> {
    caps::raise(None, CapSet::Inheritable, Capability::CAP_NET_ADMIN)?;
    caps::raise(None, CapSet::Ambient, Capability::CAP_NET_ADMIN)?;

    Ok(())
}

fn exists_tap(name: &str) -> bool {
    Command::new("ip")
        .args(&["link", "show", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("ip link show failed")
        .success()
}

fn add_tap(name: &str) -> ExResult<()> {
    let err_msg = "ip tuntap add failed";

    let status = Command::new("ip")
        .args(&["tuntap", "add", name, "mode", "tap"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err(err_msg.into());
    }

    Ok(())
}

fn del_tap(name: &str) -> ExResult<()> {
    let err_msg = "ip tuntap del failed";

    let status = Command::new("ip")
        .args(&["tuntap", "del", name, "mode", "tap"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err(err_msg.into());
    }

    Ok(())
}

pub fn create_tap(raw_name: &str) -> ExResult<Option<String>> {
    let name = prefix_name(raw_name);
    if !exists_tap(&name) {
        add_tap(&name)?;
        return Ok(Some(name));
    }

    Ok(None)
}

pub fn remove_tap(raw_name: &str) -> ExResult<Option<String>> {
    let name = prefix_name(raw_name);
    if exists_tap(&name) {
        del_tap(&name)?;
        return Ok(Some(name));
    }

    Ok(None)
}

fn process_status(status: ExitStatus, cmd: &[&str]) -> ExResult<()> {
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

fn check_call(cmd: &[&str]) -> ExResult<()> {
    let status = Command::new(cmd[0])
        .args(&cmd[1..])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    process_status(status, &cmd)
}

fn get_stdout(cmd: &[&str]) -> ExResult<String> {
    let output = Command::new(cmd[0]).args(&cmd[1..]).output()?;

    process_status(output.status, &cmd)?;

    let stdout = String::from_utf8(output.stdout)?;
    return Ok(stdout);
}

fn exists_address(name: &str, address: &str) -> ExResult<bool> {
    let stdout = get_stdout(&["ip", "-o", "address", "show", "dev", &name])?;
    Ok(stdout.contains(address))
}

pub fn add_address_tap(
    raw_name: &str,
    address: &str,
) -> ExResult<Option<String>> {
    let name = prefix_name(raw_name);

    if exists_address(&name, address)? {
        Ok(None)
    } else {
        check_call(&["ip", "address", "add", address, "dev", &name])?;
        Ok(Some(String::from(address)))
    }
}

pub fn del_address_tap(
    raw_name: &str,
    address: &str,
) -> ExResult<Option<String>> {
    let name = prefix_name(raw_name);

    if !exists_address(&name, address)? {
        Ok(None)
    } else {
        check_call(&["ip", "address", "del", address, "dev", &name])?;
        Ok(Some(String::from(address)))
    }
}
