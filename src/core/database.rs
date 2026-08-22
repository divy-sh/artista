use std::sync::OnceLock;

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;

type Pool = r2d2::Pool<SqliteConnectionManager>;

#[derive(Debug)]
pub struct Database {
    pub pool: Pool,
}

static DB_INSTANCE: OnceLock<Database> = OnceLock::new();

impl Database {
    pub fn get_db() -> &'static Database {
        DB_INSTANCE.get_or_init(|| {
            let manager = SqliteConnectionManager::file("db.sqlite3");
            let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
            let db = Database { pool };

            _ = db.init_conversation_dao();

            db
        })
    }

    pub fn init_conversation_dao(&self) -> Result<()> {
        let conn = self.pool.get().expect("Failed to get connection from pool");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                image BLOB NOT NULL,
                lastUpdated TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn get_conn(&self) -> r2d2::PooledConnection<SqliteConnectionManager> {
        self.pool.get().expect("Database pool exhausted")
    }

    pub fn upsert_conversation(&self, id: String, title: String, image: Vec<u8>) -> Result<()> {
        let conn = self.get_conn();

        conn.execute(
            "INSERT INTO conversations (id, title, image, lastUpdated)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(id) DO UPDATE SET
                    title = excluded.title,
                    image = excluded.image,
                    lastUpdated = DATETIME('now')",
            rusqlite::params![id, title, image],
        )?;

        Ok(())
    }
}
