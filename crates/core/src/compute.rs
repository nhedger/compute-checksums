use std::path::Path;

use crate::{
    ChecksumError, ChecksumOptions, FileChecksum, discovery::discover_files, hash::hash_file,
    patterns::CompiledPatterns,
};

/// Computes checksums for files under `root` that match the provided patterns.
///
/// Patterns prefixed with `!` are treated as exclude patterns. If no files
/// match, an empty list is returned.
///
/// # Errors
///
/// Returns an error when patterns are invalid or file system operations fail.
pub fn compute_checksums<P, S>(
    root: P,
    patterns: &[S],
    options: &ChecksumOptions,
) -> Result<Vec<FileChecksum>, ChecksumError>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let root = root.as_ref();
    if !root.is_dir() {
        return Err(ChecksumError::InvalidRoot {
            path: root.to_path_buf(),
        });
    }

    let compiled_patterns = CompiledPatterns::from_patterns(patterns)?;
    if !compiled_patterns.has_includes() {
        return Ok(Vec::new());
    }

    let files = discover_files(root, &compiled_patterns, options.follow_symlinks)?;
    if files.is_empty() {
        return Ok(Vec::new());
    }

    let algorithms = options.normalized_algorithms();
    let mut checksums = Vec::with_capacity(files.len());

    for file in files {
        let (size_bytes, digests) = hash_file(&file.absolute_path, &algorithms)?;
        checksums.push(FileChecksum {
            path: file.relative_path,
            size_bytes,
            digests,
        });
    }

    Ok(checksums)
}
