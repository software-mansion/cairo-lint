use std::path::{Path, PathBuf};

use anyhow::Result;
use which::which;

pub fn get_scarb_path() -> Result<&Path> {
    which("scarb")
        .map(|path| PathBuf::as_path(path.as_ref()))
        .map_err(|_| anyhow::anyhow!("`scarb` not found in `PATH`"))
    // .expect("running tests requires a `scarb` binary available in `PATH`")
}
