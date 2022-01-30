use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ServerSettings {
    pub tcp: TcpServerSettings,
    #[cfg(unix)]
    pub unix_socket: UnixSocketServerSettings,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TcpServerSettings {
    pub enabled: bool,
    pub listen_address: IpAddr,
    pub port: PortSetting,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PortSetting {
    Fixed(u16),
    Range((u16, u16)),
}

impl Default for TcpServerSettings {
    fn default() -> Self {
        Self {
            enabled: cfg!(windows),
            listen_address: IpAddr::from([127, 0, 0, 1]),
            port: PortSetting::Range((13400, 13500)),
        }
    }
}

#[cfg(unix)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnixSocketServerSettings {
    pub enabled: bool,
}

#[cfg(unix)]
impl Default for UnixSocketServerSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}
