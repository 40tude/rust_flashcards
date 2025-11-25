use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn create_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let manager = SqliteConnectionManager::file(database_url)
        .with_flags(OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE);

    let pool = Pool::builder()
        .max_size(5)
        .build(manager)?;

    tracing::info!("Database connection pool created for: {}", database_url);

    Ok(pool)
}