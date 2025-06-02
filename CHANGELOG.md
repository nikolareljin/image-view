## 0.2.0
- Update Wiki page with the latest parameters of the main program source code (./image-view executable when being compiled and built). Push the updated information into `CLI.md` file.

## 0.1.0
- Rust script to display local image with full path: `image-view <file path>`
- Update dependency `actions/checkout@v4`
- Update dependency `actions-rs/toolchain@v1`
- Auto-generate executable files for different platforms when the code gets tagged.
- Add Dockerfile, build.sh and test.sh scripts to verify building of the artifacts
- Add Release info after tagging and release GH Action completes building artifacts
- Add links to built artifacts in the Release information after the process completes.