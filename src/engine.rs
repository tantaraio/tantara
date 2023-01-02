use tantivy::schema::*;
use tantivy::IndexWriter;
use tantivy::ReloadPolicy;
use tantivy::Searcher;

pub struct Registry {
    writer: IndexWriter,
    searcher: Searcher,
}

pub fn create() -> tantivy::Result<Registry> {
    let schema_builder = Schema::builder();

    // TODO: add schema

    let schema = schema_builder.build();
    let index = tantivy::Index::create_in_ram(schema.clone());

    // index writer
    let writer = index.writer(50_000_000)?;

    // index reader
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;
    let searcher = reader.searcher();

    Ok(Registry { writer, searcher })
}

pub fn add() {}

pub fn remove() {}

pub fn search() {}
