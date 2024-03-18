use std::{env, sync::Arc};

use datafusion::{
    catalog::{schema::{MemorySchemaProvider, SchemaProvider}, CatalogProvider, MemoryCatalogProvider, MemoryCatalogProviderList}, datasource::{
        file_format::csv::CsvFormat,
        listing::{ListingOptions, ListingTable, ListingTableConfig, ListingTableUrl}, TableProvider,
    }, execution::{config::SessionConfig, context::{SessionContext, SessionState}}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let session_configuration = SessionConfig::new()
        .set_bool("datafusion.catalog.create_default_catalog_and_schema", false)
        .set_bool("datafusion.catalog.information_schema", true);

    let mut session_context = SessionContext::new_with_config(session_configuration);
    let session_state = Arc::new(session_context.state());

    let catalog_list_provider = Arc::new(MemoryCatalogProviderList::new());
    session_context.register_catalog_list(catalog_list_provider.clone());

    // Create the catalog/schema hierarchy
    let schema_provider = MemorySchemaProvider::new();

    // Add tables
    schema_provider.register_table("transactions".to_string(), get_table(session_state.clone(), "examples/data/transactions.csv".to_string()).await).unwrap();
    schema_provider.register_table("registrations".to_string(), get_table(session_state.clone(), "examples/data/registrations.csv".to_string()).await).unwrap();

    let catalog_provider = MemoryCatalogProvider::new();
    catalog_provider.register_schema("finance", Arc::new(schema_provider)).unwrap();

    session_context.register_catalog("project", Arc::new(catalog_provider));

    // Test
    let query = &args[1];
    let df = session_context.sql(&query).await?;
    df.show().await?;

    Ok(())
}

async fn get_table(session_state: Arc<SessionState>, table_path: String) -> Arc<dyn TableProvider> {
    let table_url = ListingTableUrl::parse(table_path).unwrap();

    let file_format = CsvFormat::default();
    let listing_options = ListingOptions::new(Arc::new(file_format)).with_file_extension(".csv");

    let resolved_table_schema = listing_options
        .infer_schema(&session_state, &table_url)
        .await.unwrap();

    let config = ListingTableConfig::new(table_url)
        .with_listing_options(listing_options)
        .with_schema(resolved_table_schema);

    let table = ListingTable::try_new(config).unwrap();
    let table_provider = Arc::new(table) as Arc<dyn TableProvider>;

    table_provider
}