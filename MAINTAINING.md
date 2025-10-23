# Cairo-lint Maintenance

## Release procedure

To release a new version of `cairo-lint`:

1. Bump `cairo-lint` version in `Cargo.toml` file.
2. Make sure all the `cairo-lang-*` dependencies are set to a version appropriate for your release.
   You can use the following command:
    ```shell
    cargo xtask upgrade cairo VERSION
    ```
    where `VERSION` is the appropriate version.

    The `patch` section in `Cargo.toml` should be **empty** after doing it.
3. Push the changes, create a PR and verify if the CI passes.
4. If releasing for the first time, run:
    ```shell
    cargo login
    ```
    and follow terminal prompts to generate a token with at least `publish-update` permission.
5. Run
    ```shell
    cargo publish
    ```
    OR (if using multiple tokens for multiple crates):
    ```shell
    cargo publish --token <token>
    ```
    to publish the package.
6. Create a tag corresponding to the release. The naming convention is `v{VERSION}` e.g. `v2.12.0`.
