use std::process::{Command, Stdio};

use crate::cmd;
use crate::device::prefix_name;
use crate::errors::ExResult;

pub fn exists_tap(name: &str) -> bool {
    Command::new("ip")
        .args(&["link", "show", name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("ip link show failed")
        .success()
}

pub fn create_tap(raw_name: &str) -> ExResult<Option<String>> {
    let name = prefix_name(raw_name);
    if !exists_tap(&name) {
        cmd::check_call(&["ip", "tuntap", "add", &name, "mode", "tap"])?;
        return Ok(Some(name));
    }

    Ok(None)
}

pub fn remove_tap(raw_name: &str) -> ExResult<Option<String>> {
    let name = prefix_name(raw_name);
    if exists_tap(&name) {
        cmd::check_call(&["ip", "tuntap", "del", &name, "mode", "tap"])?;
        return Ok(Some(name));
    }

    Ok(None)
}

fn exists_address(name: &str, address: &str) -> ExResult<bool> {
    let stdout =
        cmd::get_output(&["ip", "-o", "address", "show", "dev", &name])?;
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
        cmd::check_call(&["ip", "address", "add", address, "dev", &name])?;
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
        cmd::check_call(&["ip", "address", "del", address, "dev", &name])?;
        Ok(Some(String::from(address)))
    }
}
