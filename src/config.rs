use config::{ConfigError, File};

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
    pub fn config() -> Result<Self, ConfigError> {
    
        let settings = config::Config::builder()
            .add_source(File::with_name(".config/default.toml"))
            .build()?;
        
        settings.try_deserialize::<Settings>()
    }
}
