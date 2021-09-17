use std::env;

pub struct CommonConfig {
    pub port: String,
    pub db_url: String,
    pub redis_url: String,
}

impl CommonConfig {
    pub fn new() -> CommonConfig {
        let port = env::var("PORT").unwrap_or("7777".to_string());
        let db_url = env::var("DB_URL")
            .unwrap_or("postgres://postgres:postgres@localhost/vvvvv".to_string());
        let redis_url =
            env::var("REDIS_URL").unwrap_or("redis://:redis@127.0.0.1:6379".to_string());
        return CommonConfig {
            port,
            db_url,
            redis_url,
        };
    }
}
