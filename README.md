# Compute Checksums GitHub Action

[![CI](https://github.com/nhedger/compute-checksums/actions/workflows/ci.yml/badge.svg)](https://github.com/nhedger/compute-checksums/actions/workflows/ci.yml)

**Compute Checksums** is a Docker-based GitHub Action written in Rust 🦀 that
computes the checksum of files that match one or more specified glob
patterns and exposes the results as output variables.

## Inputs

- `patterns` (required): include and exclude glob patterns. Use
  newline-separated
  values (preferred) or comma-separated values. Prefix excludes with `!`.
- `algorithms` (optional): hashing algorithms to compute for each file.
  Supported: `md5`, `sha1`, `sha256`, `sha512`. Default: `sha256`.
- `root` (optional): directory to search from. Default: `.`.
- `follow_symlinks` (optional): whether to follow symlinks while traversing.
  Default: `false`.

## Outputs

- `all`: JSON array of file results.
- `files`: JSON object keyed by relative file path.
- `md5`, `sha1`, `sha256`, `sha512`: algorithm-specific checksum-file outputs in
  `digest  path` format. Only emitted for selected algorithms.

## Quick Start

This example illustrates how to compute the SHA256 checksums of all `.tar.gz`
files in the `dist` directory and write the results to a `SHA256SUM.txt` file.

```yaml
- name: Checkout
  uses: actions/checkout@v5

- name: Compute checksums
  id: checksum
  uses: nhedger/compute-checksums@v1
  with:
    patterns: dist/*.tar.gz
    algorithms: sha256

- name: Create SHA256SUM file
  run: |
    echo "${{ steps.checksum.outputs.sha256 }}" > SHA256SUM.txt
```

If no files match, the action succeeds and returns an empty `all` array.

## Examples

### Multiple algorithms

This example computes both SHA256 and SHA512 checksums for all `.zip`
files in the `artifacts` directory.

```yaml
- name: Checkout
  uses: actions/checkout@v5

- name: Compute checksums
  id: checksum
  uses: nhedger/compute-checksums@v1
  with:
    patterns: artifacts/*.zip
    algorithms: sha256, sha512

- name: Create checksum files
  run: |
    echo "${{ steps.checksum.outputs.sha256 }}" > SHA256SUM.txt
    echo "${{ steps.checksum.outputs.sha512 }}" > SHA512SUM.txt
```

### Retrieve file-specific checksums

This example demonstrates how to access checksums for individual files using the
`files` output.

```yaml
- name: Checkout
  uses: actions/checkout@v5

- name: Compute checksums
  id: checksum
  uses: nhedger/compute-checksums@v1
  with: 
    patterns: |
      dist/some-file.zip
      dist/other-file.zip
    algorithms: sha256

- name: Use file-specific checksums
  run: |
    echo "SHA256 of some-file.zip: ${{ fromJson(steps.checksum.outputs.files)['dist/some-file.zip'].sha256 }}"
    echo "SHA256 of other-file.zip: ${{ fromJson(steps.checksum.outputs.files)['dist/other-file.zip'].sha256 }}"
```

## License

This project is licensed under both the MIT License and the Apache License (Version 2.0).
See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.