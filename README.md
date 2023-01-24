# Rust Repository Template

[![CI](https://github.com/runtime-machines/rust-template/workflows/CI/badge.svg)](https://github.com/runtime-machines/rust-template/actions)

⭐ `Star` this repository if you find it valuable and worth maintaining.

👁 `Watch` this repository to get notified about new releases, issues, etc.

## Description

This is a GitHub repository template for a Go application.
You can use it:

- to create a new repoisitory with automation and environment setup,
- as reference when improving automation for an existing repository.

It includes:

- continuous integration via [GitHub Actions](https://github.com/features/actions)
  - CI Workflows:
    - Test
    - Format check
    - Clippy
    - Docs
    - Publish
    - Code coverage report on PR
  - CD Workflows (Release artifacts on Cargo and Github):
    - os: macos-latest
      target: x86_64-apple-darwin
    - os: ubuntu-latest
      target: x86_64-unknown-linux-gnu
    - os: windows-latest
      target: x86_64-pc-windows-msvc
    - os: ubuntu-latest
      target: aarch64-unknown-linux-gnu
    - os: ubuntu-latest
      target: i686-unknown-linux-gnu
  - Security:
    - Audit everyday at 00.00 UTC
    - dependabot to update dependencies weekly
- build automation via [Cargo](https://doc.rust-lang.org/cargo/commands/cargo-build.html),
- dependency management using [Cargo Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html),
- code formatting using [rust fmt](https://rust-lang.github.io/rustfmt/),
- linting with [clippy](https://github.com/rust-lang/rust-clippy),
- code coverage [HTML report](https://docs.codecov.com/docs) and tests
- releasing using [Github Release](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository),
- dependencies scanning and updating thanks to [Dependabot](https://dependabot.com),
- security code analysis using [Audit-check](https://github.com/actions-rs/audit-check),

## Usage

1. Sign up on [Codecov](https://codecov.io/) and configure
   [Codecov GitHub Application](https://github.com/apps/codecov) for all repositories.
1. Click the `Use this template` button (alt. clone or download this repository).
1. Replace all occurrences of `runtime-machines/rust-template` to `your_org/repo_name` in all files.
1. Replace all occurrences of `rust-template` to `repo_name` in all files.
1. Set up `CODECOV_TOKEN` secret in `https://github.com/runtime-machines/REPO_NAME/settings/secrets/actions`.
1. Update the following files:
   - [CHANGELOG.md](CHANGELOG.md)
   - [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
   - [LICENSE](LICENSE) (when defined)
   - [README.md](README.md)
1. Uncomment the necessary workflow paths in the ci.yml workflow and add the ${{ secrets.CODECOV_TOKEN }} secret if you're planning to keep the repository private. Also you might need to add a /script/run.sh and a migration folder, depending on your SQLX needs
   - To add a deploy key:
     - generate the key using `ssh-keygen -t ed25519 -C "<key_description:key_owner@runtimemachines.com>"`
     - Add the .pub to the source repository deploy keys
     - Add the private key in this repository secrets
     - Add the key to the `ssh-private-key` in the `ssh-agent` step 

## License

This project license

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the license, shall be
licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
