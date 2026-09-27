use std::fmt::Display;

use bytes::Bytes;
use futures_util::StreamExt;

use crate::{
    containers::bollard_wrapper::{
        Client, body_full,
        types::{DownloadFromContainerOptionsBuilder, UploadToContainerOptionsBuilder},
    },
    domain::{entities::FileEntry, errors::ContainerError},
};

/// Transfers tar archives between the host and a specific container.
pub struct TarTransfer {
    client: Client,
    container_id: String,
}

impl TarTransfer {
    pub fn new(client: Client, container_id: String) -> Self {
        Self {
            client,
            container_id,
        }
    }

    /// Packs file entries into an in-memory tar archive.
    pub fn pack(&self, entries: &[FileEntry]) -> Result<Vec<u8>, ContainerError> {
        let mut builder = tar::Builder::new(Vec::new());
        entries
            .iter()
            .try_for_each(|entry| self.append_entry(&mut builder, entry))?;
        self.finish_archive(builder)
    }

    fn append_entry(
        &self,
        builder: &mut tar::Builder<Vec<u8>>,
        entry: &FileEntry,
    ) -> Result<(), ContainerError> {
        let mut header = tar::Header::new_gnu();
        header.set_size(entry.content().len() as u64);
        header.set_mode(entry.mode());

        builder
            .append_data(&mut header, entry.path(), entry.content())
            .map_err(|error| self.copy_failed(&error))
    }

    fn finish_archive(&self, builder: tar::Builder<Vec<u8>>) -> Result<Vec<u8>, ContainerError> {
        builder
            .into_inner()
            .map_err(|error| self.copy_failed(&error))
    }

    /// Uploads a tar archive into the container at `container_path`.
    pub async fn upload(
        &self,
        container_path: &str,
        archive: Vec<u8>,
    ) -> Result<(), ContainerError> {
        let options = UploadToContainerOptionsBuilder::new()
            .path(container_path)
            .build();
        self.client
            .upload_to_container(
                &self.container_id,
                Some(options),
                body_full(Bytes::from(archive)),
            )
            .await
            .map_err(|error| self.copy_failed(&error))
    }

    /// Downloads the tar archive stored under `container_path` out of the
    /// container.
    pub async fn download(&self, container_path: &str) -> Result<Vec<u8>, ContainerError> {
        let options = DownloadFromContainerOptionsBuilder::new()
            .path(container_path)
            .build();
        let mut stream = self
            .client
            .download_from_container(&self.container_id, Some(options));
        let mut archive = Vec::new();
        while let Some(chunk) = stream.next().await {
            archive.extend_from_slice(&chunk.map_err(|error| self.copy_failed(&error))?);
        }
        Ok(archive)
    }

    /// Unpacks the file entries stored in a downloaded tar archive.
    pub fn unpack(&self, archive: &[u8]) -> Result<Vec<FileEntry>, ContainerError> {
        let mut archive = tar::Archive::new(archive);
        archive
            .entries()
            .map_err(|error| self.copy_failed(&error))?
            .map(|entry| self.read_entry(entry.map_err(|error| self.copy_failed(&error))?))
            .collect()
    }

    fn read_entry(&self, mut entry: tar::Entry<'_, &[u8]>) -> Result<FileEntry, ContainerError> {
        let path = entry
            .path()
            .map_err(|error| self.copy_failed(&error))?
            .to_string_lossy()
            .to_string();
        let mode = entry
            .header()
            .mode()
            .map_err(|error| self.copy_failed(&error))?;
        let mut content = Vec::new();
        std::io::Read::read_to_end(&mut entry, &mut content)
            .map_err(|error| self.copy_failed(&error))?;
        Ok(FileEntry::new(path, content, mode))
    }

    fn copy_failed(&self, error: &dyn Display) -> ContainerError {
        ContainerError::CopyFailed(self.container_id.clone(), error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::TarTransfer;
    use crate::{
        containers::bollard_wrapper::{API_DEFAULT_VERSION, Client},
        domain::entities::FileEntry,
    };

    #[test]
    fn pack_and_unpack_preserve_nested_repository_paths_over_one_hundred_bytes() {
        let client = Client::connect_with_http("http://127.0.0.1:2375", 120, API_DEFAULT_VERSION)
            .expect("container client");
        let transfer = TarTransfer::new(client, "container".to_string());
        let path = concat!(
            "crates/infrastructure/tests/infrastructure/persistence/project_branding/",
            "cargo_project_branding_store_tests.rs"
        );
        let entry = FileEntry::new(path, vec![], 0o644);

        let archive = transfer.pack(&[entry]).expect("archive");
        let entries = transfer.unpack(&archive).expect("unpacked entries");

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path(), path);
    }

    #[test]
    fn pack_accepts_paths_over_the_ustar_path_limit() {
        let client = Client::connect_with_http("http://127.0.0.1:2375", 120, API_DEFAULT_VERSION)
            .expect("container client");
        let transfer = TarTransfer::new(client, "container".to_string());
        let path = format!("{}file.txt", "segment/".repeat(32));
        let entry = FileEntry::new(path, vec![1, 2, 3], 0o644);

        let archive = transfer
            .pack(std::slice::from_ref(&entry))
            .expect("archive");
        let entries = transfer.unpack(&archive).expect("unpacked entries");

        assert_eq!(entries, vec![entry]);
    }
}
