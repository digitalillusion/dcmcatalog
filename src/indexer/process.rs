use std::sync::{Mutex, MutexGuard};

use fs_extra::dir::{get_dir_content2, DirOptions};
use log::{debug, error, info};
use rayon::prelude::*;
use tantivy::{
    doc,
    schema::{Field, Schema},
    Index,
};

use super::{dicom::read_dicom_file, IndexerError, IndexerStats};

fn update_indexer_stats<F>(indexer_stats: &Mutex<IndexerStats>, lambda: F)
where
    F: FnOnce(&mut MutexGuard<IndexerStats>),
{
    match indexer_stats.lock() {
        Ok(mut stats) => lambda(&mut stats),
        Err(e) => error!("Couldn't update indexer stats: {e}"),
    }
}

pub fn perform_indexing(index: &Index, path: String, depth: u64, schema: Schema) -> Result<IndexerStats, IndexerError> {
    let index_writer = Mutex::new(index.writer(50_000_000).map_err(IndexerError::InstanceWriter)?);
    let indexer_stats = Mutex::new(IndexerStats::default());

    let options = DirOptions { depth };
    let dir_content = get_dir_content2(path, &options).map_err(IndexerError::GetDirContent)?;

    let fields = schema.fields().map(|(field, _)| field).collect::<Vec<Field>>();
    dir_content
        .files
        .par_iter()
        .for_each(|file| match (read_dicom_file(file), index_writer.lock()) {
            (Ok(dicom_info), Ok(mut index_writer)) => {
                match index_writer.add_document(doc!(
                    fields[0] => dicom_info.patient_id.to_string(),
                    fields[1] => dicom_info.person_name.to_string(),
                    fields[2] => file.to_string()
                )) {
                    Ok(_) => match index_writer.commit() {
                        Ok(commited) => {
                            info!(
                                "Indexed: {0: <30} - Patient ID: {1: <10} - Person Name: {2: <20}",
                                file, dicom_info.patient_id, dicom_info.person_name
                            );
                            debug!("Committed {commited} documents")
                        }
                        Err(e) => debug!("Could not commit index, operation will be retried: {e}"),
                    },
                    Err(e) => {
                        error!("Error indexing {file}: {e}");
                        update_indexer_stats(&indexer_stats, |stats| {
                            stats.error_files.push(file.to_string());
                            stats.errors += 1
                        });
                    }
                }
            }
            (Err(e), _) => {
                error!("Error reading DICOM info from {file}: {e}");
                update_indexer_stats(&indexer_stats, |stats| {
                    stats.error_files.push(file.to_string());
                    stats.errors += 1
                });
            }
            (_, Err(e)) => {
                error!("Index writer unavailable, file {file} will be skipped: {e}");
                update_indexer_stats(&indexer_stats, |stats| {
                    stats.skipped_files.push(file.to_string());
                    stats.skipped += 1
                });
            }
        });

    index_writer
        .lock()
        .map_err(|e| IndexerError::CommitIndex(e.to_string()))
        .and_then(|mut writer| writer.commit().map_err(|e| IndexerError::CommitIndex(e.to_string())))?;

    indexer_stats.into_inner().map_err(IndexerError::GetStats)
}
