use crate::HashAlgorithm;

/// Options that control checksum computation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumOptions {
    /// Algorithms to compute for each matched file.
    pub algorithms: Vec<HashAlgorithm>,
    /// Whether symbolic links should be followed during traversal.
    pub follow_symlinks: bool,
}

impl ChecksumOptions {
    /// Returns a normalized algorithm list with defaults and deduplication applied.
    #[must_use]
    pub fn normalized_algorithms(&self) -> Vec<HashAlgorithm> {
        let input = if self.algorithms.is_empty() {
            vec![HashAlgorithm::Sha256]
        } else {
            self.algorithms.clone()
        };

        let mut normalized = Vec::with_capacity(input.len());
        for algorithm in input {
            if !normalized.contains(&algorithm) {
                normalized.push(algorithm);
            }
        }

        normalized
    }
}

impl Default for ChecksumOptions {
    fn default() -> Self {
        Self {
            algorithms: vec![HashAlgorithm::Sha256],
            follow_symlinks: false,
        }
    }
}

/// A single digest value for one algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDigest {
    pub algorithm: HashAlgorithm,
    pub hex: String,
}

/// All computed checksums for one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChecksum {
    pub path: String,
    pub size_bytes: u64,
    pub digests: Vec<FileDigest>,
}
