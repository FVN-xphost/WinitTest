#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub volume: i32,
    pub text_speed: i32,
    pub is_fullscreen: bool,
    #[serde(skip, default)]
    sqlite_connection: Option<rusqlite::Connection>,
}

impl Config {
    pub fn new(path: String) -> Option<Self> {
        Some(Self {
            volume: 0,
            text_speed: 0,
            is_fullscreen: true,
            sqlite_connection: rusqlite::Connection::open(path).ok(),
        })
    }
}
