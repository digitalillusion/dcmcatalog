use clap::Parser;
use thiserror::Error;

use crate::{indexer::IndexerError, search::SearchError};

#[derive(Parser, Debug)]
#[command(version, about, long_about = Some("
DICOM stands for “Digital Imaging and Communications in Medicine” and is the most common open standard for medical images. 
Each DICOM file contains a set of metadata about the patient, clinic, and acquisition, followed by the pixel data or contents of the acquisition itself.
Hospitals often collect all their DICOM files into a single large, shared drive. The goal of this command-line utility is to catalog the patients in such a drive, and help users identify files related to a specific patient for further analysis. A large clinic may store millions of files with paths such as “drive/1234/001/0001234.dcm”, making it difficult to locate the right files
without a catalog.

The CLI utility should:
  1. Locate DICOM files under a specified path.
  2. Print each file's DICOM patient name and patient ID.
"))]
pub struct Args {
    /// Path to scan for DICOM files
    #[arg(required = true)]
    pub path: String,

    /// Value to search for in the PatientId tag
    #[arg(short = 'i', long)]
    pub patient_id: Option<String>,

    /// Value to search for in the PersonName tag
    #[arg(short = 'n', long)]
    pub person_name: Option<String>,

    /// Depth of search into subdirectories
    #[arg(short, long, default_value_t = 20)]
    pub max_depth: u64,

    /// The index of the first result to return
    #[arg(short, long, default_value_t = 0)]
    pub offset: usize,

    /// The count of results to return
    #[arg(short, long)]
    pub limit: Option<u64>,

    /// Index to the specified directory, useful for very large dataset that won't otherwise fit into memory
    #[arg(short, long)]
    pub disk_index: Option<String>,

    /// Force regeneration of disk index. Ignored if indexing is done in memory
    #[arg(short, long)]
    pub regenerate: bool,

    /// The maximum Levenshtein distance (number of different letters) under which the query still matches. Max is 2
    #[arg(short, long, default_value_t = 2)]
    pub search_sensibility: u8,
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Error indexing files: {0}")]
    Indexing(IndexerError),
    #[error("Error searching result: {0}")]
    Search(SearchError),
}
