#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub fn resolve_config() -> ServerConfig {
    ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 4042,
    }
}
