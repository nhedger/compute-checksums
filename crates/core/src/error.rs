use std::path::PathBuf;

use thiserror::Error;

/// Errors that can occur while collecting or hashing matched files.
#[derive(Debug, Error)]
pub enum ChecksumError {
    #[error("root path is not a directory: {path}")]
    InvalidRoot { path: PathBuf },

    #[error("invalid glob pattern `{pattern}`: {source}")]
    InvalidGlobPattern {
        pattern: String,
        #[source]
        source: globset::Error,
    },

    #[error("invalid glob matcher set for {kind}: {source}")]
    InvalidGlobSet {
        kind: &'static str,
        #[source]
        source: globset::Error,
    },

    #[error("failed to walk directory `{path}`: {source}")]
    WalkDirectory {
        path: PathBuf,
        #[source]
        source: walkdir::Error,
    },

    #[error("failed to strip root prefix `{root}` from `{path}`")]
    StripPrefix { root: PathBuf, path: PathBuf },

    #[error("failed to open file `{path}`: {source}")]
    OpenFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to read file `{path}`: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
