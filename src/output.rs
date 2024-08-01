use log::info;
use rayon::prelude::*;
use tantivy::{
    schema::{Field, Value},
    DocAddress, Score, TantivyDocument,
};

use crate::indexer::Catalog;

fn output_field(doc: &TantivyDocument, field: Field) -> String {
    doc.get_first(field).and_then(|v| v.as_str()).unwrap_or("##ERROR##").to_string()
}

pub fn output_results(catalog: &Catalog, results: Vec<(Score, DocAddress)>) {
    info!("{0: <10} | {1: <10} | {2: <20} | {3: <30}", "Weight", "Patient ID", "Person Name", "File path");

    let fields = catalog.schema.fields().map(|(field, _)| field).collect::<Vec<Field>>();
    results
        .into_par_iter()
        .for_each(|(weight, doc)| match catalog.searcher.doc::<TantivyDocument>(doc) {
            Ok(doc) => {
                info!(
                    "{0: <10} | {1: <10} | {2: <20} | {3: <30}",
                    weight,
                    output_field(&doc, fields[0]),
                    output_field(&doc, fields[1]),
                    output_field(&doc, fields[2])
                );
            }
            Err(e) => info!("Error deserializing document: {e}"),
        });
}
