use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use serde::Deserialize;
use std::env;

const CONFIG_FILE: &str = "CONFIG_FILE";

/// Extension methods for "configuration structs" which can be deserialized.
pub trait ConfigExt
where
    Self: for<'de> Deserialize<'de>,
{
    /// Load the configuration from the file at the value of the `CONFIG_FILE` environment variable
    /// or `config.yaml` by default, with an overlay provided by environment variables prefixed with
    /// `"APP__"` and split/nested via `"__"`.
    fn load() -> Result<Self, figment::Error> {
        let config_file = env::var(CONFIG_FILE)
            .map(Yaml::file_exact)
            .unwrap_or(Yaml::file_exact("config.yaml"));

        let config = Figment::new()
            .merge(config_file)
            .merge(Env::prefixed("APP__").split("__"))
            .extract()?;

        Ok(config)
    }
}

impl<T> ConfigExt for T where T: for<'de> Deserialize<'de> {}

#[cfg(test)]
mod tests {
    use crate::{
        config::{ConfigExt, CONFIG_FILE},
        telemetry,
    };
    use assert_matches::assert_matches;
    use serde::Deserialize;
    use std::env;

    #[test]
    fn test_load() {
        env::set_var("APP__FOO", "foo");

        let config = Config::load();
        assert_matches!(
            config,
            Ok(Config {
                foo,
                telemetry_config: telemetry::Config {
                    tracing_config: telemetry::TracingConfig {
                        enabled,
                        ..
                    }
                },
            }) if foo == "foo" && !enabled
        );

        env::set_var(CONFIG_FILE, "nonexistent.yaml");
        let config = Config::load();
        assert!(config.is_err());
    }

    #[derive(Debug, Deserialize)]
    struct Config {
        foo: String,

        #[serde(rename = "telemetry")]
        telemetry_config: telemetry::Config,
    }
}
