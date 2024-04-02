use thiserror::Error;


#[derive(Error, Clone, Debug)]
pub enum Errors {
    #[error("Entry already exists")]
    AlreadyExists,
    #[error("Could not find entry in FS or Cache")]
    EntryNotFound,
    #[error("Could not write entry to FS")]
    DocumentWritingError,
    #[error("Could not finish DB configuration")]
    DbConfigurationError
}
