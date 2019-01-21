pub type App = clap::App<'static, 'static>;

pub type CliResult = Result<(), CliError>;

#[derive(Debug, failure::Fail)]
pub enum CliError {
    #[fail(display = "{}", error)]
    ClapError { error: clap::Error },

    #[fail(display = "{}", error)]
    IOError { error: std::io::Error },

    #[fail(display = "{}", error)]
    TomlError { error: toml::de::Error },

    #[fail(display = "{}", error)]
    YamlError { error: serde_yaml::Error },

    #[fail(display = "{}", error)]
    FailError { error: failure::Error },

    #[fail(display = "Unsupported extension")]
    UnsupportedExtension,

    #[fail(display = "Invalid repository")]
    InvalidRepository,
}

impl From<clap::Error> for CliError {
    fn from(error: clap::Error) -> Self {
        CliError::ClapError { error }
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        CliError::IOError { error }
    }
}

impl From<toml::de::Error> for CliError {
    fn from(error: toml::de::Error) -> Self {
        CliError::TomlError { error }
    }
}

impl From<serde_yaml::Error> for CliError {
    fn from(error: serde_yaml::Error) -> Self {
        CliError::YamlError { error }
    }
}

impl From<failure::Error> for CliError {
    fn from(error: failure::Error) -> Self {
        CliError::FailError { error }
    }
}
