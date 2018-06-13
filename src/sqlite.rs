use std::path::Path;
use std::error::Error;

extern crate rusqlite;
use self::rusqlite::Connection;
use self::rusqlite::OpenFlags;

use repository::Repository;

pub struct Sqlite {
    conn: Option<Connection>
}

impl Sqlite {
    fn init_db(&mut self) {
        if let Some(ref conn) = self.conn {
            conn.execute(queries::CREATE_SQL, &[]).expect("Could not initialize backing store.");
        }
    }

    pub fn connected(&self) -> bool {
        self.conn.is_some()
    }
}

impl Repository for Sqlite {
    fn new() -> Sqlite {
        Sqlite { conn: None }
    }

    /// Initializes an `Sqlite` backed repository at the given location.
    fn init(&mut self, location: &str) -> Result<bool, Box<Error>> {
        if location == "in-memory" { // Test-mode
            self.conn = Some(Connection::open_in_memory()?);
            self.init_db()
        }

        let p = Path::new(location);
        if p.exists() {
            self.conn = Some(Connection::open_with_flags(p, OpenFlags::SQLITE_OPEN_READ_WRITE)?);
            Ok(false)
        } else {
            self.conn = Some(Connection::open(p)?);
            self.init_db();
            Ok(true)
        }
    }
}


/// Private module containing SQL format strings.
mod queries {
    /// The SQL query to retrieve database metadata
    pub const GET_META_SQL: &str = "
        PRAGMA foreign_keys = ON;
        SELECT * FROM meta WHERE magic = 'CTF' LIMIT 1;
    ";

    /// The SQL query to create a new database.
    pub const CREATE_SQL: &str = "
        PRAGMA foreign_keys = ON;

        CREATE TABLE meta (
            magic   CHAR(3) NOT NULL,
            version INTEGER NOT NULL
        );
        INSERT INTO meta VALUES ('CTF', 1);

        CREATE TABLE users (
            id     INTEGER PRIMARY KEY,
            handle VARCHAR(30) NOT NULL UNIQUE,
            access INTEGER NOT NULL DEFAULT 1,
            added  TEXT NOT NULL DEFAULT date('now'),
        );

        CREATE TABLE events (
            id    INTEGER PRIMARY KEY,
            name  VARCHAR(50) NOT NULL,
            url   TEXT NOT NULL,
            start TEXT NOT NULL,
            end   TEXT NOT NULL,
        );

        CREATE TABLE challenges (
            id  INTEGERR PRIMARY KEY,
            cid INTEGER NOT NULL,
            eid INTEGER,
            --
            points      INTEGER NOT NULL,
            name        VARCHAR(255),
            url         VARCHAR(1024) NOT NULL,
            notes       TEXT,
            flag        VARCHAR(255),
            added       TEXT NOT NULL DEFAULT date('now'),
            --
            FOREIGN KEY(eid) REFERENCES events(id) ON DELETE SET NULL,
        );

        CREATE TABLE submissions (
            id INTEGER PRIMARY KEY,
            cid INTEGER NOT NULL,
            --
            flag VARCHAR(255),
            date TEXT NOT NULL DEFAULT date('now'),
            valid INTEGER NOT NULL DEFAULT 0,
            --
            FOREIGN KEY(cid) REFERENCES challenges(id) ON DELETE CASCADE,
        );
    ";
}

extern crate tempfile;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_sqlite_new_db() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("db.sqlite");

        let mut sql = Sqlite::new();
        let r = sql.init(db.to_str().unwrap());
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), true);
        assert!(sql.connected());
    }

    #[test]
    fn init_sqlite_existing_db() {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("db.sqlite");
    }

    #[test]
    fn init_sqlite_invalid_db() {
    }
}

