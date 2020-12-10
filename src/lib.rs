use std::error::Error;
use std::result::Result as StdResult;

use caps::{CapSet, Capability};

mod cmd;
mod device;
pub mod tap;

pub type Result<T> = StdResult<T, Box<dyn Error + 'static>>;

pub fn set_ambient_cap() -> Result<()> {
    caps::raise(None, CapSet::Inheritable, Capability::CAP_NET_ADMIN)?;
    caps::raise(None, CapSet::Ambient, Capability::CAP_NET_ADMIN)?;

    Ok(())
}
