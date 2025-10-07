# Testing Canvadot

If you are here, it may be or either because you clicked out of curiosity or because
you are contributing to Canvadot and want more information on how testing is performed
within. If it's the later, thanks.

## Requirements

Canvadot is tested in the remote GitHub repository, it uses the following tools
that you may want to install if you want to replicate the essential CI procedures
locally instead of making meaningless commits you will need to squash and force-push
later.

- `cargo clippy`: Used to enforce lints, https://doc.rust-lang.org/clippy/usage.html
- `cargo fmt`: A code formatter, checks are run for this, https://doc.rust-lang.org/cargo/commands/cargo-fmt.html
- `cargo llvm-cov`: Used for test coverage, https://crates.io/crates/cargo-llvm-cov

## What's being tested?

This is a mono-repo that uses two targets, `wasm32-unknown-unknown` and `x86_64-unknown-linux-gnu`,
windows and arm is not supported in this guide due to the nature of the application, the target
for `wasm32-unknown-unknown` is not tested due to the lack of support within WASM testing tools.

This means that the front-end is only partially tested, the components and other rendering is excluded
from coverage.

If you are going to add features to the front-end try to move as more logic as possible to any folder
that's not `core/frontend/src/components` which is excluded from coverage, and try to not write it in
`core/frontend/src/app.rs` or `core/frontend/src/main.rs` as these are also partially excluded from
coverage.

In the case of the back-end, all of the files are being tested because there are no target incompatibilities.

## In what order should I run the tests?

This project integrates `just` which is a recipe runner, you may download it with `cargo-binstall`,
refer to their official guide @ https://just.systems/man/en/ .

You can verify the code you are running by checking the `Justfile` in the root of the repository.

There is a helper recipe @ `just test`, that will take as argument the name of the tool you want
to test, and run it with the application configuration. If you want to test everything passes just
run `just test fmt clippy code`, if that exits with code 0, you are good to go.

## Automating the tests

If you are going to contribute to this repository more than once, you might want to check all of your
commits, for this run `just setup`, this includes setting the folder for repository hooks to `.git_hooks`
relative to the repository root.

## Adding new tests

As earlier mentioned, this project does coverage testing, for which you need to test every part of
the code you add.

Adding tests in the front-end is as simple as adding a test for each module or group of modules
inside `core/frontend/src/tests/`.

Adding tests in the back-end requires you to divide between integration and unit testing, integration
testing will be verified by whoever is in charge of reviewing your pull request and should amount
to testing the feature you add among the known cases of possible failure. Unit testing on the other
hand, is checked by code coverage for which you only need coverage to pass.
