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
