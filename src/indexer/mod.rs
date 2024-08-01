mod dicom;
mod process;
mod types;

use std::path::Path;

use log::info;
use process::perform_indexing;
use tantivy::{
    schema::{Schema, STORED, TEXT},
    Index,
};
pub use types::{Catalog, IndexerError, IndexerStats};

pub fn index_path(
    path: String,
    depth: u64,
    disk_index: Option<String>,
    regenerate: bool,
) -> Result<Catalog, IndexerError> {
    let mut schema_builder = Schema::builder();
    let _ = schema_builder.add_text_field("patient_id", TEXT | STORED);
    let _ = schema_builder.add_text_field("person_name", TEXT | STORED);
    let _ = schema_builder.add_text_field("file_path", TEXT | STORED);
    let schema = schema_builder.build();

    let (index, indexer_stats) = match disk_index {
        Some(disk_index_location) => {
            let index_run_location = format!("{}/{}", disk_index_location, urlencoding::encode(&path));
            let indexer_run_path = Path::new(&index_run_location);
            if regenerate {
                std::fs::remove_dir_all(indexer_run_path)
                    .map_err(|e| IndexerError::CreateIndexerRemoveDir(index_run_location.to_string(), e))?;
            }
            if indexer_run_path.exists() {
                info!("Opening disk index at location {index_run_location}");
                let indexer = Index::open_in_dir(index_run_location).map_err(IndexerError::OpenIndexerInDir)?;
                (indexer, None)
            } else {
                info!("Creating disk index at location {index_run_location}");
                std::fs::create_dir_all(indexer_run_path)
                    .map_err(|e| IndexerError::CreateIndexerMakeDir(index_run_location.to_string(), e))?;
                let indexer = Index::create_in_dir(index_run_location, schema.clone())
                    .map_err(IndexerError::CreateIndexerInDir)?;
                let indexer_stats = perform_indexing(&indexer, path, depth, schema.clone())?;
                (indexer, Some(indexer_stats))
            }
        }
        None => {
            let indexer = Index::create_in_ram(schema.clone());
            let indexer_stats = perform_indexing(&indexer, path, depth, schema.clone())?;
            (indexer, Some(indexer_stats))
        }
    };

    let reader = index.reader().map_err(IndexerError::ProduceReader)?;

    let searcher = reader.searcher();

    Ok(Catalog { schema, searcher, stats: indexer_stats })
}
