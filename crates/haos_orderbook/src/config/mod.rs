#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub reboot_enabled: bool,
    pub reboot_auth_required: bool,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn allow_reboot(&self) -> bool {
        self.reboot_enabled
    }

    pub fn requires_auth(&self) -> bool {
        self.reboot_auth_required
    }
}

pub fn resolve_config() -> ServerConfig {
    ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 4042,
        reboot_enabled: true,
        reboot_auth_required: true,
    }
}
