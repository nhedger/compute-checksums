use std::fs;

use checksum_core::{ChecksumOptions, compute_checksums};

#[test]
fn returns_results_sorted_by_relative_path() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();

    fs::create_dir_all(root.join("dir")).expect("create nested directory");
    fs::write(root.join("b.txt"), b"b").expect("write b file");
    fs::write(root.join("a.txt"), b"a").expect("write a file");
    fs::write(root.join("dir/c.txt"), b"c").expect("write c file");

    let results = compute_checksums(root, &["**/*.txt"], &ChecksumOptions::default())
        .expect("compute checksums");

    let paths: Vec<&str> = results
        .iter()
        .map(|checksum| checksum.path.as_str())
        .collect();
    assert_eq!(paths, vec!["a.txt", "b.txt", "dir/c.txt"]);
}

#[test]
fn produces_stable_output_across_multiple_runs() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();

    fs::create_dir_all(root.join("nested")).expect("create nested directory");
    fs::write(root.join("nested/a.txt"), b"alpha").expect("write alpha file");
    fs::write(root.join("nested/b.txt"), b"beta").expect("write beta file");

    let first = compute_checksums(root, &["**/*.txt"], &ChecksumOptions::default())
        .expect("compute first run checksums");
    let second = compute_checksums(root, &["**/*.txt"], &ChecksumOptions::default())
        .expect("compute second run checksums");

    assert_eq!(first, second);
}
