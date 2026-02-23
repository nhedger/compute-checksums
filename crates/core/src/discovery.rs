use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::{ChecksumError, patterns::CompiledPatterns};

pub(crate) struct DiscoveredFile {
    pub(crate) absolute_path: PathBuf,
    pub(crate) relative_path: String,
}

pub(crate) fn discover_files(
    root: &Path,
    patterns: &CompiledPatterns,
    follow_symlinks: bool,
) -> Result<Vec<DiscoveredFile>, ChecksumError> {
    let mut files_by_relative_path = BTreeMap::new();

    let walker = WalkDir::new(root).follow_links(follow_symlinks);
    for entry in walker {
        let entry = entry.map_err(|source| ChecksumError::WalkDirectory {
            path: root.to_path_buf(),
            source,
        })?;

        if !entry.file_type().is_file() {
            continue;
        }

        let absolute_path = entry.into_path();
        let relative_path =
            absolute_path
                .strip_prefix(root)
                .map_err(|_| ChecksumError::StripPrefix {
                    root: root.to_path_buf(),
                    path: absolute_path.clone(),
                })?;

        let normalized_relative_path = normalize_relative_path(relative_path);
        if !patterns.is_match(&normalized_relative_path) {
            continue;
        }

        files_by_relative_path
            .entry(normalized_relative_path)
            .or_insert(absolute_path);
    }

    Ok(files_by_relative_path
        .into_iter()
        .map(|(relative_path, absolute_path)| DiscoveredFile {
            absolute_path,
            relative_path,
        })
        .collect())
}

fn normalize_relative_path(relative_path: &Path) -> String {
    relative_path
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
