use crate::prelude::*;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSetting,
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u32,
    pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseSetting {
    pub username: String,
    pub password: SecretBox<String>,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSetting {
    pub fn connection_string(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.db_name
        )))
    }

    pub fn connection_string_without_db(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        )))
    }
}

pub enum Environment {
    Production,
    Local,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "development",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \n Use either 'local' or 'production'.",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let base_path = env::current_dir().expect("Failed to determine the current directory.");
    let config_path = base_path.join("config");

    // Detect the running environment.
    // Default to 'local' if not specified.
    let environment: Environment = env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let config = Config::builder()
        .add_source(config::File::from(config_path.join("base.yaml")))
        .add_source(config::File::from(config_path.join(environment_filename)))
        .build()?;

    config.try_deserialize::<Settings>()
}
