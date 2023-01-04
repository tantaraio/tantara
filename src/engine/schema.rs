use tantivy::schema::{NumericOptions, SchemaBuilder, TextOptions};

struct FieldOptions {
    stored: bool,
    indexed: bool,
}

enum FieldType {
    Text,
    U64,
}

pub struct Field {
    name: String,
    field_type: FieldType,
    options: FieldOptions,
}

pub type SchemaConfig = Vec<Field>;

pub fn create_schema(config: &SchemaConfig, schema_builder: &mut SchemaBuilder) {
    config.iter().for_each(|field| {
        match field.field_type {
            FieldType::Text => {
                let options = get_text_options(&field.options);
                schema_builder.add_text_field(&field.name, options);
            }
            FieldType::U64 => {
                let options = get_u64_options(&field.options);
                schema_builder.add_u64_field(&field.name, options);
            }
        };
    })
}

fn get_text_options(config: &FieldOptions) -> TextOptions {
    let mut options = TextOptions::default();

    if config.stored {
        options = options.set_stored();
    }

    options
}

fn get_u64_options(config: &FieldOptions) -> NumericOptions {
    let mut options = NumericOptions::default();

    if config.stored {
        options = options.set_stored();
    }
    if config.indexed {
        options = options.set_indexed();
    }

    options
}
