use dotenv::dotenv;
use std::fmt;
use url::Url;

// Define a custom error type
#[derive(Debug)]
pub enum ConfigError {
    EnvVarMissing(String),
    UrlParseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::EnvVarMissing(var) => {
                write!(f, "Environment variable '{var}' is missing")
            }
            ConfigError::UrlParseError(msg) => write!(f, "URL parsing error: {msg}"),
        }
    }
}

/// # Errors
///
/// Will return `ConfigError` if the required environment variables aren't set or the provided
/// link is unable to be parsed
pub fn as_struct() -> Result<Config, ConfigError> {
    dotenv().ok();

    Ok(Config {
        username: Username {
            github: std::env::var("GH_USERNAME")
                .map_err(|_| ConfigError::EnvVarMissing("GH_USERNAME".into()))?,
            lastfm: std::env::var("LASTFM_USERNAME")
                .map_err(|_| ConfigError::EnvVarMissing("LASTFM_USERNAME".into()))?,
            letterboxd: std::env::var("LETTERBOXD_USERNAME")
                .map_err(|_| ConfigError::EnvVarMissing("LETTERBOXD_USERNAME".into()))?,
        },
        link: Link {
            goodreads: Url::parse(
                &std::env::var("GOODREADS_SHELF")
                    .map_err(|_| ConfigError::EnvVarMissing("GOODREADS_SHELF".into()))?,
            )
            .map_err(|e| ConfigError::UrlParseError(e.to_string()))?,
        },
        key: Key {
            lastfm: std::env::var("LASTFM_KEY")
                .map_err(|_| ConfigError::EnvVarMissing("LASTFM_KEY".into()))?,
        },
    })
}

pub struct Config {
    pub username: Username,
    pub link: Link,
    pub key: Key,
}

pub struct Username {
    pub github: String,
    pub lastfm: String,
    pub letterboxd: String,
}

pub struct Link {
    pub goodreads: Url,
}

pub struct Key {
    pub lastfm: String,
}
