use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

pub async fn connect() -> Result<Pool<Sqlite>, sqlx::Error> {
    const DB_URL: &str = "sqlite://db.sqlite";

    // Sqlite
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5) // Adjust maximum connections as needed
        .connect(DB_URL)
        .await?;

    Ok(pool)
}
