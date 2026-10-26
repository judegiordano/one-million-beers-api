pub mod controllers;
pub mod errors;
pub mod models;
pub mod types;

pub mod env {
    use tracing::Level;

    use crate::errors::AppError;

    #[derive(Debug, Clone)]
    pub struct Env {
        pub log_level: Level,
        pub mongo_uri: String,
        pub discord_token: String,
        pub discord_private_key: String,
    }

    impl Env {
        fn _get_optional_string(key: &str) -> Option<String> {
            std::env::var(key.trim().to_uppercase()).ok()
        }

        fn _get_required_string(key: &str) -> Result<String, AppError> {
            match std::env::var(key.trim().to_uppercase()) {
                Ok(value) => Ok(value),
                Err(err) => {
                    eprintln!("{key} not found: {err}");
                    Err(AppError::env_error(err))
                }
            }
        }

        pub fn log_level() -> Level {
            Self::_get_optional_string("LOG_LEVEL").map_or(Level::ERROR, |value| {
                match value.to_uppercase().as_str() {
                    "DEBUG" => Level::DEBUG,
                    "INFO" => Level::INFO,
                    "WARN" => Level::WARN,
                    "TRACE" => Level::TRACE,
                    _ => Level::ERROR,
                }
            })
        }

        pub fn load() -> Result<Self, AppError> {
            if cfg!(debug_assertions) {
                use dotenv::dotenv;
                dotenv().ok();
            }
            Ok(Self {
                log_level: Self::log_level(),
                mongo_uri: Self::_get_required_string("MONGO_URI")?,
                discord_token: Self::_get_required_string("DISCORD_TOKEN")?,
                discord_private_key: Self::_get_required_string("DISCORD_PRIVATE_KEY")?,
            })
        }
    }
}

pub mod logger {
    use tracing_subscriber::FmtSubscriber;

    use crate::{env::Env, errors::AppError};

    pub fn init() -> Result<(), AppError> {
        let Env { log_level, .. } = Env::load()?;
        let subscriber: FmtSubscriber = FmtSubscriber::builder().with_max_level(log_level).finish();
        tracing::subscriber::set_global_default(subscriber)
            .map_err(AppError::internal_server_error)?;
        Ok(())
    }
}
