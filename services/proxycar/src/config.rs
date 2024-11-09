use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpConfig {
    pub listen_port: String,
    pub target_service: TargetServiceConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TargetServiceConfig {
    pub host: String,
    pub port: u16,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TracingConfig {
    pub collector_endpoint: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SidecarConfig {
    pub http: HttpConfig,
    pub tracing: TracingConfig,
}

impl SidecarConfig {
    pub(crate) fn new(file_path: &Path) -> anyhow::Result<Self> {
        let mut file = File::open(file_path)
            .map_err(|err| anyhow!("Can't open file {:?}: {}", file_path, err))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| anyhow!("Can't read {:?}: {}", file_path, err))?;
        let config = serde_yaml::from_str(&contents)
            .map_err(|err| anyhow!("Can't read yaml {:?}: {}", file_path, err))?;
        Ok(config)
    }
}
