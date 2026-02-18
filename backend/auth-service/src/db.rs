use mongodb::{options::IndexOptions, Client, Database, IndexModel};
use mongodb::bson::doc;

pub async fn get_database() -> mongodb::error::Result<Database> {
    let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client = Client::with_uri_str(&mongo_uri).await?;
    let db = client.database("webchat");
    
    // Create indexes for performance
    create_indexes(&db).await?;
    
    Ok(db)
}

async fn create_indexes(db: &Database) -> mongodb::error::Result<()> {
    // Users collection indexes
    let users = db.collection::<mongodb::bson::Document>("users");
    users.create_index(
        IndexModel::builder()
            .keys(doc! { "email": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build(),
        None
    ).await?;
    
    Ok(())
}
