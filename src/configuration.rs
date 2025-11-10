use crate::prelude::*;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSetting,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSetting {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSetting {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::new("configuration.yaml", FileFormat::Yaml))
        .build()?;

    settings.try_deserialize::<Settings>()
}
