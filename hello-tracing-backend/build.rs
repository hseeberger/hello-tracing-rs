use anyhow::{Context, Result};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    vec,
};
use walkdir::WalkDir;

const PROTOS: &str = "proto";

fn main() -> Result<()> {
    let protos = list_protos(Path::new(PROTOS))?;
    tonic_build::configure()
        .build_client(false)
        .compile(&protos, &[PROTOS])
        .context("compile protos")
}

fn list_protos(dir: &Path) -> Result<Vec<PathBuf>> {
    WalkDir::new(dir)
        .into_iter()
        .try_fold(vec![], |mut protos, entry| {
            let entry = entry.context("read proto file")?;
            let path = entry.path();
            if path.extension().and_then(OsStr::to_str) == Some("proto") {
                protos.push(path.to_path_buf());
            }
            Ok(protos)
        })
}
