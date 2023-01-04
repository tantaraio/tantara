use schema::{create_schema, SchemaConfig};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::DocAddress;
use tantivy::IndexReader;
use tantivy::IndexWriter;
use tantivy::ReloadPolicy;
use tantivy::Result;

mod schema;

pub struct Registry {
    index: tantivy::Index,
    schema: Schema,
    fields: Vec<Field>,
    writer: IndexWriter,
    reader: IndexReader,
}

pub struct Input {
    schema: SchemaConfig,
}

pub fn create(input: Input) -> Result<Registry> {
    let mut schema_builder = Schema::builder();

    create_schema(&input.schema, &mut schema_builder);

    let fields = vec![];
    let schema = schema_builder.build();
    let index = tantivy::Index::create_in_ram(schema.clone());
    let writer = index.writer(50_000_000)?;
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    Ok(Registry {
        index,
        schema,
        fields,
        writer,
        reader,
    })
}

pub fn add(index: Registry, docs: Vec<Document>) -> Result<Registry> {
    docs.iter().try_for_each(|doc| -> Result<()> {
        index.writer.add_document(doc.clone())?;
        Ok(())
    })?;

    Ok(index)
}

pub fn commit(mut index: Registry) -> Result<Registry> {
    index.writer.commit()?;
    index.reader.reload()?;

    Ok(index)
}

pub fn remove(index: Registry, field: Field, text: &str) -> Result<Registry> {
    let term = Term::from_field_text(field, text);

    index.writer.delete_term(term);

    Ok(index)
}

pub fn search(
    index: Registry,
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
