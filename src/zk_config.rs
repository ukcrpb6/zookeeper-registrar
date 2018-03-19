use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZkServiceRegistration {
  pub id: String,
  pub name: String,
  address: String,
  port: u16,
  #[serde(rename = "sslPort")]
  ssl_port: Option<String>,
  payload: Option<String>,
  #[serde(rename = "registrationTimeUTC")]
  registration_time_utc: u64,
  #[serde(rename = "serviceType")]
  service_type: String,
  #[serde(rename = "uriSpec")]
  uri_spec: UriSpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parts {
  value: String,
  variable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UriSpec {
  parts: Vec<Parts>,
}

impl Parts {
  fn define<S: ToString>(value: S, variable: bool) -> Parts {
    Parts {
      value: value.to_string(),
      variable: variable
    }
  }
}

impl ToString for ZkServiceRegistration {
  fn to_string(&self) -> String {
    match self.ssl_port {
      Some(ref port) => format!("https://{}:{}/{}", self.address, port, self.name),
      None => format!("http://{}:{}/{}", self.address, self.port, self.name),
    }
  }
}

impl ZkServiceRegistration {
  pub fn define<S: ToString, T: ToString>(name: S, address: T, port: u16) -> ZkServiceRegistration {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    ZkServiceRegistration {
      id: Uuid::new_v4().to_string(),
      name: name.to_string(),
      address: address.to_string(),
      port: port,
      ssl_port: None,
      payload: None,
      registration_time_utc: now.as_secs() * 1_000 + now.subsec_nanos() as u64 / 1_000_000,
      service_type: "DYNAMIC".to_owned(),
      uri_spec: UriSpec {
        parts: vec![
          Parts::define("scheme", true),
          Parts::define("://", false),
          Parts::define("address", true),
          Parts::define(":", false),
          Parts::define("port", true),
        ]
      }
    }
  }
}
