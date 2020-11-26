use caps::{CapSet, Capability};

pub use crate::errors::ExResult;

mod cmd;
mod device;
mod errors;
pub mod tap;

pub fn set_ambient_cap() -> ExResult<()> {
    caps::raise(None, CapSet::Inheritable, Capability::CAP_NET_ADMIN)?;
    caps::raise(None, CapSet::Ambient, Capability::CAP_NET_ADMIN)?;

    Ok(())
}
