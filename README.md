# GitHub Actions Workflows for Rust

This repository contains some copy-pastable GitHub Actions Workflows suited for Rust projects.

When it comes to CI workflows, there is no _one size fits all_ solution.
This repository rather aims to offer _few sizes fit many_ solutions.

## Example Workflows

### Minimal CI

The [`minimal-ci`](./.github/workflows/minimal-ci.yml) workflow offers a simple CI workflow using minimal third-party tools.
The only third-party action it uses is [`Swatinem/rust-cache`](https://github.com/Swatinem/rust-cache). This is a no-brainer, and as the author, I am allowed to be biased :-)
Otherwise, it relies only on standard Rust tooling available as `rustup` components.

It contains the following CI jobs:

- A **lint** job running `rustfmt` and `clippy`.
- A **documentation** job running doctests and checking some rustdoc lints.
- A **test** job running across Linux and Windows covering all non-doc targets.

### Complete CI

The [`complete-ci`](./.github/workflows/complete-ci.yml) workflow has a more complete solution pulling in a bunch more tools.

- The **lint** job is additionally using [`cargo-semver-checks`](https://github.com/obi1kenobi/cargo-semver-checks) to lint for SemVer violations.
- The **test** job is using [`nextest`](https://nexte.st/) as the test runner and [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) to collect code coverage.
- Both test results and code coverage results are pushed to [`codecov`](https://app.codecov.io/gh/Swatinem/rust-gha-workflows).
- There is an additional **benchmark** job that uploads results to [`codspeed`](https://codspeed.io/Swatinem/rust-gha-workflows).
- Last but not least, it has a **miri** job, running the testsuite (excluding benchmarks) through [`miri`](https://github.com/rust-lang/miri).

## Missing Features

The _example workflows_ above are quite bare-bones thus far. Either intentionally (the `minimal-ci` workflow is _supposed to be minimal_), or because a lot of checks are just not hooked up (yet).
Here are some ideas of things that could (or really _should_) be checked within CI, to maintain a high code/crate quality.

### Feature Combinations

I would love to add [`cargo-hack --feature-powerset`](https://github.com/taiki-e/cargo-hack) to this mix at some point to check _every combination of features_.
However, lack of [mutually exclusive features](https://github.com/rust-lang/cargo/issues/2980) makes this less than ideal right now.

The current workflows are running with `--all-features`, which also does not satisfy _one size fits all_ for crates that have mutually exclusive features.

### MSRV / Minimal Versions

It would also be nice to have a job that verifies everything running well with the advertised minimum supported rust version (_MSRV_).
Similarly, hooking up [`cargo-minimal-versions`](https://github.com/taiki-e/cargo-minimal-versions) would be nice to verify that the crate builds with the advertised dependency versions, and does not use any features introduced in higher versions (necessitating a bump within `Cargo.toml`).

### Regular Maintenance Workflow

I started working on [`Swatinem/rust-maintain`](https://github.com/Swatinem/rust-maintain) which is supposed to be a workflow that runs `cargo update` regularly, and applies auto-fixed from `clippy`.
This does not currently work fully as intended yet however.

Apart from bumping crate dependencies, it would also be nice if that workflow could bump the Rust toolchain version, and possibly do edition updates as well.

### Publish Workflow

Another thing currently missing is a workflow that publishes new crate versions, possibly using [Trusted Publishing](https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html).

### Dependency Audit

The **lint** job could also potentially be extended using a dependency audit step, using [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) or a similar tool.

## Contribution

I would love to extend this _collection of few sizes fit many_ workflows.
Feel free to open a PR to add other variants of workflows solving different use-cases.
Or tackle some of the _missing features_ mentioned above.
