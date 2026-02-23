use std::path::PathBuf;

use checksum_core::{ChecksumOptions, FileChecksum, HashAlgorithm, compute_checksums};

#[test]
fn computes_expected_hashes_for_repository_fixtures() {
    let options = ChecksumOptions {
        algorithms: vec![HashAlgorithm::Md5, HashAlgorithm::Sha256],
        follow_symlinks: false,
    };

    let checksums = compute_checksums(fixture_root(), &["**/*.txt", "!ignored/**"], &options)
        .expect("compute fixture checksums");

    let paths: Vec<&str> = checksums
        .iter()
        .map(|checksum| checksum.path.as_str())
        .collect();
    assert_eq!(paths, vec!["a.txt", "nested/b.txt"]);

    let a_checksum = checksums
        .iter()
        .find(|checksum| checksum.path == "a.txt")
        .expect("a.txt checksum exists");
    assert_eq!(a_checksum.size_bytes, 4);
    assert_eq!(
        digest_hex(a_checksum, HashAlgorithm::Md5),
        "0bee89b07a248e27c83fc3d5951213c1"
    );
    assert_eq!(
        digest_hex(a_checksum, HashAlgorithm::Sha256),
        "edeaaff3f1774ad2888673770c6d64097e391bc362d7d6fb34982ddf0efd18cb",
    );

    let b_checksum = checksums
        .iter()
        .find(|checksum| checksum.path == "nested/b.txt")
        .expect("nested/b.txt checksum exists");
    assert_eq!(b_checksum.size_bytes, 6);
    assert_eq!(
        digest_hex(b_checksum, HashAlgorithm::Md5),
        "b1946ac92492d2347c6235b4d2611184"
    );
    assert_eq!(
        digest_hex(b_checksum, HashAlgorithm::Sha256),
        "5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03",
    );
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/action")
}

fn digest_hex(checksum: &FileChecksum, algorithm: HashAlgorithm) -> &str {
    checksum
        .digests
        .iter()
        .find(|digest| digest.algorithm == algorithm)
        .map(|digest| digest.hex.as_str())
        .expect("digest exists for algorithm")
}
