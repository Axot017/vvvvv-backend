use std::env;

pub struct CommonConfig {
    pub port: String,
    pub db_url: String,
}

impl CommonConfig {
    pub fn new() -> CommonConfig {
        let port = env::var("PORT").unwrap_or("7777".to_string());
        let db_url = env::var("DB_URL")
            .unwrap_or("postgres://postgres:postgres@localhost/vvvvv".to_string());
        return CommonConfig { port, db_url };
    }
}
