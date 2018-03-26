extern crate ctrlc;
extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate uuid;
extern crate zookeeper;

use std::io::{BufReader};
use std::time::Duration;
use std::fs::File;
use std::thread;
use std::sync::Arc;

use docopt::Docopt;
use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZooKeeper};

mod config;
use config::Config;
use config::Service;

mod zk_config;
use zk_config::ZkServiceRegistration;

const USAGE: &'static str = "
Zookeeper Registrar

Registers provided service fixtures in zookeeper.

Usage:
  zookeeper-registrar [options] <path>

Options:
  -z --zookeeper=<address>  Zookeeper connection string [default: localhost:2181].
  -h --help                 Show this screen.
  --version                 Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
  arg_path: String,
  flag_zookeeper: String,
}

struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("{:?}", e)
    }
}

fn main() {
  // apply default rust log level if not configured
  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "zookeeper_registrar=info");
  }

  env_logger::Builder::from_default_env()
    .default_format_timestamp(false)
    .init();

  // parse CLI arguments
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  let instance = ZooKeeper::connect(&args.flag_zookeeper, Duration::from_secs(15), LoggingWatcher).unwrap();

  info!("Connected to ZooKeeper on {}", args.flag_zookeeper);

  match read_config(args.arg_path).and_then(|config| register(&instance, config.services)) {
    Ok(_) => {
      // add sigterm handler to unpark thread to terminate
      let arc_thr = Arc::new(thread::current());
      let thr = arc_thr.clone();
      ctrlc::set_handler(move || thr.unpark()).expect("sigterm handler");

      thread::park(); // park the thread until killed
    },
    Err(e) => error!("{:?}", e),
  };
}

// read service configuration
fn read_config(path: String) -> Result<Config, String> {
  let file = File::open(path).map_err(|e| e.to_string())?;
  let reader = BufReader::new(file);

  match serde_json::from_reader(reader) {
    Ok(config) => Ok(config),
    Err(e) => Err(e.to_string())
  }
}

// register services in zookeeper
fn register(instance: &ZooKeeper, services: Vec<Service>) -> Result<(), String> {
  for service in services {
    let data = ZkServiceRegistration::define(&service.name, &service.address, service.port);

    let path_str = &format!("services/{}", &service.name);

    let mut path = "".to_owned();
    for component in path_str.split('/') {
      path = format!("{}/{}", path, component);
      if instance.exists(&path, false).map_err(|e| e.to_string())?.is_none() {
        info!("Creating path {}", path);
        instance.create(
          &path,
          vec![],
          Acl::open_unsafe().clone(),
          CreateMode::Persistent
        ).map_err(|e| e.to_string())?;
      }
    }

    instance.create(
      &format!("/services/{}/{}", &service.name, data.id),
      serde_json::to_vec(&data).unwrap(),
      Acl::open_unsafe().clone(),
      CreateMode::Ephemeral
    ).map_err(|e| e.to_string())?;

    info!("Registered {}", data.to_string());
  }
  Ok(())
}
