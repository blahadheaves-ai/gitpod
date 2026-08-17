Release checklist (DARPA-oriented)

1) Confirm classification & export:
   - Confirm whether content is unclassified and releasable.
   - Insert authorized Distribution Statement into DISTRIBUTION_STATEMENT.txt.

2) License:
   - Choose and add LICENSE file (Apache-2.0 was selected for this release). Confirm sponsor/CO acceptance.

3) Remove secrets:
   - Run git-secrets and trufflehog; remove or rotate any found credentials.

4) Automated scans:
   - Rust: `cargo audit`; `cargo-deny`
   - C: `clang-tidy` / `clang-scan` / coverity (if available)
   - SAST: run static analysis tools as appropriate

Commands:
- `git-secrets --scan`
- `trufflehog filesystem --pattern '.*' .`
- `cargo audit --json > audit.json`
- `cargo deny check -f deny.toml`
- `syft packages dir:. -o cyclonedx-json > sbom-cyclonedx.json`
- `syft packages dir:. -o spdx-json > sbom-spdx.json`

5) Third-party licenses:
   - `cargo-about generate` OR `cargo license` -> aggregate license list

6) Create release tag and release notes:
   - Tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
   - GitHub release: include SBOM, audit.json, distribution statement, third-party notices, and release notes.

7) Archive and provenance:
   - Optionally archive to Zenodo or Software Heritage; link DOI in release notes.

8) Final review:
   - Legal/contracts sign-off (program manager/CO)
   - Security sign-off (CISO/security lead)
