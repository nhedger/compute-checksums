use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use md5::{Digest as _, Md5};
use sha1::Sha1;
use sha2::{Sha256, Sha512};

use crate::ChecksumError;
use crate::{FileDigest, HashAlgorithm};

const READ_BUFFER_BYTES: usize = 64 * 1024;

enum HasherVariant {
    Md5(Md5),
    Sha1(Sha1),
    Sha256(Sha256),
    Sha512(Sha512),
}

impl HasherVariant {
    fn new(algorithm: HashAlgorithm) -> Self {
        match algorithm {
            HashAlgorithm::Md5 => Self::Md5(Md5::new()),
            HashAlgorithm::Sha1 => Self::Sha1(Sha1::new()),
            HashAlgorithm::Sha256 => Self::Sha256(Sha256::new()),
            HashAlgorithm::Sha512 => Self::Sha512(Sha512::new()),
        }
    }

    fn update(&mut self, bytes: &[u8]) {
        match self {
            Self::Md5(hasher) => hasher.update(bytes),
            Self::Sha1(hasher) => hasher.update(bytes),
            Self::Sha256(hasher) => hasher.update(bytes),
            Self::Sha512(hasher) => hasher.update(bytes),
        }
    }

    fn finalize(self) -> String {
        match self {
            Self::Md5(hasher) => hex::encode(hasher.finalize()),
            Self::Sha1(hasher) => hex::encode(hasher.finalize()),
            Self::Sha256(hasher) => hex::encode(hasher.finalize()),
            Self::Sha512(hasher) => hex::encode(hasher.finalize()),
        }
    }
}

struct MultiHasher {
    hashers: Vec<(HashAlgorithm, HasherVariant)>,
}

impl MultiHasher {
    fn new(algorithms: &[HashAlgorithm]) -> Self {
        let hashers = algorithms
            .iter()
            .copied()
            .map(|algorithm| (algorithm, HasherVariant::new(algorithm)))
            .collect();

        Self { hashers }
    }

    fn update(&mut self, bytes: &[u8]) {
        for (_, hasher) in &mut self.hashers {
            hasher.update(bytes);
        }
    }

    fn finalize(self) -> Vec<FileDigest> {
        self.hashers
            .into_iter()
            .map(|(algorithm, hasher)| FileDigest {
                algorithm,
                hex: hasher.finalize(),
            })
            .collect()
    }
}

pub(crate) fn hash_file(
    absolute_path: &Path,
    algorithms: &[HashAlgorithm],
) -> Result<(u64, Vec<FileDigest>), ChecksumError> {
    let file = File::open(absolute_path).map_err(|source| ChecksumError::OpenFile {
        path: absolute_path.to_path_buf(),
        source,
    })?;

    let mut reader = BufReader::new(file);
    let mut buffer = [0_u8; READ_BUFFER_BYTES];
    let mut bytes_read_total = 0_u64;
    let mut hasher = MultiHasher::new(algorithms);

    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .map_err(|source| ChecksumError::ReadFile {
                path: absolute_path.to_path_buf(),
                source,
            })?;

        if bytes_read == 0 {
            break;
        }

        bytes_read_total += bytes_read as u64;
        hasher.update(&buffer[..bytes_read]);
    }

    Ok((bytes_read_total, hasher.finalize()))
}
