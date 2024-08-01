mod indexer;
mod output;
mod search;
mod types;

use std::time::Instant;

use clap::Parser;
use indexer::index_path;
use log::{info, LevelFilter};
use output::output_results;
use search::search_catalog;
use types::{Args, CommandError};

fn main() -> Result<(), CommandError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(LevelFilter::Info.as_str()))
        .filter(Some("tantivy"), log::LevelFilter::Off)
        .init();

    let args = Args::parse();

    let index_timer = Instant::now();
    let catalog =
        index_path(args.path, args.max_depth, args.disk_index, args.regenerate).map_err(CommandError::Indexing)?;

    if let Some(ref stats) = catalog.stats {
        info!(
            "Indexed {} files ({} skipped, {} errors) in {}s",
            catalog.searcher.num_docs(),
            stats.skipped,
            stats.errors,
            index_timer.elapsed().as_secs_f32()
        );
    }

    let search_timer = Instant::now();
    let results =
        search_catalog(&catalog, args.patient_id, args.person_name, args.offset, args.limit, args.search_sensibility)
            .map_err(CommandError::Search)?;

    info!(
        "Returned {}-{}/{} results in {}s",
        args.offset,
        args.offset + results.len(),
        catalog.searcher.num_docs(),
        search_timer.elapsed().as_secs_f32()
    );

    output_results(&catalog, results);

    Ok(())
}
