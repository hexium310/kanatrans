#[cfg(feature = "cli")]
pub(crate) mod cli;
#[allow(clippy::module_inception)]
pub(crate) mod command;
#[cfg(feature = "server")]
pub(crate) mod serve;

pub(crate) use command::*;
