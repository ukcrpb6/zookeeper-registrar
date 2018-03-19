#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
  pub services: Vec<Service>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Service {
  pub name: String,
  pub address: String,
  pub port: u16,
  #[serde(rename = "sslPort")]
  pub ssl_port: Option<u16>,
}
