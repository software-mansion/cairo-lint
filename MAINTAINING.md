# Cairo-lint Maintenance

## Release procedure

To release a new version of cairo-lint:

1. Make sure all the `cairo-lang-*` dependencies are set to a version appropriate for your release.
   You can use the following command:
    ```shell
    cargo xtask upgrade cairo VERSION
    ```
    where `VERSION` is the appropriate version.

    The `patch` section in `Cargo.toml` should be **empty** after doing it.
2. Push this version to remote, create a PR and verify if the CI passes.
3. Run
    ```shell
    cargo publish
    ```
    to publish the package.
4. Create a tag corresponding to the release. The naming convention is `v{VERSION}` e.g. `v2.12.0`.
