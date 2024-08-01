mod types;

use tantivy::{
    collector::TopDocs,
    query::{AllQuery, DisjunctionMaxQuery, FuzzyTermQuery, QueryClone},
    schema::Field,
    DocAddress, Score, Term,
};
pub use types::SearchError;

use crate::indexer::Catalog;

pub fn search_catalog(
    catalog: &Catalog,
    patient_id: Option<String>,
    patient_name: Option<String>,
    offset: usize,
    limit: Option<u64>,
    search_sensibility: u8,
) -> Result<Vec<(Score, DocAddress)>, SearchError> {
    let limit = limit.unwrap_or(catalog.searcher.num_docs());
    let collector = TopDocs::with_limit(limit.try_into().unwrap_or(usize::MAX)).and_offset(offset);
    let fields = catalog.schema.fields().map(|(field, _)| field).collect::<Vec<Field>>();
    match (patient_id, patient_name) {
        (Some(patient_id), Some(patient_name)) => {
            let query1 =
                Box::new(FuzzyTermQuery::new(Term::from_field_text(fields[0], &patient_id), search_sensibility, true));
            let query2 = Box::new(FuzzyTermQuery::new(
                Term::from_field_text(fields[1], &patient_name),
                search_sensibility,
                true,
            ));

            let queries = vec![query1.box_clone(), query2.box_clone()];
            let disjunction = DisjunctionMaxQuery::new(queries);

            Ok(catalog.searcher.search(&disjunction, &collector)?)
        }
        (Some(patient_id), None) => {
            let query = FuzzyTermQuery::new(Term::from_field_text(fields[0], &patient_id), search_sensibility, true);
            Ok(catalog.searcher.search(&query, &collector)?)
        }
        (None, Some(patient_name)) => {
            let query = FuzzyTermQuery::new(Term::from_field_text(fields[1], &patient_name), search_sensibility, true);
            Ok(catalog.searcher.search(&query, &collector)?)
        }
        (None, None) => Ok(catalog.searcher.search(&AllQuery, &collector)?),
    }
}
