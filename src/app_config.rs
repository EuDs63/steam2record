use config::{Config, ConfigError, File};

#[derive(Debug)] 
pub struct AppConfig {
    // pub enable_api: i64,
    pub neodb_enable: bool,
    pub neodb_token: String,
    pub bangumi_enable: bool,
    pub bangumi_token: String,
}

impl AppConfig {
    pub fn from_file(file_path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(file_path))
            .build()?;
        
        //let enable_api:i64 = config.get_int("enable_api")?;
        let neodb_enable = config.get_bool("neodb_enable")?;
        let neodb_token = config.get_string("neodb_token")?;
        let bangumi_enable = config.get_bool("bangumi_enable")?;
        let bangumi_token = config.get_string("bangumi_token")?;

        Ok(Self {
            // enable_api,
            neodb_enable,
            neodb_token,
            bangumi_enable,
            bangumi_token
        })
    }
}