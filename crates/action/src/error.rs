use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ActionError {
    #[error("missing required input `patterns`")]
    MissingPatterns,

    #[error("input `patterns` must contain at least one non-empty pattern")]
    EmptyPatterns,

    #[error("invalid value for input `follow_symlinks`: `{value}`")]
    InvalidFollowSymlinks { value: String },

    #[error(
        "invalid value for input `algorithms`: `{value}`; supported values: md5, sha1, sha256, sha512"
    )]
    InvalidAlgorithm { value: String },

    #[error(transparent)]
    Core(#[from] checksum_core::ChecksumError),

    #[error(transparent)]
    Serialize(#[from] serde_json::Error),

    #[error("GITHUB_OUTPUT is not set")]
    MissingGithubOutput,

    #[error("failed to open GITHUB_OUTPUT file `{path}`: {source}")]
    OpenGithubOutput { path: PathBuf, source: io::Error },

    #[error("failed to write output `{key}`: {source}")]
    WriteOutput { key: String, source: io::Error },
}
