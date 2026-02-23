use globset::{Glob, GlobSet, GlobSetBuilder};

use crate::ChecksumError;

pub(crate) struct CompiledPatterns {
    include_set: GlobSet,
    exclude_set: Option<GlobSet>,
    has_includes: bool,
}

impl CompiledPatterns {
    pub(crate) fn from_patterns<S: AsRef<str>>(patterns: &[S]) -> Result<Self, ChecksumError> {
        let mut include_builder = GlobSetBuilder::new();
        let mut exclude_builder = GlobSetBuilder::new();
        let mut has_includes = false;
        let mut has_excludes = false;

        for raw_pattern in patterns {
            let pattern = raw_pattern.as_ref().trim();
            if pattern.is_empty() {
                continue;
            }

            if let Some(exclude_pattern) = pattern.strip_prefix('!') {
                if exclude_pattern.is_empty() {
                    continue;
                }

                let glob = Glob::new(exclude_pattern).map_err(|source| {
                    ChecksumError::InvalidGlobPattern {
                        pattern: pattern.to_owned(),
                        source,
                    }
                })?;

                exclude_builder.add(glob);
                has_excludes = true;
                continue;
            }

            let glob = Glob::new(pattern).map_err(|source| ChecksumError::InvalidGlobPattern {
                pattern: pattern.to_owned(),
                source,
            })?;

            include_builder.add(glob);
            has_includes = true;
        }

        let include_set =
            include_builder
                .build()
                .map_err(|source| ChecksumError::InvalidGlobSet {
                    kind: "include patterns",
                    source,
                })?;

        let exclude_set = if has_excludes {
            Some(
                exclude_builder
                    .build()
                    .map_err(|source| ChecksumError::InvalidGlobSet {
                        kind: "exclude patterns",
                        source,
                    })?,
            )
        } else {
            None
        };

        Ok(Self {
            include_set,
            exclude_set,
            has_includes,
        })
    }

    #[must_use]
    pub(crate) const fn has_includes(&self) -> bool {
        self.has_includes
    }

    #[must_use]
    pub(crate) fn is_match(&self, relative_path: &str) -> bool {
        if !self.has_includes || !self.include_set.is_match(relative_path) {
            return false;
        }

        if let Some(exclude_set) = &self.exclude_set {
            return !exclude_set.is_match(relative_path);
        }

        true
    }
}
