# Contribution Guideline


## Environment setup

- Install the latest Rust via [rustup](https://doc.rust-lang.org/cargo/getting-started/installation.html).
- Install the latest [Scarb via ASDF](https://docs.swmansion.com/scarb/download.html#install-via-asdf).

## Contributing

- Before you open a pull request, it is always a good idea to search the issues and verify if the feature you would like
to add hasn't been already discussed.
- We also appreciate creating a feature request before making a contribution, so it can be discussed before you get to
work.
- If the change you are introducing is changing or breaking the behavior of any already existing features, make sure to
include that information in the pull request description.

## Adding new lint rule

In order to add a new rule, you must extend a [context](crates/cairo-lint-core/src/context.rs) with a new lint or whole lint group.

Each individual lint rule should be documented. When implementing [Lint trait](crates/cairo-lint-core/src/context.rs#L118) for the Lint rule, remember to include a documentation for it which should look like this:

```rust
/// ## What it does
///
/// ## Example
/// ```cairo
/// // example code
/// ```
impl Lint for MyRule {
  // implementation ...
}
```

## Testing

### Running tests

To run the tests you'll need to provide the path to the cairo corelib (at some point this should be automated but we're
not there yet).

```sh
CORELIB_PATH="/path/to/corelib/src" cargo test
```

### Reviewing snapshot changes

```sh
cargo insta review
```

### CLI instructions

To add a new test you can use the dev cli with:

```
cargo xtask create-test --name "Your lint name" --group "Your lint group name"
```

### Manual instructions

Each lint should have its own tests and should be extensive. To create a new test for a lint you need to create a new file/module
in the [test_files folder](./crates/cairo-lint-core/tests) and should be named as your lint. The file should

As for tests, we are using [insta](https://insta.rs/) snapshot library. 
There are 2 testing macros:
- [test_lint_diagnostics](crates/cairo-lint-core/tests/helpers/mod.rs)
- [test_lint_fixer](crates/cairo-lint-core/tests/helpers/mod.rs)

Tests should use only the inline snapshots.


When creating a new test, you can run `CORELIB_PATH={path} cargo test`, and see if your snapshots match. It's recommended to use the the [cargo-insta](https://crates.io/crates/cargo-insta) tool to review the snapshots. Just remember to first run the tests with `cargo test`, and after that run `cargo insta review` to review any snapshot differences.
