use std::env;

pub struct CommonConfig {
    pub port: String,
}

impl CommonConfig {
    pub fn new() -> CommonConfig {
        let port = env::var("PORT").unwrap_or("7777".to_string());
        return CommonConfig { port };
    }
}
