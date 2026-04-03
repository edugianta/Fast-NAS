use std::path::PathBuf;
use crate::errors;

pub fn determine_storage_point(operating_system:&str) -> Result<PathBuf, errors::StorageError> {
    if operating_system == "windows" {
        let mut full_storage_location = dirs::public_dir().unwrap();
        let storage_location = "/Nas Storage/";

        full_storage_location.push(storage_location);

        Ok(full_storage_location)
    } else if operating_system == "linux" {
        Ok(PathBuf::from("/srv/nas"))
    } else {
        Err(errors::StorageError::UnsupportedOSError)
    }

}