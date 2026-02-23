# Checksum GitHub Action

**Checksum** is a Docker-based GitHub Action that computes the checksum of
files that match one or more specified glob patterns and exposes the results as
output variables.

## Inputs

- `patterns` (required): include and exclude glob patterns. Use newline-separated
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

## Example

```yaml
name: checksum

on:
  push:

jobs:
  checksums:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5

      - id: checksum
        uses: ./.
        with:
          patterns: |
            src/**/*.rs
            !target/**
          algorithms: |
            sha256
            sha512

      - name: Print all JSON
        run: echo '${{ steps.checksum.outputs.all }}'

      - name: Print sha256 checksums
        run: echo '${{ steps.checksum.outputs.sha256 }}'

      - name: Create SHA256SUM.txt
        env:
          SHA256_SUM: ${{ steps.checksum.outputs.sha256 }}
        run: printf '%s\n' "$SHA256_SUM" > SHA256SUM.txt

      - name: Verify SHA256SUM.txt
        run: sha256sum -c SHA256SUM.txt

      - name: Resolve per-file digest from files JSON
        run: |
          echo "sha256=${{ fromJson(steps.checksum.outputs.files)['src/lib.rs'].sha256 }}"
```

If no files match, the action succeeds and returns an empty `all` array.
