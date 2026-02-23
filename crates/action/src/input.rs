use std::{env, path::PathBuf, str::FromStr};

use checksum_core::HashAlgorithm;

use crate::error::ActionError;

pub(crate) struct ActionInput {
    pub(crate) root: PathBuf,
    pub(crate) patterns: Vec<String>,
    pub(crate) algorithms: Vec<HashAlgorithm>,
    pub(crate) follow_symlinks: bool,
}

impl ActionInput {
    pub(crate) fn from_env() -> Result<Self, ActionError> {
        let raw_patterns = read_input("patterns").ok_or(ActionError::MissingPatterns)?;
        let patterns = parse_list_input(&raw_patterns);
        if patterns.is_empty() {
            return Err(ActionError::EmptyPatterns);
        }

        let raw_algorithms = read_input("algorithms").unwrap_or_default();
        let algorithms = parse_algorithms(&raw_algorithms)?;

        let root = parse_root(read_input("root"));
        let follow_symlinks = parse_follow_symlinks(read_input("follow_symlinks"))?;

        Ok(Self {
            root,
            patterns,
            algorithms,
            follow_symlinks,
        })
    }
}

fn parse_algorithms(raw: &str) -> Result<Vec<HashAlgorithm>, ActionError> {
    let mut algorithms = Vec::new();

    for value in parse_list_input(raw) {
        let algorithm =
            HashAlgorithm::from_str(&value).map_err(|_| ActionError::InvalidAlgorithm {
                value: value.clone(),
            })?;
        algorithms.push(algorithm);
    }

    Ok(algorithms)
}

fn parse_root(raw_root: Option<String>) -> PathBuf {
    if let Some(root) = raw_root {
        let trimmed = root.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    if let Ok(workspace) = env::var("GITHUB_WORKSPACE") {
        let trimmed = workspace.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }

    PathBuf::from(".")
}

fn parse_follow_symlinks(raw_value: Option<String>) -> Result<bool, ActionError> {
    let Some(value) = raw_value else {
        return Ok(false);
    };

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(false);
    }

    match trimmed.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "y" | "on" => Ok(true),
        "0" | "false" | "no" | "n" | "off" => Ok(false),
        _ => Err(ActionError::InvalidFollowSymlinks {
            value: value.to_owned(),
        }),
    }
}

fn parse_list_input(raw: &str) -> Vec<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    if trimmed.contains('\n') || trimmed.contains('\r') {
        return trimmed
            .lines()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .collect();
    }

    trimmed
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn read_input(input_id: &str) -> Option<String> {
    let mut keys = vec![format!("INPUT_{}", input_id.to_ascii_uppercase())];

    let normalized = input_id.to_ascii_uppercase();
    let underscore_variant = format!("INPUT_{}", normalized.replace('-', "_"));
    if !keys.contains(&underscore_variant) {
        keys.push(underscore_variant);
    }

    let hyphen_variant = format!("INPUT_{}", normalized.replace('_', "-"));
    if !keys.contains(&hyphen_variant) {
        keys.push(hyphen_variant);
    }

    for key in keys {
        if let Ok(value) = env::var(&key) {
            return Some(value);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{parse_follow_symlinks, parse_list_input};

    #[test]
    fn parse_list_prefers_newline_split() {
        let values = parse_list_input(" src/**/*.rs\n !target/** \n\n");
        assert_eq!(values, vec!["src/**/*.rs", "!target/**"]);
    }

    #[test]
    fn parse_list_uses_comma_split_without_newlines() {
        let values = parse_list_input("md5, sha256,sha512");
        assert_eq!(values, vec!["md5", "sha256", "sha512"]);
    }

    #[test]
    fn parse_list_preserves_single_value() {
        let values = parse_list_input("**/*.txt");
        assert_eq!(values, vec!["**/*.txt"]);
    }

    #[test]
    fn parse_follow_symlinks_supports_common_true_values() {
        assert!(parse_follow_symlinks(Some("true".to_owned())).expect("parse true"));
        assert!(parse_follow_symlinks(Some("1".to_owned())).expect("parse 1"));
        assert!(parse_follow_symlinks(Some("On".to_owned())).expect("parse on"));
    }

    #[test]
    fn parse_follow_symlinks_supports_common_false_values() {
        assert!(!parse_follow_symlinks(Some("false".to_owned())).expect("parse false"));
        assert!(!parse_follow_symlinks(Some("0".to_owned())).expect("parse 0"));
        assert!(!parse_follow_symlinks(Some("off".to_owned())).expect("parse off"));
        assert!(!parse_follow_symlinks(None).expect("parse missing"));
    }
}
