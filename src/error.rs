use snafu::Snafu;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Environment variable '{key}' is required but not found"))]
    MissingEnvVar { key: String },

    #[snafu(display(
        "Unsupported storage provider: {provider}. Allowed: 'oss' | 's3' | 'minio' | 'fs'"
    ))]
    UnsupportedProvider { provider: String },

    #[snafu(display("Path does not exist: {}", path.display()))]
    PathNotFound { path: PathBuf },

    #[snafu(display("Invalid path: {path}"))]
    InvalidPath { path: String },

    #[snafu(display("Cannot delete directory without -R flag: {path}"))]
    DirectoryDeletionNotRecursive { path: String },

    #[snafu(display("Use -R to upload directories"))]
    DirectoryUploadNotRecursive,

    #[snafu(display("Partial deletion failure: {} path(s) failed to delete", failed_paths.len()))]
    PartialDeletion { failed_paths: Vec<String> },

    #[snafu(display("Failed to delete '{paths}' (recursive: {recursive}): {source}"))]
    DeleteFailed {
        paths: String,
        recursive: bool,
        source: Box<Error>,
    },

    #[snafu(display("Failed to download '{remote_path}' to '{local_path}': {source}"))]
    DownloadFailed {
        remote_path: String,
        local_path: String,
        source: Box<Error>,
    },

    #[snafu(display("Failed to upload '{local_path}' to '{remote_path}': {source}"))]
    UploadFailed {
        local_path: String,
        remote_path: String,
        source: Box<Error>,
    },

    #[snafu(display("Failed to copy '{src_path}' to '{dest_path}': {source}"))]
    CopyFailed {
        src_path: String,
        dest_path: String,
        source: Box<Error>,
    },

    #[snafu(display("Failed to move '{src_path}' to '{dest_path}': {source}"))]
    MoveFailed {
        src_path: String,
        dest_path: String,
        source: Box<Error>,
    },

    #[snafu(display("Failed to list directory '{path}': {source}"))]
    ListDirectoryFailed { path: String, source: Box<Error> },

    #[snafu(display("Failed to get disk usage for '{path}': {source}"))]
    DiskUsageFailed { path: String, source: Box<Error> },

    #[snafu(display("Failed to create directory '{path}': {source}"))]
    DirectoryCreationFailed { path: String, source: Box<Error> },

    #[snafu(display("Failed to read file '{path}': {source}"))]
    CatFailed { path: String, source: Box<Error> },

    #[snafu(display("OpenDAL error: {source}"))]
    OpenDal { source: opendal::Error },

    #[snafu(display("IO error: {source}"))]
    Io { source: std::io::Error },

    #[snafu(display("JSON serialization error: {source}"))]
    Json { source: serde_json::Error },
}

impl From<opendal::Error> for Error {
    fn from(error: opendal::Error) -> Self {
        Error::OpenDal { source: error }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io { source: error }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json { source: error }
    }
}
