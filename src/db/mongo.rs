use dotenv::dotenv;
use mongodb::{error::Result as MongoResult, options::ClientOptions, Client, Database};

pub struct Mongo;

impl Mongo {
    pub async fn init() -> MongoResult<Database> {
        dotenv().ok();

        let dbms = std::env::var("DBMS").expect("No DBMS env variable");
        let db_name = std::env::var("DB_NAME").expect("No DB_NAME env variable");
        let host = std::env::var("DB_HOST").expect("No DB_HOST env variable");
        let port = std::env::var("DB_PORT").unwrap_or("27017".into());

        let fqdm = format!("{dbms}://{host}:{port}");

        let client_options = ClientOptions::parse(fqdm).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(db_name.as_str());
        Ok(db)
    }
}
