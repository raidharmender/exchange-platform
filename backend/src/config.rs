use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    #[cfg(feature = "database")]
    pub database: DatabaseConfig,
    #[cfg(feature = "database")]
    pub redis: RedisConfig,
    #[cfg(feature = "database")]
    pub jwt: JwtConfig,
    #[cfg(feature = "database")]
    pub cors: CorsConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[cfg(feature = "database")]
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[cfg(feature = "database")]
#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
}

#[cfg(feature = "database")]
#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: u64,
}

#[cfg(feature = "database")]
#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        #[cfg(feature = "database")]
        {
            let config = config::Config::builder()
                .set_default("server.host", "0.0.0.0")?
                .set_default("server.port", 8080)?
                .set_default("database.max_connections", 10)?
                .set_default("database.min_connections", 2)?
                .set_default("jwt.expiration", 86400)?
                .set_default("cors.allowed_origins", vec!["*"])?
                .set_default("cors.allowed_methods", vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])?
                .set_default("cors.allowed_headers", vec!["*"])?
                .add_source(config::Environment::default().separator("__"))
                .build()?;

            config.try_deserialize()
        }

        #[cfg(not(feature = "database"))]
        {
            let config = config::Config::builder()
                .set_default("server.host", "0.0.0.0")?
                .set_default("server.port", 8080)?
                .add_source(config::Environment::default().separator("__"))
                .build()?;

            config.try_deserialize()
        }
    }
}

impl From<config::Config> for Config {
    fn from(config: config::Config) -> Self {
        #[cfg(feature = "database")]
        {
            Config {
                server: ServerConfig {
                    host: config.get_string("server.host").unwrap_or_else(|_| "0.0.0.0".to_string()),
                    port: config.get_int("server.port").unwrap_or(8080) as u16,
                },
                database: DatabaseConfig {
                    url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                    max_connections: config.get_int("database.max_connections").unwrap_or(10) as u32,
                    min_connections: config.get_int("database.min_connections").unwrap_or(2) as u32,
                },
                redis: RedisConfig {
                    url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
                },
                jwt: JwtConfig {
                    secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
                    expiration: config.get_int("jwt.expiration").unwrap_or(86400) as u64,
                },
                cors: CorsConfig {
                    allowed_origins: config.get_array("cors.allowed_origins")
                        .unwrap_or_else(|_| vec![config::Value::from("*")])
                        .into_iter()
                        .filter_map(|v| v.into_string().ok())
                        .collect(),
                    allowed_methods: config.get_array("cors.allowed_methods")
                        .unwrap_or_else(|_| vec![config::Value::from("GET"), config::Value::from("POST")])
                        .into_iter()
                        .filter_map(|v| v.into_string().ok())
                        .collect(),
                    allowed_headers: config.get_array("cors.allowed_headers")
                        .unwrap_or_else(|_| vec![config::Value::from("*")])
                        .into_iter()
                        .filter_map(|v| v.into_string().ok())
                        .collect(),
                },
            }
        }

        #[cfg(not(feature = "database"))]
        {
            Config {
                server: ServerConfig {
                    host: config.get_string("server.host").unwrap_or_else(|_| "0.0.0.0".to_string()),
                    port: config.get_int("server.port").unwrap_or(8080) as u16,
                },
            }
        }
    }
} 