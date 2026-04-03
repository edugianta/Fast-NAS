use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {

    #[error("Unable to allocate folder for NAS file storage")]
    FileStorageCreationError,

    #[error("Unsupported OS for file system storage")]
    UnsupportedOSError
}