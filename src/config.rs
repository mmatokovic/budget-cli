use config::ConfigError;

#[derive(Debug, serde::Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub database: Database,
}

#[derive(Debug, serde::Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    pub path: String,
}
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    
        let config_path = base_path.join(".config").join("default.toml");
    
        let settings = config::Config::builder()
            .add_source(config::File::from(config_path))
            .build()?;
    
        settings.try_deserialize::<Settings>()
    }
}
