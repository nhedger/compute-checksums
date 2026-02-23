use std::fs;

use checksum_core::{ChecksumOptions, HashAlgorithm, compute_checksums};

#[test]
fn computes_multiple_algorithms_for_each_file() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    fs::write(root.join("a.txt"), b"abc").expect("write test file");

    let options = ChecksumOptions {
        algorithms: vec![
            HashAlgorithm::Md5,
            HashAlgorithm::Sha1,
            HashAlgorithm::Sha256,
            HashAlgorithm::Sha512,
        ],
        follow_symlinks: false,
    };

    let results = compute_checksums(root, &["**/*.txt"], &options).expect("compute checksums");
    assert_eq!(results.len(), 1);

    let checksum = &results[0];
    assert_eq!(checksum.path, "a.txt");
    assert_eq!(checksum.size_bytes, 3);
    assert_eq!(checksum.digests.len(), 4);

    let expected = [
        (HashAlgorithm::Md5, "900150983cd24fb0d6963f7d28e17f72"),
        (
            HashAlgorithm::Sha1,
            "a9993e364706816aba3e25717850c26c9cd0d89d",
        ),
        (
            HashAlgorithm::Sha256,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            HashAlgorithm::Sha512,
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
             2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
        ),
    ];

    for ((expected_algorithm, expected_hex), digest) in expected.into_iter().zip(&checksum.digests)
    {
        assert_eq!(digest.algorithm, expected_algorithm);
        assert_eq!(digest.hex, expected_hex.replace(' ', ""));
    }
}

#[test]
fn defaults_to_sha256_when_algorithms_are_empty() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    fs::write(root.join("a.txt"), b"abc").expect("write test file");

    let options = ChecksumOptions {
        algorithms: Vec::new(),
        follow_symlinks: false,
    };

    let results = compute_checksums(root, &["**/*.txt"], &options).expect("compute checksums");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].digests.len(), 1);
    assert_eq!(results[0].digests[0].algorithm, HashAlgorithm::Sha256);
    assert_eq!(
        results[0].digests[0].hex,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
    );
}

#[test]
fn returns_empty_results_when_no_files_match() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    fs::write(root.join("a.txt"), b"abc").expect("write test file");

    let results = compute_checksums(root, &["**/*.md"], &ChecksumOptions::default())
        .expect("compute checksums");

    assert!(results.is_empty());
}

#[test]
fn deduplicates_algorithms_while_preserving_first_occurrence_order() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let root = temp_dir.path();
    fs::write(root.join("a.txt"), b"abc").expect("write test file");

    let options = ChecksumOptions {
        algorithms: vec![
            HashAlgorithm::Sha256,
            HashAlgorithm::Md5,
            HashAlgorithm::Sha256,
            HashAlgorithm::Sha1,
            HashAlgorithm::Md5,
        ],
        follow_symlinks: false,
    };

    let results = compute_checksums(root, &["**/*.txt"], &options).expect("compute checksums");
    assert_eq!(results.len(), 1);

    let algorithms: Vec<HashAlgorithm> = results[0]
        .digests
        .iter()
        .map(|digest| digest.algorithm)
        .collect();

    assert_eq!(
        algorithms,
        vec![
            HashAlgorithm::Sha256,
            HashAlgorithm::Md5,
            HashAlgorithm::Sha1
        ],
    );
}
