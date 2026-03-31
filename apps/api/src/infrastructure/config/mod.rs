pub struct Settings {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://abaco:abaco@localhost:5432/abaco".to_string()),
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
        }
    }
}
