use std::{
    collections::BTreeMap,
    env,
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

use checksum_core::{FileChecksum, HashAlgorithm};
use serde::Serialize;

use crate::error::ActionError;

pub(crate) fn write_action_outputs(
    checksums: &[FileChecksum],
    selected_algorithms: &[HashAlgorithm],
) -> Result<(), ActionError> {
    let output_path = env::var("GITHUB_OUTPUT").map_err(|_| ActionError::MissingGithubOutput)?;
    let mut output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output_path)
        .map_err(|source| ActionError::OpenGithubOutput {
            path: PathBuf::from(&output_path),
            source,
        })?;

    let rendered_outputs = render_outputs(checksums, selected_algorithms)?;
    for (index, (key, value)) in rendered_outputs.into_iter().enumerate() {
        write_output_line(&mut output_file, &key, &value, index)
            .map_err(|source| ActionError::WriteOutput { key, source })?;
    }

    Ok(())
}

#[derive(Debug, Serialize)]
struct JsonFileChecksum {
    path: String,
    size_bytes: u64,
    digests: BTreeMap<String, String>,
}

#[derive(Debug, Serialize)]
struct JsonFileDetails {
    size_bytes: u64,
    #[serde(flatten)]
    digests: BTreeMap<String, String>,
}

fn render_outputs(
    checksums: &[FileChecksum],
    selected_algorithms: &[HashAlgorithm],
) -> Result<Vec<(String, String)>, ActionError> {
    let mut rendered_all = Vec::with_capacity(checksums.len());
    let mut rendered_files = BTreeMap::new();
    let mut checksum_lines_by_algorithm: BTreeMap<String, Vec<String>> = selected_algorithms
        .iter()
        .map(|algorithm| (algorithm.as_str().to_owned(), Vec::new()))
        .collect();

    for checksum in checksums {
        let mut digest_map = BTreeMap::new();
        for digest in &checksum.digests {
            let algorithm = digest.algorithm.as_str().to_owned();
            digest_map.insert(algorithm.clone(), digest.hex.clone());

            if let Some(lines) = checksum_lines_by_algorithm.get_mut(&algorithm) {
                lines.push(format!("{}  {}", digest.hex, checksum.path));
            }
        }

        rendered_all.push(JsonFileChecksum {
            path: checksum.path.clone(),
            size_bytes: checksum.size_bytes,
            digests: digest_map.clone(),
        });

        rendered_files.insert(
            checksum.path.clone(),
            JsonFileDetails {
                size_bytes: checksum.size_bytes,
                digests: digest_map,
            },
        );
    }

    let mut outputs = vec![
        ("all".to_owned(), serde_json::to_string(&rendered_all)?),
        ("files".to_owned(), serde_json::to_string(&rendered_files)?),
    ];

    for algorithm in selected_algorithms {
        let key = algorithm.as_str().to_owned();
        let lines = checksum_lines_by_algorithm.remove(&key).unwrap_or_default();
        outputs.push((key, lines.join("\n")));
    }

    Ok(outputs)
}

fn write_output_line(
    writer: &mut impl Write,
    key: &str,
    value: &str,
    sequence: usize,
) -> io::Result<()> {
    let mut delimiter = format!("__CHECKSUM_OUTPUT_{sequence}__");
    while value.contains(&delimiter) {
        delimiter.push('_');
    }

    writeln!(writer, "{key}<<{delimiter}")?;
    writeln!(writer, "{value}")?;
    writeln!(writer, "{delimiter}")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use checksum_core::{FileChecksum, FileDigest, HashAlgorithm};

    use super::render_outputs;

    #[test]
    fn renders_all_files_and_per_algorithm_outputs() {
        let checksums = vec![
            FileChecksum {
                path: "a.txt".to_owned(),
                size_bytes: 3,
                digests: vec![
                    FileDigest {
                        algorithm: HashAlgorithm::Sha256,
                        hex: "a256".to_owned(),
                    },
                    FileDigest {
                        algorithm: HashAlgorithm::Md5,
                        hex: "amd5".to_owned(),
                    },
                ],
            },
            FileChecksum {
                path: "nested/b.txt".to_owned(),
                size_bytes: 4,
                digests: vec![
                    FileDigest {
                        algorithm: HashAlgorithm::Sha256,
                        hex: "b256".to_owned(),
                    },
                    FileDigest {
                        algorithm: HashAlgorithm::Md5,
                        hex: "bmd5".to_owned(),
                    },
                ],
            },
        ];

        let outputs = render_outputs(&checksums, &[HashAlgorithm::Sha256, HashAlgorithm::Md5])
            .expect("render outputs");
        let output_map: HashMap<String, String> = outputs.into_iter().collect();

        let all: serde_json::Value =
            serde_json::from_str(output_map.get("all").expect("all")).expect("parse all json");
        assert_eq!(all[0]["path"], "a.txt");
        assert_eq!(all[0]["digests"]["sha256"], "a256");

        let files: serde_json::Value =
            serde_json::from_str(output_map.get("files").expect("files"))
                .expect("parse files json");
        assert_eq!(files["a.txt"]["size_bytes"], 3);
        assert_eq!(files["a.txt"]["sha256"], "a256");
        assert_eq!(files["nested/b.txt"]["md5"], "bmd5");

        assert_eq!(
            output_map.get("sha256").expect("sha256"),
            "a256  a.txt\nb256  nested/b.txt"
        );
        assert_eq!(
            output_map.get("md5").expect("md5"),
            "amd5  a.txt\nbmd5  nested/b.txt"
        );
        assert!(!output_map.contains_key("sha1"));
    }

    #[test]
    fn renders_empty_algorithm_output_when_selected_but_no_matches() {
        let outputs =
            render_outputs(&[], &[HashAlgorithm::Sha512]).expect("render outputs with no files");
        let output_map: HashMap<String, String> = outputs.into_iter().collect();

        assert_eq!(output_map.get("all").expect("all"), "[]");
        assert_eq!(output_map.get("files").expect("files"), "{}");
        assert_eq!(output_map.get("sha512").expect("sha512"), "");
        assert!(!output_map.contains_key("md5"));
    }
}
