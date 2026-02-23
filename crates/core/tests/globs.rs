use std::fs;

use checksum_core::{ChecksumOptions, compute_checksums};

#[test]
fn applies_include_and_exclude_patterns() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();

    fs::create_dir_all(root.join("src")).expect("create src directory");
    fs::create_dir_all(root.join("target")).expect("create target directory");
    fs::write(root.join("src/lib.rs"), b"lib").expect("write lib file");
    fs::write(root.join("src/main.rs"), b"main").expect("write main file");
    fs::write(root.join("target/generated.rs"), b"generated").expect("write generated file");

    let patterns = ["**/*.rs", "!target/**", "!**/main.rs"];
    let results =
        compute_checksums(root, &patterns, &ChecksumOptions::default()).expect("compute checksums");

    let paths: Vec<&str> = results
        .iter()
        .map(|checksum| checksum.path.as_str())
        .collect();
    assert_eq!(paths, vec!["src/lib.rs"]);
}

#[test]
fn excludes_only_patterns_match_nothing() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    fs::write(root.join("a.txt"), b"hello").expect("write test file");

    let patterns = ["!**/*.txt"];
    let results =
        compute_checksums(root, &patterns, &ChecksumOptions::default()).expect("compute checksums");

    assert!(results.is_empty());
}

#[test]
fn deduplicates_files_matched_by_multiple_include_patterns() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    fs::write(root.join("a.txt"), b"hello").expect("write test file");

    let patterns = ["**/*.txt", "a.*"];
    let results =
        compute_checksums(root, &patterns, &ChecksumOptions::default()).expect("compute checksums");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].path, "a.txt");
}
