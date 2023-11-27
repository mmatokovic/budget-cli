#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub path: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    
    let config_path = base_path.join(".config").join("bud.toml");

    let settings = config::Config::builder()
        .add_source(config::File::from(config_path))
        .build()?;

    settings.try_deserialize::<Settings>()
}
