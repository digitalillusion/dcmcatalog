use std::{str::Utf8Error, sync::PoisonError};

use tantivy::{schema::Schema, Searcher};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Error getting directory content: {0}")]
    GetDirContent(fs_extra::error::Error),
    #[error("Error opening indexer on disk {0}")]
    OpenIndexerInDir(tantivy::TantivyError),
    #[error("Error creating indexer on disk {0}")]
    CreateIndexerInDir(tantivy::TantivyError),
    #[error("Error making directory {0} for indexer: {1}")]
    CreateIndexerMakeDir(String, std::io::Error),
    #[error("Error removing directory {0} for indexer: {1}")]
    CreateIndexerRemoveDir(String, std::io::Error),
    #[error("Error instancing writer: {0}")]
    InstanceWriter(tantivy::TantivyError),
    #[error("Error in the final index commit: {0}")]
    CommitIndex(String),
    #[error("Error producing indexer reader: {0}")]
    ProduceReader(tantivy::TantivyError),
    #[error("Error retrieving indexer stats: {0}")]
    GetStats(PoisonError<IndexerStats>),
}

#[derive(Error, Debug)]
pub enum DicomError {
    #[error("Error opening file {0}: {1}")]
    OpenFile(String, dicom::object::ReadError),
    #[error("Error retrieving element '{0}': {1}")]
    GetElement(String, dicom::object::AccessError),
    #[error("Error retrieving element '{0}' bytes: {1}")]
    GetElementBytes(String, dicom::core::value::ConvertValueError),
    #[error("Error retrieving element '{0}' value as UTF-8 string: {1}")]
    ToString(String, Utf8Error),
}

pub struct DicomInfo {
    pub person_name: String,
    pub patient_id: String,
}

#[derive(Default)]
pub struct IndexerStats {
    pub errors: u64,
    pub error_files: Vec<String>,
    pub skipped: u64,
    pub skipped_files: Vec<String>,
}

pub struct Catalog {
    pub schema: Schema,
    pub searcher: Searcher,
    pub stats: Option<IndexerStats>,
}
