// Libs

use std::{fmt::Debug, process::exit, str::FromStr};

use tracing::error;

/**
A struct to manage the environment variables.
*/
#[derive(Debug)]
pub struct EnvManager;

impl EnvManager {
    /**
    A method to unsure all the required environment variables are set.
    */
    pub fn is_env_defined() -> bool {
        let required_env_vars = vec![
            "BOT_POKE_FLEE_TIME_SECS",
            "BOT_POKE_SPAWN_RATE",
            "BOT_POKE_SHINY_RATE",
            "BOT_POKEBALL_EMOJI_ID",
            "BOT_POKEDEX_POKES_PER_PAGE",
            "BOT_POKEDEX_TIMEOUT_SECS",
            "DATABASE_HOST",
            "DATABASE_NAMESPACE",
            "DATABASE_NAME",
            "DATABASE_USER",
            "DATABASE_PASS",
            "DISCORD_TOKEN",
            "POKEAPI_URL",
            "CACHE_HOST",
            "CACHE_NAMESPACE",
            "CACHE_USER",
            "CACHE_PASS",
        ];

        for var in required_env_vars {
            if std::env::var(var).is_err() {
                error!("Environment variable '{}' is not defined.", var);
                return false;
            }
        }

        true
    }

    /**
    A method to get the value of an environment variable.

    It'll panic if the environment variable is not defined or if the value can't be parsed.

    ## Parameters:
    - `key`: The key of the environment variable.
    */
    pub fn get_var<T>(key: &str) -> T
    where
        T: FromStr,
    {
        let value = match std::env::var(key) {
            Ok(value) => value,
            Err(e) => {
                error!("Error getting the environment variable '{}': {:?}", key, e);
                exit(1);
            }
        };

        match value.parse() {
            Ok(value) => value,
            Err(_) => {
                error!("Error parsing the environment variable '{}'", key);
                exit(1);
            }
        }
    }
}
