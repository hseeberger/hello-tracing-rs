use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use serde::Deserialize;
use std::env;
use thiserror::Error;

const CONFIG_FILE: &str = "CONFIG_FILE";

pub trait ConfigExt
where
    Self: for<'de> Deserialize<'de>,
{
    /// Load the configuration from the file at the value of the `CONFIG_FILE` environment variable
    /// or `config.yaml` by default, with an overlay provided by environment variables prefixed with
    /// `"APP__"` and split/nested via `"__"`.
    fn load() -> Result<Self, Error> {
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

/// Possible errors when loading the configuration.
#[derive(Debug, Error)]
#[error("cannot load configuration")]
pub struct Error(#[from] figment::Error);

#[cfg(test)]
mod tests {
    use crate::config::ConfigExt;
    use assert_matches::assert_matches;
    use serde::Deserialize;
    use std::env;

    #[test]
    fn test_load() {
        env::set_var("APP__FOO__BAZ", 666.to_string());

        let config = Config::load();
        assert_matches!(
            config,
            Ok(Config {
                foo: Foo { bar, baz }
            }) if bar == "bar" && baz == 666
        );
    }

    #[derive(Debug, Deserialize)]
    struct Config {
        foo: Foo,
    }

    #[derive(Debug, Deserialize)]
    struct Foo {
        bar: String,
        baz: u64,
    }
}
