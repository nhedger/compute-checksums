use std::{fmt, str::FromStr};

use thiserror::Error;

/// Hashing algorithms supported by the checksum engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

impl HashAlgorithm {
    /// Returns the canonical lowercase identifier for this algorithm.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Md5 => "md5",
            Self::Sha1 => "sha1",
            Self::Sha256 => "sha256",
            Self::Sha512 => "sha512",
        }
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for HashAlgorithm {
    type Err = ParseHashAlgorithmError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let normalized = raw.trim().to_ascii_lowercase();

        match normalized.as_str() {
            "md5" => Ok(Self::Md5),
            "sha1" => Ok(Self::Sha1),
            "sha256" => Ok(Self::Sha256),
            "sha512" => Ok(Self::Sha512),
            _ => Err(ParseHashAlgorithmError {
                algorithm: raw.to_owned(),
            }),
        }
    }
}

/// Error returned when parsing an unsupported hashing algorithm.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("unsupported hash algorithm `{algorithm}`")]
pub struct ParseHashAlgorithmError {
    algorithm: String,
}
