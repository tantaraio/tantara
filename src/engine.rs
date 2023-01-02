use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::DocAddress;
use tantivy::IndexReader;
use tantivy::IndexWriter;
use tantivy::ReloadPolicy;
use tantivy::Result;

pub struct Index {
    index: tantivy::Index,
    schema: Schema,
    fields: Vec<Field>,
    writer: IndexWriter,
    reader: IndexReader,
}

pub fn create() -> Result<Index> {
    let schema_builder = Schema::builder();

    // @TODO: create schema
    // @TODO: create fields from schema input

    let fields = vec![];
    let schema = schema_builder.build();
    let index = tantivy::Index::create_in_ram(schema.clone());
    let writer = index.writer(50_000_000)?;
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    Ok(Index {
        index,
        schema,
        fields,
        writer,
        reader,
    })
}

pub fn add(index: Index, docs: Vec<Document>) -> Result<Index> {
    docs.iter().try_for_each(|doc| -> Result<()> {
        index.writer.add_document(doc.clone())?;
        Ok(())
    })?;

    Ok(index)
}

pub fn commit(mut index: Index) -> Result<Index> {
    index.writer.commit()?;
    index.reader.reload()?;

    Ok(index)
}

pub fn remove(index: Index, field: Field, text: &str) -> Result<Index> {
    let term = Term::from_field_text(field, text);

    index.writer.delete_term(term);

    Ok(index)
}

pub fn search(
    index: Index,
    query: &str,
    limit: Option<usize>,
) -> tantivy::Result<Vec<(f32, DocAddress)>> {
    let limit = limit.unwrap_or(10);
    let parser = QueryParser::for_index(&index.index, index.fields);
    let query = parser.parse_query(query)?;
    let searcher = index.reader.searcher();
    let result = searcher.search(&query, &TopDocs::with_limit(limit))?;

    Ok(result)
}
