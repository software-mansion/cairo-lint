use std::{
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use anyhow::{Context, Result};
use cairo_lang_filesystem::db::CORELIB_CRATE_NAME;
use indoc::indoc;
use scarb_metadata::{Metadata, MetadataCommand};
use semver::Version;
use tempfile::tempdir;
use which::which;

pub const SCARB_TOML: &str = "Scarb.toml";

struct UnmanagedCore {
    path: PathBuf,
    version: Version,
}

fn get_scarb_path() -> Result<PathBuf> {
    which("scarb").map_err(|_| anyhow::anyhow!("`scarb` not found in `PATH`"))
    // .expect("running tests requires a `scarb` binary available in `PATH`")
}

/// Calls `scarb metadata` on an empty Scarb package to find the `core` package.
fn get_scarb_metadata(manifest: &Path) -> Result<Metadata> {
    let scarb_path = get_scarb_path()?;

    MetadataCommand::new()
        .scarb_path(scarb_path)
        .manifest_path(manifest)
        .inherit_stderr()
        .exec()
        .context("failed to execute: scarb metadata")
}

/// Try to find a Scarb-managed `core` package if we have Scarb toolchain.
///
/// The easiest way to do this is to create an empty Scarb package and run `scarb metadata` on it.
/// The `core` package will be a component of this empty package.
/// For minimal packages, `scarb metadata` should be pretty fast.
pub fn find_scarb_managed_core() -> Option<PathBuf> {
    let lookup = || {
        let workspace = tempdir()
            .context("failed to create temporary directory")
            .inspect_err(|e| eprintln!("{e:?}"))
            .ok()?;

        let scarb_toml = workspace.path().join(SCARB_TOML);
        fs::write(
            &scarb_toml,
            indoc! {r#"
              [package]
              name = "cairo_lint_unmanaged_core_lookup"
              version = "1.0.0"
          "#},
        )
        .context("failed to write Scarb.toml")
        .inspect_err(|e| eprintln!("{e:?}"))
        .ok()?;

        let metadata = get_scarb_metadata(&scarb_toml)
            .inspect_err(|e| eprintln!("{e:?}"))
            .ok()?;

        // Ensure the workspace directory is deleted after running Scarb.
        // We are ignoring the error, leaving doing proper clean-up to the OS.
        let _ = workspace
            .close()
            .context("failed to wipe temporary directory")
            .inspect_err(|e| eprintln!("{e:?}"));

        // Scarb is expected to generate only one compilation unit (for our stub package)
        // that will consist of this package and the `core` crate.
        // Therefore, we allow ourselves to liberally just look for any first usage of a package
        // named `core` in all compilation units components we got.
        let path = metadata
            .compilation_units
            .into_iter()
            .find_map(|compilation_unit| {
                compilation_unit
                    .components
                    .iter()
                    .find(|component| component.name == CORELIB_CRATE_NAME)
                    .map(|component| component.source_root().to_path_buf().into_std_path_buf())
            })?;

        let version = metadata
            .packages
            .into_iter()
            .find(|package| package.name == CORELIB_CRATE_NAME)
            .map(|package| package.version)?;

        Some(path)
    };

    static CACHE: OnceLock<Option<PathBuf>> = OnceLock::new();
    CACHE.get_or_init(lookup).clone()
}
