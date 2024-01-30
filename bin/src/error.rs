use thiserror::Error;
use rayhunter::diag_device::DiagDeviceError;

use crate::qmdl_store::QmdlStoreError;

#[derive(Error, Debug)]
pub enum RayhunterError{
    #[error("Config file parsing error: {0}")]
    ConfigFileParsingError(#[from] toml::de::Error),
    #[error("Diag intialization error: {0}")]
    DiagInitError(DiagDeviceError),
    #[error("Diag read error: {0}")]
    DiagReadError(DiagDeviceError),
    #[error("Tokio error: {0}")]
    TokioError(#[from] tokio::io::Error),
    #[error("QmdlStore error: {0}")]
    QmdlStoreError(#[from] QmdlStoreError),
    #[error("No QMDL store found at path {0}, but can't create a new one due to readonly mode")]
    NoStoreReadonlyMode(String),
}
