use thiserror::Error;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Error during search {0}")]
    Searcher(#[from] tantivy::error::TantivyError),
}
