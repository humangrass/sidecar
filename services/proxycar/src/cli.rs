use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom path to config file
    #[arg(short, long, value_name = "CONFIG_FILE", default_value = "proxycar.config.yaml")]
    pub config: PathBuf,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_arguments() {
        let args = Cli::try_parse_from(&["test-app"]).unwrap();
        assert_eq!(args.config, PathBuf::from("template.config.yaml"));
    }

    #[test]
    fn test_custom_arguments() {
        let args = Cli::try_parse_from(&[
            "test-app",
            "--config", "custom_config.yaml",
        ]).unwrap();

        assert_eq!(args.config, PathBuf::from("custom_config.yaml"));
    }
}