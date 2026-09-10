use std::fmt::Display;

use bytes::Bytes;
use futures_util::StreamExt;

use crate::application::dtos::responses::FileEntryResponse;
use crate::domain::errors::ContainerError;
use crate::infrastructure::containers::bollard_wrapper::Client;
use crate::infrastructure::containers::bollard_wrapper::body_full;
use crate::infrastructure::containers::bollard_wrapper::types::DownloadFromContainerOptionsBuilder;
use crate::infrastructure::containers::bollard_wrapper::types::UploadToContainerOptionsBuilder;

/// Packs file entries into an in-memory tar archive.
pub(super) fn pack_entries(
    entries: &[FileEntryResponse],
    container_id: &str,
) -> Result<Vec<u8>, ContainerError> {
    let mut builder = tar::Builder::new(Vec::new());
    for entry in entries {
        append_entry(&mut builder, entry, container_id)?;
    }
    finish_archive(builder, container_id)
}

fn append_entry(
    builder: &mut tar::Builder<Vec<u8>>,
    entry: &FileEntryResponse,
    container_id: &str,
) -> Result<(), ContainerError> {
    let mut header = tar::Header::new_gnu();
    header
        .set_path(entry.path())
        .map_err(|error| copy_failed(container_id, error))?;
    header.set_size(entry.content().len() as u64);
    header.set_mode(entry.mode());
    header.set_cksum();

    builder
        .append(&header, entry.content())
        .map_err(|error| copy_failed(container_id, error))
}

fn finish_archive(
    builder: tar::Builder<Vec<u8>>,
    container_id: &str,
) -> Result<Vec<u8>, ContainerError> {
    builder
        .into_inner()
        .map_err(|error| copy_failed(container_id, error))
}

/// Uploads a tar archive into the container at `container_path`.
pub(super) async fn upload_archive(
    client: &Client,
    container_id: &str,
    container_path: &str,
    archive: Vec<u8>,
) -> Result<(), ContainerError> {
    let options = UploadToContainerOptionsBuilder::new()
        .path(container_path)
        .build();
    client
        .upload_to_container(container_id, Some(options), body_full(Bytes::from(archive)))
        .await
        .map_err(|error| copy_failed(container_id, error))
}

/// Downloads the tar archive stored under `container_path` out of the
/// container.
pub(super) async fn download_archive(
    client: &Client,
    container_id: &str,
    container_path: &str,
) -> Result<Vec<u8>, ContainerError> {
    let options = DownloadFromContainerOptionsBuilder::new()
        .path(container_path)
        .build();
    let mut stream = client.download_from_container(container_id, Some(options));
    let mut archive = Vec::new();
    while let Some(chunk) = stream.next().await {
        archive.extend_from_slice(&chunk.map_err(|error| copy_failed(container_id, error))?);
    }
    Ok(archive)
}

/// Unpacks the file entries stored in a downloaded tar archive.
pub(super) fn unpack_entries(
    archive: &[u8],
    container_id: &str,
) -> Result<Vec<FileEntryResponse>, ContainerError> {
    let mut unpacked = Vec::new();
    let mut archive = tar::Archive::new(archive);
    for entry in archive
        .entries()
        .map_err(|error| copy_failed(container_id, error))?
    {
        let entry = entry.map_err(|error| copy_failed(container_id, error))?;
        unpacked.push(read_entry(entry, container_id)?);
    }
    Ok(unpacked)
}

fn read_entry<R: std::io::Read>(
    mut entry: tar::Entry<'_, R>,
    container_id: &str,
) -> Result<FileEntryResponse, ContainerError> {
    let path = entry
        .path()
        .map_err(|error| copy_failed(container_id, error))?
        .to_string_lossy()
        .to_string();
    let mode = entry
        .header()
        .mode()
        .map_err(|error| copy_failed(container_id, error))?;
    let mut content = Vec::new();
    std::io::Read::read_to_end(&mut entry, &mut content)
        .map_err(|error| copy_failed(container_id, error))?;
    Ok(FileEntryResponse::new(path, content, mode))
}

fn copy_failed(container_id: &str, error: impl Display) -> ContainerError {
    ContainerError::CopyFailed(container_id.to_string(), error.to_string())
}
