use axum::{routing::get, Router};
use mongodb::{bson::doc, Client, Database};
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Get database URL from environment
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    // Connect to MongoDB
    let client = Client::with_uri_str(&database_url).await.expect("Failed to connect to MongoDB");
    let database: Database = client.database("instanttexte");

    // Test connection by pinging
    match database.run_command(doc! { "ping": 1 }).await {
        Ok(_) => println!("Connected to MongoDB successfully!"),
        Err(e) => eprintln!("Failed to ping MongoDB: {}", e),
    }

    // Build the application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/test-db", get(test_db))
        .with_state(database);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello from Axum backend!"
}

async fn test_db(
    axum::extract::State(database): axum::extract::State<Database>,
) -> String {
    // Test by inserting a document
    let collection = database.collection::<mongodb::bson::Document>("test_collection");
    let doc = doc! { "message": "Hello from Rust backend!", "timestamp": mongodb::bson::DateTime::now() };

    match collection.insert_one(doc).await {
        Ok(result) => format!("Inserted document with ID: {:?}", result.inserted_id),
        Err(e) => format!("Failed to insert: {}", e),
    }
}
