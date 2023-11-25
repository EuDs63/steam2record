use config::{Config, ConfigError, File};

#[derive(Debug)] 
pub struct AppConfig {
    pub neodb_token: String,
}

impl AppConfig {
    pub fn from_file(file_path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(file_path))
            .build()?;
        
        let neodb_token = config.get_string("neodb_token")?;

        Ok(Self {
            neodb_token
        })
    }
}