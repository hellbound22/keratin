#[derive(Clone, Debug)]
pub enum Errors {
    AlreadyExists,
    EntryNotFound,
    DocumentWritingError,
    DbConfigurationError
}
