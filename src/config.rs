use crate::{error::Error, proxy::Proxies};
use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    /// Debug model
    pub debug: bool,

    /// Server bind address
    pub bind: SocketAddr,

    /// Forward timeout (seconds)
    pub timeout: u64,

    /// Forward connect timeout (seconds)
    pub connect_timeout: u64,

    /// Forward TCP keepalive (seconds)
    pub tcp_keepalive: Option<u64>,

    /// Server Enforces a limit on the concurrent number of requests the underlying
    pub concurrent: usize,

    /// Upstream proxy, support multiple proxy
    /// Type: interface/proxy/cidr
    pub proxies: Vec<Proxies>,

    /// TLS certificate file path
    pub tls_cert: Option<PathBuf>,

    /// TLS private key file path (EC/PKCS8/RSA)
    pub tls_key: Option<PathBuf>,

    /// Authentication Key
    pub api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            bind: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080),
            timeout: 60,
            connect_timeout: 10,
            tcp_keepalive: Some(90),
            concurrent: 100,
            proxies: Default::default(),
            tls_cert: Default::default(),
            tls_key: Default::default(),
            api_key: Default::default(),
        }
    }
}

pub fn generate_template(path: PathBuf) -> crate::Result<()> {
    // Check if the output is a directory
    if path.is_dir() {
        return Err(Error::IOError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{} is a directory", path.display()),
        )));
    }

    // Check if the parent directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let write = |out: PathBuf| -> crate::Result<()> {
        #[cfg(target_family = "unix")]
        {
            use std::{fs::Permissions, os::unix::prelude::PermissionsExt};
            std::fs::File::create(&out)?.set_permissions(Permissions::from_mode(0o755))?;
        }

        #[cfg(target_family = "windows")]
        std::fs::File::create(&out)?;

        let yaml_config = serde_yaml::to_string(&Config::default())?;

        std::fs::write(out, yaml_config).map_err(Into::into)
    };

    if !path.exists() {
        write(path)?;
    }
    Ok(())
}
