THIRD-PARTY NOTICES & LICENSES
------------------------------
Generate a dependency and license manifest and include third-party license text.

Recommended commands:
- Rust: `cargo license --avoid-dev-deps --output-format json > third_party_rust.json`
- OR: `cargo-about init && cargo-about generate > third_party_licenses.txt`
- For C dependencies: collect license files in `third_party/C/`

List each third-party component, version, and license and include either the license text or a link to it.

Action items:
- Run the recommended commands and add the resulting artifacts to THIRD_PARTY_NOTICES.md or `third_party/`
- Ensure any required attributions are included in the project documentation and the release.
