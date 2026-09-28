use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Writes content through a uniquely named sibling file and atomically replaces the target.
pub(crate) fn write(path: &Path, contents: impl AsRef<[u8]>) -> io::Result<()> {
    let temporary_path = temporary_path(path);
    fs::write(&temporary_path, contents)
        .and_then(|_| fs::rename(&temporary_path, path))
        .inspect_err(|_| {
            let _ = fs::remove_file(&temporary_path);
        })
}

fn temporary_path(path: &Path) -> PathBuf {
    path.with_extension(format!("tmp-{}", uuid::Uuid::new_v4()))
}

#[cfg(test)]
mod atomic_write_tests;
