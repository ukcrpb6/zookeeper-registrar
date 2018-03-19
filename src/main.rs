extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate zookeeper;

use std::io;
use std::time::Duration;

use docopt::Docopt;

use zookeeper::{Watcher, WatchedEvent, ZooKeeper};

mod zk_node;
use zk_node::ZkNode;

const USAGE: &'static str = "
Zookeeper-Sync

Usage:
  zookeeper-sync <source>

Options:
  -h --help  Show this screen.
  --version  Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
  arg_source: String
}

struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("{:?}", e)
    }
}

fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  let master = ZooKeeper::connect(&args.arg_source, Duration::from_secs(15), LoggingWatcher).unwrap();
  let slave = ZooKeeper::connect("localhost:2181", Duration::from_secs(15), LoggingWatcher).unwrap();

  ZkNode::read_node(&master, "/services")
    .sync(&slave)
    .unwrap();

  let mut tmp = String::new();
  println!("press enter to exit example");
  io::stdin().read_line(&mut tmp).unwrap();
}
