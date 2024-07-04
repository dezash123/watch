use thiserror_no_std::Error;

use crate::os::kernel::interfaces::CommunicationError;

#[derive(Debug, Error)]
pub enum ConfigurationError {
    AlreadyApplied,
}

#[derive(Debug, Error)]
pub enum DeviceError {
    Setup(#[from] SetupError),
    Communication(#[from] CommunicationError),
    Configuration(#[from] ConfigurationError),
}

#[derive(Debug, Error)]
pub enum SetupError {
    AlreadyEnabled,
    NoDisable,
    AlreadyDisabled,
}
