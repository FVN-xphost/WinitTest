use rusqlite::{Connection, params};

pub struct Config {
    pub volume: i32,
    pub text_speed: i32,
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    pub is_fullscreen: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            volume: 100,
            text_speed: 100,
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            is_fullscreen: true,
        }
    }
    pub fn load(&mut self) -> Result<(), rusqlite::Error> {
        let path = crate::utils::path::CONFIG_LOCAL_DIR.get().unwrap();
        if !path.exists() {
            let _ = std::fs::create_dir_all(&path);
        }
        let mut conn = Connection::open(path.join("data.db"))?;
        conn.execute(
            r#"
CREATE TABLE IF NOT EXISTS config (
    name TEXT PRIMARY KEY,
    value TEXT
);
        "#,
            [],
        )?;
        let tx = conn.transaction()?;
        tx.execute(
            r#"INSERT OR IGNORE INTO config (name, value) VALUES (?1, ?2)"#,
            params!["volume", &(self.volume.to_string())],
        )?;
        tx.execute(
            r#"INSERT OR IGNORE INTO config (name, value) VALUES (?1, ?2)"#,
            params!["text_speed", &(self.text_speed.to_string())],
        )?;
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        tx.execute(
            r#"INSERT OR IGNORE INTO config (name, value) VALUES (?1, ?2)"#,
            params![
                "is_fullscreen",
                &(if self.is_fullscreen { "1" } else { "0" })
            ],
        )?;
        tx.commit()?;
        let mut stmt = conn.prepare("SELECT value FROM config WHERE name = ?1")?;
        let volume: String = stmt.query_one(params!["volume"], |row| row.get(0))?;
        let text_speed: String = stmt.query_one(params!["text_speed"], |row| row.get(0))?;
        self.volume = volume.parse::<i32>().unwrap_or(100);
        self.text_speed = text_speed.parse::<i32>().unwrap_or(100);
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            let is_fullscreen: String =
                stmt.query_one(params!["is_fullscreen"], |row| row.get(0))?;
            self.is_fullscreen = is_fullscreen == "1";
        }
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), rusqlite::Error> {
        let path = crate::utils::path::CONFIG_LOCAL_DIR.get().unwrap();
        if !path.exists() {
            let _ = std::fs::create_dir_all(&path);
        }
        let mut conn = Connection::open(path.join("data.db"))?;
        conn.execute(
            r#"
CREATE TABLE IF NOT EXISTS config (
    name TEXT PRIMARY KEY,
    value TEXT
);
        "#,
            [],
        )?;
        let tx = conn.transaction()?;
        tx.execute(
            r#"INSERT OR IGNORE INTO config (name, value) VALUES (?1, ?2)"#,
            params!["volume", &(self.volume.to_string())],
        )?;
        tx.execute(
            r#"INSERT OR IGNORE INTO config (name, value) VALUES (?1, ?2)"#,
            params!["text_speed", &(self.text_speed.to_string())],
        )?;
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        tx.execute(
            r#"INSERT OR IGNORE INTO config (name, value) VALUES (?1, ?2)"#,
            params![
                "is_fullscreen",
                &(if self.is_fullscreen { "1" } else { "0" })
            ],
        )?;
        tx.commit()?;
        let tx = conn.transaction()?;
        tx.execute(
            r#"UPDATE config SET value = ?2 WHERE name = ?1"#,
            params!["volume", &(self.volume.to_string())],
        )?;
        tx.execute(
            r#"UPDATE config SET value = ?2 WHERE name = ?1"#,
            params!["text_speed", &(self.text_speed.to_string())],
        )?;
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        tx.execute(
            r#"UPDATE config SET value = ?2 WHERE name = ?1"#,
            params![
                "is_fullscreen",
                &(if self.is_fullscreen { "1" } else { "0" })
            ],
        )?;
        tx.commit()?;
        Ok(())
    }
}
